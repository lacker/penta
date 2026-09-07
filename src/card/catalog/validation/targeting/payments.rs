fn validate_payment_references(
    payment: EffectPaymentDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    validate_single_payment_payer(payment.payer)?;
    validate_player_set(payment.payer, target_count, scope)?;
    for cost in payment.cost.subcosts() {
        if let Some((predicate, _, _)) = cost.object_selection() {
            validate_object_predicate_references(predicate, target_count, scope)?;
        }
        match cost {
            CostDef::GenericMana(amount)
            | CostDef::ColoredMana { amount, .. }
            | CostDef::Repeat { times: amount, .. } => {
                validate_value_target_references(amount, target_count, scope)?;
            }
            CostDef::Action(effect) => validate_effect_references(*effect, target_count, scope)?,
            CostDef::CreateTokens { token, .. } => {
                if let Some(stats) = token.variable_stats {
                    validate_value_target_references(stats.power, target_count, scope)?;
                    validate_value_target_references(stats.toughness, target_count, scope)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_single_payment_payer(
    players: PlayerSetDef,
) -> Result<(), GrantedAbilityValidationError> {
    if matches!(
        players,
        PlayerSetDef::All | PlayerSetDef::Related(PlayerRelation::Any)
    ) {
        Err(GrantedAbilityValidationError::InvalidPaymentPayer { players })
    } else {
        Ok(())
    }
}

fn validate_payment_shape(
    payment: EffectPaymentDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    validate_player_set_shape(payment.payer, targets)?;
    if let PlayerSetDef::LegalTargets(target) = payment.payer {
        validate_target_shape(target, targets, RecipientExpectation::Any, true)?;
        validate_target_projection(target, targets, RecipientExpectation::Player)?;
    }
    for cost in payment.cost.subcosts() {
        if let Some((predicate, _, _)) = cost.object_selection() {
            validate_object_predicate_shape(predicate, targets)?;
        }
        match cost {
            CostDef::GenericMana(amount)
            | CostDef::ColoredMana { amount, .. }
            | CostDef::Repeat { times: amount, .. } => validate_value_shape(amount, targets)?,
            CostDef::Action(effect) => validate_effect_target_shapes(*effect, targets, None)?,
            CostDef::CreateTokens { token, .. } => {
                if let Some(stats) = token.variable_stats {
                    validate_value_shape(stats.power, targets)?;
                    validate_value_shape(stats.toughness, targets)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
