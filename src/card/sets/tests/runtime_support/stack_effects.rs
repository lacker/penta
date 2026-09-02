use super::*;
use crate::card::{
    ArrivalAttachmentDef, ChooseDef, DiscardSelectionDef, EffectPaymentDef, ObjectChoiceBindingDef,
    PerPlayerSelectionDef, ValueDef,
};

pub(in super::super) fn shared_stack_effect(effect: EffectDef) -> bool {
    shared_stack_effect_at_position(effect, true)
}

fn shared_object_collection_continuation(
    effect: EffectDef,
    deferred_decision_allowed: bool,
) -> bool {
    effect == EffectDef::None || shared_stack_effect_at_position(effect, deferred_decision_allowed)
}

fn shared_object_collection(collection: crate::card::ObjectCollectionSourceDef) -> bool {
    match collection {
        crate::card::ObjectCollectionSourceDef::ObjectSet(input) => {
            shared_effect_recipient(EffectRecipientDef::objects(input))
        }
        crate::card::ObjectCollectionSourceDef::TopCards { player, .. } => {
            shared_effect_recipient(EffectRecipientDef::player(player))
        }
        crate::card::ObjectCollectionSourceDef::TopCardsThroughFirstMatching { player, object } => {
            shared_effect_recipient(EffectRecipientDef::player(player))
                && shared_object_predicate(object)
        }
    }
}

fn shared_effect_payment(payment: EffectPaymentDef) -> bool {
    !matches!(
        payment.payer,
        PlayerSetDef::All | PlayerSetDef::Related(PlayerRelation::Any)
    ) && shared_effect_recipient(EffectRecipientDef::players(payment.payer))
        && match payment.cost {
            crate::card::EffectPaymentCostDef::RemoveAnyNumberOfCounters { object, .. } => {
                shared_effect_recipient(object)
            }
            _ => true,
        }
}

fn shared_choose(choice: ChooseDef) -> bool {
    choice.maximum > 0
        && choice.minimum <= choice.maximum
        && match choice.binding {
            ObjectChoiceBindingDef::Object(_) => choice.maximum == 1,
            ObjectChoiceBindingDef::Objects(_) | ObjectChoiceBindingDef::OrderedObjects(_) => true,
        }
        && shared_effect_recipient(EffectRecipientDef::player(choice.chooser))
        && shared_effect_recipient(EffectRecipientDef::objects(choice.candidates))
        && choice
            .exclude
            .is_none_or(|object| shared_effect_recipient(EffectRecipientDef::object(object)))
}

fn shared_damage_prevention(prevention: crate::card::DamagePreventionDef) -> bool {
    fn shared_amount(value: ValueDef) -> bool {
        match value {
            ValueDef::Constant(_) | ValueDef::DamageEventAmount => true,
            ValueDef::Negate(value) => shared_amount(*value),
            ValueDef::Scaled(value) => shared_amount(value.value),
            ValueDef::Sum(value) => shared_amount(value.left) && shared_amount(value.right),
            ValueDef::Halved(value) => shared_amount(value.value),
            ValueDef::Quotient(value) => {
                shared_amount(value.numerator) && shared_amount(value.denominator)
            }
            _ => false,
        }
    }

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
        // None of these reaches a prevention: they exist for a static
        // shield and for a trigger, and each is refused where it does not
        // belong.
        DamageRecipientMatcherDef::MatchingObject(_)
        | DamageRecipientMatcherDef::AffectedObject
        | DamageRecipientMatcherDef::PlayerOrPlaneswalker => false,
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
    source_is_shared
        && recipient_is_shared
        && capacity_is_shared
        && shared_amount(prevention.amount)
        && follow_up_is_shared
}

/// Resolving sequences preserve their unprocessed tail, so a queued decision
/// may suspend at any sequence component. Other callers still pass false when
/// their own continuation cannot be suspended.
/// The chooser is a player and the choices are their own battlefield, so
/// only the predicate needs checking. The follow-up runs inside the
/// sacrifice's continuation, which can establish a fresh decision.
fn shared_sacrifice_of_choice(effect: EffectDef) -> bool {
    let EffectDef::SacrificeOfChoice {
        player,
        object,
        then,
        otherwise,
        ..
    } = effect
    else {
        return false;
    };
    shared_effect_recipient(player)
        && shared_object_predicate(object)
        && then.is_none_or(|effect| shared_stack_effect_at_position(*effect, true))
        // The declined branch runs in the same continuation, so it is bound
        // by exactly the same rule as the follow-up.
        && otherwise.is_none_or(|effect| shared_stack_effect_at_position(*effect, true))
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
        EffectDef::ChooseExact(choice) => {
            deferred_decision_allowed
                && shared_effect_recipient(EffectRecipientDef::player(choice.chooser))
                && shared_effect_recipient(EffectRecipientDef::objects(choice.candidates))
                && choice
                    .exclude
                    .is_none_or(|object| shared_effect_recipient(EffectRecipientDef::object(object)))
                && shared_stack_effect_at_position(*choice.then, true)
        }
        EffectDef::ChooseCounterKind { object, then } => {
            deferred_decision_allowed
                && shared_effect_recipient(object)
                && shared_stack_effect_at_position(*then, true)
        }
        EffectDef::ChooseEffect { player, choices } => {
            deferred_decision_allowed
                && shared_effect_recipient(player)
                && !choices.is_empty()
                && choices
                    .iter()
                    .all(|choice| {
                        choice.effect == EffectDef::None
                            || shared_stack_effect_at_position(choice.effect, true)
                    })
        }
        EffectDef::BindObjects(definition) => {
            shared_object_collection(definition.source)
                && shared_object_collection_continuation(*definition.then, deferred_decision_allowed)
        }
        EffectDef::ChooseCardsFromCollection(definition) => {
            deferred_decision_allowed
                && definition.maximum > 0
                && definition.minimum <= definition.maximum
                && shared_object_collection(definition.source)
                && shared_effect_recipient(EffectRecipientDef::player(definition.actor))
                && shared_object_predicate(definition.object)
                && shared_object_collection_continuation(*definition.then, true)
        }
        EffectDef::RevealAndClassifyCards(definition) => {
            shared_object_collection(definition.source)
                && shared_object_predicate(definition.object)
                && shared_object_collection_continuation(
                    *definition.then,
                    deferred_decision_allowed,
                )
        }
        EffectDef::IfNoObjects(definition) => {
            shared_effect_recipient(EffectRecipientDef::objects(definition.input))
                && shared_object_collection_continuation(*definition.if_empty, deferred_decision_allowed)
                && shared_object_collection_continuation(*definition.otherwise, deferred_decision_allowed)
        }
        EffectDef::ClassifyObjects(definition) => {
            shared_effect_recipient(EffectRecipientDef::objects(definition.input))
                && shared_object_predicate(definition.object)
                && shared_object_collection_continuation(*definition.then, deferred_decision_allowed)
        }
        EffectDef::CombineObjects(definition) => {
            definition.inputs.iter().copied().all(|input| {
                shared_effect_recipient(EffectRecipientDef::objects(input))
            }) && shared_object_collection_continuation(*definition.then, deferred_decision_allowed)
        }
        EffectDef::RandomizeObjectOrder(definition) => {
            shared_effect_recipient(EffectRecipientDef::objects(definition.input))
                && shared_object_collection_continuation(*definition.then, deferred_decision_allowed)
        }
        EffectDef::RevealObjects(definition) => {
            shared_effect_recipient(EffectRecipientDef::objects(definition.input))
                && shared_object_collection_continuation(*definition.then, deferred_decision_allowed)
        }
        EffectDef::MoveObjects(definition) => {
            matches!(
                definition.zone,
                ZoneKind::Battlefield
                    | ZoneKind::Hand
                    | ZoneKind::Graveyard
                    | ZoneKind::Exile
                    | ZoneKind::Library
            ) && shared_effect_recipient(EffectRecipientDef::objects(definition.input))
                && shared_object_collection_continuation(*definition.then, deferred_decision_allowed)
        }
        EffectDef::PutObjectsOntoBattlefieldFaceDown(definition) => {
            shared_effect_recipient(EffectRecipientDef::objects(definition.input))
                && shared_effect_recipient(EffectRecipientDef::player(definition.controller))
                && shared_object_collection_continuation(*definition.then, deferred_decision_allowed)
        }
        EffectDef::ChooseObjectOrder(definition) => {
            deferred_decision_allowed
                && shared_effect_recipient(EffectRecipientDef::player(definition.actor))
                && shared_effect_recipient(EffectRecipientDef::objects(definition.input))
                && shared_object_collection_continuation(*definition.then, true)
        }
        EffectDef::LookAtObjects(definition) => {
            deferred_decision_allowed
                && shared_effect_recipient(EffectRecipientDef::player(definition.actor))
                && shared_object_collection(definition.source)
                && shared_object_collection_continuation(*definition.then, true)
        }
        EffectDef::PartitionGroup(definition) => {
            deferred_decision_allowed
                && shared_effect_recipient(EffectRecipientDef::player(definition.actor))
                && shared_effect_recipient(EffectRecipientDef::objects(definition.input))
                && shared_object_collection_continuation(*definition.then, true)
        }
        EffectDef::ChooseGroup(definition) => {
            deferred_decision_allowed
                && shared_effect_recipient(EffectRecipientDef::player(definition.actor))
                && shared_effect_recipient(EffectRecipientDef::objects(definition.first))
                && shared_effect_recipient(EffectRecipientDef::objects(definition.second))
                && shared_object_collection_continuation(*definition.then, true)
        }
        EffectDef::ChooseOneOfEach(definition) => {
            deferred_decision_allowed
                && shared_effect_recipient(EffectRecipientDef::player(definition.actor))
                && shared_effect_recipient(EffectRecipientDef::objects(definition.input))
                && definition
                    .predicates
                    .iter()
                    .copied()
                    .all(shared_object_predicate)
                && shared_object_collection_continuation(*definition.then, true)
        }
        EffectDef::ChooseForEachPlayer(choice) => {
            deferred_decision_allowed
                && choice.chosen != choice.unchosen
                && shared_effect_recipient(choice.player)
                && shared_object_predicate(choice.candidates)
                && matches!(choice.zone, ZoneKind::Battlefield | ZoneKind::Hand)
                && match choice.selection {
                    PerPlayerSelectionDef::OneOfEach(selectors) => {
                        !selectors.is_empty()
                            && selectors.iter().copied().all(shared_object_predicate)
                    }
                    PerPlayerSelectionDef::Count(_) => true,
                }
                && shared_stack_effect_at_position(*choice.then, true)
        }
        EffectDef::RevealAtRandomFromHand { player, .. }
        | EffectDef::Mill { player, .. } => shared_effect_recipient(player),
        EffectDef::Destroy { object, then, .. } => {
            shared_effect_recipient(object)
                && match then {
                    Some(follow_up) => shared_stack_effect_at_position(
                        *follow_up.effect,
                        deferred_decision_allowed,
                    ),
                    None => true,
                }
        }
        EffectDef::PayOr(payment) => {
            deferred_decision_allowed
                && shared_effect_payment(payment.payment)
                && (payment.if_paid.is_some() || payment.otherwise.is_some())
                && payment
                    .if_paid
                    .iter()
                    .chain(payment.otherwise.iter())
                    .all(|effect| {
                        matches!(**effect, EffectDef::None)
                            || shared_stack_effect_at_position(**effect, true)
                    })
        }
        // A spell copying itself asks its chooser for targets, which is a
        // decision window like any other. Proliferate asks over permanents
        // and players at once, which is the same kind of window and reads
        // nothing off a recipient either.
        EffectDef::Proliferate => {
            deferred_decision_allowed
        }

        // Naming a card is a decision window, and what follows it is bound by
        // the same rule as anything after one.
        EffectDef::ChooseCardName { then, .. }
        | EffectDef::SearchZone { then: Some(then), .. } => {
            deferred_decision_allowed && shared_stack_effect_at_position(*then, true)
        }
        EffectDef::SelectAtRandomFromZone {
            player,
            object,
            ..
        } => {
            shared_effect_recipient(player) && shared_object_predicate(object)
        }
        // A resolving ability names each mana as it is added, one question
        // per mana, so both a choice of colour and a combination are
        // supported here as long as the resolution is allowed to ask. What
        // separates them is enumeration before an activation is offered,
        // which a resolution never does.
        EffectDef::AddMana(AddManaEffectDef {
            mana: ManaSelectionDef::Choice(_) | ManaSelectionDef::Combination(_),
            ..
        }) => deferred_decision_allowed && shared_mana_effect(effect, true),
        EffectDef::AddMana(_) => shared_mana_effect(effect, false),
        EffectDef::DealDamageFrom {
            source, recipient, ..
        } => {
            shared_effect_recipient(EffectRecipientDef::object(source))
                && shared_effect_recipient(recipient)
        }
        EffectDef::DealDamageSimultaneously(assignments) => assignments.iter().all(|assignment| {
            assignment
                .source
                .is_none_or(|source| shared_effect_recipient(EffectRecipientDef::object(source)))
                && shared_effect_recipient(assignment.recipient)
        }),
        EffectDef::Fight {
            first,
            second,
            excess,
        } => {
            shared_effect_recipient(EffectRecipientDef::object(first))
                && shared_effect_recipient(EffectRecipientDef::object(second))
                && excess.is_none_or(|continuation| {
                    shared_effect_recipient(EffectRecipientDef::object(continuation.recipient))
                        && shared_stack_effect_at_position(
                            *continuation.then,
                            deferred_decision_allowed,
                        )
                })
        }
        EffectDef::DealDamage { recipient, .. }
        | EffectDef::DealDamageAndApply { recipient, .. }
        | EffectDef::DrainLife { recipient, .. }
        | EffectDef::GainLife { recipient, .. }
        | EffectDef::SetLifeTotal { recipient, .. }
        | EffectDef::AddPlayerCounters { recipient, .. }
        | EffectDef::DrawCards { recipient, .. }
        | EffectDef::ShuffleLibrary { player: recipient }
        | EffectDef::BuryGraveyard { player: recipient }
        | EffectDef::EmptyManaPool { player: recipient }
        | EffectDef::TakeExtraTurn { player: recipient }
        | EffectDef::LoseLife { recipient, .. }
        // The permission it grants belongs to the resolving controller, so
        // nothing beyond the recipient has to be read here.
        | EffectDef::ExileTopOfLibraryToPlay {
            player: recipient, ..
        }
        // Every card in the named zones is offered, so there is no predicate
        // to check -- only who is searching.
        | EffectDef::SearchZonesAndExileRest {
            player: recipient, ..
        }
        // The same predicate over the same cards; what differs is only where
        // they land and what the matched one carries.
        | EffectDef::ExileFromTopUntil {
            player: recipient, ..
        }
        | EffectDef::LoseTheGame { player: recipient }
        | EffectDef::WinTheGame { player: recipient }
        | EffectDef::LookAtHand { player: recipient }
        | EffectDef::LookAtRandomCardInHand { player: recipient }
        | EffectDef::RevealHand { player: recipient } => shared_effect_recipient(recipient),
        EffectDef::Discard {
            recipient,
            selection,
            then,
            ..
        } => {
            let follow_up_is_shared = then.is_none_or(|follow_up| {
                shared_object_predicate(follow_up.counted)
                    && shared_stack_effect_at_position(*follow_up.effect, true)
            });
            shared_effect_recipient(recipient)
                && match selection {
                    DiscardSelectionDef::RecipientChooses => {
                        deferred_decision_allowed && follow_up_is_shared
                    }
                    DiscardSelectionDef::Random => then.is_none(),
                    DiscardSelectionDef::RandomMatching(predicate) => {
                        then.is_none() && shared_object_predicate(*predicate)
                    }
                }
        }
        EffectDef::ExileOneFromEachZone(pile) => shared_effect_recipient(pile.player),
        EffectDef::MillWhileMatching(mill) => {
            shared_effect_recipient(mill.player)
                && shared_object_predicate(mill.object)
                && shared_stack_effect_at_position(*mill.body, deferred_decision_allowed)
                && shared_stack_effect_at_position(*mill.on_match, deferred_decision_allowed)
        }
        EffectDef::MillUntil(mill) => {
            shared_effect_recipient(mill.player)
                && mill
                    .until
                    .filter
                    .is_none_or(|filter| shared_object_predicate(filter.predicate()))
        }
        // The move binds the permanent it created, so what is left to check
        // is that the card it takes and the composed follow-up are supported.
        EffectDef::PutOntoBattlefieldThen {
            object: recipient,
            then,
            ..
        } => {
            shared_effect_recipient(recipient)
                && shared_stack_effect_at_position(*then, deferred_decision_allowed)
        }
        EffectDef::SacrificeOfChoice { .. } => shared_sacrifice_of_choice(effect),
        EffectDef::PermitLookAtExiled {
            object,
            player,
            then,
        } => {
            shared_effect_recipient(object)
                && shared_effect_recipient(EffectRecipientDef::player(player))
                && shared_object_collection_continuation(*then, deferred_decision_allowed)
        }
        EffectDef::SearchZone {
            player,
            source,
            object,
            minimum,
            maximum,
            destination,
            shuffle,
            attachment,
            ..
        } => {
            // A constant maximum is still checked against the minimum and
            // against the one-card ceiling a library destination needs. A
            // maximum sized from the board answers neither question here, so
            // it is supported everywhere except back into a library.
            let constant_maximum = match maximum {
                ValueDef::Constant(value) => usize::try_from(value).ok(),
                _ => None,
            };
            deferred_decision_allowed
                && shared_effect_recipient(player)
                && shared_object_predicate(object)
                && constant_maximum.is_none_or(|bound| minimum <= bound)
                && (destination != ZoneKind::Library
                    || constant_maximum.is_some_and(|bound| bound <= 1))
                && (!shuffle || source == ZoneKind::Library)
                && match attachment {
                    None => true,
                    Some(ArrivalAttachmentDef::ArrivalToPlayer(player)) => {
                        destination == ZoneKind::Battlefield
                            && shared_effect_recipient(EffectRecipientDef::player(player))
                    }
                    Some(
                        ArrivalAttachmentDef::SourceToArrival
                        | ArrivalAttachmentDef::ArrivalToHost(_),
                    ) => false,
                }
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
                            | CardChoiceSourceDef::Zone(
                                ZoneKind::Exile | ZoneKind::Graveyard | ZoneKind::Hand
                            )
                    )
                })
                // An outside-game import has one destination the runtime
                // knows; a card already in a zone can also be put onto the
                // battlefield, which the choice continuation has always done,
                // or onto either end of its owner's library, which is what
                // "put two cards from your hand on top of your library" is.
                && (matches!(destination, ZoneKind::Hand | ZoneKind::Exile)
                    || (matches!(destination, ZoneKind::Battlefield | ZoneKind::Library)
                        && sources
                            .iter()
                            .all(|source| !matches!(source, CardChoiceSourceDef::OutsideGame))))
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
        EffectDef::ExchangeControl {
            first,
            second,
            otherwise,
        } => {
            shared_effect_recipient(first)
                && shared_effect_recipient(second)
                && otherwise.is_none_or(|otherwise| {
                    shared_stack_effect_at_position(*otherwise, deferred_decision_allowed)
                })
        }
        // Only the two destinations the return path knows.
        EffectDef::ReturnLinkedExiles { zone, .. } => {
            matches!(zone, ZoneKind::Battlefield | ZoneKind::Hand)
        }
        // The set is the whole of it: which cards may be played is what the
        // clause names, and every set the shared walk understands works.
        // Either duration is shared -- one grants and stops, the other
        // grants and offers, and both are the same walk's business.
        EffectDef::MayPlayWithoutPaying(permission) => {
            shared_effect_recipient(EffectRecipientDef::objects(permission.objects))
        }
        // Populate copies whatever the choice landed on, so like the rest of
        // these only its recipient has to be one the runtime understands.
        // The destination is always a library and the depth is an ordinary
        // value the shared walk already resolves, so only the recipient is
        // an open question.
        EffectDef::PutIntoLibraryBeneathTop { object, .. }
        | EffectDef::Endure { object, .. }
        | EffectDef::Regenerate { object }
        | EffectDef::Tap { object }
        | EffectDef::RemoveFromCombat { object }
        | EffectDef::SkipNextUntapSteps { object, .. }
        | EffectDef::DoubleCounters { object, .. }
        | EffectDef::RemoveAllCounters { object, .. }
        | EffectDef::Untap { object }
        | EffectDef::Saddle { object }
        | EffectDef::Sacrifice { object }
        | EffectDef::SacrificeYours { object }
        | EffectDef::DiscardCards { object }
        | EffectDef::ExileLinkedToSource { object, .. }
        | EffectDef::ExileGrantingOwnerPlay { object, .. }
        | EffectDef::ExileGrantingControllerPlayThisTurn { object }
        | EffectDef::PermitCastFromGraveyardThisTurn { object }
        | EffectDef::Detain { object }
        | EffectDef::GainControl { object, .. }
        | EffectDef::AddCounters { object, .. }
        | EffectDef::ModifyCounters { object, .. }
        | EffectDef::RemoveCounters { object, .. }
        | EffectDef::Attach { object }
        | EffectDef::AttachToSource { object }
        | EffectDef::Reconfigure { object }
        | EffectDef::Unattach { object }
        | EffectDef::PairWithSource { object }
        | EffectDef::PhaseOut { object }
        | EffectDef::ChangeTextBasicLandType { object }
        // The colour is named at resolution, so the declaration only has to
        // say who receives it and for how long.
        | EffectDef::ChooseColor { object, .. }
        | EffectDef::BecomeCopyOf { object, .. }
        // Each waits on a deferred decision, the same as any other: the
        // owner's answer, the offer to cast what was pointed at, and the
        // "top of library or graveyard" a nonland explore ends in.
        | EffectDef::PutSpellIntoOwnersLibrary { object }
        | EffectDef::MayCastTargetWithoutPaying { object, .. }
        | EffectDef::Explore { object } => {
            deferred_decision_allowed && shared_effect_recipient(object)
        }
        // Copying a stack object asks the same kind of deferred question,
        // over the recipient carried by its composite definition.
        EffectDef::CopyStackObject(copy) => {
            deferred_decision_allowed && shared_effect_recipient(copy.object)
        }
        EffectDef::ChangeStackTargets(change) => {
            deferred_decision_allowed
                && shared_effect_recipient(change.object)
                && match change.change {
                    crate::card::StackTargetChangeDef::ChooseNew { .. } => true,
                    crate::card::StackTargetChangeDef::ReplaceOneWith(replacement) => {
                        shared_effect_recipient(replacement)
                    }
                }
        }
        EffectDef::Counter { object, zone, .. } => {
            // The four places a countered card can end up. A library is one
            // of them because Memory Lapse puts it back on top rather than
            // into a graveyard, and a hand is another because Remand does.
            matches!(
                zone,
                ZoneKind::Graveyard | ZoneKind::Exile | ZoneKind::Hand | ZoneKind::Library
            ) && shared_effect_recipient(object)
        }
        // Neither needs a recipient: both concern the resolving controller.
        // The amount is computed when the effect resolves, so nothing has
        // to read it ahead of time the way a mana ability does.
        // Both of a land substitution's types are chosen as it resolves, so
        // it has no recipient to check either.
        // Cascade names nothing and asks nothing: the spell it is printed on
        // supplies the bound, the controller, and the library.
        // What a token clause does next runs in the same resolution with the
        // tokens bound, so it is checked here rather than trusted.
        EffectDef::CreateToken { copy, created, .. } => {
            copy.is_none_or(|copy| shared_effect_recipient(*copy.object))
                && created.is_none_or(|created| {
                    shared_stack_effect_at_position(*created.then, deferred_decision_allowed)
                })
        }
        EffectDef::Cascade
        | EffectDef::CreateMyriadTokens
        | EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. }
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::CreateAttachedToken { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::Transform { .. }
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::DamageCannotBePreventedThisTurn
        // The crown goes to one named player, and nothing has to be read off
        // the board to know which.
        // A vote waits on one decision per player, so it belongs to the
        // deferred half, and its ballot is a predicate the shared walk
        // already reads.
        | EffectDef::VoteForPermanentToExile { .. }
        // The card it puts onto the battlefield is its own source, so it
        // names no recipient to check.
        | EffectDef::PutSourceOntoBattlefieldAttacking
        | EffectDef::BecomeMonarch { .. }
        | EffectDef::GainClassLevel { .. }
        | EffectDef::ContinueReplacedDraw => true,
        // Each of these asks a question and then runs an inner effect,
        // so the question has to be allowed here and the answer has to be
        // something the shared procedure can carry out.
        EffectDef::May { player, effect } => {
            deferred_decision_allowed
                && shared_effect_recipient(player)
                && shared_stack_effect_at_position(*effect, true)
        }
        // The offer to cast is a standing decision, and what happens when it
        // is declined runs in that decision's own continuation.
        EffectDef::ExileTopAndMayCast { player, otherwise } => {
            deferred_decision_allowed
                && shared_effect_recipient(player)
                && otherwise.is_none_or(|effect| shared_stack_effect_at_position(*effect, true))
        }
        // Scheduling creates a fresh resolution boundary. A decision may
        // therefore be the delayed effect's root even when scheduling it
        // is itself one component of a sequence.
        EffectDef::BindOutput { effect, .. }
        | EffectDef::ForEachInBinding { effect, .. } => {
            shared_stack_effect_at_position(*effect, deferred_decision_allowed)
        }
        effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
            let conditional = effect
                .conditional()
                .expect("conditional variants expose their shared shape");
            shared_stack_effect_at_position(*conditional.then, deferred_decision_allowed)
                && conditional.otherwise.is_none_or(|otherwise| {
                    shared_stack_effect_at_position(*otherwise, deferred_decision_allowed)
                })
        }
        EffectDef::WithBattlefieldArrival { effect, arrival } => {
            shared_stack_effect_at_position(*effect, deferred_decision_allowed)
                && match arrival.attachment {
                    None | Some(ArrivalAttachmentDef::SourceToArrival) => true,
                    Some(ArrivalAttachmentDef::ArrivalToHost(host)) => {
                        shared_effect_recipient(EffectRecipientDef::object(host))
                    }
                    Some(ArrivalAttachmentDef::ArrivalToPlayer(player)) => {
                        shared_effect_recipient(EffectRecipientDef::player(player))
                    }
                }
        }
        EffectDef::WithZoneMoveResult { effect, then, .. } => {
            shared_stack_effect_at_position(*effect, deferred_decision_allowed)
                && shared_stack_effect_at_position(*then, true)
        }
        // Installing an ability is a resolution like any other; what it
        // installs has to be an ability the shared runtime can fire.
        EffectDef::InstallTrigger(trigger) => shared_definition_ability(trigger.ability),
        // The effect object is where a turn-scoped replacement lives, and
        // both zone-move walks read it there.
        EffectDef::CreateOngoingEffect(ongoing)
            if matches!(
                ongoing.ability.definition,
                DeclarativeAbilityDef::Replacement(_)
            ) =>
        {
            let DeclarativeAbilityDef::Replacement(definition) = ongoing.ability.definition else {
                return false;
            };
            ongoing.affected.is_none()
                && matches!(
                    definition.event,
                    ReplacementEventDef::AnyObjectWouldMove { .. }
                )
                && matches!(
                    ongoing.ability.declarative_replacement(),
                    Some(ReplacementEffectDef::MoveToZone(_))
                )
        }
        EffectDef::CreateOngoingEffect(ongoing) => {
            let (definition, mana) = match ongoing.ability.definition {
                DeclarativeAbilityDef::Activated(definition) => (definition, false),
                DeclarativeAbilityDef::ActivatedMana(definition) => (definition, true),
                _ => return false,
            };
            ongoing.affected.is_none_or(shared_effect_recipient)
                && definition.procedure == AbilityProcedureDef::Shared
                && definition.source_zones == [ZoneKind::Command]
                && definition.targets.is_empty()
                && definition.modes.is_none()
                && definition.activation_limit.is_none()
                && !definition.any_player_may_activate
                && definition.condition.is_none()
                && definition.costs.as_slice().iter().all(|cost| {
                    if mana {
                        matches!(cost, AbilityCostDef::PayLife(_))
                    } else {
                        matches!(cost, AbilityCostDef::Mana(cost) if !cost.variable_x)
                    }
                })
                && ongoing.duration != ResolvedEffectDurationDef::WhileSourceTapped
                && ongoing.ability.declarative_effect().is_some_and(|effect| {
                    shared_stack_effect_at_position(effect, false)
                })
        }
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
        EffectDef::None | EffectDef::ConditionalStatic(_) | EffectDef::StaticApply { .. }
        | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::CannotBeForcedToDiscard
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::ModifyCost(_)
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::CannotAttackIf(_)
        | EffectDef::Special(_) => false,
    }
}
