fn validate_object_collection_references(
    collection: crate::card::ObjectCollectionSourceDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match collection {
        crate::card::ObjectCollectionSourceDef::ObjectSet(input) => validate_recipient_target_references(
            EffectRecipientDef::objects(input),
            target_count,
            scope,
        ),
        crate::card::ObjectCollectionSourceDef::TopCards { player, count } => {
            validate_player_reference(player, target_count, scope)?;
            validate_value_target_references(count, target_count, scope)
        }
        crate::card::ObjectCollectionSourceDef::TopCardsThroughFirstMatching { player, object } => {
            validate_player_reference(player, target_count, scope)?;
            validate_object_predicate_references(object, target_count, scope)
        }
    }
}

fn scope_after_immediate_effect(
    effect: EffectDef,
    scope: BindingScope<'_>,
) -> Result<BindingScope<'_>, GrantedAbilityValidationError> {
    match effect {
        EffectDef::BindOutput { binding, .. } => scope.with_declared_object_set(binding),
        _ => Ok(scope),
    }
}

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
        return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
            context: "then continuation does not consume its declared binding; use Sequence",
            operation,
        });
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
    let nested = scope.with_object_set(binding)?;
    validate_effect_references(effect, target_count, nested)?;
    let read = if binding == crate::ParentBinding {
        nested.parent_binding_was_read()
    } else {
        nested.binding_was_read(binding)
    };
    if !read {
        return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
            context: "then continuation does not consume its declared binding; use Sequence",
            operation,
        });
    }
    Ok(())
}

fn validate_card_name_continuation(
    binding: Binding,
    effect: EffectDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    if binding != crate::ParentBinding {
        return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
            context: "then continuation",
            operation: "ChooseCardName continuations must use ParentBinding",
        });
    }
    let nested = scope.with_object_set(binding)?;
    let chosen_name_reads = nested.chosen_name_read_count();
    validate_effect_references(effect, target_count, nested)?;
    if !nested.parent_binding_was_read()
        && nested.chosen_name_read_count() == chosen_name_reads
    {
        return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
            context: "then continuation",
            operation: "a continuation that does not read ParentBinding or the chosen name; use Sequence",
        });
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn validate_effect_references(
    effect: EffectDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        EffectDef::BindOutput { effect, binding } => {
            validate_effect_references(*effect, target_count, scope)?;
            if binding == crate::ParentBinding {
                return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                    context: "binding",
                    operation: "BindOutput requires a durable labeled binding",
                });
            }
            let _ = has_bindable_output(*effect)?;
            let _ = scope.declare_binding(binding)?;
            Ok(())
        }
        EffectDef::WithBattlefieldArrival { effect, arrival } => {
            validate_effect_references(*effect, target_count, scope)?;
            match arrival.attachment {
                None | Some(ArrivalAttachmentDef::SourceToArrival) => Ok(()),
                Some(ArrivalAttachmentDef::ArrivalToHost(host)) => {
                    validate_object_reference(host, target_count, scope)
                }
                Some(ArrivalAttachmentDef::ArrivalToPlayer(player)) => {
                    validate_player_reference(player, target_count, scope)
                }
            }
        }
        EffectDef::WithZoneMoveResult {
            effect,
            binding,
            then,
        } => {
            validate_effect_references(*effect, target_count, scope)?;
            validate_object_set_continuation(
                binding,
                *then,
                target_count,
                scope,
                "WithZoneMoveResult must expose a moved-object binding consumed by their continuation",
            )
        }
        EffectDef::Sequence(effects) => {
            let mut scope = scope;
            for effect in effects {
                validate_effect_references(*effect, target_count, scope)?;
                scope = scope_after_immediate_effect(*effect, scope)?;
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
        EffectDef::SelectAtRandomFromZone {
            player,
            object,
            amount,
            ..
        } => {
            validate_recipient_target_references(player, target_count, scope)?;
            validate_object_predicate_references(object, target_count, scope)?;
            validate_value_target_references(amount, target_count, scope)
        }
        EffectDef::Destroy {
            object,
            then: Some(follow_up),
            ..
        } => {
            validate_recipient_target_references(object, target_count, scope)?;
            validate_object_set_continuation(
                follow_up.binding,
                *follow_up.effect,
                target_count,
                scope,
                "Destroy follow-ups must expose a result binding consumed by their continuation",
            )
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
            validate_card_name_continuation(binding, *then, target_count, scope)
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
            if choice.unchosen.is_none() {
                return match choice.binding {
                    crate::card::ObjectChoiceBindingDef::Object(binding) => {
                        validate_object_continuation(
                            binding,
                            *choice.then,
                            target_count,
                            scope,
                            "single-output Choose continuations must expose a result binding consumed by their continuation",
                        )
                    }
                    crate::card::ObjectChoiceBindingDef::Objects(binding)
                    | crate::card::ObjectChoiceBindingDef::OrderedObjects(binding) => {
                        validate_object_set_continuation(
                            binding,
                            *choice.then,
                            target_count,
                            scope,
                            "single-output Choose continuations must expose a result binding consumed by their continuation",
                        )
                    }
                };
            }
            let nested = match choice.binding {
                crate::card::ObjectChoiceBindingDef::Object(binding) => scope.with_object(binding)?,
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
        EffectDef::ChooseExact(choice) => {
            validate_player_reference(choice.chooser, target_count, scope)?;
            validate_recipient_target_references(
                EffectRecipientDef::objects(choice.candidates),
                target_count,
                scope,
            )?;
            if let Some(excluded) = choice.exclude {
                validate_object_reference(excluded, target_count, scope)?;
            }
            validate_value_target_references(choice.amount, target_count, scope)?;
            validate_object_set_continuation(
                choice.binding,
                *choice.then,
                target_count,
                scope,
                "ChooseExact continuations must expose a result binding consumed by their continuation",
            )
        }
        EffectDef::ChooseCardsFromCollection(choice) => {
            validate_object_collection_references(choice.source, target_count, scope)?;
            validate_player_reference(choice.actor, target_count, scope)?;
            validate_object_predicate_references(choice.object, target_count, scope)?;
            if choice.minimum > choice.maximum {
                return Err(GrantedAbilityValidationError::InvalidObjectChoiceBounds {
                    binding: crate::card::ObjectChoiceBindingDef::Objects(choice.chosen),
                    minimum: choice.minimum,
                    maximum: choice.maximum,
                });
            }
            let nested = scope
                .with_object_set(choice.chosen)?
                .with_object_set(choice.remainder)?;
            validate_effect_references(*choice.then, target_count, nested)
        }
        EffectDef::BindObjects(definition) => {
            validate_object_collection_references(definition.source, target_count, scope)?;
            validate_object_set_continuation(
                definition.binding,
                *definition.then,
                target_count,
                scope,
                "BindObjects continuations must expose a result binding consumed by their continuation",
            )
        }
        EffectDef::IfNoObjects(definition) => {
            validate_recipient_target_references(
                EffectRecipientDef::objects(definition.input),
                target_count,
                scope,
            )?;
            validate_effect_references(*definition.if_empty, target_count, scope)?;
            validate_effect_references(*definition.otherwise, target_count, scope)
        }
        EffectDef::ClassifyObjects(definition) => {
            validate_recipient_target_references(
                EffectRecipientDef::objects(definition.input),
                target_count,
                scope,
            )?;
            validate_object_predicate_references(definition.object, target_count, scope)?;
            let nested = scope
                .with_object_set(definition.matching)?
                .with_object_set(definition.remainder)?;
            validate_effect_references(*definition.then, target_count, nested)
        }
        EffectDef::RevealAndClassifyCards(definition) => {
            validate_object_collection_references(definition.source, target_count, scope)?;
            validate_object_predicate_references(definition.object, target_count, scope)?;
            let nested = scope
                .with_object_set(definition.matching)?
                .with_object_set(definition.remainder)?;
            validate_effect_references(*definition.then, target_count, nested)
        }
        EffectDef::CombineObjects(definition) => {
            for input in definition.inputs {
                validate_recipient_target_references(
                    EffectRecipientDef::objects(*input),
                    target_count,
                    scope,
                )?;
            }
            validate_object_set_continuation(
                definition.combined,
                *definition.then,
                target_count,
                scope,
                "CombineObjects continuations must expose a result binding consumed by their continuation",
            )
        }
        EffectDef::RandomizeObjectOrder(definition) => {
            validate_recipient_target_references(
                EffectRecipientDef::objects(definition.input),
                target_count,
                scope,
            )?;
            validate_object_set_continuation(
                definition.randomized,
                *definition.then,
                target_count,
                scope,
                "RandomizeObjectOrder continuations must expose a result binding consumed by their continuation",
            )
        }
        EffectDef::RevealObjects(definition) => {
            validate_recipient_target_references(
                EffectRecipientDef::objects(definition.input),
                target_count,
                scope,
            )?;
            if matches!(*definition.then, EffectDef::None) {
                Ok(())
            } else {
                Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                    context: "then continuation",
                    operation: "RevealObjects has no output dependency; use Sequence",
                })
            }
        }
        EffectDef::MoveObjects(definition) => {
            validate_recipient_target_references(
                EffectRecipientDef::objects(definition.input),
                target_count,
                scope,
            )?;
            match definition.moved {
                Some(binding) => validate_object_set_continuation(
                    binding,
                    *definition.then,
                    target_count,
                    scope,
                    "MoveObjects continuations must expose a moved-object binding consumed by their continuation",
                ),
                None if matches!(*definition.then, EffectDef::None) => Ok(()),
                None => Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                    context: "then continuation",
                    operation: "MoveObjects has no output dependency; use Sequence",
                }),
            }
        }
        EffectDef::PutObjectsOntoBattlefieldFaceDown(definition) => {
            validate_recipient_target_references(
                EffectRecipientDef::objects(definition.input),
                target_count,
                scope,
            )?;
            validate_player_reference(definition.controller, target_count, scope)?;
            match definition.moved {
                Some(binding) => validate_object_set_continuation(
                    binding,
                    *definition.then,
                    target_count,
                    scope,
                    "face-down move continuations must expose a moved-object binding consumed by their continuation",
                ),
                None if matches!(*definition.then, EffectDef::None) => Ok(()),
                None => Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                    context: "then continuation",
                    operation: "a face-down move has no output dependency; use Sequence",
                }),
            }
        }
        EffectDef::ChooseObjectOrder(definition) => {
            validate_player_reference(definition.actor, target_count, scope)?;
            validate_recipient_target_references(
                EffectRecipientDef::objects(definition.input),
                target_count,
                scope,
            )?;
            validate_object_set_continuation(
                definition.ordered,
                *definition.then,
                target_count,
                scope,
                "ChooseObjectOrder continuations must expose a result binding consumed by their continuation",
            )
        }
        EffectDef::LookAtObjects(definition) => {
            validate_player_reference(definition.actor, target_count, scope)?;
            validate_object_collection_references(definition.source, target_count, scope)?;
            if matches!(*definition.then, EffectDef::None) {
                Ok(())
            } else {
                Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                    context: "then continuation",
                    operation: "LookAtObjects has no output dependency; use Sequence",
                })
            }
        }
        EffectDef::PartitionGroup(definition) => {
            validate_player_reference(definition.actor, target_count, scope)?;
            validate_recipient_target_references(
                EffectRecipientDef::objects(definition.input),
                target_count,
                scope,
            )?;
            let nested = scope
                .with_object_set(definition.first)?
                .with_object_set(definition.second)?;
            validate_effect_references(*definition.then, target_count, nested)
        }
        EffectDef::ChooseGroup(definition) => {
            validate_player_reference(definition.actor, target_count, scope)?;
            validate_recipient_target_references(
                EffectRecipientDef::objects(definition.first),
                target_count,
                scope,
            )?;
            validate_recipient_target_references(
                EffectRecipientDef::objects(definition.second),
                target_count,
                scope,
            )?;
            let nested = scope
                .with_object_set(definition.chosen)?
                .with_object_set(definition.unchosen)?;
            validate_effect_references(*definition.then, target_count, nested)
        }
        EffectDef::ChooseOneOfEach(definition) => {
            validate_player_reference(definition.actor, target_count, scope)?;
            validate_recipient_target_references(
                EffectRecipientDef::objects(definition.input),
                target_count,
                scope,
            )?;
            for predicate in definition.predicates {
                validate_object_predicate_references(*predicate, target_count, scope)?;
            }
            let nested = scope
                .with_object_set(definition.chosen)?
                .with_object_set(definition.remainder)?;
            validate_effect_references(*definition.then, target_count, nested)
        }
        EffectDef::ForEachInBinding {
            objects,
            binding,
            effect,
        } => {
            scope.validate_object_set_reference(objects)?;
            validate_object_continuation(
                binding,
                *effect,
                target_count,
                scope,
                "ForEachInBinding must expose a current-object binding consumed by its effect",
            )
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
            validate_value_target_references(prevention.amount, target_count, scope)
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
        EffectDef::DealDamageSimultaneously(assignments) => {
            for assignment in assignments {
                if let Some(source) = assignment.source {
                    validate_object_reference(source, target_count, scope)?;
                }
                validate_recipient_target_references(
                    assignment.recipient,
                    target_count,
                    scope,
                )?;
                validate_value_target_references(assignment.amount, target_count, scope)?;
            }
            Ok(())
        }
        EffectDef::Fight {
            first,
            second,
            excess,
        } => {
            validate_object_reference(first, target_count, scope)?;
            validate_object_reference(second, target_count, scope)?;
            if let Some(excess) = excess {
                validate_object_reference(excess.recipient, target_count, scope)?;
                validate_effect_references(*excess.then, target_count, scope)?;
            }
            Ok(())
        }
        EffectDef::SetLifeTotal { recipient, total } => {
            validate_recipient_target_references(recipient, target_count, scope)?;
            validate_value_target_references(total, target_count, scope)
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
        EffectDef::PermitLookAtExiled {
            object,
            player,
            then,
        } => {
            validate_recipient_target_references(object, target_count, scope)?;
            validate_player_reference(player, target_count, scope)?;
            validate_effect_references(*then, target_count, scope)
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
        | EffectDef::SacrificeYours { object }
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
        | EffectDef::Endure { object, .. }
        | EffectDef::ChooseCounterKind { object, .. }
        | EffectDef::ModifyCounters { object, .. }
        | EffectDef::MoveToZone { object, .. } => {
            validate_recipient_target_references(object, target_count, scope)
        }
        EffectDef::CopyStackObject(copy) => {
            validate_recipient_target_references(copy.object, target_count, scope)
        }
        EffectDef::ChangeStackTargets(change) => {
            validate_recipient_target_references(change.object, target_count, scope)?;
            if let crate::card::StackTargetChangeDef::ReplaceOneWith(replacement) = change.change {
                validate_recipient_target_references(replacement, target_count, scope)?;
            }
            Ok(())
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
                let Some(binding) = follow_up.bound else {
                    return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                        context: "then continuation",
                        operation: "Discard continuations must expose a discarded-card binding consumed by their continuation",
                    });
                };
                validate_object_set_continuation(
                    binding,
                    *follow_up.effect,
                    target_count,
                    scope,
                    "Discard continuations must expose a discarded-card binding consumed by their continuation",
                )?;
            }
            Ok(())
        }
        EffectDef::CreateToken {
            token,
            controller,
            count,
            copy,
            created,
            ..
        } => {
            validate_value_target_references(count, target_count, scope)?;
            if let Some(controller) = controller {
                validate_player_reference(controller, target_count, scope)?;
            }
            if let Some(stats) = token.variable_stats {
                validate_value_target_references(stats.power, target_count, scope)?;
                validate_value_target_references(stats.toughness, target_count, scope)?;
            }
            if let Some(copy) = copy {
                validate_recipient_target_references(*copy.object, target_count, scope)?;
            }
            match created {
                Some(created) => {
                    validate_object_set_continuation(
                        created.binding,
                        *created.then,
                        target_count,
                        scope,
                        "CreateToken continuations must expose a created-token binding consumed by their continuation",
                    )
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
            validate_object_set_continuation(
                binding,
                *then,
                target_count,
                scope,
                "PutOntoBattlefieldThen must expose an arrival binding consumed by its continuation",
            )
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
            let Some(binding) = binding else {
                return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                    context: "then continuation",
                    operation: "a SearchZone continuation without a result dependency; use Sequence",
                });
            };
            validate_object_set_continuation(
                binding,
                *then,
                target_count,
                scope,
                "SearchZone continuations must expose a found-card binding consumed by their continuation",
            )
        }
        EffectDef::BecomeMonarch { player } => {
            validate_player_reference(player, target_count, scope)
        }
        EffectDef::ChooseForEachPlayer(choice) => {
            validate_recipient_target_references(choice.player, target_count, scope)?;
            validate_object_predicate_references(choice.candidates, target_count, scope)?;
            match choice.selection {
                PerPlayerSelectionDef::OneOfEach(selectors) => {
                    for selector in selectors {
                        validate_object_predicate_references(*selector, target_count, scope)?;
                    }
                }
                PerPlayerSelectionDef::Count(amount) => {
                    validate_value_target_references(amount, target_count, scope)?;
                }
            }
            let nested = scope
                .with_object_set(choice.chosen)?
                .with_object_set(choice.unchosen)?;
            validate_effect_references(*choice.then, target_count, nested)
        }
        EffectDef::ChooseEffect { player, .. }
        | EffectDef::SearchZonesAndExileRest { player, .. }
        | EffectDef::ExileTopOfLibraryToPlay { player, .. }
        | EffectDef::ExileFromTopUntil { player, .. }
        | EffectDef::ChooseCards { player, .. }
        | EffectDef::TakeExtraTurn { player }
        | EffectDef::LookAtHand { player }
        | EffectDef::LookAtRandomCardInHand { player }
        | EffectDef::RevealAtRandomFromHand { player, .. }
        | EffectDef::RevealHand { player } => {
            validate_recipient_target_references(player, target_count, scope)
        }
        EffectDef::May { player, effect }
        | EffectDef::ReplaceNextDrawThisTurn { player, effect } => {
            validate_recipient_target_references(player, target_count, scope)?;
            validate_effect_references(*effect, target_count, scope)
        }
        EffectDef::Mill { player, amount, .. } => {
            validate_recipient_target_references(player, target_count, scope)?;
            validate_value_target_references(amount, target_count, scope)
        }
        EffectDef::MillUntil(mill) => {
            validate_recipient_target_references(mill.player, target_count, scope)
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
        effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
            let conditional = effect
                .conditional()
                .expect("conditional variants expose their shared shape");
            validate_trigger_condition(*conditional.condition, target_count, scope)?;
            validate_effect_references(*conditional.then, target_count, scope)?;
            conditional.otherwise.map_or(Ok(()), |otherwise| {
                validate_effect_references(*otherwise, target_count, scope)
            })
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
        EffectDef::ConditionalStatic(conditional) => {
            validate_object_set_target_references(
                *conditional.condition.objects,
                target_count,
                scope,
            )?;
            if let Some(filter) = conditional.condition.filter {
                validate_object_predicate_references(filter.predicate(), target_count, scope)?;
            }
            validate_recipient_target_references(
                conditional.then.recipient,
                target_count,
                scope,
            )?;
            validate_applied_effect_target_references(
                conditional.then.effect,
                target_count,
                scope,
            )
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
        EffectDef::AddMana(mana) => match mana.mana {
            crate::card::ManaSelectionDef::Choice(types)
            | crate::card::ManaSelectionDef::Combination(types) => match types.source {
                crate::card::ManaTypeSourceDef::ProducedBy(reference) => {
                    validate_object_reference(reference, target_count, scope)
                }
                crate::card::ManaTypeSourceDef::CouldBeProducedBy(objects) => {
                    validate_object_set_target_references(objects, target_count, scope)
                }
                crate::card::ManaTypeSourceDef::Fixed(_) => Ok(()),
            },
            crate::card::ManaSelectionDef::One(_)
            | crate::card::ManaSelectionDef::ColorsOfLinkedExiles => Ok(()),
        },
        // The chosen player is recorded on the permanent, not read from a
        // target slot.
        // A prohibition names a card shape, never a target.
        EffectDef::ModifyCost(_)
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::CannotAttackIf(_)
        | EffectDef::None
        | EffectDef::ContinueReplacedDraw
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::DamageCannotBePreventedThisTurn
        // The ballot is a predicate, not a target: nothing is pointed at.
        | EffectDef::PutSourceOntoBattlefieldAttacking
        | EffectDef::VoteForPermanentToExile { .. }
        | EffectDef::ReturnLinkedExiles { .. }
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
        EffectDef::MayPlayWithoutPaying(definition) => validate_recipient_target_references(
            EffectRecipientDef::objects(definition.objects),
            target_count,
            scope,
        ),
    }
}
