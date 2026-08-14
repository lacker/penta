use super::*;
use crate::card::{
    ChooseDef, EffectPaymentDef, ObjectChoiceBindingDef, PartitionItemsDef, SplitIntoPilesDef,
};

pub(in super::super) fn shared_stack_effect(effect: EffectDef) -> bool {
    shared_stack_effect_at_position(effect, true)
}

fn shared_effect_payment(payment: EffectPaymentDef) -> bool {
    !matches!(
        payment.payer,
        PlayerSetDef::All | PlayerSetDef::Related(PlayerRelation::Any)
    ) && shared_effect_recipient(EffectRecipientDef::players(payment.payer))
}

fn shared_choose(choice: ChooseDef) -> bool {
    choice.maximum > 0
        && choice.minimum <= choice.maximum
        && match choice.binding {
            ObjectChoiceBindingDef::Object(_) => choice.maximum == 1,
            ObjectChoiceBindingDef::Objects(_) => true,
        }
        && shared_effect_recipient(EffectRecipientDef::player(choice.chooser))
        && shared_effect_recipient(EffectRecipientDef::objects(choice.candidates))
        && choice
            .exclude
            .is_none_or(|object| shared_effect_recipient(EffectRecipientDef::object(object)))
}

fn shared_partition(partition: SplitIntoPilesDef) -> bool {
    let items_are_shared = match partition.items {
        PartitionItemsDef::Objects(objects) => {
            shared_effect_recipient(EffectRecipientDef::objects(objects))
        }
        PartitionItemsDef::TopOfLibrary { player, .. } => {
            shared_effect_recipient(EffectRecipientDef::player(player))
        }
    };
    items_are_shared
        && !matches!(
            partition.divider,
            PlayerSetDef::All | PlayerSetDef::Related(PlayerRelation::Any)
        )
        && !matches!(
            partition.chooser,
            PlayerSetDef::All | PlayerSetDef::Related(PlayerRelation::Any)
        )
        && shared_effect_recipient(EffectRecipientDef::players(partition.divider))
        && shared_effect_recipient(EffectRecipientDef::players(partition.chooser))
}

fn shared_damage_prevention(prevention: crate::card::DamagePreventionDef) -> bool {
    let source_is_shared = match prevention.matcher.source {
        DamageSourceMatcherDef::Any | DamageSourceMatcherDef::Group(_) => true,
        DamageSourceMatcherDef::Object(source) | DamageSourceMatcherDef::Except(source) => {
            shared_effect_recipient(EffectRecipientDef::object(source))
        }
        DamageSourceMatcherDef::Matching(source) => shared_object_predicate(source),
        DamageSourceMatcherDef::AffectedObject => false,
    };
    let recipient_is_shared = match prevention.matcher.recipient {
        DamageRecipientMatcherDef::Any => true,
        DamageRecipientMatcherDef::Recipients(recipient) => shared_effect_recipient(recipient),
        DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(player) => {
            shared_effect_recipient(EffectRecipientDef::player(player))
        }
        DamageRecipientMatcherDef::AffectedObject => false,
    };
    let capacity_is_shared = match prevention.capacity {
        DamagePreventionCapacityDef::Amount(_) | DamagePreventionCapacityDef::Unlimited => true,
        DamagePreventionCapacityDef::Events(events) => events > 0,
    };
    let follow_up_is_shared = prevention
        .follow_up
        .is_none_or(|follow_up| match follow_up {
            DamagePreventionFollowUpDef::GainLife(player) => {
                shared_effect_recipient(EffectRecipientDef::player(player))
            }
        });
    source_is_shared && recipient_is_shared && capacity_is_shared && follow_up_is_shared
}

/// Resolving sequences preserve their unprocessed tail, so a queued decision
/// may suspend at any sequence component. Other callers still pass false when
/// their own continuation cannot be suspended.
/// The effects whose whole procedure is a decision the shared runtime
/// asks for. Their callers have already established that a deferred
/// decision is allowed where they sit; this checks only their arguments.
fn shared_decision_effect(effect: EffectDef) -> bool {
    match effect {
        EffectDef::LookAtTopAndSelect { player, selection } => {
            let supported_zone = |zone| {
                matches!(
                    zone,
                    ZoneKind::Hand | ZoneKind::Library | ZoneKind::Graveyard | ZoneKind::Exile
                )
            };
            shared_effect_recipient(player)
                && selection.object.is_none_or(shared_object_predicate)
                && selection.minimum <= selection.maximum
                && supported_zone(selection.selected_zone)
                && supported_zone(selection.rest_zone)
                && selection
                    .then
                    .is_none_or(|effect| shared_stack_effect_at_position(*effect, true))
        }
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
        EffectDef::PreventDamage { prevention, .. } => shared_damage_prevention(prevention),
        EffectDef::Choose(choice) => {
            deferred_decision_allowed
                && shared_choose(choice)
                && shared_stack_effect_at_position(*choice.then, true)
        }
        EffectDef::PayOr(payment) => {
            deferred_decision_allowed
                && shared_effect_payment(payment.payment)
                && (payment.if_paid.is_some() || payment.otherwise.is_some())
                && payment
                    .if_paid
                    .iter()
                    .chain(payment.otherwise.iter())
                    .all(|effect| shared_stack_effect_at_position(**effect, true))
        }
        EffectDef::SplitIntoPiles(partition) => {
            deferred_decision_allowed
                && shared_partition(partition)
                && shared_stack_effect_at_position(*partition.then, true)
        }
        EffectDef::AddMana(_) => shared_mana_effect(effect, false),
        EffectDef::DealDamage { recipient, .. }
        | EffectDef::DrainLife { recipient, .. }
        | EffectDef::GainLife { recipient, .. }
        | EffectDef::AddPoisonCounters { recipient, .. }
        | EffectDef::DrawCards { recipient, .. }
        | EffectDef::Discard { recipient, .. }
        | EffectDef::ShuffleLibrary { player: recipient }
        | EffectDef::EmptyManaPool { player: recipient }
        | EffectDef::TakeExtraTurn { player: recipient }
        | EffectDef::LoseLife { recipient, .. }
        | EffectDef::Mill {
            player: recipient, ..
        }
        | EffectDef::LoseTheGame { player: recipient }
        | EffectDef::LookAtHand { player: recipient } => shared_effect_recipient(recipient),
        EffectDef::SacrificeOfChoice { .. } => shared_sacrifice_of_choice(effect),
        EffectDef::LookAtTopAndSelect { .. } => {
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
        // Populate copies whatever the choice landed on, so like the rest of
        // these only its recipient has to be one the runtime understands.
        EffectDef::CreateTokenCopyOf { object }
        | EffectDef::Regenerate { object }
        | EffectDef::Tap { object }
        | EffectDef::RemoveFromCombat { object }
        | EffectDef::DestroyAtEndOfCombat { object, .. }
        | EffectDef::SkipNextUntapSteps { object, .. }
        | EffectDef::RemoveAllCounters { object, .. }
        | EffectDef::Untap { object }
        | EffectDef::Destroy { object, .. }
        | EffectDef::Sacrifice { object }
        | EffectDef::DiscardCards { object }
        | EffectDef::ExileLinkedToSource { object }
        | EffectDef::Detain { object }
        | EffectDef::GainControl { object, .. }
        | EffectDef::AddCounters { object, .. }
        | EffectDef::Attach { object }
        | EffectDef::Reconfigure { object }
        | EffectDef::ChangeTextBasicLandType { object }
        | EffectDef::BecomeCopyOf { object, .. } => shared_effect_recipient(object),
        EffectDef::Counter { object, zone } => {
            matches!(zone, ZoneKind::Graveyard | ZoneKind::Exile) && shared_effect_recipient(object)
        }
        // Neither needs a recipient: both concern the resolving controller.
        // The amount is computed when the effect resolves, so nothing has
        // to read it ahead of time the way a mana ability does.
        EffectDef::AddManaEqualTo { .. }
        | EffectDef::CreateToken { .. }
        | EffectDef::CreateAttachedToken { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::Transform { .. }
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::GrantFlashToNextSorcery => true,
        // Each of these asks a question and then runs an inner effect,
        // so the question has to be allowed here and the answer has to be
        // something the shared procedure can carry out.
        EffectDef::May { player, effect } => {
            deferred_decision_allowed
                && shared_effect_recipient(player)
                && shared_stack_effect_at_position(*effect, true)
        }
        // Scheduling creates a fresh resolution boundary. A decision may
        // therefore be the delayed effect's root even when scheduling it
        // is itself one component of a sequence.
        EffectDef::IfCondition { then: effect, .. } => {
            shared_stack_effect_at_position(*effect, deferred_decision_allowed)
        }
        // Installing an ability is a resolution like any other; what it
        // installs has to be an ability the shared runtime can fire.
        EffectDef::InstallTrigger(trigger) => shared_definition_ability(trigger.ability),
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
        | EffectDef::StaticApply { .. }
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::Special(_) => false,
    }
}
