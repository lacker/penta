#[allow(clippy::too_many_lines)]
fn top_level_ability_error(
    definition: &CardDefinition,
    part: CardPartId,
    ability: AbilityId,
    problem: &GrantedAbilityValidationError,
) -> CatalogError {
    match problem {
        GrantedAbilityValidationError::TooManyGrantSites { count } => {
            CatalogError::TooManyAbilityGrantSites {
                definition: definition.id,
                part,
                ability,
                count: *count,
            }
        }
        GrantedAbilityValidationError::EmptyText => CatalogError::EmptyAbilityText {
            definition: definition.id,
            part,
            ability,
        },
        GrantedAbilityValidationError::MissingImplementationExplanation => {
            CatalogError::MissingImplementationExplanation {
                definition: definition.id,
                part,
                ability,
            }
        }
        GrantedAbilityValidationError::LegacyProcedureRequiresCustomExecution => {
            CatalogError::LegacyProcedureRequiresCustomExecution {
                definition: definition.id,
                part,
                ability,
            }
        }
        GrantedAbilityValidationError::HasNoSourceZone => CatalogError::AbilityHasNoSourceZone {
            definition: definition.id,
            part,
            ability,
        },
        GrantedAbilityValidationError::ManaAbilityHasTargets => {
            CatalogError::ManaAbilityHasTargets {
                definition: definition.id,
                part,
                ability,
            }
        }
        GrantedAbilityValidationError::ReplacementAbilityRequiresReplacementProgram => {
            CatalogError::ReplacementAbilityRequiresReplacementProgram {
                definition: definition.id,
                part,
                ability,
            }
        }
        GrantedAbilityValidationError::ReplacementProgramRequiresReplacementAbility => {
            CatalogError::ReplacementProgramRequiresReplacementAbility {
                definition: definition.id,
                part,
                ability,
            }
        }
        GrantedAbilityValidationError::UnsupportedReplacementProgram { event, operation } => {
            CatalogError::UnsupportedReplacementProgram {
                definition: definition.id,
                part,
                ability,
                event: *event,
                operation,
            }
        }
        GrantedAbilityValidationError::UnsupportedInstalledTriggerAbility => {
            CatalogError::UnsupportedInstalledTriggerAbility {
                definition: definition.id,
                part,
                ability,
            }
        }
        GrantedAbilityValidationError::UnsupportedTriggerEvent { event } => {
            CatalogError::UnsupportedTriggerEvent {
                definition: definition.id,
                part,
                ability,
                event: *event,
            }
        }
        GrantedAbilityValidationError::UnsupportedTriggeredManaProgram => {
            CatalogError::UnsupportedTriggeredManaProgram {
                definition: definition.id,
                part,
                ability,
            }
        }
        GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect => {
            CatalogError::UnsupportedResolvingAppliedEffect {
                definition: definition.id,
                part,
                ability,
            }
        }
        GrantedAbilityValidationError::UnsupportedEffectProgramContext { context, operation } => {
            CatalogError::UnsupportedAbilityEffectProgramContext {
                definition: definition.id,
                part,
                ability,
                context,
                operation,
            }
        }
        GrantedAbilityValidationError::TooManyTargets { count } => {
            CatalogError::TooManyAbilityTargets {
                definition: definition.id,
                part,
                ability,
                count: *count,
            }
        }
        GrantedAbilityValidationError::InvalidTargetBounds {
            target,
            minimum,
            maximum,
        } => CatalogError::InvalidAbilityTargetBounds {
            definition: definition.id,
            part,
            ability,
            target: *target,
            minimum: *minimum,
            maximum: *maximum,
        },
        GrantedAbilityValidationError::UnsupportedActivatedTargetChoice { target } => {
            CatalogError::UnsupportedActivatedAbilityTargetChoice {
                definition: definition.id,
                part,
                ability,
                target: *target,
            }
        }
        GrantedAbilityValidationError::TargetReferenceOutOfBounds {
            target,
            target_count,
        } => CatalogError::AbilityTargetReferenceOutOfBounds {
            definition: definition.id,
            part,
            ability,
            target: *target,
            target_count: *target_count,
        },
        GrantedAbilityValidationError::TargetReferenceKindMismatch {
            target,
            predicate,
            expected,
        } => CatalogError::AbilityTargetReferenceKindMismatch {
            definition: definition.id,
            part,
            ability,
            target: *target,
            predicate: *predicate,
            expected: *expected,
        },
        GrantedAbilityValidationError::TargetReferenceRequiresSingular { target, maximum } => {
            CatalogError::AbilityTargetReferenceRequiresSingular {
                definition: definition.id,
                part,
                ability,
                target: *target,
                maximum: *maximum,
            }
        }
        GrantedAbilityValidationError::EffectRecipientKindMismatch {
            recipient,
            expected,
        } => CatalogError::AbilityEffectRecipientKindMismatch {
            definition: definition.id,
            part,
            ability,
            recipient: *recipient,
            expected: *expected,
        },
        GrantedAbilityValidationError::InvalidScalarChoice { list, destination } => {
            CatalogError::InvalidAbilityScalarChoice {
                definition: definition.id,
                part,
                ability,
                list: *list,
                destination: *destination,
            }
        }
        GrantedAbilityValidationError::UnsupportedStaticPlayerRecipient { recipient } => {
            CatalogError::UnsupportedStaticAbilityPlayerRecipient {
                definition: definition.id,
                part,
                ability,
                recipient: *recipient,
            }
        }
        GrantedAbilityValidationError::InvalidObjectChoiceBounds {
            binding,
            minimum,
            maximum,
        } => CatalogError::InvalidAbilityObjectChoiceBounds {
            definition: definition.id,
            part,
            ability,
            binding: *binding,
            minimum: *minimum,
            maximum: *maximum,
        },
        GrantedAbilityValidationError::InvalidPileRole { role, players } => {
            CatalogError::InvalidAbilityPileRole {
                definition: definition.id,
                part,
                ability,
                role,
                players: *players,
            }
        }
        GrantedAbilityValidationError::InvalidPaymentPayer { players } => {
            CatalogError::InvalidAbilityPaymentPayer {
                definition: definition.id,
                part,
                ability,
                players: *players,
            }
        }
        GrantedAbilityValidationError::ObjectBindingReferenceOutOfScope { binding } => {
            CatalogError::AbilityObjectBindingReferenceOutOfScope {
                definition: definition.id,
                part,
                ability,
                binding: *binding,
            }
        }
        GrantedAbilityValidationError::ObjectBindingAlreadyInScope { binding } => {
            CatalogError::AbilityObjectBindingAlreadyInScope {
                definition: definition.id,
                part,
                ability,
                binding: *binding,
            }
        }
        GrantedAbilityValidationError::ObjectSetBindingReferenceOutOfScope { binding } => {
            CatalogError::AbilityObjectSetBindingReferenceOutOfScope {
                definition: definition.id,
                part,
                ability,
                binding: *binding,
            }
        }
        GrantedAbilityValidationError::ObjectSetBindingAlreadyInScope { binding } => {
            CatalogError::AbilityObjectSetBindingAlreadyInScope {
                definition: definition.id,
                part,
                ability,
                binding: *binding,
            }
        }
        GrantedAbilityValidationError::ExecutableStaticAbility => {
            unreachable!("only granted static abilities are rejected")
        }
    }
}
