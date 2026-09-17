// Continuations must consume the binding they introduce.

fn validate_object_continuation(
    binding: Binding,
    effect: EffectDef,
    target_count: usize,
    scope: BindingScope<'_>,
    operation: &'static str,
) -> Result<(), GrantedAbilityValidationError> {
    let nested = scope.with_object(binding)?;
    validate_effect_references(effect, target_count, nested)?;
    let read = if binding == crate::ParentBinding {
        nested.parent_binding_was_read()
    } else {
        nested.binding_was_read(binding)
    };
    if !read {
        return Err(
            GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                context: "then continuation does not consume its declared binding; use Sequence",
                operation,
            },
        );
    }
    Ok(())
}

fn validate_object_set_continuation(
    binding: Binding,
    effect: EffectDef,
    target_count: usize,
    scope: BindingScope<'_>,
    operation: &'static str,
) -> Result<(), GrantedAbilityValidationError> {
    let may_escape = scope.object_set_may_escape(binding);
    let nested = scope.with_object_set(binding)?;
    validate_effect_references(effect, target_count, nested)?;
    let read = if binding == crate::ParentBinding {
        nested.parent_binding_was_read()
    } else {
        nested.binding_was_read(binding)
    };
    if !read && !may_escape {
        return Err(
            GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                context: "then continuation does not consume its declared binding; use Sequence",
                operation,
            },
        );
    }
    Ok(())
}
