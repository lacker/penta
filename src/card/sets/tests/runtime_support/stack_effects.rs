use super::*;
use crate::{CostDef, PaymentDef};

pub(in super::super) fn shared_stack_effect(effect: EffectDef) -> bool {
    shared_stack_effect_at_position(effect, true)
}

fn shared_optional_payment(payment: PaymentDef, if_paid: &'static EffectDef) -> bool {
    !matches!(
        payment.payer,
        PlayerRelation::Any | PlayerRelation::ChosenPlayer | PlayerRelation::EventPlayer
    ) && matches!(payment.costs, [CostDef::Mana(_)])
        && shared_stack_effect_at_position(*if_paid, true)
}

/// Resolving sequences preserve their unprocessed tail, so a queued decision
/// may suspend at any sequence component. Other callers still pass false when
/// their own continuation cannot be suspended.
/// The effects whose whole procedure is a decision the shared runtime
/// asks for. Their callers have already established that a deferred
/// decision is allowed where they sit; this checks only their arguments.
fn shared_decision_effect(effect: EffectDef) -> bool {
    match effect {
        // Both halves of the split are asked for, so only the player
        // needs checking.
        EffectDef::SplitPermanentsAndSacrificeAPile { player } => shared_effect_recipient(player),
        // The reveal, the split, and the choice are all asked for, and
        // the library is the resolving object's controller's own.
        EffectDef::RevealAndSplitIntoPiles { .. } => true,
        // Looking is private and the offer is the only visible part, and
        // a chosen destruction reaches only the chooser's own battlefield.
        EffectDef::LookAtTopAndMayTake { player, object }
        | EffectDef::DestroyOfChoice { player, object, .. } => {
            shared_effect_recipient(player) && shared_object_predicate(object)
        }
        EffectDef::LookAtTopAndSelect { player, selection } => {
            let supported_zone = |zone| {
                matches!(
                    zone,
                    ZoneKind::Hand | ZoneKind::Library | ZoneKind::Graveyard | ZoneKind::Exile
                )
            };
            shared_effect_recipient(player)
                && selection.minimum <= selection.maximum
                && supported_zone(selection.selected_zone)
                && supported_zone(selection.rest_zone)
                && selection
                    .then
                    .is_none_or(|effect| shared_stack_effect_at_position(*effect, true))
        }
        EffectDef::ChoosePermanent {
            chooser, object, ..
        } => shared_effect_recipient(chooser) && shared_object_predicate(object),
        _ => false,
    }
}

/// The chooser is a player and the choices are their own battlefield, so
/// only the predicate needs checking. The follow-up runs inside the
/// sacrifice's continuation, which can establish a fresh decision.
fn shared_sacrifice_of_choice(effect: EffectDef) -> bool {
    let EffectDef::SacrificeOfChoice {
        player,
        object,
        then,
        ..
    } = effect
    else {
        return false;
    };
    shared_effect_recipient(player)
        && shared_object_predicate(object)
        && then.is_none_or(|effect| shared_stack_effect_at_position(*effect, true))
}

#[allow(clippy::too_many_lines)]
fn shared_stack_effect_at_position(effect: EffectDef, deferred_decision_allowed: bool) -> bool {
    match effect {
        EffectDef::Sequence(effects) => {
            !effects.is_empty()
                && effects.iter().copied().all(|effect| {
                    shared_stack_effect_at_position(effect, deferred_decision_allowed)
                })
        }
        EffectDef::Randomized {
            on_success,
            on_failure,
            ..
        } => {
            let branch_is_shared = |branch: EffectDef| {
                branch == EffectDef::None
                    || shared_stack_effect_at_position(branch, deferred_decision_allowed)
            };
            branch_is_shared(*on_success) && branch_is_shared(*on_failure)
        }
        EffectDef::ChoosePermanent { then, .. } => {
            deferred_decision_allowed
                && shared_decision_effect(effect)
                && shared_stack_effect_at_position(*then, true)
        }
        EffectDef::AddMana(_) => shared_mana_effect(effect, false),
        EffectDef::DealDamage { recipient, .. }
        | EffectDef::DrainLife { recipient, .. }
        | EffectDef::GainLife { recipient, .. }
        | EffectDef::DrawCards { recipient, .. }
        | EffectDef::Discard { recipient, .. }
        | EffectDef::ShuffleLibrary { player: recipient }
        | EffectDef::EmptyManaPool { player: recipient }
        | EffectDef::LoseLife { recipient, .. }
        | EffectDef::Mill {
            player: recipient, ..
        }
        | EffectDef::CannotCastNoncreatureSpellsThisTurn { player: recipient }
        | EffectDef::LoseTheGame { player: recipient }
        | EffectDef::LookAtHand { player: recipient } => shared_effect_recipient(recipient),
        EffectDef::SacrificeOfChoice { .. } => shared_sacrifice_of_choice(effect),
        // The choice is asked of whoever controls the candidates, and the
        // candidates are their own battlefield, so only the player and
        // the predicate need checking.
        EffectDef::DestroyOfChoice { .. }
        | EffectDef::SplitPermanentsAndSacrificeAPile { .. }
        | EffectDef::RevealAndSplitIntoPiles { .. }
        | EffectDef::LookAtTopAndMayTake { .. }
        | EffectDef::LookAtTopAndSelect { .. } => {
            deferred_decision_allowed && shared_decision_effect(effect)
        }
        EffectDef::SearchZone {
            player,
            source,
            object,
            minimum,
            maximum,
            destination,
            shuffle,
            ..
        } => {
            deferred_decision_allowed
                && shared_effect_recipient(player)
                && shared_object_predicate(object)
                && minimum <= maximum
                && (destination != ZoneKind::Library || maximum <= 1)
                && (destination != ZoneKind::Battlefield || maximum <= 1)
                && (!shuffle || source == ZoneKind::Library)
                && matches!(
                    source,
                    ZoneKind::Library | ZoneKind::Hand | ZoneKind::Graveyard | ZoneKind::Exile
                )
                && matches!(
                    destination,
                    ZoneKind::Library
                        | ZoneKind::Hand
                        | ZoneKind::Battlefield
                        | ZoneKind::Graveyard
                        | ZoneKind::Exile
                )
        }
        EffectDef::ChooseCards {
            player,
            sources,
            object,
            minimum,
            maximum,
            destination,
            ..
        } => {
            deferred_decision_allowed
                && shared_effect_recipient(player)
                && shared_object_predicate(object)
                && minimum <= maximum
                && !sources.is_empty()
                && sources.iter().all(|source| {
                    matches!(
                        source,
                        CardChoiceSourceDef::OutsideGame
                            | CardChoiceSourceDef::Zone(ZoneKind::Exile)
                    )
                })
                && destination == ZoneKind::Hand
        }
        EffectDef::ReplaceNextDrawThisTurn { player, effect } => {
            shared_effect_recipient(player) && shared_stack_effect_at_position(*effect, true)
        }
        EffectDef::IfFormat {
            then, otherwise, ..
        } => {
            shared_stack_effect_at_position(*then, deferred_decision_allowed)
                && shared_stack_effect_at_position(*otherwise, deferred_decision_allowed)
        }
        // Only the two destinations the return path knows.
        EffectDef::ReturnLinkedExiles { zone, .. } => {
            matches!(zone, ZoneKind::Battlefield | ZoneKind::Hand)
        }
        EffectDef::Tap { object }
        | EffectDef::Untap { object }
        | EffectDef::PreventCombatDamageThisTurn { object }
        | EffectDef::PreventCombatDamageDealtByThisTurn { object }
        | EffectDef::Destroy { object, .. }
        | EffectDef::Sacrifice { object }
        | EffectDef::ExileLinkedToSource { object }
        | EffectDef::MakeUnblockableThisTurn { object }
        | EffectDef::GainControlThisTurn { object }
        | EffectDef::AddCounters { object, .. }
        | EffectDef::Attach { object }
        | EffectDef::ChangeTextBasicLandType { object }
        | EffectDef::BecomeCopyOf { object, .. } => shared_effect_recipient(object),
        EffectDef::Counter { object, zone } | EffectDef::CounterUnlessPaid { object, zone, .. } => {
            matches!(zone, ZoneKind::Graveyard | ZoneKind::Exile) && shared_effect_recipient(object)
        }
        // Neither needs a recipient: both concern the resolving controller.
        // The amount is computed when the effect resolves, so nothing has
        // to read it ahead of time the way a mana ability does.
        EffectDef::AddManaEqualTo { .. }
        | EffectDef::CreateToken { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::Transform { .. }
        | EffectDef::AdditionalCombatPhase
        | EffectDef::GrantFlashToNextSorcery => true,
        // Each of these asks a question and then runs an inner effect,
        // so the question has to be allowed here and the answer has to be
        // something the shared procedure can carry out.
        EffectDef::May { player, effect } => {
            deferred_decision_allowed
                && shared_effect_recipient(player)
                && shared_stack_effect_at_position(*effect, true)
        }
        EffectDef::UnlessPaid {
            otherwise: effect, ..
        } => deferred_decision_allowed && shared_stack_effect_at_position(*effect, true),
        EffectDef::OptionalPayment { payment, if_paid } => {
            deferred_decision_allowed && shared_optional_payment(payment, if_paid)
        }
        // Scheduling creates a fresh resolution boundary. A decision may
        // therefore be the delayed effect's root even when scheduling it
        // is itself one component of a sequence.
        EffectDef::IfCondition { then: effect, .. } => {
            shared_stack_effect_at_position(*effect, deferred_decision_allowed)
        }
        EffectDef::AtNextStep { effect, .. } => shared_stack_effect_at_position(*effect, true),
        // Installing an ability is a resolution like any other; what it
        // installs has to be an ability the shared runtime can fire.
        EffectDef::TriggerUntilYourNextTurn { ability } => shared_definition_ability(ability),
        EffectDef::Apply {
            recipient,
            effect,
            duration,
        } => shared_resolving_apply(recipient, effect, duration),
        // Only the moves the runtime actually performs are inside the
        // boundary. A move to the stack or command zone is still a seam.
        EffectDef::MoveToZone { object, zone, .. } => {
            matches!(
                zone,
                ZoneKind::Battlefield
                    | ZoneKind::Hand
                    | ZoneKind::Graveyard
                    | ZoneKind::Exile
                    | ZoneKind::Library
            ) && shared_effect_recipient(object)
        }
        EffectDef::None
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::PlayersCantPlay(_)
        | EffectDef::MultiplyEventAmount(_)
        | EffectDef::Replacement(_)
        | EffectDef::ChooseCardName { .. }
        | EffectDef::ChoosePlayer { .. }
        | EffectDef::CopyPermanentAsItEnters { .. }
        | EffectDef::ChooseCreatureType { .. }
        | EffectDef::Special(_) => false,
    }
}
