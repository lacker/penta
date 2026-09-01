fn has_bindable_output(effect: EffectDef) -> Result<bool, GrantedAbilityValidationError> {
    match effect {
        EffectDef::Mill { .. }
        | EffectDef::MillUntil(_)
        | EffectDef::SelectAtRandomFromZone { .. }
        | EffectDef::RevealAtRandomFromHand { .. } => Ok(true),
        EffectDef::IfCondition { then, .. } => has_bindable_output(*then),
        EffectDef::IfFormat {
            then, otherwise, ..
        }
        | EffectDef::Randomized {
            on_success: then,
            on_failure: otherwise,
            ..
        } => Ok(has_bindable_output(*then)? || has_bindable_output(*otherwise)?),
        EffectDef::None => Ok(false),
        _ => Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
            context: "bound effect output",
            operation: "an effect that does not expose an output",
        }),
    }
}
