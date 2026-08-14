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
        EffectDef::Choose(choice) => {
            validate_player_reference_shape(choice.chooser, targets)?;
            validate_object_set_shape(choice.candidates, targets)?;
            if let Some(excluded) = choice.exclude {
                validate_object_reference_shape(excluded, targets)?;
            }
            validate_effect_target_shapes(*choice.then, targets, triggering_object_zone)
        }
        EffectDef::PayOr(payment) => {
            validate_payment_shape(payment.payment, targets)?;
            for effect in payment.if_paid.iter().chain(payment.otherwise.iter()) {
                validate_effect_target_shapes(**effect, targets, triggering_object_zone)?;
            }
            Ok(())
        }
        EffectDef::SplitIntoPiles(partition) => {
            validate_player_set_shape(partition.divider, targets)?;
            validate_player_set_shape(partition.chooser, targets)?;
            match partition.items {
                crate::card::PartitionItemsDef::Objects(objects) => {
                    validate_object_set_shape(objects, targets)?;
                }
                crate::card::PartitionItemsDef::TopOfLibrary { player, count } => {
                    validate_player_reference_shape(player, targets)?;
                    validate_value_shape(count, targets)?;
                }
            }
            validate_effect_target_shapes(*partition.then, targets, triggering_object_zone)
        }
        EffectDef::PreventDamage { prevention, .. } => {
            validate_damage_matcher_shape(prevention.matcher, targets)?;
            if let DamagePreventionCapacityDef::Amount(amount) = prevention.capacity {
                validate_value_shape(amount, targets)?;
            }
            Ok(())
        }
        EffectDef::DealDamage { recipient, amount }
        | EffectDef::DrainLife { recipient, amount } => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Any)?;
            validate_value_shape(amount, targets)
        }
        EffectDef::GainLife { recipient, amount }
        | EffectDef::AddPoisonCounters { recipient, amount }
        | EffectDef::DrawCards { recipient, amount }
        | EffectDef::Discard {
            recipient, amount, ..
        }
        | EffectDef::LoseLife { recipient, amount } => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_value_shape(amount, targets)
        }
        EffectDef::ShuffleLibrary { player }
        | EffectDef::EmptyManaPool { player }
        | EffectDef::LoseTheGame { player }
        | EffectDef::SearchZone { player, .. }
        | EffectDef::ChooseCards { player, .. }
        | EffectDef::TakeExtraTurn { player }
        | EffectDef::LookAtHand { player } => {
            validate_recipient_shape(player, targets, RecipientExpectation::Player)
        }
        EffectDef::SacrificeOfChoice {
            player,
            object,
            then,
            ..
        } => {
            validate_recipient_shape(player, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(object, targets)?;
            if let Some(effect) = then {
                validate_effect_target_shapes(*effect, targets, triggering_object_zone)?;
            }
            Ok(())
        }
        EffectDef::LookAtTopAndSelect { player, selection } => {
            validate_recipient_shape(player, targets, RecipientExpectation::Player)?;
            if selection.minimum > selection.maximum
                || !matches!(
                    selection.selected_zone,
                    ZoneKind::Hand | ZoneKind::Library | ZoneKind::Graveyard | ZoneKind::Exile
                )
                || !matches!(
                    selection.rest_zone,
                    ZoneKind::Hand | ZoneKind::Library | ZoneKind::Graveyard | ZoneKind::Exile
                )
            {
                return Err(
                    GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                        context: "resolving",
                        operation: "LookAtTopAndSelect with invalid bounds or unsupported destination zones",
                    },
                );
            }
            validate_value_shape(selection.count, targets)?;
            if let Some(predicate) = selection.object {
                validate_object_predicate_shape(predicate, targets)?;
            }
            if let Some(effect) = selection.then {
                validate_effect_target_shapes(*effect, targets, triggering_object_zone)?;
            }
            Ok(())
        }
        EffectDef::May { player, effect }
        | EffectDef::ReplaceNextDrawThisTurn { player, effect } => {
            validate_recipient_shape(player, targets, RecipientExpectation::Player)?;
            validate_effect_target_shapes(*effect, targets, triggering_object_zone)
        }
        EffectDef::Mill { player, amount } => {
            validate_recipient_shape(player, targets, RecipientExpectation::Player)?;
            validate_value_shape(amount, targets)
        }
        EffectDef::DiscardCards { object }
        | EffectDef::Regenerate { object }
        | EffectDef::Tap { object }
        | EffectDef::RemoveFromCombat { object }
        | EffectDef::Untap { object }
        | EffectDef::Attach { object }
        | EffectDef::Reconfigure { object }
        | EffectDef::Destroy { object, .. }
        | EffectDef::DestroyAtEndOfCombat { object }
        | EffectDef::Detain { object }
        | EffectDef::RemoveAllCounters { object, .. }
        | EffectDef::SkipNextUntapSteps { object, .. }
        | EffectDef::Sacrifice { object }
        | EffectDef::ChangeTextBasicLandType { object }
        | EffectDef::BecomeCopyOf { object, .. }
        | EffectDef::ExileLinkedToSource { object }
        | EffectDef::GainControl { object, .. }
        | EffectDef::Transform { object }
        | EffectDef::MoveToZone { object, .. }
        | EffectDef::Counter { object, .. }
        | EffectDef::CreateTokenCopyOf { object } => {
            validate_recipient_shape(object, targets, RecipientExpectation::Object)
        }
        EffectDef::AddCounters { object, amount, .. } => {
            validate_recipient_shape(object, targets, RecipientExpectation::Object)?;
            validate_value_shape(amount, targets)
        }
        EffectDef::CreateToken { count, .. }
        | EffectDef::ReduceGenericCostBy(count)
        | EffectDef::AddManaEqualTo { amount: count, .. } => validate_value_shape(count, targets),
        EffectDef::CreateAttachedToken { .. } => Ok(()),
        EffectDef::IfCondition { condition, then } => {
            validate_trigger_condition_shape(*condition, targets)?;
            validate_effect_target_shapes(*then, targets, triggering_object_zone)
        }
        EffectDef::IfFormat {
            then, otherwise, ..
        } => {
            validate_effect_target_shapes(*then, targets, triggering_object_zone)?;
            validate_effect_target_shapes(*otherwise, targets, triggering_object_zone)
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
                | DeclarativeAbilityDef::SpecialAction(_)
                | DeclarativeAbilityDef::Keyword(_)
                | DeclarativeAbilityDef::Legacy => None,
            };
            validate_program_target_shapes(
                trigger.ability.effect.definition,
                targets,
                trigger_event,
            )
        }
        EffectDef::CannotAttackUnless(query) => validate_query_shape(*query, targets),
        EffectDef::StaticApply { recipient, effect } => {
            validate_applied_effect_shapes(recipient, effect, targets, true)
        }
        EffectDef::Apply {
            recipient,
            effect,
            duration,
        } => {
            validate_applied_effect_shapes(recipient, effect, targets, false)?;
            if applied_effect_adds_ability(effect)
                && recipient_may_name_nonbattlefield_object(
                    recipient,
                    targets,
                    triggering_object_zone,
                )
                && (duration != ResolvedEffectDurationDef::UntilEndOfTurn
                    || !nonbattlefield_ability_grants_are_supported(effect)
                    || !recipient_nonbattlefield_zones_support_flashback(
                        recipient,
                        targets,
                        triggering_object_zone,
                    ))
            {
                return Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect);
            }
            Ok(())
        }
        EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::CreateEmblem { .. }
        | EffectDef::Special(_) => Ok(()),
    }
}

fn validate_replacement_effect_target_shapes(
    effect: ReplacementEffectDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
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
            if matches!(
                (choice.list, choice.destination),
                (
                    ScalarChoiceListDef::CardNames,
                    BattlefieldEntryChoiceDestinationDef::CardName
                ) | (
                    ScalarChoiceListDef::CreatureTypes,
                    BattlefieldEntryChoiceDestinationDef::CreatureType
                )
            ) {
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
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::Choose(ReplacementChoiceDef::Player(_))
        | ReplacementEffectDef::CopyEntering { .. } => Ok(()),
    }
}

#[cfg(test)]
mod recipient_shape_tests {
    use super::*;
    use crate::card::{
        BattlefieldEntryScalarChoiceDef, PlayActionMatcherDef, PlayRestrictionDef,
        ResolvedEffectDurationDef, TopCardSelectionDef, ZonePlacement,
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
    fn scalar_entry_choices_reject_mismatched_lists_and_destinations() {
        let choice = BattlefieldEntryScalarChoiceDef {
            list: ScalarChoiceListDef::CardNames,
            destination: BattlefieldEntryChoiceDestinationDef::CreatureType,
        };
        assert_eq!(
            validate_replacement_ability_targets(
                &[],
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(choice)),
            ),
            Err(GrantedAbilityValidationError::InvalidScalarChoice {
                list: choice.list,
                destination: choice.destination,
            }),
        );
    }

    #[test]
    fn top_card_selections_reject_invalid_bounds_and_unsupported_destinations() {
        static INVALID_BOUNDS: TopCardSelectionDef = TopCardSelectionDef {
            count: ValueDef::Constant(2),
            object: None,
            minimum: 2,
            maximum: 1,
            reveal_selected: false,
            selected_zone: ZoneKind::Hand,
            selected_placement: ZonePlacement::Top,
            rest_zone: ZoneKind::Library,
            rest_placement: ZonePlacement::Bottom,
            then: None,
        };
        static INVALID_ZONE: TopCardSelectionDef = TopCardSelectionDef {
            count: ValueDef::Constant(1),
            object: None,
            minimum: 0,
            maximum: 1,
            reveal_selected: false,
            selected_zone: ZoneKind::Battlefield,
            selected_placement: ZonePlacement::Top,
            rest_zone: ZoneKind::Library,
            rest_placement: ZonePlacement::Bottom,
            then: None,
        };

        for selection in [&INVALID_BOUNDS, &INVALID_ZONE] {
            assert_eq!(
                validate_ability_targets(
                    &[],
                    EffectDef::LookAtTopAndSelect {
                        player: EffectRecipientDef::Controller,
                        selection,
                    },
                ),
                Err(
                    GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                        context: "resolving",
                        operation: "LookAtTopAndSelect with invalid bounds or unsupported destination zones",
                    }
                ),
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
