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

pub(in super::super) fn shared_replacement_event(event: ReplacementEventDef) -> bool {
    match event {
        ReplacementEventDef::SourceEntersBattlefield
        | ReplacementEventDef::WouldGainLife(_)
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
        EffectDef::LookAtTopAndSelect { selection, .. } => {
            if let Some(effect) = selection.then {
                assert_nested_definition_abilities(card_name, *effect);
            }
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
        | EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::MakeUnblockableThisTurn { .. }
        | EffectDef::GainControlThisTurn { .. }
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::PlayersCantPlay(_)
        | EffectDef::MultiplyEventAmount(_)
        | EffectDef::Replacement(_)
        | EffectDef::MoveToZone { .. }
        | EffectDef::ChooseCardName { .. }
        | EffectDef::ChoosePlayer { .. }
        | EffectDef::CopyPermanentAsItEnters { .. }
        | EffectDef::ChooseCreatureType { .. }
        | EffectDef::Special(_) => {}
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
