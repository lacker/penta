// Created-token continuation shapes.
//
// Token creation supplies one fact ordinary binding validation cannot know:
// its output binding contains battlefield permanents. Preserve that fact
// through a sequence so a token can gain a noncopiable ability and also
// receive a delayed instruction in the same printed clause.

fn validate_created_token_continuation(
    effect: EffectDef,
    created_binding: Binding,
    targets: &[AbilityTargetDef],
    triggering_object_zone: Option<ZoneKind>,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        EffectDef::Sequence(effects) => {
            for effect in effects {
                validate_created_token_continuation(
                    *effect,
                    created_binding,
                    targets,
                    triggering_object_zone,
                )?;
            }
            Ok(())
        }
        EffectDef::Apply {
            recipient:
                EffectRecipientDef(EffectRecipientSetDef::Objects(ObjectSetDef::Binding(binding))),
            effect,
            duration,
        } if binding == created_binding => {
            validate_applied_effect_shapes(
                EffectRecipientDef::objects(ObjectSetDef::Binding(binding)),
                effect,
                targets,
                false,
            )?;
            if duration_is_valid_for_applied_effect(duration, effect) {
                Ok(())
            } else {
                Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect)
            }
        }
        _ => validate_effect_target_shapes(effect, targets, triggering_object_zone),
    }
}
