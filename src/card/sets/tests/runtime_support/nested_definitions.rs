use super::*;

fn trigger_predicate_requires_live_battlefield(predicate: ObjectPredicateDef) -> bool {
    match predicate {
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => predicates
            .iter()
            .copied()
            .any(trigger_predicate_requires_live_battlefield),
        ObjectPredicateDef::Not(predicate) => {
            trigger_predicate_requires_live_battlefield(*predicate)
        }
        ObjectPredicateDef::HasNonManaActivatedAbility => true,
        _ => false,
    }
}

#[allow(clippy::too_many_lines)]
pub(in super::super) fn shared_trigger_event(event: TriggerEventDef) -> bool {
    match event {
        TriggerEventDef::ZoneChanged(matcher) => {
            const COMMITTED_TRANSITIONS: [(ZoneKind, ZoneKind); 9] = [
                (ZoneKind::Library, ZoneKind::Battlefield),
                (ZoneKind::Hand, ZoneKind::Battlefield),
                (ZoneKind::Graveyard, ZoneKind::Battlefield),
                (ZoneKind::Exile, ZoneKind::Battlefield),
                (ZoneKind::Stack, ZoneKind::Battlefield),
                (ZoneKind::Battlefield, ZoneKind::Graveyard),
                (ZoneKind::Battlefield, ZoneKind::Exile),
                (ZoneKind::Battlefield, ZoneKind::Hand),
                (ZoneKind::Battlefield, ZoneKind::Library),
            ];
            let can_match_departure =
                COMMITTED_TRANSITIONS
                    .iter()
                    .any(|(actual_from, actual_to)| {
                        *actual_from == ZoneKind::Battlefield
                            && *actual_to != ZoneKind::Battlefield
                            && matcher.from.is_none_or(|expected| expected == *actual_from)
                            && matcher.to.is_none_or(|expected| expected == *actual_to)
                    });
            shared_object_predicate(matcher.object)
                && (!can_match_departure
                    || !trigger_predicate_requires_live_battlefield(matcher.object))
                && COMMITTED_TRANSITIONS
                    .iter()
                    .any(|(actual_from, actual_to)| {
                        matcher.from.is_none_or(|expected| expected == *actual_from)
                            && matcher.to.is_none_or(|expected| expected == *actual_to)
                    })
                && matcher.previously_damaged_by.is_none_or(|reference| {
                    matcher
                        .from
                        .is_none_or(|from| from == ZoneKind::Battlefield)
                        && matcher.to.is_none_or(|to| to == ZoneKind::Graveyard)
                        && matches!(
                            reference,
                            ObjectRefDef::Source
                                | ObjectRefDef::AttachedToSource
                                | ObjectRefDef::TriggeringObject
                        )
                })
        }
        TriggerEventDef::Tapped(matcher) => shared_object_predicate(matcher.object),
        TriggerEventDef::Attacks(matcher) => {
            shared_object_predicate(matcher.attacker)
                && matcher.declaration.minimum > 0
                && matcher
                    .declaration
                    .maximum
                    .is_none_or(|maximum| matcher.declaration.minimum <= maximum)
                && matcher.attack_number.is_none_or(|number| number > 0)
        }
        TriggerEventDef::BecomesBlocked(object) | TriggerEventDef::Transforms(object) => {
            shared_object_predicate(object)
        }
        TriggerEventDef::SpellCast(object) => {
            shared_object_predicate(object) && !trigger_predicate_requires_live_battlefield(object)
        }
        TriggerEventDef::StepBegins { .. }
        | TriggerEventDef::LifeGained(_)
        | TriggerEventDef::StateCondition => true,
        TriggerEventDef::DamageDealt(matcher) => {
            let source = match matcher.source {
                DamageSourceMatcherDef::Matching(object) => {
                    shared_object_predicate(object)
                        && !trigger_predicate_requires_live_battlefield(object)
                }
                DamageSourceMatcherDef::Any | DamageSourceMatcherDef::Group(_) => true,
                DamageSourceMatcherDef::AffectedObject => false,
                DamageSourceMatcherDef::Object(reference)
                | DamageSourceMatcherDef::Except(reference) => matches!(
                    reference,
                    ObjectRefDef::Source
                        | ObjectRefDef::AttachedToSource
                        | ObjectRefDef::TriggeringObject
                ),
            };
            let recipient = match matcher.recipient {
                DamageRecipientMatcherDef::Recipients(EffectRecipientDef(
                    EffectRecipientSetDef::Objects(ObjectSetDef::One(reference)),
                )) => matches!(
                    reference,
                    ObjectRefDef::Source
                        | ObjectRefDef::AttachedToSource
                        | ObjectRefDef::TriggeringObject
                ),
                DamageRecipientMatcherDef::Any
                | DamageRecipientMatcherDef::Recipients(EffectRecipientDef(
                    EffectRecipientSetDef::Players(_),
                ))
                | DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(
                    PlayerRefDef::EffectController | PlayerRefDef::EventPlayer,
                ) => true,
                DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(
                    PlayerRefDef::ControllerOf(reference) | PlayerRefDef::OwnerOf(reference),
                ) => matches!(
                    reference,
                    ObjectRefDef::Source
                        | ObjectRefDef::AttachedToSource
                        | ObjectRefDef::TriggeringObject
                ),
                DamageRecipientMatcherDef::AffectedObject
                | DamageRecipientMatcherDef::Recipients(_)
                | DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(
                    PlayerRefDef::Target(_),
                ) => false,
            };
            source && recipient
        }
        TriggerEventDef::AttacksAndIsNotBlocked { attacker: source }
        | TriggerEventDef::BlocksOrBecomesBlockedBy { object: source } => {
            shared_object_predicate(source)
        }
    }
}

pub(super) fn shared_entry_replacement_effect(effect: ReplacementEffectDef) -> bool {
    match effect {
        ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::CopyEntering { .. } => true,
        ReplacementEffectDef::Sequence(effects) => {
            !effects.is_empty() && effects.iter().copied().all(shared_entry_replacement_effect)
        }
        ReplacementEffectDef::Conditional {
            condition,
            if_true,
            if_false,
        } => {
            let condition_is_supported = match condition {
                ConditionDef::Exists(query) => {
                    query.zones == [ZoneKind::Battlefield] && shared_object_predicate(query.object)
                }
            };
            condition_is_supported
                && if_true.iter().copied().all(shared_entry_replacement_effect)
                && if_false
                    .iter()
                    .copied()
                    .all(shared_entry_replacement_effect)
        }
        ReplacementEffectDef::PayOr {
            payment,
            if_paid,
            if_declined,
        } => {
            !matches!(
                payment.payer,
                PlayerSetDef::All | PlayerSetDef::Related(PlayerRelation::Any)
            ) && if_paid.iter().copied().all(shared_entry_replacement_effect)
                && if_declined
                    .iter()
                    .copied()
                    .all(shared_entry_replacement_effect)
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::Perform(_)
        | ReplacementEffectDef::MultiplyEventAmount(_) => false,
    }
}

pub(in super::super) fn shared_begin_turn_replacement_effect(effect: ReplacementEffectDef) -> bool {
    match effect {
        ReplacementEffectDef::ReplaceEventWithNothing => true,
        ReplacementEffectDef::Perform(effect) => matches!(
            *effect,
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            }
        ),
        ReplacementEffectDef::Sequence(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(shared_begin_turn_replacement_effect)
                && effects
                    .iter()
                    .any(|effect| matches!(effect, ReplacementEffectDef::ReplaceEventWithNothing))
        }
        ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::CopyEntering { .. }
        | ReplacementEffectDef::Conditional { .. }
        | ReplacementEffectDef::PayOr { .. } => false,
    }
}

pub(in super::super) fn shared_battlefield_exit_replacement_effect(
    effect: ReplacementEffectDef,
) -> bool {
    match effect {
        ReplacementEffectDef::MoveToZone(zone) => zone == ZoneKind::Exile,
        ReplacementEffectDef::Perform(effect) => matches!(
            *effect,
            EffectDef::TakeExtraTurn {
                player: EffectRecipientDef::Controller,
            }
        ),
        ReplacementEffectDef::Sequence(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(shared_battlefield_exit_replacement_effect)
                && effects
                    .iter()
                    .any(|effect| matches!(effect, ReplacementEffectDef::MoveToZone(_)))
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::CopyEntering { .. }
        | ReplacementEffectDef::Conditional { .. }
        | ReplacementEffectDef::PayOr { .. } => false,
    }
}

pub(super) fn shared_replacement_event(event: ReplacementEventDef) -> bool {
    match event {
        ReplacementEventDef::SourceEntersBattlefield
        | ReplacementEventDef::WouldGainLife(_)
        | ReplacementEventDef::WouldBeginTurn { .. } => true,
        ReplacementEventDef::ObjectEntersBattlefield { object, .. } => {
            shared_object_predicate(object)
        }
        ReplacementEventDef::WouldMove { cause, .. } => shared_zone_move_cause(cause),
        // Only graveyard placement funnels through one procedure the
        // replacement can sit in front of.
        ReplacementEventDef::AnyObjectWouldMove { to } => to == ZoneKind::Graveyard,
        ReplacementEventDef::Special(_) => false,
    }
}

fn assert_nested_installed_ability(card_name: &str, ability: &AbilityDef) {
    assert!(
        shared_definition_ability(ability),
        "{card_name} installs a triggered ability outside the shared runtime boundary: {ability:?}",
    );
    assert_nested_program_abilities(card_name, ability.effect.definition);
}

pub(in super::super) fn assert_nested_program_abilities(
    card_name: &str,
    program: AbilityProgramDef,
) {
    match program {
        AbilityProgramDef::Effects(effect) => assert_nested_definition_abilities(card_name, effect),
        AbilityProgramDef::Replacement(effect) => {
            assert_nested_replacement_definition_abilities(card_name, effect);
        }
    }
}

// Long because the effect vocabulary is wide, not because the function
// does several things: every arm is one variant walked the same way.
#[allow(clippy::too_many_lines)]
pub(in super::super) fn assert_nested_definition_abilities(card_name: &str, effect: EffectDef) {
    match effect {
        EffectDef::Sequence(effects) => {
            for effect in effects {
                assert_nested_definition_abilities(card_name, *effect);
            }
        }
        EffectDef::Randomized {
            on_success,
            on_failure,
            ..
        } => {
            assert_nested_definition_abilities(card_name, *on_success);
            assert_nested_definition_abilities(card_name, *on_failure);
        }
        EffectDef::Choose(choice) => {
            assert_nested_definition_abilities(card_name, *choice.then);
        }
        EffectDef::PayOr(payment) => {
            for effect in payment.if_paid.iter().chain(payment.otherwise.iter()) {
                assert_nested_definition_abilities(card_name, **effect);
            }
        }
        EffectDef::SplitIntoPiles(partition) => {
            assert_nested_definition_abilities(card_name, *partition.then);
        }
        EffectDef::May { effect, .. }
        | EffectDef::IfCondition { then: effect, .. }
        | EffectDef::ReplaceNextDrawThisTurn { effect, .. } => {
            assert_nested_definition_abilities(card_name, *effect);
        }
        EffectDef::IfFormat {
            then, otherwise, ..
        } => {
            assert_nested_definition_abilities(card_name, *then);
            assert_nested_definition_abilities(card_name, *otherwise);
        }
        EffectDef::InstallTrigger(trigger) => {
            assert_nested_installed_ability(card_name, trigger.ability);
        }
        EffectDef::StaticApply { effect, .. } | EffectDef::Apply { effect, .. } => {
            assert_nested_definition_applied_effect(card_name, effect);
        }
        EffectDef::LookAtTopAndSelect { selection, .. } => {
            assert_nested_selection_abilities(card_name, *selection);
        }
        EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::DealDamage { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::GainLife { .. }
        | EffectDef::AddPoisonCounters { .. }
        | EffectDef::DrawCards { .. }
        | EffectDef::Discard { .. }
        | EffectDef::DiscardCards { .. }
        | EffectDef::ShuffleLibrary { .. }
        | EffectDef::EmptyManaPool { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::Regenerate { .. }
        | EffectDef::Tap { .. }
        | EffectDef::RemoveFromCombat { .. }
        | EffectDef::DestroyAtEndOfCombat { .. }
        | EffectDef::SkipNextUntapSteps { .. }
        | EffectDef::RemoveAllCounters { .. }
        | EffectDef::Untap { .. }
        | EffectDef::PreventDamage { .. }
        | EffectDef::Attach { .. }
        | EffectDef::Reconfigure { .. }
        | EffectDef::CreateToken { .. }
        | EffectDef::CreateAttachedToken { .. }
        | EffectDef::CreateTokenCopyOf { .. }
        | EffectDef::Destroy { .. }
        | EffectDef::Sacrifice { .. }
        | EffectDef::SacrificeOfChoice { .. }
        | EffectDef::Mill { .. }
        | EffectDef::LookAtHand { .. }
        | EffectDef::SearchZone { .. }
        | EffectDef::ChooseCards { .. }
        | EffectDef::Counter { .. }
        | EffectDef::AddCounters { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::CreateEmblem { .. }
        | EffectDef::Transform { .. }
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::TakeExtraTurn { .. }
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::Detain { .. }
        | EffectDef::GainControl { .. }
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::MoveToZone { .. }
        | EffectDef::Special(_) => {}
    }
}

fn assert_nested_selection_abilities(card_name: &str, selection: TopCardSelectionDef) {
    if let Some(effect) = selection.then {
        assert_nested_definition_abilities(card_name, *effect);
    }
}

pub(in super::super) fn assert_nested_replacement_definition_abilities(
    card_name: &str,
    effect: ReplacementEffectDef,
) {
    match effect {
        ReplacementEffectDef::Sequence(effects) => {
            for effect in effects {
                assert_nested_replacement_definition_abilities(card_name, *effect);
            }
        }
        ReplacementEffectDef::Perform(effect) => {
            assert_nested_definition_abilities(card_name, *effect);
        }
        ReplacementEffectDef::Conditional {
            if_true, if_false, ..
        } => {
            for effect in if_true.iter().chain(if_false.iter()) {
                assert_nested_replacement_definition_abilities(card_name, *effect);
            }
        }
        ReplacementEffectDef::PayOr {
            if_paid,
            if_declined,
            ..
        } => {
            for effect in if_paid.iter().chain(if_declined.iter()) {
                assert_nested_replacement_definition_abilities(card_name, *effect);
            }
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::CopyEntering { .. } => {}
    }
}

pub(in super::super) fn assert_nested_definition_applied_effect(
    card_name: &str,
    effect: AppliedEffectDef,
) {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                assert_nested_definition_applied_effect(card_name, *effect);
            }
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => {
            if ability.declarative_effect().is_some() {
                assert!(
                    shared_definition_ability(ability),
                    "{card_name} contains a nested shared declarative ability outside the shared runtime boundary: {ability:?}",
                );
            }
            assert_nested_program_abilities(card_name, ability.effect.definition);
        }
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => {}
    }
}
