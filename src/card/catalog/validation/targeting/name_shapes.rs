fn validate_card_name_shape(
    name: CardNameDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match name {
        CardNameDef::NameOf(reference) => validate_object_reference_shape(reference, targets),
        CardNameDef::Literal(_) | CardNameDef::Binding(_) => Ok(()),
    }
}

fn validate_card_name_set_shape(
    names: CardNameSetDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match names {
        CardNameSetDef::Union(sets) => sets
            .iter()
            .copied()
            .try_for_each(|names| validate_card_name_set_shape(names, targets)),
        CardNameSetDef::NamesOf(objects)
        | CardNameSetDef::NamesAppearingAtLeast { objects, .. } => {
            validate_object_set_shape(*objects, targets)
        }
        CardNameSetDef::AllCardNames
        | CardNameSetDef::NonlandCardNames
        | CardNameSetDef::LandCardNames
        | CardNameSetDef::NonbasicLandCardNames
        | CardNameSetDef::CardNamesOtherThanBasicLands
        | CardNameSetDef::BasicLandNames => Ok(()),
    }
}
