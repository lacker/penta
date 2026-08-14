#[allow(clippy::too_many_lines)]
fn validate_effect_references(
    effect: EffectDef,
    target_count: usize,
    scope: BindingScope,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        EffectDef::Sequence(effects) => {
            for effect in effects {
                validate_effect_references(*effect, target_count, scope)?;
            }
            Ok(())
        }
        EffectDef::Randomized {
            on_success,
            on_failure,
            ..
        } => {
            validate_effect_references(*on_success, target_count, scope)?;
            validate_effect_references(*on_failure, target_count, scope)
        }
        EffectDef::Choose(choice) => {
            validate_player_reference(choice.chooser, target_count, scope)?;
            validate_recipient_target_references(
                EffectRecipientDef::objects(choice.candidates),
                target_count,
                scope,
            )?;
            if let Some(excluded) = choice.exclude {
                validate_object_reference(excluded, target_count, scope)?;
            }
            if choice.minimum > choice.maximum
                || matches!(
                    choice.binding,
                    crate::card::ObjectChoiceBindingDef::Object(_)
                ) && choice.maximum > 1
            {
                return Err(GrantedAbilityValidationError::InvalidObjectChoiceBounds {
                    binding: choice.binding,
                    minimum: choice.minimum,
                    maximum: choice.maximum,
                });
            }
            let nested = match choice.binding {
                crate::card::ObjectChoiceBindingDef::Object(binding) => {
                    scope.with_object(binding)?
                }
                crate::card::ObjectChoiceBindingDef::Objects(binding) => {
                    scope.with_object_set(binding)?
                }
            };
            validate_effect_references(*choice.then, target_count, nested)
        }
        EffectDef::PayOr(payment) => {
            validate_payment_references(payment.payment, target_count, scope)?;
            for branch in payment.if_paid.iter().chain(payment.otherwise.iter()) {
                validate_effect_references(**branch, target_count, scope)?;
            }
            Ok(())
        }
        EffectDef::PreventDamage { prevention, .. } => {
            validate_damage_matcher_references(prevention.matcher, target_count, scope)?;
            if let DamagePreventionCapacityDef::Amount(amount) = prevention.capacity {
                validate_value_target_references(amount, target_count, scope)?;
            }
            Ok(())
        }
        EffectDef::SplitIntoPiles(partition) => {
            validate_pile_role("divider", partition.divider)?;
            validate_pile_role("chooser", partition.chooser)?;
            match partition.items {
                crate::card::PartitionItemsDef::Objects(objects) => {
                    validate_recipient_target_references(
                        EffectRecipientDef::objects(objects),
                        target_count,
                        scope,
                    )?;
                }
                crate::card::PartitionItemsDef::TopOfLibrary { player, count } => {
                    validate_player_reference(player, target_count, scope)?;
                    validate_value_target_references(count, target_count, scope)?;
                }
            }
            validate_player_set(partition.divider, target_count, scope)?;
            validate_player_set(partition.chooser, target_count, scope)?;
            let nested = scope
                .with_object_set(partition.chosen)?
                .with_object_set(partition.unchosen)?;
            validate_effect_references(*partition.then, target_count, nested)
        }
        EffectDef::DealDamage { recipient, amount }
        | EffectDef::DrainLife { recipient, amount }
        | EffectDef::GainLife { recipient, amount }
        | EffectDef::AddPoisonCounters { recipient, amount }
        | EffectDef::DrawCards { recipient, amount }
        | EffectDef::Discard {
            recipient, amount, ..
        }
        | EffectDef::LoseLife { recipient, amount } => {
            validate_recipient_target_references(recipient, target_count, scope)?;
            validate_value_target_references(amount, target_count, scope)
        }
        EffectDef::LoseTheGame { player: object }
        | EffectDef::ShuffleLibrary { player: object }
        | EffectDef::EmptyManaPool { player: object }
        | EffectDef::Regenerate { object }
        | EffectDef::Tap { object }
        | EffectDef::RemoveFromCombat { object }
        | EffectDef::DestroyAtEndOfCombat { object, .. }
        | EffectDef::SkipNextUntapSteps { object, .. }
        | EffectDef::RemoveAllCounters { object, .. }
        | EffectDef::Untap { object }
        | EffectDef::Attach { object }
        | EffectDef::Reconfigure { object }
        | EffectDef::Destroy { object, .. }
        | EffectDef::Sacrifice { object }
        | EffectDef::DiscardCards { object }
        | EffectDef::ChangeTextBasicLandType { object }
        | EffectDef::BecomeCopyOf { object, .. }
        | EffectDef::ExileLinkedToSource { object }
        | EffectDef::Detain { object }
        | EffectDef::GainControl { object, .. }
        | EffectDef::Transform { object }
        | EffectDef::MoveToZone { object, .. }
        | EffectDef::Counter { object, .. }
        | EffectDef::CreateTokenCopyOf { object } => {
            validate_recipient_target_references(object, target_count, scope)
        }
        EffectDef::CreateToken { count, .. } | EffectDef::ReduceGenericCostBy(count) => {
            validate_value_target_references(count, target_count, scope)
        }
        EffectDef::CreateAttachedToken { .. } => Ok(()),
        EffectDef::SacrificeOfChoice { player, then, .. } => {
            validate_recipient_target_references(player, target_count, scope)?;
            if let Some(effect) = then {
                validate_effect_references(*effect, target_count, scope)?;
            }
            Ok(())
        }
        EffectDef::SearchZone { player, .. }
        | EffectDef::ChooseCards { player, .. }
        | EffectDef::TakeExtraTurn { player }
        | EffectDef::LookAtHand { player } => {
            validate_recipient_target_references(player, target_count, scope)
        }
        EffectDef::LookAtTopAndSelect { player, selection } => {
            validate_recipient_target_references(player, target_count, scope)?;
            validate_value_target_references(selection.count, target_count, scope)?;
            if let Some(effect) = selection.then {
                validate_effect_references(*effect, target_count, scope)?;
            }
            Ok(())
        }
        EffectDef::May { player, effect }
        | EffectDef::ReplaceNextDrawThisTurn { player, effect } => {
            validate_recipient_target_references(player, target_count, scope)?;
            validate_effect_references(*effect, target_count, scope)
        }
        EffectDef::Mill { player, amount } => {
            validate_recipient_target_references(player, target_count, scope)?;
            validate_value_target_references(amount, target_count, scope)
        }
        EffectDef::AddCounters { object, amount, .. } => {
            validate_recipient_target_references(object, target_count, scope)?;
            validate_value_target_references(amount, target_count, scope)
        }
        EffectDef::InstallTrigger(trigger) => {
            let DeclarativeAbilityDef::Triggered(definition) = trigger.ability.definition else {
                return Err(GrantedAbilityValidationError::UnsupportedInstalledTriggerAbility);
            };
            if definition.procedure != AbilityProcedureDef::Shared
                || definition.source_zones != [ZoneKind::Battlefield]
                || !definition.targets.is_empty()
                || trigger.ability.declarative_effect().is_none()
            {
                return Err(GrantedAbilityValidationError::UnsupportedInstalledTriggerAbility);
            }
            if definition.event == TriggerEventDef::StateCondition {
                return Err(unsupported_trigger_event(definition.event));
            }
            if let Some(condition) = definition.condition {
                validate_trigger_condition(*condition, target_count, scope)?;
            }
            validate_trigger_event_references(definition.event, target_count, scope)?;
            if let crate::card::InstalledTriggerLifetimeDef::UntilNextTurn(player) =
                trigger.lifetime
            {
                validate_player_reference(player, target_count, scope)?;
            }
            validate_program_references(trigger.ability.effect.definition, target_count, scope)
        }
        EffectDef::IfCondition { condition, then } => {
            validate_trigger_condition(*condition, target_count, scope)?;
            validate_effect_references(*then, target_count, scope)
        }
        EffectDef::IfFormat {
            then, otherwise, ..
        } => {
            validate_effect_references(*then, target_count, scope)?;
            validate_effect_references(*otherwise, target_count, scope)
        }
        EffectDef::StaticApply { recipient, effect } => {
            validate_recipient_target_references(recipient, target_count, scope)?;
            validate_applied_effect_target_references(effect, target_count, scope)
        }
        EffectDef::Apply {
            recipient, effect, ..
        } => {
            validate_recipient_target_references(recipient, target_count, scope)?;
            validate_resolving_applied_effect(recipient, effect)?;
            validate_applied_effect_target_references(effect, target_count, scope)
        }
        // The chosen player is recorded on the permanent, not read from a
        // target slot.
        // A prohibition names a card shape, never a target.
        EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::Special(_) => Ok(()),
    }
}

fn validate_replacement_effect_target_references(
    effect: ReplacementEffectDef,
    target_count: usize,
    scope: BindingScope,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        ReplacementEffectDef::Sequence(effects) => {
            for effect in effects {
                validate_replacement_effect_target_references(*effect, target_count, scope)?;
            }
            Ok(())
        }
        ReplacementEffectDef::Conditional {
            condition,
            if_true,
            if_false,
        } => {
            validate_condition(condition, target_count, scope)?;
            for effect in if_true.iter().chain(if_false.iter()) {
                validate_replacement_effect_target_references(*effect, target_count, scope)?;
            }
            Ok(())
        }
        ReplacementEffectDef::PayOr {
            payment,
            if_paid,
            if_declined,
        } => {
            validate_payment_references(payment, target_count, scope)?;
            for effect in if_paid.iter().chain(if_declined.iter()) {
                validate_replacement_effect_target_references(*effect, target_count, scope)?;
            }
            Ok(())
        }
        ReplacementEffectDef::Perform(effect) => {
            validate_effect_references(*effect, target_count, scope)
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::CopyEntering { .. } => Ok(()),
    }
}
