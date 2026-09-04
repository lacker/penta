//! Shared availability and exclusivity rules for tap-based payments.
//!
//! Convoke, Improvise, and printed activation costs remain distinct payment
//! mechanics, but they all spend the same battlefield resource: one untapped
//! permanent that must survive any mana ability used during the payment.

use super::{CostDef, Game, GameObjectId, ManaAbilityActivation, Permanent};

impl Game {
    /// Whether a permanent is still available to be committed to one tap
    /// payment. A commitment spends the whole permanent, not one abstract
    /// "tap", so the same object cannot be chosen twice.
    pub(super) fn permanent_can_pay_tap_cost(
        permanent: &Permanent,
        committed: &[GameObjectId],
    ) -> bool {
        !permanent.tapped && !committed.contains(&permanent.card.id)
    }

    /// Whether activating this mana ability preserves one permanent already
    /// committed to a later tap payment. The mana source may be different and
    /// still consume the payer as its chosen cost object.
    pub(super) fn mana_activation_preserves_tap_payment(
        permanent: &Permanent,
        activation: &ManaAbilityActivation,
        payer: GameObjectId,
    ) -> bool {
        if activation.cost_object == Some(payer) {
            return false;
        }
        if activation.source != payer {
            return true;
        }
        if activation.costs.iter().any(|cost| {
            matches!(
                cost,
                CostDef::TapSource
                    | CostDef::SacrificeSource
                    | CostDef::ExileSource
                    | CostDef::ReturnSourceToHand
                    | CostDef::SacrificePermanents { .. }
            )
        }) {
            return false;
        }
        activation
            .effect
            .sacrifice_source_when_out_of
            .is_none_or(|kind| {
                let removed = activation.costs.iter().fold(0_u16, |removed, cost| {
                    if let CostDef::RemoveCountersFromSource {
                        kind: removed_kind,
                        amount,
                    } = cost
                        && *removed_kind == kind
                    {
                        return removed.saturating_add(*amount);
                    }
                    removed
                });
                permanent.counters(kind) > removed
            })
    }
}
