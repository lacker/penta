// Compatibility for older kicker/offspring clauses that store a complete total.
// Replacing the printed mana cost must leave their additional part payable.
impl Game {
    fn permission_additional_alternative_mana(
        &self,
        card: GameObjectId,
        option: &PlayOptionDef,
        configuration: &CostConfiguration,
    ) -> Option<ManaCost> {
        let selected = configuration.alternative()?;
        let (_, instance) = self.card_in_nonbattlefield_zone(card)?;
        let definition = self.catalog.get(instance.definition)?;
        let (_, _, kind) = Self::alternative_cast_clause(definition, option, selected)?;
        if !matches!(
            kind,
            AlternativeCastKindDef::Kicked | AlternativeCastKindDef::Offspring
        ) {
            return None;
        }
        let total = option
            .alternative_costs
            .iter()
            .find(|cost| cost.id == selected)?
            .mana_cost;
        let printed = option.mana_cost?;
        let mut additional = ManaCost {
            generic: total.generic.checked_sub(printed.generic)?,
            white: total.white.checked_sub(printed.white)?,
            blue: total.blue.checked_sub(printed.blue)?,
            black: total.black.checked_sub(printed.black)?,
            red: total.red.checked_sub(printed.red)?,
            green: total.green.checked_sub(printed.green)?,
            colorless: total.colorless.checked_sub(printed.colorless)?,
            ..ManaCost::default()
        };
        for (index, count) in additional.hybrid.iter_mut().enumerate() {
            *count = total.hybrid[index].checked_sub(printed.hybrid[index])?;
        }
        for (index, count) in additional.additional_flexible.iter_mut().enumerate() {
            *count =
                total.additional_flexible[index].checked_sub(printed.additional_flexible[index])?;
        }
        additional.x_multiplier = total.x_multiplier.checked_sub(printed.x_multiplier)?;
        additional.variable_x = additional.x_multiplier != 0;
        Some(additional)
    }
}
