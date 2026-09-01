fn zone_move_destination(effect: EffectDef) -> Option<ZoneKind> {
    match effect {
        EffectDef::MoveToZone { zone, .. } | EffectDef::ReturnLinkedExiles { zone, .. } => {
            Some(zone)
        }
        EffectDef::ChooseCards { destination, .. } => Some(destination),
        EffectDef::WithBattlefieldArrival { effect, .. } => zone_move_destination(*effect),
        _ => None,
    }
}

fn validate_zone_move_follow_up_shapes(
    effect: EffectDef,
    battlefield_recipient: Option<EffectRecipientDef>,
    targets: &[AbilityTargetDef],
    triggering_object_zone: Option<ZoneKind>,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        EffectDef::Sequence(effects) => {
            for effect in effects {
                validate_zone_move_follow_up_shapes(
                    *effect,
                    battlefield_recipient,
                    targets,
                    triggering_object_zone,
                )?;
            }
            Ok(())
        }
        EffectDef::Apply {
            recipient,
            effect,
            ..
        } if Some(recipient) == battlefield_recipient => {
            validate_applied_effect_shapes(recipient, effect, targets, false)
        }
        _ => validate_effect_target_shapes(effect, targets, triggering_object_zone),
    }
}

fn validate_battlefield_arrival_target_shapes(
    effect: &'static EffectDef,
    arrival: crate::card::BattlefieldArrivalDef,
    targets: &[AbilityTargetDef],
    triggering_object_zone: Option<ZoneKind>,
) -> Result<(), GrantedAbilityValidationError> {
    if !matches!(*effect, EffectDef::MoveToZone { .. }) {
        return Err(
            GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                context: "battlefield-arrival",
                operation: "non-zone-move effect",
            },
        );
    }
    validate_effect_target_shapes(*effect, targets, triggering_object_zone)?;
    match arrival.attachment {
        None | Some(ArrivalAttachmentDef::SourceToArrival) => Ok(()),
        Some(ArrivalAttachmentDef::ArrivalToHost(host)) => {
            validate_object_reference_shape(host, targets)
        }
        Some(ArrivalAttachmentDef::ArrivalToPlayer(player)) => {
            validate_player_reference_shape(player, targets)
        }
    }
}

fn validate_zone_move_result_target_shapes(
    effect: &'static EffectDef,
    binding: Binding,
    then: &'static EffectDef,
    targets: &[AbilityTargetDef],
    triggering_object_zone: Option<ZoneKind>,
) -> Result<(), GrantedAbilityValidationError> {
    let Some(destination) = zone_move_destination(*effect) else {
        return Err(
            GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                context: "zone-move-result",
                operation: "non-zone-moving effect",
            },
        );
    };
    validate_effect_target_shapes(*effect, targets, triggering_object_zone)?;
    validate_zone_move_follow_up_shapes(
        *then,
        (destination == ZoneKind::Battlefield)
            .then(|| EffectRecipientDef::binding_zone_change_successors(binding)),
        targets,
        triggering_object_zone,
    )
}
