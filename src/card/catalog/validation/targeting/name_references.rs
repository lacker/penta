fn validate_card_name_set_references(
    names: CardNameSetDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match names {
        CardNameSetDef::Union(sets) => sets
            .iter()
            .copied()
            .try_for_each(|names| validate_card_name_set_references(names, target_count, scope)),
        CardNameSetDef::NamesOf(objects)
        | CardNameSetDef::NamesAppearingAtLeast { objects, .. } => {
            validate_object_set_target_references(*objects, target_count, scope)
        }
        CardNameSetDef::AllCardNames
        | CardNameSetDef::NonlandCardNames
        | CardNameSetDef::LandCardNames
        | CardNameSetDef::NonbasicLandCardNames
        | CardNameSetDef::CardNamesOtherThanBasicLands
        | CardNameSetDef::BasicLandNames => Ok(()),
    }
}

fn validate_subtype_references(
    subtype: crate::card::SubtypeDef,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    if let crate::card::SubtypeDef::Binding(binding) = subtype
        && (binding == crate::ParentBinding || scope.binding_bit(binding, false)?.is_some())
    {
        return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
            context: "subtype binding",
            operation: "requires a durable source creature-type binding",
        });
    }
    Ok(())
}
