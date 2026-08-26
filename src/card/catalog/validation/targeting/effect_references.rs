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
        EffectDef::ExileOneFromEachZone(pile) => {
            validate_recipient_target_references(pile.player, target_count, scope)
        }
        EffectDef::MillWhileMatching(mill) => {
            validate_recipient_target_references(mill.player, target_count, scope)?;
            validate_effect_references(*mill.body, target_count, scope)?;
            validate_effect_references(*mill.on_match, target_count, scope)
        }
        EffectDef::RevealAtRandomFromHand { player, then, .. } => {
            validate_recipient_target_references(player, target_count, scope)?;
            validate_effect_references(*then, target_count, scope)
        }
        EffectDef::BindMatching {
            objects,
            binding,
            then,
        } => {
            validate_recipient_target_references(
                EffectRecipientDef::objects(objects),
                target_count,
                scope,
            )?;
            let nested = scope.with_object_set(binding)?;
            validate_effect_references(*then, target_count, nested)
        }
        EffectDef::SelectAtRandomFromZone {
            player,
            object,
            binding,
            then,
            ..
        } => {
            validate_recipient_target_references(player, target_count, scope)?;
            validate_object_predicate_references(object, target_count, scope)?;
            validate_effect_references(*then, target_count, scope.with_object_set(binding)?)
        }
        EffectDef::Destroy {
            object,
            then: Some(follow_up),
            ..
        } => {
            validate_recipient_target_references(object, target_count, scope)?;
            let nested = scope.with_object_set(follow_up.binding)?;
            validate_effect_references(*follow_up.effect, target_count, nested)
        }
        EffectDef::ChooseCardName {
            chooser,
            matched_in,
            binding,
            then,
            ..
        } => {
            validate_player_reference(chooser, target_count, scope)?;
            validate_player_reference(matched_in, target_count, scope)?;
            // The cards of the chosen name are bound as it is answered, so
            // the follow-up may name that set.
            let nested = scope.with_object_set(binding)?;
            validate_effect_references(*then, target_count, nested)
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
                crate::card::ObjectChoiceBindingDef::Objects(binding)
                | crate::card::ObjectChoiceBindingDef::OrderedObjects(binding) => {
                    scope.with_object_set(binding)?
                }
            };
            // Both halves of the partition are in scope for the follow-up:
            // a clause that says what happens to the rest has to be able to
            // name the rest.
            let nested = match choice.unchosen {
                Some(binding) => nested.with_object_set(binding)?,
                None => nested,
            };
            validate_effect_references(*choice.then, target_count, nested)
        }
        EffectDef::ForEachInBinding {
            objects,
            binding,
            effect,
        } => {
            if scope.object_sets & (1 << objects.index()) == 0 {
                return Err(
                    GrantedAbilityValidationError::ObjectSetBindingReferenceOutOfScope {
                        binding: objects,
                    },
                );
            }
            validate_effect_references(*effect, target_count, scope.with_object(binding)?)
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
        | EffectDef::DealDamageAndApply {
            recipient, amount, ..
        }
        | EffectDef::DrainLife { recipient, amount }
        | EffectDef::GainLife { recipient, amount }
        | EffectDef::AddPlayerCounters {
            recipient, amount, ..
        }
        | EffectDef::DrawCards { recipient, amount }
        | EffectDef::LoseLife { recipient, amount } => {
            validate_recipient_target_references(recipient, target_count, scope)?;
            validate_value_target_references(amount, target_count, scope)
        }
        EffectDef::SetLifeTotal { recipient, total } => {
            validate_recipient_target_references(recipient, target_count, scope)?;
            validate_value_target_references(total, target_count, scope)
        }
        EffectDef::Scry { player, count }
        | EffectDef::LookAtTopAndDistribute { player, count, .. } => {
            validate_recipient_target_references(player, target_count, scope)?;
            validate_value_target_references(count, target_count, scope)
        }
        EffectDef::DealDamageFrom {
            source,
            recipient,
            amount,
        } => {
            validate_object_reference(source, target_count, scope)?;
            validate_recipient_target_references(recipient, target_count, scope)?;
            validate_value_target_references(amount, target_count, scope)
        }
        EffectDef::ExchangeControl {
            first,
            second,
            otherwise,
        } => {
            validate_recipient_target_references(first, target_count, scope)?;
            validate_recipient_target_references(second, target_count, scope)?;
            match otherwise {
                Some(otherwise) => validate_effect_references(*otherwise, target_count, scope),
                None => Ok(()),
            }
        }
        EffectDef::ExileLinkedToSource { object, then, .. } => {
            validate_recipient_target_references(object, target_count, scope)?;
            match then {
                Some(then) => validate_effect_references(*then, target_count, scope),
                None => Ok(()),
            }
        }
        EffectDef::MayCastTargetWithoutPaying { object, .. }
        | EffectDef::Explore { object }
        | EffectDef::LoseTheGame { player: object }
        | EffectDef::WinTheGame { player: object }
        | EffectDef::ShuffleLibrary { player: object }
        | EffectDef::BuryGraveyard { player: object }
        | EffectDef::EmptyManaPool { player: object }
        | EffectDef::Regenerate { object }
        | EffectDef::Tap { object }
        | EffectDef::RemoveFromCombat { object }
        | EffectDef::SkipNextUntapSteps { object, .. }
        | EffectDef::DoubleCounters { object, .. }
        | EffectDef::RemoveAllCounters { object, .. }
        | EffectDef::Untap { object }
        | EffectDef::Saddle { object }
        | EffectDef::Attach { object }
        | EffectDef::AttachToSource { object }
        | EffectDef::Reconfigure { object }
        | EffectDef::Unattach { object }
        | EffectDef::PairWithSource { object }
        | EffectDef::PhaseOut { object }
        | EffectDef::Destroy {
            object, then: None, ..
        }
        | EffectDef::Sacrifice { object }
        | EffectDef::PermitCastFromGraveyardThisTurn { object }
        | EffectDef::DiscardCards { object }
        | EffectDef::ChangeTextBasicLandType { object }
        | EffectDef::ChooseColor { object, .. }
        | EffectDef::BecomeCopyOf { object, .. }
        | EffectDef::ExileGrantingOwnerPlay { object, .. }
        | EffectDef::ExileGrantingControllerPlayThisTurn { object }
        | EffectDef::Detain { object }
        | EffectDef::GainControl { object, .. }
        | EffectDef::Transform { object }
        | EffectDef::PutIntoLibraryBeneathTop { object, .. }
        | EffectDef::Counter { object, .. }
        | EffectDef::PutSpellIntoOwnersLibrary { object }
        | EffectDef::Endure { object, .. } => {
            validate_recipient_target_references(object, target_count, scope)
        }
        EffectDef::CopyStackObject(copy) => {
            validate_recipient_target_references(copy.object, target_count, scope)
        }
        EffectDef::MoveToZone {
            object, attachment, ..
        } => {
            validate_recipient_target_references(object, target_count, scope)?;
            match attachment {
                None | Some(ArrivalAttachmentDef::SourceToArrival) => Ok(()),
                Some(ArrivalAttachmentDef::ArrivalToHost(host)) => {
                    validate_object_reference(host, target_count, scope)
                }
                Some(ArrivalAttachmentDef::ArrivalToPlayer(player)) => {
                    validate_player_reference(player, target_count, scope)
                }
            }
        }
        EffectDef::Discard {
            recipient,
            amount,
            then,
            ..
        } => {
            validate_recipient_target_references(recipient, target_count, scope)?;
            validate_value_target_references(amount, target_count, scope)?;
            if let Some(follow_up) = then {
                let nested = match follow_up.bound {
                    Some(binding) => scope.with_object_set(binding)?,
                    None => scope,
                };
                validate_effect_references(*follow_up.effect, target_count, nested)?;
            }
            Ok(())
        }
        EffectDef::CreateToken {
            count,
            copy,
            created,
            ..
        } => {
            validate_value_target_references(count, target_count, scope)?;
            if let Some(copy) = copy {
                validate_recipient_target_references(*copy.object, target_count, scope)?;
            }
            match created {
                Some(created) => {
                    let nested = scope.with_object_set(created.binding)?;
                    validate_effect_references(*created.then, target_count, nested)
                }
                None => Ok(()),
            }
        }
        EffectDef::ReduceGenericCostBy(count)
        | EffectDef::ModifyCost(CostModificationDef::Spell(SpellCostModificationDef {
            adjustment:
                CostAdjustmentDef::Add(CostAmountDef::Generic(count))
                | CostAdjustmentDef::Subtract(CostAmountDef::Generic(count)),
            ..
        })) => {
            validate_value_target_references(count, target_count, scope)
        }
        EffectDef::SacrificeOfChoice {
            count,
            player,
            then,
            otherwise,
            ..
        } => {
            validate_recipient_target_references(player, target_count, scope)?;
            validate_value_target_references(count, target_count, scope)?;
            for effect in then.into_iter().chain(otherwise) {
                validate_effect_references(*effect, target_count, scope)?;
            }
            Ok(())
        }
        EffectDef::PutOntoBattlefieldThen {
            object,
            binding,
            then,
            ..
        } => {
            validate_recipient_target_references(object, target_count, scope)?;
            validate_effect_references(*then, target_count, scope.with_object_set(binding)?)
        }
        EffectDef::SearchZone {
            player,
            object,
            attachment,
            binding,
            then,
            ..
        } => {
            validate_recipient_target_references(player, target_count, scope)?;
            validate_object_predicate_references(object, target_count, scope)?;
            if let Some(attachment) = attachment {
                match attachment {
                    ArrivalAttachmentDef::SourceToArrival => {}
                    ArrivalAttachmentDef::ArrivalToHost(host) => {
                        validate_object_reference(host, target_count, scope)?;
                    }
                    ArrivalAttachmentDef::ArrivalToPlayer(player) => {
                        validate_player_reference(player, target_count, scope)?;
                    }
                }
            }
            let Some(then) = then else {
                return Ok(());
            };
            // The cards a search found are in scope for its own follow-up,
            // the same way every other binding is scoped to the effect that
            // introduces it.
            let nested = match binding {
                Some(binding) => scope.with_object_set(binding)?,
                None => scope,
            };
            validate_effect_references(*then, target_count, nested)
        }
        EffectDef::BecomeMonarch { player } => {
            validate_player_reference(player, target_count, scope)
        }
        EffectDef::SimultaneousChoose(choice) => {
            validate_recipient_target_references(choice.player, target_count, scope)?;
            let nested = scope
                .with_object_set(choice.chosen)?
                .with_object_set(choice.unchosen)?;
            validate_effect_references(*choice.then, target_count, nested)
        }
        EffectDef::SearchZonesAndExileRest { player, .. }
        | EffectDef::ExileTopOfLibraryToPlay { player, .. }
        | EffectDef::ExileFromTopUntil { player, .. }
        | EffectDef::ManifestDread { player }
        | EffectDef::ChooseCards { player, .. }
        | EffectDef::TakeExtraTurn { player }
        | EffectDef::LookAtHand { player }
        | EffectDef::LookAtRandomCardInHand { player }
        | EffectDef::RevealHand { player } => {
            validate_recipient_target_references(player, target_count, scope)
        }
        EffectDef::LookAtTopAndSelect {
            player,
            looker,
            selection,
        } => {
            validate_recipient_target_references(player, target_count, scope)?;
            validate_recipient_target_references(looker, target_count, scope)?;
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
        EffectDef::Mill {
            player,
            amount,
            binding,
            then,
        } => {
            validate_recipient_target_references(player, target_count, scope)?;
            validate_value_target_references(amount, target_count, scope)?;
            let Some(then) = then else {
                return Ok(());
            };
            // The cards a mill put there are in scope for its own follow-up,
            // the same way a search's found cards are.
            let nested = match binding {
                Some(binding) => scope.with_object_set(binding)?,
                None => scope,
            };
            validate_effect_references(*then, target_count, nested)
        }
        EffectDef::MillUntil(mill) => {
            let (player, binding, then) = (mill.player, mill.binding, mill.then);
            validate_recipient_target_references(player, target_count, scope)?;
            let Some(then) = then else {
                return Ok(());
            };
            let nested = match binding {
                Some(binding) => scope.with_object_set(binding)?,
                None => scope,
            };
            validate_effect_references(*then, target_count, nested)
        }
        EffectDef::AddCounters { object, amount, .. }
        | EffectDef::RemoveCounters { object, amount, .. } => {
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
        EffectDef::CreateOngoingEffect(ongoing) => {
            let nested = match (ongoing.affected, ongoing.binding) {
                (Some(affected), Some(binding)) => {
                    validate_recipient_target_references(affected, target_count, scope)?;
                    scope.with_object(binding)?
                }
                (None, None) => scope,
                _ => {
                    return Err(GrantedAbilityValidationError::UnsupportedInstalledTriggerAbility);
                }
            };
            // The installed ability does not retain the creating spell's
            // target namespace. It receives only the concrete affected
            // recipient through the declared binding.
            validate_program_references(ongoing.ability.effect.definition, 0, nested)
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
        EffectDef::ExileTopAndMayCast { player, otherwise } => {
            validate_recipient_target_references(player, target_count, scope)?;
            match otherwise {
                Some(otherwise) => validate_effect_references(*otherwise, target_count, scope),
                None => Ok(()),
            }
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
        EffectDef::ModifyCost(_)
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::CannotAttackIf(_)
        | EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::DamageCannotBePreventedThisTurn
            | EffectDef::GrantFlashToNextSorcery
        // The ballot is a predicate, not a target: nothing is pointed at.
        | EffectDef::PutSourceOntoBattlefieldAttacking
        | EffectDef::VoteForPermanentToExile { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::MayPlayWithoutPaying { .. }
        | EffectDef::Cascade
        | EffectDef::Proliferate
        | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::CannotBeForcedToDiscard
            | EffectDef::GainClassLevel { .. }
        | EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. }
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::CreateAttachedToken { .. }
        | EffectDef::CreateMyriadTokens
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
        | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::AddToEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_)
        | ReplacementEffectDef::CopyEntering { .. } => Ok(()),
    }
}
