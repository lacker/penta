fn validate_predicated_player_rule_shape(
    recipient: EffectRecipientDef,
    predicate: ObjectPredicateDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
    validate_object_predicate_shape(predicate, targets)
}

fn validate_attack_restriction_shape(
    recipient: EffectRecipientDef,
    restriction: AttackRestrictionDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    let expectation = match restriction.defender {
        AttackDefenderScopeDef::Any => RecipientExpectation::Object,
        AttackDefenderScopeDef::AffectedPlayer
        | AttackDefenderScopeDef::AffectedPlayerOrPlaneswalker => RecipientExpectation::Player,
    };
    validate_recipient_shape(recipient, targets, expectation)?;
    validate_object_predicate_shape(restriction.attacker, targets)
}

fn validate_block_restriction_shape(
    recipient: EffectRecipientDef,
    restriction: BlockRestrictionDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    validate_recipient_shape(recipient, targets, RecipientExpectation::Object)?;
    match restriction {
        BlockRestrictionDef::Pair { counterpart, .. } => match counterpart {
            BlockRestrictionMatchDef::Any => Ok(()),
            BlockRestrictionMatchDef::Matching(predicate)
            | BlockRestrictionMatchDef::Except(predicate) => {
                validate_object_predicate_shape(predicate, targets)
            }
        },
        BlockRestrictionDef::MinimumBlockers(_) | BlockRestrictionDef::MaximumBlockers(_) => {
            Ok(())
        }
    }
}
