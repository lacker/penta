// Minimal object combinations used by semantic spell costs.
impl Game {
    /// Every way to reach `total` mana value that wastes nothing: a set
    /// counts only if dropping any one of its cards would leave it short.
    /// The rules permit exiling more than that, but every superset is a
    /// strictly worse payment of the same cost, and enumerating them all
    /// would grow the action list exponentially in the size of a graveyard.
    fn mana_value_combinations(
        &self,
        candidates: &[GameObjectId],
        total: u16,
    ) -> Vec<Vec<GameObjectId>> {
        let values = candidates
            .iter()
            .map(|id| {
                self.card_in_nonbattlefield_zone(*id)
                    .and_then(|(_, card)| self.catalog.get(card.definition))
                    .map_or(0, |definition| {
                        definition.rules.printed_mana_cost().mana_value()
                    })
            })
            .collect::<Vec<_>>();
        let mut payments = Vec::new();
        for size in 1..=candidates.len() {
            for combination in Self::object_combinations(candidates, size) {
                let sum = combination
                    .iter()
                    .map(|id| {
                        candidates
                            .iter()
                            .position(|candidate| candidate == id)
                            .map_or(0, |index| values[index])
                    })
                    .fold(0_u16, u16::saturating_add);
                if sum < total {
                    continue;
                }
                let minimal = combination.iter().all(|id| {
                    let value = candidates
                        .iter()
                        .position(|candidate| candidate == id)
                        .map_or(0, |index| values[index]);
                    sum.saturating_sub(value) < total
                });
                if minimal {
                    payments.push(combination);
                }
            }
        }
        payments
    }

    /// Every way to reach `types` distinct card types between the chosen
    /// cards, minimal in the same sense the mana-value search is: a set
    /// counts only if dropping any one of its cards would leave it short.
    fn card_type_combinations(
        &self,
        candidates: &[GameObjectId],
        types: u16,
    ) -> Vec<Vec<GameObjectId>> {
        let sets = candidates
            .iter()
            .map(|id| {
                self.card_in_nonbattlefield_zone(*id)
                    .and_then(|(_, card)| self.catalog.get(card.definition))
                    .map_or_else(crate::card::CardTypeSet::empty, |definition| {
                        definition.rules.types()
                    })
            })
            .collect::<Vec<_>>();
        let union = |combination: &[GameObjectId]| {
            combination
                .iter()
                .filter_map(|id| candidates.iter().position(|candidate| candidate == id))
                .fold(crate::card::CardTypeSet::empty(), |seen, index| {
                    seen.union(sets[index])
                })
        };
        let mut payments = Vec::new();
        for size in 1..=candidates.len() {
            for combination in Self::object_combinations(candidates, size) {
                if union(&combination).count() < types {
                    continue;
                }
                let minimal = combination.iter().all(|dropped| {
                    let without = combination
                        .iter()
                        .copied()
                        .filter(|id| id != dropped)
                        .collect::<Vec<_>>();
                    union(&without).count() < types
                });
                if minimal {
                    payments.push(combination);
                }
            }
        }
        payments
    }

    /// Every `size`-element combination of `candidates`, in candidate order.
    /// An empty requirement has exactly one payment: the empty one.
    pub(in crate::game) fn object_combinations(
        candidates: &[GameObjectId],
        size: usize,
    ) -> Vec<Vec<GameObjectId>> {
        if size == 0 {
            return vec![Vec::new()];
        }
        if candidates.len() < size {
            return Vec::new();
        }
        let mut combinations = Vec::new();
        for (index, candidate) in candidates.iter().enumerate() {
            for mut rest in Self::object_combinations(&candidates[index + 1..], size - 1) {
                let mut combination = vec![*candidate];
                combination.append(&mut rest);
                combinations.push(combination);
            }
        }
        combinations
    }
}
