//! Which activation and casting costs the shared runtime can pay.
//!
//! Split out of `runtime_support.rs` for the source-size budget: the file
//! next door decides what an ability may *do*, and this decides what it may
//! be paid with.

use super::*;
use crate::card::CostDef;

fn linked_card_mana_costs_supported(battlefield: bool, costs: &[CostDef]) -> bool {
    let priced_bindings = costs
        .iter()
        .filter_map(|cost| match cost {
            CostDef::ManaCostOf(ObjectRefDef::Binding(binding)) => Some(*binding),
            _ => None,
        })
        .collect::<Vec<_>>();
    let moved_bindings = costs
        .iter()
        .filter_map(|cost| match cost {
            CostDef::MoveToZone(movement)
                if movement.from == ZoneKind::Graveyard
                    && movement.to == ZoneKind::Exile
                    && movement.fixed_count() == Some(1) =>
            {
                movement.binding
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let chosen_object_costs = costs
        .iter()
        .filter(|cost| {
            matches!(
                cost,
                CostDef::SacrificePermanent { .. }
                    | CostDef::SacrificePermanents { .. }
                    | CostDef::ReturnUnblockedAttackerToHand
                    | CostDef::TapPermanents { .. }
                    | CostDef::MoveToZone(_)
                    | CostDef::DiscardCardMatching(_)
                    | CostDef::RevealCardFromHand(_)
                    | CostDef::ExileCardFromHand(_)
            )
        })
        .count();
    priced_bindings.len() <= 1
        && (priced_bindings.is_empty()
            || (battlefield && moved_bindings == priced_bindings && chosen_object_costs == 1))
}

fn at_most_one_deferred_activation_cost(costs: &[CostDef]) -> bool {
    costs
        .iter()
        .filter(|cost| {
            matches!(
                cost,
                CostDef::SacrificePermanents { .. }
                    | CostDef::TapPermanents { .. }
                    | CostDef::TapCreaturesWithTotalPower { .. }
            )
        })
        .count()
        <= 1
}

/// Multiple tap payers are currently chosen after the mana plan. Keep that
/// mixed shape outside advertised shared coverage until both use one joint
/// plan. A single payer is enumerated early enough to be reserved by it.
fn multi_tap_cost_has_no_mana_component(costs: &[CostDef]) -> bool {
    !costs
        .iter()
        .any(|cost| matches!(cost, CostDef::TapPermanents { count, .. } if *count > 1))
        || !costs.iter().any(|cost| {
            matches!(
                cost,
                CostDef::Mana(_) | CostDef::ManaCostOf(_) | CostDef::ManaValueOfTarget { .. }
            )
        })
}

fn at_most_one_source_exit_cost(costs: &[CostDef]) -> bool {
    costs
        .iter()
        .filter(|cost| {
            matches!(
                cost,
                CostDef::SacrificeSource | CostDef::ExileSource | CostDef::ReturnSourceToHand
            )
        })
        .count()
        <= 1
}

fn at_most_one_sacrifice_of_each_kind(costs: &[CostDef]) -> bool {
    let choices = costs
        .iter()
        .filter(|cost| matches!(cost, CostDef::SacrificePermanent { .. }))
        .count();
    let fixed = costs
        .iter()
        .filter(|cost| matches!(cost, CostDef::SacrificeObject(_)))
        .count();
    choices <= 1 && fixed <= 1
}

pub(in super::super) fn shared_activated_costs(zones: &[ZoneKind], costs: &[CostDef]) -> bool {
    let battlefield = zones == [ZoneKind::Battlefield];
    let hand = zones == [ZoneKind::Hand];
    let graveyard = zones == [ZoneKind::Graveyard];
    let exile = zones == [ZoneKind::Exile];
    at_most_one_sacrifice_of_each_kind(costs)
        && at_most_one_source_exit_cost(costs)
        && at_most_one_deferred_activation_cost(costs)
        && multi_tap_cost_has_no_mana_component(costs)
        && linked_card_mana_costs_supported(battlefield, costs)
        && costs.iter().all(|cost| match cost {
            // A variable X is offered one activation per affordable value.
            // More than one is not: nothing enumerates a cost charging X twice.
            CostDef::Mana(cost) => cost.x_multiplier <= 1,
            CostDef::ManaValueOfTarget { multiplier, .. } => {
                battlefield && *multiplier > 0
            }
            // The chosen object comes from the battlefield or from the
            // activating player's own graveyard, so only the predicate
            // needs checking.
            // The discard reads the payer's hand rather than the
            // battlefield, but the shape is the same: a permanent to activate
            // from and a predicate the shared walk can read.
            // The many-at-once form is paid by a decision rather than by
            // enumeration, which asks the same question of the same walk.
            CostDef::SacrificePermanent { object, .. } => {
                (battlefield || exile) && shared_object_predicate(*object)
            }
            CostDef::SacrificePermanents { object, .. }
            | CostDef::DiscardCardMatching(object)
            | CostDef::RevealCardFromHand(object)
            | CostDef::ExileCardFromHand(object) => {
                battlefield && shared_object_predicate(*object)
            }
            CostDef::MoveToZone(movement) => {
                battlefield
                    && matches!(
                        (movement.from, movement.to),
                        (ZoneKind::Hand | ZoneKind::Graveyard, ZoneKind::Exile)
                            | (ZoneKind::Hand, ZoneKind::Graveyard)
                    )
                    && movement.fixed_count().is_some_and(|count| count > 0)
                    && movement
                        .binding
                        .is_none_or(|_| movement.fixed_count() == Some(1))
                    && shared_object_predicate(movement.object)
            }
            // What pays the tap is out on the battlefield wherever the
            // ability is activated from, so a card in a graveyard can name
            // one too.
            CostDef::TapPermanents { object, count, .. } => {
                *count > 0
                    && (battlefield || (graveyard && *count == 1))
                    && shared_object_predicate(*object)
            }
            // Exiling the source and milling the activating player's library
            // are the source-independent costs supported from a graveyard;
            // the rest of these need a permanent to act on.
            CostDef::ExileSource | CostDef::MillCards(_) => {
                battlefield || graveyard
            }
            // A fixed object sacrifice is supported only when it names the
            // source whose activation is being checked.
            CostDef::ManaCostOf(ObjectRefDef::Binding(_))
            | CostDef::SacrificeObject(
                ObjectRefDef::Source | ObjectRefDef::AbilityGrantSource,
            )
            | CostDef::TapSource
            | CostDef::ExertSource
            | CostDef::UntapSource
            | CostDef::SacrificeSource
            // The source leaves the battlefield to pay either way; only
            // where it lands differs.
            | CostDef::ReturnSourceToHand
            | CostDef::RemoveCountersFromSource { .. }
            // Open-ended only in the declaration: one activation per size is
            // built by the mana path, which is why the caller also requires
            // the effect to be an AddMana.
            | CostDef::RemoveAnyNumberOfCountersFromSource(_)
            | CostDef::PayLife(_)
            | CostDef::Loyalty(_)
            // Nobody chooses which cards go, so a random discard needs no
            // decision procedure -- only a permanent to activate from. A
            // mill cost similarly names the top cards without a choice.
            | CostDef::DiscardCardsAtRandom(_)
            // "Discard your hand" takes every card and asks nothing, so like
            // the random discard it needs only a permanent to activate from.
            | CostDef::DiscardHand
            // Crew and saddle name no predicate: what may pay is every other
            // untapped creature the payer controls, and the decision that
            // asks reads the battlefield directly.
            | CostDef::TapCreaturesWithTotalPower { .. } => battlefield,
            CostDef::ExileTopCards(amount) => battlefield && *amount > 0,
            // Ninjutsu's cost joins the discard here: what it may return is
            // combat state rather than a predicate, and both are paid by a
            // card in hand.
            CostDef::DiscardSource
            | CostDef::ReturnUnblockedAttackerToHand => hand,
            _ => false,
        })
}

pub(in super::super) fn shared_spell_additional_cost(cost: Option<CostDef>) -> bool {
    cost.is_none_or(shared_spell_additional_cost_def)
}

fn shared_spell_additional_cost_def(cost: CostDef) -> bool {
    match cost {
        CostDef::Forage | CostDef::Mana(_) | CostDef::PayLife(_) => true,
        CostDef::ManaTimes { quantity, .. } | CostDef::PayLifeTimes(quantity) => {
            shared_scalar_cost_quantity(quantity)
        }
        CostDef::Sacrifice { object, quantity }
        | CostDef::Discard { object, quantity }
        | CostDef::ReturnToHand { object, quantity }
        | CostDef::Tap { object, quantity } => {
            shared_object_cost_quantity(quantity) && shared_object_predicate(object)
        }
        CostDef::Exile {
            object,
            from,
            quantity,
        } => {
            matches!(
                from,
                ZoneKind::Battlefield | ZoneKind::Graveyard | ZoneKind::Hand
            ) && shared_object_cost_quantity(quantity)
                && shared_object_predicate(object)
        }
        CostDef::All(costs) => {
            !costs.is_empty() && costs.iter().copied().all(shared_spell_additional_cost_def)
        }
        CostDef::Choice(costs) => {
            !costs.is_empty()
                && costs.iter().copied().all(shared_spell_additional_cost_def)
                // Cast actions currently carry the selected objects, not a
                // separate cost-branch ID. Two objectless branches would
                // therefore serialize identically and could not be replayed
                // unambiguously.
                && costs
                    .iter()
                    .copied()
                    .filter(|cost| spell_cost_can_be_objectless(*cost))
                    .count()
                    <= 1
        }
        _ => false,
    }
}

fn spell_cost_can_be_objectless(cost: CostDef) -> bool {
    match cost {
        CostDef::Mana(_)
        | CostDef::PayLife(_)
        | CostDef::ManaTimes { .. }
        | CostDef::PayLifeTimes(_) => true,
        CostDef::Sacrifice { quantity, .. }
        | CostDef::Discard { quantity, .. }
        | CostDef::Exile { quantity, .. }
        | CostDef::ReturnToHand { quantity, .. }
        | CostDef::Tap { quantity, .. } => matches!(
            quantity,
            crate::card::CostQuantityDef::ChosenX
                | crate::card::CostQuantityDef::ModeCount
                | crate::card::CostQuantityDef::TargetCount
                | crate::card::CostQuantityDef::Subtract(_, _)
        ),
        CostDef::All(costs) => costs.iter().copied().all(spell_cost_can_be_objectless),
        CostDef::Choice(costs) => costs.iter().copied().any(spell_cost_can_be_objectless),
        _ => false,
    }
}

fn shared_object_cost_quantity(quantity: crate::card::CostQuantityDef) -> bool {
    match quantity {
        crate::card::CostQuantityDef::Fixed(count) => count >= 1,
        crate::card::CostQuantityDef::ChosenX
        | crate::card::CostQuantityDef::ModeCount
        | crate::card::CostQuantityDef::TargetCount
        | crate::card::CostQuantityDef::ObjectSetValueAtLeast(_) => true,
        crate::card::CostQuantityDef::Subtract(left, right) => {
            shared_scalar_cost_quantity(*left) && shared_scalar_cost_quantity(*right)
        }
    }
}

fn shared_scalar_cost_quantity(quantity: crate::card::CostQuantityDef) -> bool {
    match quantity {
        crate::card::CostQuantityDef::Fixed(_)
        | crate::card::CostQuantityDef::ChosenX
        | crate::card::CostQuantityDef::ModeCount
        | crate::card::CostQuantityDef::TargetCount => true,
        crate::card::CostQuantityDef::Subtract(left, right) => {
            shared_scalar_cost_quantity(*left) && shared_scalar_cost_quantity(*right)
        }
        crate::card::CostQuantityDef::ObjectSetValueAtLeast(_) => false,
    }
}
