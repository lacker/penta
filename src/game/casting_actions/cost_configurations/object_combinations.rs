// Minimal object combinations used by semantic spell costs.
impl Game {
    /// Every minimal way to make a composed value over the chosen objects
    /// reach its threshold. Supersets are legal but strictly worse payments,
    /// so omitting them keeps graveyard-sized action lists bounded.
    fn object_set_value_combinations(
        &self,
        candidates: &[GameObjectId],
        requirement: crate::card::ObjectSetValueAtLeastDef,
    ) -> Vec<Vec<GameObjectId>> {
        let mut payments = Vec::new();
        for size in 1..=candidates.len() {
            for combination in Self::object_combinations(candidates, size) {
                if self.object_set_value(&combination, requirement.value) < requirement.minimum {
                    continue;
                }
                let minimal = combination.iter().all(|dropped| {
                    let without = combination
                        .iter()
                        .copied()
                        .filter(|id| id != dropped)
                        .collect::<Vec<_>>();
                    self.object_set_value(&without, requirement.value) < requirement.minimum
                });
                if minimal {
                    payments.push(combination);
                }
            }
        }
        payments
    }

    fn object_set_value(
        &self,
        objects: &[GameObjectId],
        value: crate::card::ObjectSetValueDef,
    ) -> u16 {
        match value {
            crate::card::ObjectSetValueDef::CardTypeCount => objects
                .iter()
                .filter_map(|id| self.card_in_nonbattlefield_zone(*id))
                .filter_map(|(_, card)| self.catalog.get(card.definition))
                .fold(crate::card::CardTypeSet::empty(), |seen, definition| {
                    seen.union(definition.rules.types())
                })
                .count(),
            crate::card::ObjectSetValueDef::Aggregate { select, operation } => {
                let values = objects.iter().map(|id| match select {
                    crate::card::ObjectValueDef::ManaValue => {
                        self.current_or_last_known_mana_value(*id).unwrap_or(0)
                    }
                    crate::card::ObjectValueDef::Power => self
                        .current_or_last_known_power(*id)
                        .unwrap_or(0)
                        .max(0)
                        .cast_unsigned(),
                    crate::card::ObjectValueDef::Toughness => self
                        .current_or_last_known_toughness(*id)
                        .unwrap_or(0)
                        .max(0)
                        .cast_unsigned(),
                });
                match operation {
                    crate::card::AggregateOperationDef::Minimum => values.min().unwrap_or(0),
                    crate::card::AggregateOperationDef::Maximum => values.max().unwrap_or(0),
                    crate::card::AggregateOperationDef::Sum => {
                        values.fold(0_u16, u16::saturating_add)
                    }
                }
            }
        }
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
