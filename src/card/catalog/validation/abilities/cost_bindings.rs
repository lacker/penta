// Cost names form a separate namespace from effect-output bindings. A card
// part declares the namespace before any of its ability references are checked.

fn validate_alternative_cost_bindings(
    definition: &CardDefinition,
    part: CardPartId,
    abilities: &[AbilityDef],
) -> Result<Vec<crate::Binding>, CatalogError> {
    let mut bindings = Vec::new();
    for (index, ability) in abilities.iter().enumerate() {
        let DeclarativeAbilityDef::AlternativeCast(alternative) = ability.definition else {
            continue;
        };
        let Some(binding) = alternative.binding else {
            continue;
        };
        let reason = if binding.label().is_none() {
            Some("a cost binding must have a durable name")
        } else if bindings.contains(&binding) {
            Some("more than one alternative cost declares this name in the card part")
        } else {
            None
        };
        if let Some(reason) = reason {
            return Err(CatalogError::InvalidAlternativeCostBinding {
                definition: definition.id,
                part,
                ability: AbilityId::from_index(index).expect("ability count was validated"),
                binding,
                reason,
            });
        }
        bindings.push(binding);
    }
    Ok(bindings)
}
