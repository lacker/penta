fn validate_replacement_binding_target_shape(
    binding: crate::Binding,
    effect: &'static ReplacementEffectDef,
) -> Result<(), GrantedAbilityValidationError> {
    if binding == crate::ParentBinding {
        return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
            context: "binding",
            operation: "BindOutput requires a durable labeled binding",
        });
    }
    let ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(choice)) = *effect else {
        return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
            context: "binding",
            operation: "entry BindOutput requires a scalar choice producer",
        });
    };
    match (choice.list, choice.destination) {
        (
            ScalarChoiceListDef::CardNames(names),
            BattlefieldEntryChoiceDestinationDef::CardName,
        ) if names.is_catalog_defined() => Ok(()),
        _ => Err(GrantedAbilityValidationError::InvalidScalarChoice {
            list: choice.list,
            destination: choice.destination,
        }),
    }
}

fn validate_replacement_effect_target_shapes(
    effect: ReplacementEffectDef,
    targets: &[AbilityTargetDef],
    triggering_object_zone: Option<ZoneKind>,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        ReplacementEffectDef::BindOutput { effect, binding } => {
            validate_replacement_binding_target_shape(binding, effect)
        }
        ReplacementEffectDef::Sequence(effects) => {
            for effect in effects {
                validate_replacement_effect_target_shapes(
                    *effect,
                    targets,
                    triggering_object_zone,
                )?;
            }
            Ok(())
        }
        ReplacementEffectDef::Conditional {
            condition,
            if_true,
            if_false,
        } => {
            validate_condition_shape(condition, targets)?;
            for effect in if_true.iter().chain(if_false.iter()) {
                validate_replacement_effect_target_shapes(
                    *effect,
                    targets,
                    triggering_object_zone,
                )?;
            }
            Ok(())
        }
        ReplacementEffectDef::PayOr {
            payment,
            if_paid,
            if_declined,
        } => {
            validate_payment_shape(payment, targets)?;
            for effect in if_paid.iter().chain(if_declined.iter()) {
                validate_replacement_effect_target_shapes(
                    *effect,
                    targets,
                    triggering_object_zone,
                )?;
            }
            Ok(())
        }
        ReplacementEffectDef::Perform(effect) => {
            validate_effect_target_shapes(*effect, targets, triggering_object_zone)
        }
        ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(choice)) => {
            let valid = matches!(
                (choice.list, choice.destination),
                (ScalarChoiceListDef::Players, BattlefieldEntryChoiceDestinationDef::Player)
                    | (
                        ScalarChoiceListDef::CreatureTypes,
                        BattlefieldEntryChoiceDestinationDef::CreatureType
                    )
                    | (
                        ScalarChoiceListDef::BasicLandTypes,
                        BattlefieldEntryChoiceDestinationDef::BasicLandType
                    )
                    | (
                        ScalarChoiceListDef::Colors,
                        BattlefieldEntryChoiceDestinationDef::Color
                    )
            );
            if valid {
                Ok(())
            } else {
                Err(GrantedAbilityValidationError::InvalidScalarChoice {
                    list: choice.list,
                    destination: choice.destination,
                })
            }
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::RegenerateDestroyedObject
        | ReplacementEffectDef::RemoveDamageFromDestroyedObject
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::AddToEventAmount(_)
        | ReplacementEffectDef::LookAtHand(_)
        | ReplacementEffectDef::Choose(
            ReplacementChoiceDef::Player(_)
            | ReplacementChoiceDef::ExileMatchingFromGraveyard(_),
        )
        | ReplacementEffectDef::CopyEntering { .. } => Ok(()),
    }
}
