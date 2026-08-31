//! Which activation and casting costs the shared runtime can pay.
//!
//! Split out of `runtime_support.rs` for the source-size budget: the file
//! next door decides what an ability may *do*, and this decides what it may
//! be paid with.

use super::*;
use crate::card::SpellAdditionalCostDef;

fn linked_card_mana_costs_supported(battlefield: bool, costs: &[AbilityCostDef]) -> bool {
    let priced_bindings = costs
        .iter()
        .filter_map(|cost| match cost {
            AbilityCostDef::ManaCostOf(ObjectRefDef::Binding(binding)) => Some(*binding),
            _ => None,
        })
        .collect::<Vec<_>>();
    let moved_bindings = costs
        .iter()
        .filter_map(|cost| match cost {
            AbilityCostDef::MoveToZone(movement)
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
                AbilityCostDef::SacrificePermanent { .. }
                    | AbilityCostDef::SacrificePermanents { .. }
                    | AbilityCostDef::ReturnUnblockedAttackerToHand
                    | AbilityCostDef::TapPermanents { .. }
                    | AbilityCostDef::MoveToZone(_)
                    | AbilityCostDef::DiscardCardMatching(_)
                    | AbilityCostDef::ExileCardFromHand(_)
            )
        })
        .count();
    priced_bindings.len() <= 1
        && (priced_bindings.is_empty()
            || (battlefield && moved_bindings == priced_bindings && chosen_object_costs == 1))
}

fn at_most_one_deferred_activation_cost(costs: &[AbilityCostDef]) -> bool {
    costs
        .iter()
        .filter(|cost| {
            matches!(
                cost,
                AbilityCostDef::SacrificePermanents { .. }
                    | AbilityCostDef::TapPermanents { .. }
                    | AbilityCostDef::TapCreaturesWithTotalPower { .. }
            )
        })
        .count()
        <= 1
}

/// Multiple tap payers are currently chosen after the mana plan. Keep that
/// mixed shape outside advertised shared coverage until both use one joint
/// plan. A single payer is enumerated early enough to be reserved by it.
fn multi_tap_cost_has_no_mana_component(costs: &[AbilityCostDef]) -> bool {
    !costs
        .iter()
        .any(|cost| matches!(cost, AbilityCostDef::TapPermanents { count, .. } if *count > 1))
        || !costs.iter().any(|cost| {
            matches!(
                cost,
                AbilityCostDef::Mana(_)
                    | AbilityCostDef::ManaCostOf(_)
                    | AbilityCostDef::ManaValueOfTarget { .. }
            )
        })
}

fn at_most_one_source_exit_cost(costs: &[AbilityCostDef]) -> bool {
    costs
        .iter()
        .filter(|cost| {
            matches!(
                cost,
                AbilityCostDef::SacrificeSource
                    | AbilityCostDef::ExileSource
                    | AbilityCostDef::ReturnSourceToHand
            )
        })
        .count()
        <= 1
}

pub(in super::super) fn shared_activated_costs(
    source_zones: &[ZoneKind],
    costs: &[AbilityCostDef],
) -> bool {
    let battlefield = source_zones == [ZoneKind::Battlefield];
    let hand = source_zones == [ZoneKind::Hand];
    let graveyard = source_zones == [ZoneKind::Graveyard];
    let exile = source_zones == [ZoneKind::Exile];
    let sacrifice_choices = costs
        .iter()
        .filter(|cost| matches!(cost, AbilityCostDef::SacrificePermanent { .. }))
        .count();
    let fixed_sacrifices = costs
        .iter()
        .filter(|cost| matches!(cost, AbilityCostDef::SacrificeObject(_)))
        .count();
    sacrifice_choices <= 1
        && fixed_sacrifices <= 1
        && at_most_one_source_exit_cost(costs)
        && at_most_one_deferred_activation_cost(costs)
        && multi_tap_cost_has_no_mana_component(costs)
        && linked_card_mana_costs_supported(battlefield, costs)
        && costs.iter().all(|cost| match cost {
            // A variable X is offered one activation per affordable value.
            // More than one is not: nothing enumerates a cost charging X twice.
            AbilityCostDef::Mana(cost) => cost.x_multiplier <= 1,
            AbilityCostDef::ManaValueOfTarget { multiplier, .. } => {
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
            AbilityCostDef::SacrificePermanent { object, .. } => {
                (battlefield || exile) && shared_object_predicate(*object)
            }
            AbilityCostDef::SacrificePermanents { object, .. }
            | AbilityCostDef::DiscardCardMatching(object)
            | AbilityCostDef::ExileCardFromHand(object) => {
                battlefield && shared_object_predicate(*object)
            }
            AbilityCostDef::MoveToZone(movement) => {
                battlefield
                    && matches!(movement.from, ZoneKind::Hand | ZoneKind::Graveyard)
                    && movement.to == ZoneKind::Exile
                    && movement.fixed_count().is_some_and(|count| count > 0)
                    && movement
                        .binding
                        .is_none_or(|_| movement.fixed_count() == Some(1))
                    && shared_object_predicate(movement.object)
            }
            // What pays the tap is out on the battlefield wherever the
            // ability is activated from, so a card in a graveyard can name
            // one too.
            AbilityCostDef::TapPermanents { object, count, .. } => {
                *count > 0
                    && (battlefield || (graveyard && *count == 1))
                    && shared_object_predicate(*object)
            }
            // Exiling the source is the one cost a card can pay from its own
            // graveyard; the rest of these need a permanent to act on.
            AbilityCostDef::ExileSource => battlefield || graveyard,
            // A fixed object sacrifice is supported only when it names the
            // source whose activation is being checked.
            AbilityCostDef::ManaCostOf(ObjectRefDef::Binding(_))
            | AbilityCostDef::SacrificeObject(
                ObjectRefDef::Source | ObjectRefDef::AbilityGrantSource,
            )
            | AbilityCostDef::TapSource
            | AbilityCostDef::ExertSource
            | AbilityCostDef::UntapSource
            | AbilityCostDef::SacrificeSource
            // The source leaves the battlefield to pay either way; only
            // where it lands differs.
            | AbilityCostDef::ReturnSourceToHand
            | AbilityCostDef::RemoveCountersFromSource { .. }
            // Open-ended only in the declaration: one activation per size is
            // built by the mana path, which is why the caller also requires
            // the effect to be an AddMana.
            | AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_)
            | AbilityCostDef::PayLife(_)
            | AbilityCostDef::Loyalty(_)
            // Nobody chooses which cards go, so a random discard needs no
            // decision procedure -- only a permanent to activate from. A
            // mill cost similarly names the top cards without a choice.
            | AbilityCostDef::DiscardCardsAtRandom(_)
            // "Discard your hand" takes every card and asks nothing, so like
            // the random discard it needs only a permanent to activate from.
            | AbilityCostDef::DiscardHand
            | AbilityCostDef::MillCards(_)
            // Crew and saddle name no predicate: what may pay is every other
            // untapped creature the payer controls, and the decision that
            // asks reads the battlefield directly.
            | AbilityCostDef::TapCreaturesWithTotalPower { .. } => battlefield,
            // Ninjutsu's cost joins the discard here: what it may return is
            // combat state rather than a predicate, and both are paid by a
            // card in hand.
            AbilityCostDef::DiscardSource
            | AbilityCostDef::ReturnUnblockedAttackerToHand => hand,
            AbilityCostDef::SacrificeObject(
                ObjectRefDef::ResolvingObject
                | ObjectRefDef::CreatingSource
                | ObjectRefDef::ZoneChangeSuccessor(_)
                | ObjectRefDef::ZoneChangeResultOfTriggeringObject
                | ObjectRefDef::Binding(_)
                | ObjectRefDef::AdditionalCostObject(_)
                | ObjectRefDef::AttachedToSource
                | ObjectRefDef::Target(_)
                | ObjectRefDef::TriggeringObject
                | ObjectRefDef::DamagedObject
                | ObjectRefDef::SourceOfTargetedStackObject(_),
            )
            | AbilityCostDef::DiscardCards(_)
            | AbilityCostDef::ManaCostOf(
                ObjectRefDef::Source
                | ObjectRefDef::CreatingSource
                | ObjectRefDef::ZoneChangeSuccessor(_)
                | ObjectRefDef::ZoneChangeResultOfTriggeringObject
                | ObjectRefDef::AbilityGrantSource
                | ObjectRefDef::ResolvingObject
                | ObjectRefDef::AttachedToSource
                | ObjectRefDef::Target(_)
                | ObjectRefDef::TriggeringObject
                | ObjectRefDef::DamagedObject
                | ObjectRefDef::AdditionalCostObject(_)
                | ObjectRefDef::SourceOfTargetedStackObject(_),
            )
            | AbilityCostDef::Special(_) => false,
        })
}

pub(in super::super) fn shared_spell_additional_cost(cost: Option<SpellAdditionalCostDef>) -> bool {
    cost.is_none_or(shared_spell_additional_cost_def)
}

fn shared_spell_additional_cost_def(cost: SpellAdditionalCostDef) -> bool {
    match cost {
        SpellAdditionalCostDef::PayMana(_) | SpellAdditionalCostDef::Forage => true,
        SpellAdditionalCostDef::PayLife(quantity) => shared_scalar_cost_quantity(quantity),
        SpellAdditionalCostDef::Sacrifice { object, quantity }
        | SpellAdditionalCostDef::Discard { object, quantity }
        | SpellAdditionalCostDef::ReturnToHand { object, quantity } => {
            shared_object_cost_quantity(quantity) && shared_object_predicate(object)
        }
        SpellAdditionalCostDef::Exile {
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
        SpellAdditionalCostDef::All(costs) => {
            !costs.is_empty() && costs.iter().copied().all(shared_spell_additional_cost_def)
        }
        SpellAdditionalCostDef::Choice(costs) => {
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
    }
}

fn spell_cost_can_be_objectless(cost: SpellAdditionalCostDef) -> bool {
    match cost {
        SpellAdditionalCostDef::PayMana(_) | SpellAdditionalCostDef::PayLife(_) => true,
        SpellAdditionalCostDef::Sacrifice { quantity, .. }
        | SpellAdditionalCostDef::Discard { quantity, .. }
        | SpellAdditionalCostDef::Exile { quantity, .. }
        | SpellAdditionalCostDef::ReturnToHand { quantity, .. } => matches!(
            quantity,
            crate::card::CostQuantityDef::ChosenX
                | crate::card::CostQuantityDef::ModeCount
                | crate::card::CostQuantityDef::Subtract(_, _)
        ),
        SpellAdditionalCostDef::Forage => false,
        SpellAdditionalCostDef::All(costs) => {
            costs.iter().copied().all(spell_cost_can_be_objectless)
        }
        SpellAdditionalCostDef::Choice(costs) => {
            costs.iter().copied().any(spell_cost_can_be_objectless)
        }
    }
}

fn shared_object_cost_quantity(quantity: crate::card::CostQuantityDef) -> bool {
    match quantity {
        crate::card::CostQuantityDef::Fixed(count) => count >= 1,
        crate::card::CostQuantityDef::ChosenX
        | crate::card::CostQuantityDef::ModeCount
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
        | crate::card::CostQuantityDef::ModeCount => true,
        crate::card::CostQuantityDef::Subtract(left, right) => {
            shared_scalar_cost_quantity(*left) && shared_scalar_cost_quantity(*right)
        }
        crate::card::CostQuantityDef::ObjectSetValueAtLeast(_) => false,
    }
}
