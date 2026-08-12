use super::*;

pub(in super::super) fn shared_trigger_event(event: TriggerEventDef) -> bool {
    match event {
        TriggerEventDef::ZoneChanged { object, from, to } => {
            const COMMITTED_TRANSITIONS: [(ZoneKind, ZoneKind); 5] = [
                (ZoneKind::Hand, ZoneKind::Battlefield),
                (ZoneKind::Stack, ZoneKind::Battlefield),
                (ZoneKind::Battlefield, ZoneKind::Graveyard),
                (ZoneKind::Battlefield, ZoneKind::Exile),
                (ZoneKind::Battlefield, ZoneKind::Hand),
            ];
            shared_object_predicate(object)
                && COMMITTED_TRANSITIONS
                    .iter()
                    .any(|(actual_from, actual_to)| {
                        from.is_none_or(|expected| expected == *actual_from)
                            && to.is_none_or(|expected| expected == *actual_to)
                    })
        }
        TriggerEventDef::BecomesTapped(object)
        | TriggerEventDef::Attacks(object)
        | TriggerEventDef::AttacksFirstTimeThisTurn(object)
        | TriggerEventDef::TappedForMana(object)
        | TriggerEventDef::SpellCast(object) => shared_object_predicate(object),
        TriggerEventDef::StepBegins { .. }
        | TriggerEventDef::LifeGained(_)
        | TriggerEventDef::StateCondition
        | TriggerEventDef::TransformsIntoThisFace
        | TriggerEventDef::DamagedCreatureDied => true,
        // Only "whenever this creature is dealt damage" is committed; a
        // wider recipient has no event behind it yet.
        TriggerEventDef::DamageDealt { source, recipient } => {
            recipient == EffectRecipientDef::Source && source == ObjectPredicateDef::Any
        }
        TriggerEventDef::CombatDamageDealtToPlayer { source }
        | TriggerEventDef::CombatDamageDealtToSource { source }
        | TriggerEventDef::DamageDealtToPlayer { source, .. } => shared_object_predicate(source),
        TriggerEventDef::AbilityActivated(_)
        | TriggerEventDef::ManaAdded(_)
        | TriggerEventDef::Special(_) => false,
    }
}

pub(super) fn shared_entry_replacement_effect(effect: ReplacementEffectDef) -> bool {
    match effect {
        ReplacementEffectDef::None | ReplacementEffectDef::ModifyBattlefieldEntry(_) => true,
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
        ReplacementEffectDef::OptionalPayment {
            payment,
            if_paid,
            if_declined,
        } => {
            let payable_life = payment.costs.iter().try_fold(0_u32, |total, cost| {
                let AbilityCostDef::PayLife(amount) = cost else {
                    return None;
                };
                total.checked_add(u32::from(*amount))
            });
            payment.payer != PlayerRelation::Any
                && !payment.costs.is_empty()
                && payable_life.is_some_and(|amount| amount > 0 && i16::try_from(amount).is_ok())
                && if_paid.iter().copied().all(shared_entry_replacement_effect)
                && if_declined
                    .iter()
                    .copied()
                    .all(shared_entry_replacement_effect)
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::Perform(_) => false,
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
        ReplacementEffectDef::None
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::Conditional { .. }
        | ReplacementEffectDef::OptionalPayment { .. } => false,
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
        ReplacementEffectDef::None
        | ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::Conditional { .. }
        | ReplacementEffectDef::OptionalPayment { .. } => false,
    }
}

pub(super) fn shared_replacement_event(event: ReplacementEventDef) -> bool {
    match event {
        ReplacementEventDef::SourceEntersBattlefield
        | ReplacementEventDef::WouldGainLife(_)
        | ReplacementEventDef::WouldBeginTurn { .. }
        | ReplacementEventDef::EntersBattlefield => true,
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
    assert_nested_definition_abilities(card_name, ability.effect.definition);
}

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
        EffectDef::OptionalPayment {
            if_paid: effect, ..
        }
        | EffectDef::UnlessPaid {
            otherwise: effect, ..
        }
        | EffectDef::May { effect, .. }
        | EffectDef::ChoosePermanent { then: effect, .. }
        | EffectDef::IfCondition { then: effect, .. }
        | EffectDef::AtNextStep { effect, .. }
        | EffectDef::ReplaceNextDrawThisTurn { effect, .. } => {
            assert_nested_definition_abilities(card_name, *effect);
        }
        EffectDef::IfFormat {
            then, otherwise, ..
        } => {
            assert_nested_definition_abilities(card_name, *then);
            assert_nested_definition_abilities(card_name, *otherwise);
        }
        EffectDef::TriggerUntilYourNextTurn { ability } => {
            assert_nested_installed_ability(card_name, ability);
        }
        EffectDef::Apply { effect, .. } => {
            assert_nested_definition_applied_effect(card_name, effect);
        }
        EffectDef::Replacement(effect) => {
            assert_nested_replacement_definition_abilities(card_name, effect);
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
        | EffectDef::DrawCards { .. }
        | EffectDef::Discard { .. }
        | EffectDef::ShuffleLibrary { .. }
        | EffectDef::EmptyManaPool { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::Tap { .. }
        | EffectDef::Untap { .. }
        | EffectDef::PreventCombatDamageThisTurn { .. }
        | EffectDef::PreventCombatDamageDealtByThisTurn { .. }
        | EffectDef::Attach { .. }
        | EffectDef::CreateToken { .. }
        | EffectDef::Destroy { .. }
        | EffectDef::Sacrifice { .. }
        | EffectDef::SacrificeOfChoice { .. }
        | EffectDef::DestroyOfChoice { .. }
        | EffectDef::SplitPermanentsAndSacrificeAPile { .. }
        | EffectDef::RevealAndSplitIntoPiles { .. }
        | EffectDef::Mill { .. }
        | EffectDef::LookAtTopAndMayTake { .. }
        | EffectDef::LookAtHand { .. }
        | EffectDef::SearchZone { .. }
        | EffectDef::ChooseCards { .. }
        | EffectDef::Counter { .. }
        | EffectDef::CounterUnlessPaid { .. }
        | EffectDef::AddCounters { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::CreateEmblem { .. }
        | EffectDef::Transform { .. }
        | EffectDef::AdditionalCombatPhase
        | EffectDef::TakeExtraTurn { .. }
        | EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::MakeUnblockableThisTurn { .. }
        | EffectDef::GainControlThisTurn { .. }
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::PlayersCantPlay(_)
        | EffectDef::MultiplyEventAmount(_)
        | EffectDef::MoveToZone { .. }
        | EffectDef::ChooseCardName { .. }
        | EffectDef::ChoosePlayer { .. }
        | EffectDef::CopyPermanentAsItEnters { .. }
        | EffectDef::ChooseCreatureType { .. }
        | EffectDef::Special(_) => {}
    }
}

fn assert_nested_selection_abilities(card_name: &str, selection: TopCardSelectionDef) {
    if let Some(effect) = selection.then {
        assert_nested_definition_abilities(card_name, *effect);
    }
}

fn assert_nested_replacement_definition_abilities(card_name: &str, effect: ReplacementEffectDef) {
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
        ReplacementEffectDef::OptionalPayment {
            if_paid,
            if_declined,
            ..
        } => {
            for effect in if_paid.iter().chain(if_declined.iter()) {
                assert_nested_replacement_definition_abilities(card_name, *effect);
            }
        }
        ReplacementEffectDef::None
        | ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_) => {}
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
        AppliedEffectDef::GrantAbility(ability) => {
            if ability.declarative_effect().is_some() {
                assert!(
                    shared_definition_ability(ability),
                    "{card_name} contains a nested shared declarative ability outside the shared runtime boundary: {ability:?}",
                );
            }
            assert_nested_definition_abilities(card_name, ability.effect.definition);
        }
        AppliedEffectDef::CannotBeCountered
        | AppliedEffectDef::DoesNotUntapDuringUntapStep
        | AppliedEffectDef::CannotBeEnchanted
        | AppliedEffectDef::CannotBecomeEnchanted
        | AppliedEffectDef::CannotChangeController
        | AppliedEffectDef::CannotBeBlockedBy(_)
        | AppliedEffectDef::PreventDamageFrom(_)
        | AppliedEffectDef::AddLandTypes(_)
        | AppliedEffectDef::SetLandTypes(_)
        | AppliedEffectDef::RemoveAbilities(_)
        | AppliedEffectDef::Animate(_)
        | AppliedEffectDef::ModifyPowerToughness { .. }
        | AppliedEffectDef::Special(_) => {}
    }
}
