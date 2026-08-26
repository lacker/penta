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
                    && movement.count == 1 =>
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
                    | AbilityCostDef::TapPermanent { .. }
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

pub(in super::super) fn shared_activated_costs(
    source_zones: &[ZoneKind],
    costs: &[AbilityCostDef],
) -> bool {
    let battlefield = source_zones == [ZoneKind::Battlefield];
    let hand = source_zones == [ZoneKind::Hand];
    let graveyard = source_zones == [ZoneKind::Graveyard];
    let sacrifice_choices = costs
        .iter()
        .filter(|cost| matches!(cost, AbilityCostDef::SacrificePermanent { .. }))
        .count();
    let fixed_sacrifices = costs
        .iter()
        .filter(|cost| matches!(cost, AbilityCostDef::SacrificeObject(_)))
        .count();
    let source_exit_costs = costs
        .iter()
        .filter(|cost| {
            matches!(
                cost,
                AbilityCostDef::SacrificeSource
                    | AbilityCostDef::ExileSource
                    | AbilityCostDef::ReturnSourceToHand
            )
        })
        .count();
    sacrifice_choices <= 1
        && fixed_sacrifices <= 1
        && source_exit_costs <= 1
        && linked_card_mana_costs_supported(battlefield, costs)
        && costs.iter().all(|cost| match cost {
            // A variable X is offered one activation per affordable
            // value. More than one X in the same cost is not: nothing
            // enumerates a cost that charges X twice.
            AbilityCostDef::Mana(cost) => cost.x_multiplier <= 1,
            // The chosen object comes from the battlefield or from the
            // activating player's own graveyard, so only the predicate
            // needs checking.
            // The discard reads the payer's hand rather than the
            // battlefield, but the shape is the same: a permanent to activate
            // from and a predicate the shared walk can read.
            // The many-at-once form is paid by a decision rather than by
            // enumeration, which asks the same question of the same walk.
            AbilityCostDef::SacrificePermanent { object, .. }
            | AbilityCostDef::SacrificePermanents { object, .. }
            | AbilityCostDef::DiscardCardMatching(object)
            | AbilityCostDef::ExileCardFromHand(object) => {
                battlefield && shared_object_predicate(*object)
            }
            AbilityCostDef::MoveToZone(movement) => {
                battlefield
                    && movement.from == ZoneKind::Graveyard
                    && movement.to == ZoneKind::Exile
                    && movement.count > 0
                    && movement.binding.is_none_or(|_| movement.count == 1)
                    && shared_object_predicate(movement.object)
            }
            // What pays the tap is out on the battlefield wherever the
            // ability is activated from, so a card in a graveyard can name
            // one too.
            AbilityCostDef::TapPermanent { object, .. } => {
                (battlefield || graveyard) && shared_object_predicate(*object)
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

/// Only one object is chosen, and only from the two places the casting
/// enumeration looks: the caster's own battlefield and graveyard.
pub(in super::super) fn shared_spell_additional_cost(cost: Option<SpellAdditionalCostDef>) -> bool {
    let Some(cost) = cost else {
        return true;
    };
    // Each way of paying has to be one the runtime can enumerate, and all of
    // them have to spend what they name the same way: the payment path reads
    // one spend mode for the whole cost, and picks the zone per object.
    cost.alternatives().into_iter().all(|alternative| {
        alternative.spend == cost.spend
            // A cost counted from something else has no printed number to
            // check: what makes it payable is the X the spell is cast for,
            // or how many modes were chosen.
            && (alternative.counted != crate::card::SpellAdditionalCostCountDef::Printed
                || alternative.count >= 1)
            && matches!(
                alternative.zone,
                ZoneKind::Battlefield | ZoneKind::Graveyard | ZoneKind::Hand
            )
            && shared_object_predicate(alternative.object)
    })
}
