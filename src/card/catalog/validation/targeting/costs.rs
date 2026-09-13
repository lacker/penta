/// Named selections are outputs of the complete payment. They are available
/// only to its paid branch, including when an action was repeated or replaced.
fn payment_object_set_outputs(costs: &[CostDef], outputs: &mut Vec<Binding>) {
    fn action_outputs(action: GameActionDef, outputs: &mut Vec<Binding>) {
        match action.unnamed() {
            GameActionDef::Choose(choice) => {
                if choice.binding != crate::ParentBinding && !outputs.contains(&choice.binding) {
                    outputs.push(choice.binding);
                }
            }
            GameActionDef::Sequence(actions) | GameActionDef::Choice(actions) => {
                for action in actions {
                    action_outputs(*action, outputs);
                }
            }
            _ => {}
        }
    }
    for cost in costs {
        match cost {
            CostDef::Perform(action) => action_outputs(**action, outputs),
            CostDef::All(costs) | CostDef::Choice(costs) | CostDef::Repeated { costs, .. } => {
                payment_object_set_outputs(costs, outputs);
            }
            _ => {}
        }
    }
}

fn validate_payment_cost_references(
    cost: CostDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match cost {
        CostDef::Perform(program) => {
            validate_effect_references(EffectDef::Perform(*program), target_count, scope)
        }
        CostDef::Repeated { costs, count } => {
            validate_value_target_references(*count, target_count, scope)?;
            costs
                .iter()
                .try_for_each(|cost| validate_payment_cost_references(*cost, target_count, scope))
        }
        CostDef::All(costs) | CostDef::Choice(costs) => costs
            .iter()
            .try_for_each(|cost| validate_payment_cost_references(*cost, target_count, scope)),
        CostDef::GenericMana(amount) | CostDef::ColoredMana { amount, .. } => {
            validate_value_target_references(amount, target_count, scope)
        }
        CostDef::ObjectManaCostReducedBy { object, .. }
        | CostDef::RemoveAnyNumberOfCounters { object, .. } => {
            validate_recipient_target_references(*object, target_count, scope)
        }
        CostDef::Discard { object, .. }
        | CostDef::SacrificePermanent { object, .. }
        | CostDef::MovePermanentMatching { object, .. } => {
            validate_object_predicate_references(object, target_count, scope)
        }
        _ => validate_program_cost_references(cost, target_count, scope),
    }
}

fn validate_payment_cost_shape(
    cost: CostDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match cost {
        CostDef::Perform(program) => {
            validate_effect_target_shapes(EffectDef::Perform(*program), targets, None)
        }
        CostDef::Repeated { costs, count } => {
            validate_value_shape(*count, targets)?;
            costs
                .iter()
                .try_for_each(|cost| validate_payment_cost_shape(*cost, targets))
        }
        CostDef::All(costs) | CostDef::Choice(costs) => costs
            .iter()
            .try_for_each(|cost| validate_payment_cost_shape(*cost, targets)),
        CostDef::GenericMana(amount) | CostDef::ColoredMana { amount, .. } => {
            validate_value_shape(amount, targets)
        }
        CostDef::ObjectManaCostReducedBy { object, .. }
        | CostDef::RemoveAnyNumberOfCounters { object, .. } => {
            validate_recipient_shape(*object, targets, RecipientExpectation::Object)
        }
        CostDef::Discard { object, .. }
        | CostDef::SacrificePermanent { object, .. }
        | CostDef::MovePermanentMatching { object, .. } => {
            validate_object_predicate_shape(object, targets)
        }
        _ => validate_program_cost_shape(cost, targets),
    }
}

fn validate_program_cost_references(
    cost: crate::CostDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match cost {
        CostDef::Sacrifice { object, .. }
        | CostDef::Exile { object, .. }
        | CostDef::SacrificePermanents { object, .. } => {
            validate_object_predicate_references(object, target_count, scope)
        }
        crate::card::CostDef::CreateTokens { token, .. } => match token.creation_stats {
            Some(stats) => {
                validate_value_target_references(stats.power, target_count, scope)?;
                validate_value_target_references(stats.toughness, target_count, scope)
            }
            None => Ok(()),
        },
        crate::CostDef::Repeated { costs, count } => {
            validate_value_target_references(*count, target_count, scope)?;
            costs
                .iter()
                .try_for_each(|cost| validate_program_cost_references(*cost, target_count, scope))
        }
        crate::CostDef::All(costs) | crate::CostDef::Choice(costs) => costs
            .iter()
            .try_for_each(|cost| validate_program_cost_references(*cost, target_count, scope)),
        _ => Ok(()),
    }
}

fn validate_program_cost_shape(
    cost: crate::CostDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match cost {
        CostDef::Sacrifice { object, .. }
        | CostDef::Exile { object, .. }
        | CostDef::SacrificePermanents { object, .. } => {
            validate_object_predicate_shape(object, targets)
        }
        crate::card::CostDef::CreateTokens { token, .. } => match token.creation_stats {
            Some(stats) => {
                validate_value_shape(stats.power, targets)?;
                validate_value_shape(stats.toughness, targets)
            }
            None => Ok(()),
        },
        crate::CostDef::Repeated { costs, count } => {
            validate_value_shape(*count, targets)?;
            costs
                .iter()
                .try_for_each(|cost| validate_program_cost_shape(*cost, targets))
        }
        crate::CostDef::All(costs) | crate::CostDef::Choice(costs) => costs
            .iter()
            .try_for_each(|cost| validate_program_cost_shape(*cost, targets)),
        _ => Ok(()),
    }
}
