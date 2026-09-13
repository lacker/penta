fn reflexive_trigger_definition(
    ability: &crate::card::AbilityDef,
) -> Result<crate::card::TriggeredAbilityDef, GrantedAbilityValidationError> {
    let invalid = || GrantedAbilityValidationError::UnsupportedEffectProgramContext {
        context: "ReflexiveTrigger",
        operation: "requires an ordinary, nonmodal reflexive ability without listener restrictions",
    };
    let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
        return Err(invalid());
    };
    if definition.event != TriggerEventDef::Reflexive
        || definition.procedure != AbilityProcedureDef::Shared
        || definition.source_zones != [ZoneKind::Battlefield]
        || definition.condition.is_some()
        || definition.modes.is_some()
        || definition.trigger_limit.is_some()
        || definition.resolves_with_illegal_targets
        || ability.declarative_effect().is_none()
    {
        return Err(invalid());
    }
    Ok(definition)
}

fn validate_reflexive_trigger_references(
    ability: &crate::card::AbilityDef,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    let definition = reflexive_trigger_definition(ability)?;
    validate_target_definitions(definition.targets)?;
    // Bindings carry the completed action's result across the stack boundary;
    // target slots belong exclusively to the new ability.
    validate_program_references(ability.effect.definition, definition.targets.len(), scope)
}
