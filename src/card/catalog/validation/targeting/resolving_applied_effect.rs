fn validate_resolving_applied_effect(
    recipient: EffectRecipientDef,
    effect: AppliedEffectDef,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            if effects.is_empty() {
                return Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect);
            }
            for effect in effects {
                validate_resolving_applied_effect(recipient, *effect)?;
            }
            Ok(())
        }
        AppliedEffectDef::Rule(
            AppliedRuleDef::CannotPlay(_)
            | AppliedRuleDef::MayPlayFromGraveyard(_)
            // A timing permission is aimed at a player the same way a
            // prohibition is: no object has a casting window of its own.
            | AppliedRuleDef::MayCastAsThoughItHadFlash(_)
            // A player's protection is likewise a rule about the player: an
            // object gets protection as a keyword instead.
            | AppliedRuleDef::PlayerProtectionFrom(_)
            | AppliedRuleDef::PlayerRule(_)
            | AppliedRuleDef::RedirectDamageFromTo { .. },
        ) => {
            if matches!(recipient.0, EffectRecipientSetDef::Objects(_)) {
                Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect)
            } else {
                Ok(())
            }
        }
        AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(restriction)) => {
            if restriction
                .cost
                .is_some_and(|cost| cost.variable_x || cost.x_multiplier != 0)
            {
                return Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect);
            }
            let object_recipient = matches!(recipient.0, EffectRecipientSetDef::Objects(_));
            match restriction.defender {
                AttackDefenderScopeDef::Any if object_recipient => Ok(()),
                AttackDefenderScopeDef::AffectedPlayer
                | AttackDefenderScopeDef::AffectedPlayerOrPlaneswalker
                    if !object_recipient =>
                {
                    Ok(())
                }
                _ => Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect),
            }
        }
        AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(restriction)) => {
            let object_recipient = matches!(
                recipient.0,
                EffectRecipientSetDef::Objects(_) | EffectRecipientSetDef::LegalTargets(_)
            );
            let variable_cost = match restriction {
                BlockRestrictionDef::Pair { cost, .. } => {
                    cost.is_some_and(|cost| cost.variable_x || cost.x_multiplier != 0)
                }
                BlockRestrictionDef::MinimumBlockers(_)
                | BlockRestrictionDef::MaximumBlockers(_) => false,
            };
            if variable_cost || !object_recipient
            {
                Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect)
            } else {
                Ok(())
            }
        }
        AppliedEffectDef::Rule(
            AppliedRuleDef::CannotBeCountered | AppliedRuleDef::PreventDamage(_),
        ) => Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect),
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => {
            if matches!(recipient.0, EffectRecipientSetDef::Players(_)) {
                Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect)
            } else {
                Ok(())
            }
        }
    }
}
