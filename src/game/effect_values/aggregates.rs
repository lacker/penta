// Shared reductions of resolved object collections.

impl Game {
    /// Project live characteristics before reducing a resolved collection.
    pub(in crate::game) fn aggregate_object_values(
        &self,
        objects: Vec<Target>,
        select: crate::card::ObjectValueDef,
        operation: crate::card::AggregateOperationDef,
    ) -> i32 {
        let values = objects.into_iter().filter_map(|target| {
            let id = match target {
                Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => id,
                Target::Player(_) => return None,
            };
            match select {
                crate::card::ObjectValueDef::ManaSymbols(color) => {
                    Some(i32::from(self.object_mana_symbol_count(id, color)))
                }
                crate::card::ObjectValueDef::ManaValue => {
                    self.current_or_last_known_mana_value(id).map(i32::from)
                }
                crate::card::ObjectValueDef::Power => {
                    self.current_or_last_known_power(id).map(i32::from)
                }
                crate::card::ObjectValueDef::Toughness => {
                    self.current_or_last_known_toughness(id).map(i32::from)
                }
                crate::card::ObjectValueDef::Counters(kind) => {
                    Some(i32::from(self.current_or_last_known_counters(id, kind)))
                }
            }
        });
        match operation {
            crate::card::AggregateOperationDef::Minimum => values.min().unwrap_or(0),
            crate::card::AggregateOperationDef::Maximum => values.max().unwrap_or(0),
            crate::card::AggregateOperationDef::Sum => values.fold(0_i32, i32::saturating_add),
        }
    }
}
