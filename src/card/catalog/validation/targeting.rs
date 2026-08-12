use crate::TargetIndex;
use crate::card::catalog::GrantedAbilityValidationError;
use crate::card::{AbilityTargetDef, AppliedEffectDef, EffectDef, EffectRecipientDef, ValueDef};

pub(in crate::card::catalog) fn validate_ability_targets(
    targets: &[AbilityTargetDef],
    effect: EffectDef,
) -> Result<(), GrantedAbilityValidationError> {
    if targets.len() > usize::from(u8::MAX) + 1 {
        return Err(GrantedAbilityValidationError::TooManyTargets {
            count: targets.len(),
        });
    }
    for (position, definition) in targets.iter().enumerate() {
        let target = TargetIndex::from_index(position)
            .expect("the target count was validated before assigning positional indices");
        if definition.minimum > definition.maximum {
            return Err(GrantedAbilityValidationError::InvalidTargetBounds {
                target,
                minimum: definition.minimum,
                maximum: definition.maximum,
            });
        }
    }
    validate_effect_references(effect, targets.len(), 0)
}

fn validate_target_index(
    target: TargetIndex,
    target_count: usize,
) -> Result<(), GrantedAbilityValidationError> {
    if target.index() < target_count {
        Ok(())
    } else {
        Err(GrantedAbilityValidationError::TargetReferenceOutOfBounds {
            target,
            target_count,
        })
    }
}

fn validate_recipient_target_references(
    recipient: EffectRecipientDef,
    target_count: usize,
    choices_in_scope: u8,
) -> Result<(), GrantedAbilityValidationError> {
    match recipient {
        EffectRecipientDef::ObjectsSharingNameWithTarget(target)
        | EffectRecipientDef::Target(target)
        | EffectRecipientDef::ControllerOfTarget(target) => {
            validate_target_index(target, target_count)
        }
        EffectRecipientDef::ObjectsControlledByTarget { slot, .. }
        | EffectRecipientDef::ObjectsOwnedByTarget { slot, .. }
        | EffectRecipientDef::CardsOwnedByTarget { slot, .. } => {
            validate_target_index(slot, target_count)
        }
        EffectRecipientDef::ChosenPermanent(choice) => {
            if choices_in_scope & (1 << choice.index()) != 0 {
                Ok(())
            } else {
                Err(GrantedAbilityValidationError::ChoiceReferenceOutOfScope { choice })
            }
        }
        EffectRecipientDef::Source
        | EffectRecipientDef::AttachedPermanent
        | EffectRecipientDef::Controller
        | EffectRecipientDef::Opponent
        | EffectRecipientDef::TriggeringObject
        | EffectRecipientDef::ControllerOfTriggeringObject
        | EffectRecipientDef::EventPlayer
        | EffectRecipientDef::MatchingObjects { .. }
        | EffectRecipientDef::EachPlayer => Ok(()),
    }
}

fn validate_value_target_references(
    value: ValueDef,
    target_count: usize,
) -> Result<(), GrantedAbilityValidationError> {
    match value {
        ValueDef::Negate(value) => validate_value_target_references(*value, target_count),
        ValueDef::IfCreatureDiedThisTurn(condition) => {
            validate_value_target_references(condition.then, target_count)?;
            validate_value_target_references(condition.otherwise, target_count)
        }
        ValueDef::IfTargetMatches(condition) => {
            validate_target_index(condition.slot, target_count)?;
            validate_value_target_references(condition.then, target_count)?;
            validate_value_target_references(condition.otherwise, target_count)
        }
        ValueDef::IfMatchingObjectCount(condition) => {
            validate_value_target_references(condition.then, target_count)?;
            validate_value_target_references(condition.otherwise, target_count)
        }
        ValueDef::TargetPower(target)
        | ValueDef::TargetManaValue(target) => validate_target_index(target, target_count),
        ValueDef::Constant(_)
        | ValueDef::ChosenX
        | ValueDef::SourcePower
        | ValueDef::SourceToughness
        | ValueDef::TriggerEventAmount
        | ValueDef::CardsInHandAbove { .. }
        | ValueDef::CountMatchingObjects(_)
        | ValueDef::AnyMatchingObject(_)
        | ValueDef::CountersOnSource(_)
        // This reads the share assigned to the target currently being
        // affected; the surrounding recipient carries the slot reference.
        | ValueDef::DividedAmongTargets => Ok(()),
    }
}

fn validate_applied_effect_target_references(
    effect: AppliedEffectDef,
    target_count: usize,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                validate_applied_effect_target_references(*effect, target_count)?;
            }
            Ok(())
        }
        AppliedEffectDef::ModifyPowerToughness { power, toughness } => {
            validate_value_target_references(power, target_count)?;
            validate_value_target_references(toughness, target_count)
        }
        // A granted ability introduces its own target scope and is validated
        // separately when the grant tree is traversed.
        AppliedEffectDef::GrantAbility(_)
        | AppliedEffectDef::CannotBeCountered
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
        | AppliedEffectDef::Special(_) => Ok(()),
    }
}

#[allow(clippy::too_many_lines)]
fn validate_effect_references(
    effect: EffectDef,
    target_count: usize,
    choices_in_scope: u8,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        EffectDef::Sequence(effects) => {
            for effect in effects {
                validate_effect_references(*effect, target_count, choices_in_scope)?;
            }
            Ok(())
        }
        EffectDef::Randomized {
            on_success,
            on_failure,
            ..
        } => {
            validate_effect_references(*on_success, target_count, choices_in_scope)?;
            validate_effect_references(*on_failure, target_count, choices_in_scope)
        }
        EffectDef::ChoosePermanent {
            choice,
            chooser,
            then,
            ..
        } => {
            validate_recipient_target_references(chooser, target_count, choices_in_scope)?;
            let bit = 1 << choice.index();
            if choices_in_scope & bit != 0 {
                return Err(GrantedAbilityValidationError::ChoiceBindingAlreadyInScope { choice });
            }
            validate_effect_references(*then, target_count, choices_in_scope | bit)
        }
        EffectDef::DealDamage { recipient, amount }
        | EffectDef::DrainLife { recipient, amount }
        | EffectDef::GainLife { recipient, amount }
        | EffectDef::DrawCards { recipient, amount }
        | EffectDef::Discard {
            recipient, amount, ..
        }
        | EffectDef::LoseLife { recipient, amount } => {
            validate_recipient_target_references(recipient, target_count, choices_in_scope)?;
            validate_value_target_references(amount, target_count)
        }
        EffectDef::LoseTheGame { player: object }
        | EffectDef::ShuffleLibrary { player: object }
        | EffectDef::EmptyManaPool { player: object }
        | EffectDef::Tap { object }
        | EffectDef::Untap { object }
        | EffectDef::PreventCombatDamageThisTurn { object }
        | EffectDef::PreventCombatDamageDealtByThisTurn { object }
        | EffectDef::Attach { object }
        | EffectDef::Destroy { object, .. }
        | EffectDef::Sacrifice { object }
        | EffectDef::ChangeTextBasicLandType { object }
        | EffectDef::BecomeCopyOf { object, .. }
        | EffectDef::ExileLinkedToSource { object }
        | EffectDef::MakeUnblockableThisTurn { object }
        | EffectDef::GainControlThisTurn { object }
        | EffectDef::Transform { object }
        | EffectDef::MoveToZone { object, .. }
        | EffectDef::Counter { object, .. }
        | EffectDef::ChooseCardName { object }
        | EffectDef::ChooseCreatureType { object } => {
            validate_recipient_target_references(object, target_count, choices_in_scope)
        }
        // A reveal always comes off the resolving object's controller's own
        // library, so its count is the only part that could name a target.
        EffectDef::RevealAndSplitIntoPiles { count, .. }
        | EffectDef::CreateToken { count, .. }
        | EffectDef::ReduceGenericCostBy(count) => {
            validate_value_target_references(count, target_count)
        }
        EffectDef::SacrificeOfChoice { player, then, .. } => {
            validate_recipient_target_references(player, target_count, choices_in_scope)?;
            if let Some(effect) = then {
                validate_effect_references(*effect, target_count, choices_in_scope)?;
            }
            Ok(())
        }
        EffectDef::DestroyOfChoice { player, .. }
        | EffectDef::SplitPermanentsAndSacrificeAPile { player }
        | EffectDef::CannotCastNoncreatureSpellsThisTurn { player }
        | EffectDef::SearchLibrary { player, .. }
        | EffectDef::LookAtHand { player }
        | EffectDef::LookAtTopAndMayTake { player, .. } => {
            validate_recipient_target_references(player, target_count, choices_in_scope)
        }
        EffectDef::LookAtTopAndSelect { player, selection } => {
            validate_recipient_target_references(player, target_count, choices_in_scope)?;
            validate_value_target_references(selection.count, target_count)?;
            if let Some(effect) = selection.then {
                validate_effect_references(*effect, target_count, choices_in_scope)?;
            }
            Ok(())
        }
        EffectDef::Mill { player, amount } => {
            validate_recipient_target_references(player, target_count, choices_in_scope)?;
            validate_value_target_references(amount, target_count)
        }
        EffectDef::CounterUnlessPaid { object, amount, .. }
        | EffectDef::AddCounters { object, amount, .. } => {
            validate_recipient_target_references(object, target_count, choices_in_scope)?;
            validate_value_target_references(amount, target_count)
        }
        EffectDef::OptionalPayment {
            if_paid: effect, ..
        }
        | EffectDef::UnlessPaid {
            otherwise: effect, ..
        }
        | EffectDef::May(effect)
        | EffectDef::IfCondition { then: effect, .. }
        | EffectDef::AtNextStep { effect, .. } => {
            validate_effect_references(*effect, target_count, choices_in_scope)
        }
        EffectDef::Apply {
            recipient, effect, ..
        } => {
            validate_recipient_target_references(recipient, target_count, choices_in_scope)?;
            validate_applied_effect_target_references(effect, target_count)
        }
        // An installed ability chooses its own targets when it triggers, so
        // nothing in it can refer to this ability's target slots.
        // The chosen player is recorded on the permanent, not read from a
        // target slot.
        // A prohibition names a card shape, never a target.
        EffectDef::PlayersCantPlay(_)
        | EffectDef::ChoosePlayer { .. }
        | EffectDef::CopyPermanentAsItEnters { .. }
        | EffectDef::TriggerUntilYourNextTurn { .. }
        | EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::AdditionalCombatPhase
        | EffectDef::MultiplyEventAmount(_)
        | EffectDef::Replacement(_)
        | EffectDef::Special(_) => Ok(()),
    }
}
