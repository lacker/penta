fn validate_object_collection_shape(
    collection: crate::card::ObjectCollectionSourceDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match collection {
        crate::card::ObjectCollectionSourceDef::ObjectSet(input) => {
            validate_object_set_shape(input, targets)
        }
        crate::card::ObjectCollectionSourceDef::TopCards { player, count } => {
            validate_player_reference_shape(player, targets)?;
            validate_value_shape(count, targets)
        }
        crate::card::ObjectCollectionSourceDef::TopCardsThroughFirstMatching { player, object } => {
            validate_player_reference_shape(player, targets)?;
            validate_object_predicate_shape(object, targets)
        }
    }
}

fn validate_object_set_predicate_shape(
    predicate: crate::card::ObjectSetPredicateDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    predicate.filter.map_or(Ok(()), |filter| {
        validate_object_predicate_shape(filter.predicate(), targets)
    })
}

#[allow(clippy::too_many_lines)]
fn validate_effect_target_shapes(
    effect: EffectDef,
    targets: &[AbilityTargetDef],
    triggering_object_zone: Option<ZoneKind>,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        EffectDef::Sequence(effects) => {
            for effect in effects {
                validate_effect_target_shapes(*effect, targets, triggering_object_zone)?;
            }
            Ok(())
        }
        EffectDef::Randomized {
            on_success,
            on_failure,
            ..
        } => {
            validate_effect_target_shapes(*on_success, targets, triggering_object_zone)?;
            validate_effect_target_shapes(*on_failure, targets, triggering_object_zone)
        }
        EffectDef::ExileOneFromEachZone(pile) => {
            validate_recipient_shape(pile.player, targets, RecipientExpectation::Player)
        }
        EffectDef::MillWhileMatching(mill) => {
            validate_recipient_shape(mill.player, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(mill.object, targets)?;
            validate_effect_target_shapes(*mill.body, targets, triggering_object_zone)?;
            validate_effect_target_shapes(*mill.on_match, targets, triggering_object_zone)
        }
        EffectDef::SearchZone {
            player,
            object,
            attachment,
            then,
            ..
        } => {
            validate_recipient_shape(player, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(object, targets)?;
            if let Some(attachment) = attachment {
                match attachment {
                    ArrivalAttachmentDef::SourceToArrival => {}
                    ArrivalAttachmentDef::ArrivalToHost(host) => {
                        validate_object_reference_shape(host, targets)?;
                    }
                    ArrivalAttachmentDef::ArrivalToPlayer(player) => {
                        validate_player_reference_shape(player, targets)?;
                    }
                }
            }
            if let Some(then) = then {
                validate_effect_target_shapes(*then, targets, triggering_object_zone)?;
            }
            Ok(())
        }
        EffectDef::ChooseCardName { chooser, .. } => {
            validate_player_reference_shape(chooser, targets)
        }
        EffectDef::Choose(choice) => {
            validate_player_reference_shape(choice.chooser, targets)?;
            validate_object_set_shape(choice.candidates, targets)?;
            if let Some(excluded) = choice.exclude {
                validate_object_reference_shape(excluded, targets)?;
            }
            validate_effect_target_shapes(*choice.then, targets, triggering_object_zone)
        }
        EffectDef::ChooseExact(choice) => {
            validate_player_reference_shape(choice.chooser, targets)?;
            validate_object_set_shape(choice.candidates, targets)?;
            if let Some(excluded) = choice.exclude {
                validate_object_reference_shape(excluded, targets)?;
            }
            validate_value_shape(choice.amount, targets)?;
            validate_effect_target_shapes(*choice.then, targets, triggering_object_zone)
        }
        EffectDef::ChooseCardsFromCollection(choice) => {
            validate_object_collection_shape(choice.source, targets)?;
            validate_player_reference_shape(choice.actor, targets)?;
            validate_object_predicate_shape(choice.object, targets)?;
            validate_effect_target_shapes(*choice.then, targets, triggering_object_zone)
        }
        EffectDef::BindObjects(definition) => {
            validate_object_collection_shape(definition.source, targets)?;
            validate_effect_target_shapes(*definition.then, targets, triggering_object_zone)
        }
        EffectDef::IfNoObjects(definition) => {
            validate_object_set_shape(definition.input, targets)?;
            validate_effect_target_shapes(
                *definition.if_empty,
                targets,
                triggering_object_zone,
            )?;
            validate_effect_target_shapes(*definition.otherwise, targets, triggering_object_zone)
        }
        EffectDef::ClassifyObjects(definition) => {
            validate_object_set_shape(definition.input, targets)?;
            validate_object_predicate_shape(definition.object, targets)?;
            validate_effect_target_shapes(*definition.then, targets, triggering_object_zone)
        }
        EffectDef::RevealAndClassifyCards(definition) => {
            validate_object_collection_shape(definition.source, targets)?;
            validate_object_predicate_shape(definition.object, targets)?;
            validate_effect_target_shapes(*definition.then, targets, triggering_object_zone)
        }
        EffectDef::CombineObjects(definition) => {
            for input in definition.inputs {
                validate_object_set_shape(*input, targets)?;
            }
            validate_effect_target_shapes(*definition.then, targets, triggering_object_zone)
        }
        EffectDef::RandomizeObjectOrder(definition) => {
            validate_object_set_shape(definition.input, targets)?;
            validate_effect_target_shapes(*definition.then, targets, triggering_object_zone)
        }
        EffectDef::RevealObjects(definition) => {
            validate_object_set_shape(definition.input, targets)?;
            validate_effect_target_shapes(*definition.then, targets, triggering_object_zone)
        }
        EffectDef::MoveObjects(definition) => {
            validate_object_set_shape(definition.input, targets)?;
            validate_effect_target_shapes(*definition.then, targets, triggering_object_zone)
        }
        EffectDef::PutObjectsOntoBattlefieldFaceDown(definition) => {
            validate_object_set_shape(definition.input, targets)?;
            validate_player_reference_shape(definition.controller, targets)?;
            validate_effect_target_shapes(*definition.then, targets, triggering_object_zone)
        }
        EffectDef::ChooseObjectOrder(definition) => {
            validate_player_reference_shape(definition.actor, targets)?;
            validate_object_set_shape(definition.input, targets)?;
            validate_effect_target_shapes(*definition.then, targets, triggering_object_zone)
        }
        EffectDef::LookAtObjects(definition) => {
            validate_player_reference_shape(definition.actor, targets)?;
            validate_object_collection_shape(definition.source, targets)?;
            validate_effect_target_shapes(*definition.then, targets, triggering_object_zone)
        }
        EffectDef::PartitionGroup(definition) => {
            validate_player_reference_shape(definition.actor, targets)?;
            validate_object_set_shape(definition.input, targets)?;
            validate_effect_target_shapes(*definition.then, targets, triggering_object_zone)
        }
        EffectDef::ChooseGroup(definition) => {
            validate_player_reference_shape(definition.actor, targets)?;
            validate_object_set_shape(definition.first, targets)?;
            validate_object_set_shape(definition.second, targets)?;
            validate_effect_target_shapes(*definition.then, targets, triggering_object_zone)
        }
        EffectDef::ChooseOneOfEach(definition) => {
            validate_player_reference_shape(definition.actor, targets)?;
            validate_object_set_shape(definition.input, targets)?;
            for predicate in definition.predicates {
                validate_object_predicate_shape(*predicate, targets)?;
            }
            validate_effect_target_shapes(*definition.then, targets, triggering_object_zone)
        }
        EffectDef::BindOutput { effect, .. }
        | EffectDef::ForEachInBinding { effect, .. } => {
            validate_effect_target_shapes(*effect, targets, triggering_object_zone)
        }
        EffectDef::WithBattlefieldArrival { effect, arrival } => {
            validate_battlefield_arrival_target_shapes(
                effect,
                arrival,
                targets,
                triggering_object_zone,
            )
        }
        EffectDef::WithZoneMoveResult {
            effect,
            binding,
            then,
        } => {
            validate_zone_move_result_target_shapes(
                effect,
                binding,
                then,
                targets,
                triggering_object_zone,
            )
        }
        EffectDef::PayOr(payment) => {
            validate_payment_shape(payment.payment, targets)?;
            for effect in payment.if_paid.iter().chain(payment.otherwise.iter()) {
                validate_effect_target_shapes(**effect, targets, triggering_object_zone)?;
            }
            Ok(())
        }
        EffectDef::PreventDamage { prevention, .. } => {
            validate_damage_matcher_shape(prevention.matcher, targets)?;
            if let DamagePreventionCapacityDef::Amount(amount) = prevention.capacity {
                validate_value_shape(amount, targets)?;
            }
            validate_value_shape(prevention.amount, targets)
        }
        EffectDef::DealDamage { recipient, amount }
        | EffectDef::DealDamageAndApply {
            recipient, amount, ..
        }
        | EffectDef::DrainLife { recipient, amount } => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Any)?;
            validate_value_shape(amount, targets)
        }
        EffectDef::DealDamageSimultaneously(assignments) => {
            for assignment in assignments {
                if let Some(source) = assignment.source {
                    validate_object_reference_shape(source, targets)?;
                }
                validate_recipient_shape(
                    assignment.recipient,
                    targets,
                    RecipientExpectation::Any,
                )?;
                validate_value_shape(assignment.amount, targets)?;
            }
            Ok(())
        }
        EffectDef::Fight {
            first,
            second,
            excess,
        } => {
            validate_object_reference_shape(first, targets)?;
            validate_object_reference_shape(second, targets)?;
            if let Some(excess) = excess {
                validate_object_reference_shape(excess.recipient, targets)?;
                validate_effect_target_shapes(*excess.then, targets, triggering_object_zone)?;
            }
            Ok(())
        }
        EffectDef::DealDamageFrom {
            source,
            recipient,
            amount,
        } => {
            validate_object_reference_shape(source, targets)?;
            validate_recipient_shape(recipient, targets, RecipientExpectation::Any)?;
            validate_value_shape(amount, targets)
        }
        EffectDef::ExchangeControl {
            first,
            second,
            otherwise,
        } => {
            validate_recipient_shape(first, targets, RecipientExpectation::Object)?;
            validate_recipient_shape(second, targets, RecipientExpectation::Object)?;
            match otherwise {
                Some(otherwise) => {
                    validate_effect_target_shapes(*otherwise, targets, triggering_object_zone)
                }
                None => Ok(()),
            }
        }
        EffectDef::GainLife { recipient, amount }
        | EffectDef::AddPlayerCounters {
            recipient, amount, ..
        }
        | EffectDef::DrawCards { recipient, amount }
        | EffectDef::LoseLife { recipient, amount } => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_value_shape(amount, targets)
        }
        EffectDef::SetLifeTotal { recipient, total } => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_value_shape(total, targets)
        }
        EffectDef::Discard {
            recipient,
            amount,
            then,
            ..
        } => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_value_shape(amount, targets)?;
            if let Some(follow_up) = then {
                validate_object_predicate_shape(follow_up.counted, targets)?;
                validate_effect_target_shapes(
                    *follow_up.effect,
                    targets,
                    triggering_object_zone,
                )?;
            }
            Ok(())
        }
        EffectDef::SelectAtRandomFromZone {
            player,
            source,
            object,
            amount,
            ..
        } => {
            if !matches!(
                source,
                ZoneKind::Hand | ZoneKind::Library | ZoneKind::Graveyard | ZoneKind::Exile
            ) {
                return Err(
                    GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                        context: "resolving",
                        operation: "SelectAtRandomFromZone with an unsupported zone",
                    },
                );
            }
            validate_recipient_shape(player, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(object, targets)?;
            validate_value_shape(amount, targets)
        }
        EffectDef::ChooseEffect { player, .. }
        | EffectDef::SearchZonesAndExileRest { player, .. }
        | EffectDef::ExileTopOfLibraryToPlay { player, .. }
        | EffectDef::ExileFromTopUntil { player, .. }
        | EffectDef::ShuffleLibrary { player }
        | EffectDef::BuryGraveyard { player }
        | EffectDef::EmptyManaPool { player }
        | EffectDef::LoseTheGame { player }
        | EffectDef::WinTheGame { player }
        | EffectDef::ChooseCards { player, .. }
        | EffectDef::TakeExtraTurn { player }
        | EffectDef::LookAtHand { player }
        | EffectDef::LookAtRandomCardInHand { player }
        | EffectDef::RevealAtRandomFromHand { player, .. }
        | EffectDef::RevealHand { player } => {
            validate_recipient_shape(player, targets, RecipientExpectation::Player)
        }
        EffectDef::ChooseForEachPlayer(choice) => {
            validate_recipient_shape(choice.player, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(choice.candidates, targets)?;
            match choice.selection {
                PerPlayerSelectionDef::OneOfEach(selectors) => {
                    for selector in selectors {
                        validate_object_predicate_shape(*selector, targets)?;
                    }
                }
                PerPlayerSelectionDef::Count(amount) => validate_value_shape(amount, targets)?,
            }
            validate_effect_target_shapes(*choice.then, targets, triggering_object_zone)
        }
        EffectDef::BecomeMonarch { player } => validate_player_reference_shape(player, targets),
        EffectDef::SacrificeOfChoice {
            count,
            player,
            object,
            then,
            otherwise,
            ..
        } => {
            validate_recipient_shape(player, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(object, targets)?;
            validate_value_shape(count, targets)?;
            for effect in then.into_iter().chain(otherwise) {
                validate_effect_target_shapes(*effect, targets, triggering_object_zone)?;
            }
            Ok(())
        }
        EffectDef::May { player, effect }
        | EffectDef::ReplaceNextDrawThisTurn { player, effect } => {
            validate_recipient_shape(player, targets, RecipientExpectation::Player)?;
            validate_effect_target_shapes(*effect, targets, triggering_object_zone)
        }
        EffectDef::Mill { player, amount, .. } => {
            validate_recipient_shape(player, targets, RecipientExpectation::Player)?;
            validate_value_shape(amount, targets)
        }
        EffectDef::MillUntil(mill) => {
            validate_recipient_shape(mill.player, targets, RecipientExpectation::Player)?;
            validate_object_set_predicate_shape(mill.until, targets)
        }
        EffectDef::ExileLinkedToSource { object, then, .. } => {
            validate_recipient_shape(object, targets, RecipientExpectation::Object)?;
            match then {
                Some(then) => {
                    validate_effect_target_shapes(*then, targets, triggering_object_zone)
                }
                None => Ok(()),
            }
        }
        EffectDef::PermitLookAtExiled {
            object,
            player,
            then,
        } => {
            validate_recipient_shape(object, targets, RecipientExpectation::Object)?;
            validate_player_reference_shape(player, targets)?;
            validate_effect_target_shapes(*then, targets, triggering_object_zone)
        }
        EffectDef::DiscardCards { object }
        | EffectDef::Explore { object }
        | EffectDef::Regenerate { object }
        | EffectDef::Tap { object }
        | EffectDef::RemoveFromCombat { object }
        | EffectDef::Untap { object }
        | EffectDef::Saddle { object }
        | EffectDef::AttachToSource { object }
        | EffectDef::Reconfigure { object }
        | EffectDef::Unattach { object }
        | EffectDef::PairWithSource { object }
        | EffectDef::PhaseOut { object }
        | EffectDef::Destroy {
            object, then: None, ..
        }
        | EffectDef::Detain { object }
        | EffectDef::DoubleCounters { object, .. }
        | EffectDef::RemoveAllCounters { object, .. }
        | EffectDef::SkipNextUntapSteps { object, .. }
        | EffectDef::Sacrifice { object }
        | EffectDef::SacrificeYours { object }
        | EffectDef::ChangeTextBasicLandType { object }
        | EffectDef::ChooseColor { object, .. }
        | EffectDef::BecomeCopyOf { object, .. }
        | EffectDef::ExileGrantingOwnerPlay { object, .. }
        | EffectDef::ExileGrantingControllerPlayThisTurn { object }
        | EffectDef::PermitCastFromGraveyardThisTurn { object }
        | EffectDef::GainControl { object, .. }
        | EffectDef::Transform { object }
        | EffectDef::PutIntoLibraryBeneathTop { object, .. }
        | EffectDef::Counter { object, .. }
        | EffectDef::PutSpellIntoOwnersLibrary { object }
        | EffectDef::Endure { object, .. }
        | EffectDef::ChooseCounterKind { object, .. }
        | EffectDef::ModifyCounters { object, .. }
        | EffectDef::MoveToZone { object, .. } => {
            validate_recipient_shape(object, targets, RecipientExpectation::Object)
        }
        EffectDef::CopyStackObject(copy) => {
            validate_recipient_shape(copy.object, targets, RecipientExpectation::Object)
        }
        EffectDef::ChangeStackTargets(change) => {
            validate_recipient_shape(change.object, targets, RecipientExpectation::Object)?;
            if let crate::card::StackTargetChangeDef::ReplaceOneWith(replacement) = change.change {
                validate_recipient_shape(replacement, targets, RecipientExpectation::Any)?;
            }
            Ok(())
        }
        EffectDef::Destroy {
            object,
            then: Some(follow_up),
            ..
        } => {
            validate_recipient_shape(object, targets, RecipientExpectation::Object)?;
            validate_effect_target_shapes(*follow_up.effect, targets, triggering_object_zone)
        }
        EffectDef::Attach { object }
        | EffectDef::MayCastTargetWithoutPaying { object, .. } => {
            validate_recipient_shape(object, targets, RecipientExpectation::Any)
        }
        EffectDef::PutOntoBattlefieldThen {
            object,
            binding,
            then,
            ..
        } => {
            validate_recipient_shape(object, targets, RecipientExpectation::Object)?;
            validate_zone_move_follow_up_shapes(
                *then,
                Some(EffectRecipientDef::objects(ObjectSetDef::Binding(binding))),
                targets,
                triggering_object_zone,
            )
        }
        // A player keeps counters too -- experience is put on the player
        // rather than on anything they control -- so this admits either.
        EffectDef::AddCounters { object, amount, .. } => {
            validate_recipient_shape(object, targets, RecipientExpectation::Any)?;
            validate_value_shape(amount, targets)
        }
        EffectDef::RemoveCounters { object, amount, .. } => {
            validate_recipient_shape(object, targets, RecipientExpectation::Object)?;
            validate_value_shape(amount, targets)
        }
        EffectDef::CreateToken {
            count,
            copy,
            created,
            ..
        } => {
            validate_value_shape(count, targets)?;
            if let Some(copy) = copy {
                validate_recipient_shape(*copy.object, targets, RecipientExpectation::Object)?;
            }
            match created {
                Some(created) => validate_created_token_continuation(
                    *created.then,
                    created.binding,
                    targets,
                    triggering_object_zone,
                ),
                None => Ok(()),
            }
        }
        EffectDef::ReduceGenericCostBy(count)
        | EffectDef::ModifyCost(CostModificationDef::Spell(SpellCostModificationDef {
            adjustment:
                CostAdjustmentDef::Add(CostAmountDef::Generic(count))
                | CostAdjustmentDef::Subtract(CostAmountDef::Generic(count)),
            ..
        }))
        | EffectDef::AddManaEqualTo { amount: count, .. } => validate_value_shape(count, targets),
        effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
            let conditional = effect
                .conditional()
                .expect("conditional variants expose their shared shape");
            validate_trigger_condition_shape(*conditional.condition, targets)?;
            validate_effect_target_shapes(*conditional.then, targets, triggering_object_zone)?;
            conditional.otherwise.map_or(Ok(()), |otherwise| {
                validate_effect_target_shapes(*otherwise, targets, triggering_object_zone)
            })
        }
        EffectDef::IfFormat {
            then, otherwise, ..
        } => {
            validate_effect_target_shapes(*then, targets, triggering_object_zone)?;
            validate_effect_target_shapes(*otherwise, targets, triggering_object_zone)
        }
        EffectDef::ExileTopAndMayCast { player, otherwise } => {
            validate_recipient_shape(player, targets, RecipientExpectation::Player)?;
            match otherwise {
                Some(otherwise) => {
                    validate_effect_target_shapes(*otherwise, targets, triggering_object_zone)
                }
                None => Ok(()),
            }
        }
        EffectDef::InstallTrigger(trigger) => {
            if let crate::card::InstalledTriggerLifetimeDef::UntilNextTurn(player) =
                trigger.lifetime
            {
                validate_player_reference_shape(player, targets)?;
            }
            let trigger_event = match trigger.ability.definition {
                DeclarativeAbilityDef::Triggered(definition)
                | DeclarativeAbilityDef::TriggeredMana(definition) => Some(definition.event),
                DeclarativeAbilityDef::Spell(_)
                | DeclarativeAbilityDef::ActivatedMana(_)
                | DeclarativeAbilityDef::Activated(_)
                | DeclarativeAbilityDef::Static(_)
                | DeclarativeAbilityDef::Replacement(_)
                | DeclarativeAbilityDef::AlternativeCast(_)
                | DeclarativeAbilityDef::OptionalAdditionalCost(_)
                | DeclarativeAbilityDef::SpecialAction(_)
                | DeclarativeAbilityDef::Pregame(_)
                | DeclarativeAbilityDef::Keyword(_)
                | DeclarativeAbilityDef::DeckConstruction(_) => None,
            };
            validate_program_target_shapes(
                trigger.ability.effect.definition,
                targets,
                trigger_event,
            )
        }
        EffectDef::CreateOngoingEffect(ongoing) => {
            if let Some(affected) = ongoing.affected {
                validate_recipient_shape(affected, targets, RecipientExpectation::Any)?;
            }
            validate_program_target_shapes(ongoing.ability.effect.definition, &[], None)
        }
        EffectDef::CannotAttackUnless(query) | EffectDef::CannotAttackIf(query) => {
            validate_query_shape(*query, targets)
        }
        EffectDef::ConditionalStatic(conditional) => {
            validate_object_set_shape(*conditional.condition.objects, targets)?;
            if let Some(filter) = conditional.condition.predicate.filter {
                validate_object_predicate_shape(filter.predicate(), targets)?;
            }
            validate_applied_effect_shapes(
                conditional.then.recipient,
                conditional.then.effect,
                targets,
                true,
            )
        }
        EffectDef::StaticApply { recipient, effect } => {
            validate_applied_effect_shapes(recipient, effect, targets, true)
        }
        EffectDef::Apply {
            recipient,
            effect,
            duration,
        } => {
            validate_applied_effect_shapes(recipient, effect, targets, false)?;
            if !duration_is_valid_for_applied_effect(duration, effect) {
                return Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect);
            }
            if applied_effect_adds_ability(effect)
                && recipient_may_name_nonbattlefield_object(
                    recipient,
                    targets,
                    triggering_object_zone,
                )
                && !match duration {
                    ResolvedEffectDurationDef::UntilEndOfTurn => {
                        nonbattlefield_ability_grants_are_flashback(effect)
                            && recipient_nonbattlefield_zones_support_flashback(
                                recipient,
                                targets,
                                triggering_object_zone,
                            )
                    }
                    ResolvedEffectDurationDef::Permanent => {
                        nonbattlefield_ability_grants_are_suspend(effect)
                            || (triggering_object_zone == Some(ZoneKind::Stack)
                                && recipient == EffectRecipientDef::TriggeringObject
                                && nonbattlefield_ability_grants_are_source_entry_replacements(
                                    effect,
                                ))
                    }
                    _ => false,
                }
            {
                return Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect);
            }
            Ok(())
        }
        EffectDef::AddMana(mana) => match mana.mana {
            crate::card::ManaSelectionDef::Choice(types)
            | crate::card::ManaSelectionDef::Combination(types) => match types.source {
                crate::card::ManaTypeSourceDef::ProducedBy(reference) => {
                    validate_object_reference_shape(reference, targets)
                }
                crate::card::ManaTypeSourceDef::CouldBeProducedBy(objects) => {
                    validate_object_set_shape(objects, targets)
                }
                crate::card::ManaTypeSourceDef::Fixed(_) => Ok(()),
            },
            crate::card::ManaSelectionDef::One(_)
            | crate::card::ManaSelectionDef::ColorsOfLinkedExiles => Ok(()),
        },
        // The ballot is a predicate, not a target: nothing is pointed at.
        EffectDef::PutSourceOntoBattlefieldAttacking
        | EffectDef::VoteForPermanentToExile { .. }
        | EffectDef::ModifyCost(_)
        | EffectDef::None
        | EffectDef::ContinueReplacedDraw
        | EffectDef::DamageCannotBePreventedThisTurn
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::MayPlayWithoutPaying { .. }
        | EffectDef::Cascade
        | EffectDef::Proliferate
        | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::CannotBeForcedToDiscard
            | EffectDef::GainClassLevel { .. }
        | EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. }
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::CreateEmblem { .. }
        | EffectDef::CreateAttachedToken { .. }
        | EffectDef::CreateMyriadTokens
        | EffectDef::Special(_) => Ok(()),
    }
}

fn validate_replacement_effect_target_shapes(
    effect: ReplacementEffectDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        ReplacementEffectDef::BindOutput { effect, binding } => {
            if binding == crate::ParentBinding {
                return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                    context: "binding",
                    operation: "BindOutput requires a durable labeled binding",
                });
            }
            let ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(choice)) = *effect
            else {
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
        ReplacementEffectDef::Sequence(effects) => {
            for effect in effects {
                validate_replacement_effect_target_shapes(*effect, targets)?;
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
                validate_replacement_effect_target_shapes(*effect, targets)?;
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
                validate_replacement_effect_target_shapes(*effect, targets)?;
            }
            Ok(())
        }
        ReplacementEffectDef::Perform(effect) => {
            validate_effect_target_shapes(*effect, targets, None)
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

#[cfg(test)]
#[path = "effect_shape_entry_choice_tests.rs"]
mod entry_choice_tests;

#[cfg(test)]
mod recipient_shape_tests {
    use super::*;
    use crate::card::{
        PlayActionMatcherDef, PlayRestrictionDef, ResolvedEffectDurationDef,
    };

    const PLAYER_TARGET: AbilityTargetDef =
        AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any));
    const OBJECT_TARGET: AbilityTargetDef =
        AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Any,
            zones: &[ZoneKind::Battlefield],
            controller: None,
            owner: None,
        });
    const ANY_TARGET: AbilityTargetDef =
        AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget);

    fn cannot_play() -> AppliedEffectDef {
        AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
            PlayActionMatcherDef::CastSpell,
            ObjectPredicateDef::NoncreatureSpell,
        )))
    }

    #[test]
    fn object_and_player_effects_reject_opposite_typed_recipients() {
        assert_eq!(
            validate_ability_targets(
                &[],
                EffectDef::Tap {
                    object: EffectRecipientDef::Controller,
                },
            ),
            Err(GrantedAbilityValidationError::EffectRecipientKindMismatch {
                recipient: EffectRecipientDef::Controller,
                expected: EffectSubjectKind::Object,
            }),
        );
        assert_eq!(
            validate_ability_targets(
                &[],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Source,
                    amount: ValueDef::Constant(1),
                },
            ),
            Err(GrantedAbilityValidationError::EffectRecipientKindMismatch {
                recipient: EffectRecipientDef::Source,
                expected: EffectSubjectKind::Player,
            }),
        );
        assert_eq!(
            validate_ability_targets(
                &[],
                EffectDef::ExchangeControl {
                    first: EffectRecipientDef::Source,
                    second: EffectRecipientDef::Controller,
                    otherwise: None,
                },
            ),
            Err(GrantedAbilityValidationError::EffectRecipientKindMismatch {
                recipient: EffectRecipientDef::Controller,
                expected: EffectSubjectKind::Object,
            }),
            "both sides of an exchange are validated",
        );
    }

    #[test]
    fn target_slots_must_contain_the_subject_kind_an_effect_reads() {
        assert_eq!(
            validate_ability_targets(
                &[PLAYER_TARGET],
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            Err(GrantedAbilityValidationError::TargetReferenceKindMismatch {
                target: TargetIndex::PRIMARY,
                predicate: PLAYER_TARGET.predicate,
                expected: EffectSubjectKind::Object,
            }),
        );
        assert_eq!(
            validate_ability_targets(
                &[OBJECT_TARGET],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ),
            Err(GrantedAbilityValidationError::TargetReferenceKindMismatch {
                target: TargetIndex::PRIMARY,
                predicate: OBJECT_TARGET.predicate,
                expected: EffectSubjectKind::Player,
            }),
        );
    }

    #[test]
    fn typed_projections_make_mixed_target_filtering_explicit() {
        let effects = Box::leak(Box::new([
            EffectDef::Tap {
                object: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::target_players(TargetIndex::PRIMARY),
                effect: cannot_play(),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]));
        validate_ability_targets(&[ANY_TARGET], EffectDef::Sequence(effects))
            .expect("typed projections retain both halves of an any-target slot");
    }

    #[test]
    fn raw_target_references_require_at_most_one_selected_target() {
        let targets = [AbilityTargetDef::up_to(OBJECT_TARGET.predicate, 2)];
        assert_eq!(
            validate_ability_targets(
                &targets,
                EffectDef::Tap {
                    object: EffectRecipientDef::object(ObjectRefDef::Target(TargetIndex::PRIMARY,)),
                },
            ),
            Err(
                GrantedAbilityValidationError::TargetReferenceRequiresSingular {
                    target: TargetIndex::PRIMARY,
                    maximum: 2,
                },
            ),
        );
    }

    #[test]
    fn derived_controller_accepts_mixed_targets_but_owner_requires_an_object() {
        validate_ability_targets(
            &[ANY_TARGET],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                amount: ValueDef::Constant(1),
            },
        )
        .expect("a player is its own controller, so either half is meaningful");

        validate_ability_targets(
            &[OBJECT_TARGET],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::player(PlayerRefDef::OwnerOf(ObjectRefDef::Target(
                    TargetIndex::PRIMARY,
                ))),
                amount: ValueDef::Constant(1),
            },
        )
        .expect("an object target always has an owner");

        for target in [ANY_TARGET, PLAYER_TARGET] {
            assert_eq!(
                validate_ability_targets(
                    &[target],
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::player(PlayerRefDef::OwnerOf(
                            ObjectRefDef::Target(TargetIndex::PRIMARY),
                        )),
                        amount: ValueDef::Constant(1),
                    },
                ),
                Err(GrantedAbilityValidationError::TargetReferenceKindMismatch {
                    target: TargetIndex::PRIMARY,
                    predicate: target.predicate,
                    expected: EffectSubjectKind::Object,
                }),
            );
        }
    }

    #[test]
    fn static_player_rules_reject_event_only_selectors() {
        let recipient = EffectRecipientDef::player(PlayerRefDef::EventPlayer);
        assert_eq!(
            validate_ability_targets(
                &[],
                EffectDef::StaticApply {
                    recipient,
                    effect: cannot_play(),
                },
            ),
            Err(GrantedAbilityValidationError::UnsupportedStaticPlayerRecipient { recipient },),
        );
    }
}
