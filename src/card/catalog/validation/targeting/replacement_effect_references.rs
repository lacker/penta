fn validate_replacement_effect_target_references(
    effect: ReplacementEffectDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        ReplacementEffectDef::BindOutput { effect, .. } => {
            validate_replacement_effect_target_references(*effect, target_count, scope)
        }
        ReplacementEffectDef::Sequence(effects) => {
            for effect in effects {
                validate_replacement_effect_target_references(*effect, target_count, scope)?;
            }
            Ok(())
        }
        ReplacementEffectDef::Conditional {
            condition,
            if_true,
            if_false,
        } => {
            validate_condition(condition, target_count, scope)?;
            for effect in if_true.iter().chain(if_false.iter()) {
                validate_replacement_effect_target_references(*effect, target_count, scope)?;
            }
            Ok(())
        }
        ReplacementEffectDef::PayOr {
            payment,
            if_paid,
            if_declined,
        } => {
            validate_payment_references(payment, target_count, scope)?;
            for effect in if_paid.iter().chain(if_declined.iter()) {
                validate_replacement_effect_target_references(*effect, target_count, scope)?;
            }
            Ok(())
        }
        ReplacementEffectDef::Perform(effect) => {
            validate_effect_references(*effect, target_count, scope)
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::RegenerateDestroyedObject
        | ReplacementEffectDef::RemoveDamageFromDestroyedObject
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::AddToEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_)
        | ReplacementEffectDef::CopyEntering { .. } => Ok(()),
    }
}
