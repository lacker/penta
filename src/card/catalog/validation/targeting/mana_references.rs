// Source-relative mana restrictions and the mana-type selection's references.
fn validate_mana_references(
    mana: crate::card::AddManaEffectDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    for restriction in mana.restrictions {
        match restriction {
            crate::card::ManaRestrictionDef::CastSpell(predicate)
            | crate::card::ManaRestrictionDef::CannotCastSpell(predicate)
            | crate::card::ManaRestrictionDef::ActivateAbility(predicate) => {
                validate_object_predicate_references(*predicate, target_count, scope)?;
            }
            _ => {}
        }
    }
    match mana.mana {
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
