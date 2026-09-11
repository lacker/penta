use crate::{CardCatalog, CardDefinitionId};

/// Resolve a saved natural name in the source card's cost namespace, never in
/// a global registry of unrelated cards' bindings.
pub(in crate::game::state_checkpoint) fn restore_alternative_cost_binding(
    name: Option<&str>,
    definition: Option<CardDefinitionId>,
    catalog: &CardCatalog,
) -> Result<Option<String>, String> {
    let Some(name) = name else {
        return Ok(None);
    };
    let declared = definition
        .and_then(|key| catalog.get(key))
        .is_some_and(|card| {
            card.parts.iter().any(|part| {
                part.rules.ability_clauses().iter().any(|ability| {
                    matches!(
                        ability.definition,
                        crate::DeclarativeAbilityDef::AlternativeCast(cost)
                            if cost.binding.and_then(crate::Binding::label) == Some(name)
                    )
                })
            })
        });
    if declared {
        Ok(Some(name.to_owned()))
    } else {
        Err(format!(
            "unknown alternative-cost binding {name} for its source card"
        ))
    }
}
