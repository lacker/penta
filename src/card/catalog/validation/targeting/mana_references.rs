// Source-relative mana restrictions and the mana-type selection's references.
fn validate_mana_references(
    mana: crate::card::AddManaEffectDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    if let Some(amount) = mana.variable_amount {
        validate_value_target_references(amount, target_count, scope)?;
    }
    validate_mana_restrictions(mana.restrictions, target_count, scope)?;
    match mana.mana {
        crate::card::ManaSelectionDef::Amounts(amounts) => {
            amounts.iter().try_for_each(|(_, value)| {
                validate_value_target_references(*value, target_count, scope)
            })
        }
        crate::card::ManaSelectionDef::Choice(types)
        | crate::card::ManaSelectionDef::Combination(types) => match types.source {
            crate::card::ManaTypeSourceDef::ProducedBy(reference) => {
                validate_object_reference(reference, target_count, scope)
            }
            crate::card::ManaTypeSourceDef::CouldBeProducedBy(objects) => {
                validate_object_set_target_references(*objects, target_count, scope)
            }
            crate::card::ManaTypeSourceDef::Fixed(_) => Ok(()),
        },
        crate::card::ManaSelectionDef::One(_)
        | crate::card::ManaSelectionDef::ColorsOfLinkedExiles
        | crate::card::ManaSelectionDef::ChoiceOfBundles(_) => Ok(()),
    }
}

fn validate_mana_restrictions(
    restrictions: &[crate::card::ManaRestrictionDef],
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    for restriction in restrictions {
        match restriction {
            crate::card::ManaRestrictionDef::AnyOf(alternatives) => {
                validate_mana_restrictions(alternatives, target_count, scope)?;
            }
            crate::card::ManaRestrictionDef::CastSpell(predicate)
            | crate::card::ManaRestrictionDef::CannotCastSpell(predicate)
            | crate::card::ManaRestrictionDef::ActivateAbility(predicate) => {
                validate_object_predicate_references(*predicate, target_count, scope)?;
            }
            _ => {}
        }
    }
    Ok(())
}
