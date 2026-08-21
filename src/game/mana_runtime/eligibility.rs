//! Which mana abilities the runtime is willing to offer at all: the costs it
//! knows how to pay without a window in which to ask, and the bounds that
//! keep an offered ability from being an unbounded loop.

use super::super::{AbilityCostDef, ActivatedAbilityDef, CounterKind, Game, Permanent, ZoneKind};

impl Game {
    pub(in crate::game) fn mana_ability_is_usable(
        &self,
        permanent: &Permanent,
        definition: ActivatedAbilityDef,
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
            && !(taps_source && (permanent.tapped || !self.can_use_tap_ability(permanent)))
            && definition
                .costs
                .iter()
                .all(|cost| Self::mana_ability_cost_is_supported(definition, cost))
            && definition.costs.iter().all(|cost| match cost {
                AbilityCostDef::Mana(cost) => self.pool_covers_cost(permanent.controller, *cost),
                AbilityCostDef::PayLife(amount) => {
                    self.players[permanent.controller.index()].life
                        >= i16::try_from(*amount).unwrap_or(i16::MAX)
                }
                AbilityCostDef::RemoveCountersFromSource { .. }
                | AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_)
                | AbilityCostDef::TapSource
                | AbilityCostDef::UntapSource
                | AbilityCostDef::SacrificeSource
                | AbilityCostDef::SacrificeObject(_)
                | AbilityCostDef::ReturnSourceToHand
                | AbilityCostDef::DiscardSource
                | AbilityCostDef::DiscardCards(_)
                | AbilityCostDef::DiscardCardMatching(_)
                | AbilityCostDef::DiscardCardsAtRandom(_)
                | AbilityCostDef::SacrificePermanent { .. }
                | AbilityCostDef::SacrificePermanents { .. }
                | AbilityCostDef::ReturnUnblockedAttackerToHand
                | AbilityCostDef::TapPermanent { .. }
                | AbilityCostDef::ExileSource
                | AbilityCostDef::Loyalty(_)
                | AbilityCostDef::ExileCardsFromGraveyard { .. }
                | AbilityCostDef::Special(_) => true,
            })
            && Self::source_counter_costs_are_payable(permanent, definition.costs.as_slice())
    }

    /// The counter kind an ability lets the payer remove any number of, if
    /// it has such a cost. At most one: two open-ended sizes in one cost
    /// would be two questions with one answer.
    pub(in crate::game) fn variable_counter_removal(
        definition: ActivatedAbilityDef,
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
    /// also why hybrid and {X} are excluded -- both would need a choice the
    /// activation has no room to carry.
    pub(in crate::game) fn mana_ability_cost_is_supported(
        definition: ActivatedAbilityDef,
        cost: &AbilityCostDef,
    ) -> bool {
        match cost {
            AbilityCostDef::TapSource
            | AbilityCostDef::SacrificeSource
            | AbilityCostDef::ReturnSourceToHand
            | AbilityCostDef::ExileSource
            | AbilityCostDef::RemoveCountersFromSource { .. }
            | AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_)
            // Sacrificing another permanent spends the board just as surely
            // as spending the source does, so it bounds the ability the same
            // way. Which permanent is a choice, and it is answered by
            // enumerating one activation per candidate.
            | AbilityCostDef::SacrificePermanent { .. }
                        | AbilityCostDef::SacrificePermanents { .. }
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
                                | AbilityCostDef::SacrificeSource
                                | AbilityCostDef::ReturnSourceToHand
                                | AbilityCostDef::ExileSource
                                | AbilityCostDef::SacrificePermanent { .. }
                        | AbilityCostDef::SacrificePermanents { .. }
                        )
                    });
                !mana.variable_x && mana.hybrid.iter().all(|count| *count == 0) && bounded
            }
            AbilityCostDef::UntapSource
            | AbilityCostDef::SacrificeObject(_)
            | AbilityCostDef::DiscardSource
            | AbilityCostDef::DiscardCards(_)
            | AbilityCostDef::DiscardCardMatching(_)
            | AbilityCostDef::DiscardCardsAtRandom(_)
            | AbilityCostDef::ReturnUnblockedAttackerToHand
                | AbilityCostDef::TapPermanent { .. }
            | AbilityCostDef::Loyalty(_)
            | AbilityCostDef::ExileCardsFromGraveyard { .. }
            | AbilityCostDef::Special(_) => false,
        }
    }

    pub(in crate::game) fn source_counter_costs_are_payable(
        permanent: &Permanent,
        costs: &[AbilityCostDef],
    ) -> bool {
        let mut required = [0_u32; CounterKind::COUNT];
        for cost in costs {
            if let AbilityCostDef::RemoveCountersFromSource { kind, amount } = cost {
                required[kind.index()] = required[kind.index()].saturating_add(u32::from(*amount));
            }
        }
        CounterKind::ALL
            .into_iter()
            .all(|kind| u32::from(permanent.counters(kind)) >= required[kind.index()])
    }
}
