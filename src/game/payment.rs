//! Payment validation and execution. Automatic search proposes choices to this
//! boundary; it does not decide whether explicitly supplied choices are legal.

use super::{Game, Mana, ManaCost, ManaPaymentPurpose, ManaPool, PlayerId};

mod choices;
pub(super) mod contributions;
mod effects;
pub(super) mod funding;
mod funding_choices;
mod presentation;
pub(super) mod preview;
pub(super) mod query;
pub(super) mod resources;
pub(super) mod state;

/// Exact positions in the current normalized mana pool. A payment decision
/// freezes that pool; positions are local references, never persistent IDs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct BoundManaPayment {
    pub(super) units: Vec<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ManaPaymentObligation {
    pub(super) player: PlayerId,
    pub(super) cost: ManaCost,
    pub(super) x: u16,
    pub(super) purpose: ManaPaymentPurpose,
}

impl Game {
    pub(super) fn mana_payment_obligation(
        &self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> ManaPaymentObligation {
        let (mut cost, x) = self.restrict_x(cost, x, purpose);
        cost.generic = cost
            .generic
            .saturating_add(x.saturating_mul(cost.x_multiplier));
        cost.variable_x = false;
        cost.x_multiplier = 0;
        ManaPaymentObligation {
            player,
            cost,
            x: 0,
            purpose: purpose.clone(),
        }
    }

    /// Include unrestricted aggregate-only units used by compatibility callers.
    /// This is read-only so examining a proposed payment cannot change state.
    pub(super) fn payment_mana_units(&self, player: PlayerId) -> Vec<Mana> {
        let state = &self.players[player.index()];
        let mut counted = ManaPool::default();
        let mut units = Vec::new();
        for mana in &state.mana {
            if counted.amount(mana.color) < state.mana_pool.amount(mana.color) {
                counted.add_color(mana.color, 1);
                units.push(*mana);
            }
        }
        for color in super::ManaColor::ALL {
            units.extend(std::iter::repeat_n(
                Mana::unrestricted(color),
                usize::from(state.mana_pool.amount(color) - counted.amount(color)),
            ));
        }
        units
    }

    /// Validate a supplied allocation without searching for a different one.
    /// Every selected unit must be eligible, unique, and actually required.
    pub(super) fn validate_mana_payment(
        &self,
        obligation: &ManaPaymentObligation,
        payment: &BoundManaPayment,
    ) -> bool {
        let ManaPaymentObligation {
            player,
            cost,
            x,
            purpose,
        } = obligation;
        let player = *player;
        let available = self.payment_mana_units(player);
        let mut selected = ManaPool::default();
        for (position, index) in payment.units.iter().copied().enumerate() {
            let Some(mana) = available.get(index) else {
                return false;
            };
            if payment.units[..position].contains(&index) || !self.mana_can_pay_for(*mana, purpose)
            {
                return false;
            }
            selected.add_color(mana.color, 1);
        }
        super::mana_planning::exact_mana_payment(selected, *cost, *x)
    }

    pub(super) fn mana_selection_can_complete(
        &self,
        obligation: &ManaPaymentObligation,
        units: &[usize],
    ) -> bool {
        let available = self.payment_mana_units(obligation.player);
        let mut selected = ManaPool::default();
        for (position, index) in units.iter().copied().enumerate() {
            let Some(mana) = available.get(index) else {
                return false;
            };
            if units[..position].contains(&index)
                || !self.mana_can_pay_for(*mana, &obligation.purpose)
            {
                return false;
            }
            selected.add_color(mana.color, 1);
        }
        super::mana_planning::payment_including_units(
            self.eligible_mana_pool(obligation.player, &obligation.purpose),
            selected,
            obligation.cost,
            obligation.x,
        )
    }

    /// Commit exactly the validated allocation. Failed validation is read-only;
    /// successful payment preserves each spent unit's restrictions and riders.
    pub(super) fn commit_mana_payment(
        &mut self,
        obligation: &ManaPaymentObligation,
        payment: &BoundManaPayment,
    ) -> Option<Vec<Mana>> {
        if !self.validate_mana_payment(obligation, payment) {
            return None;
        }
        let player = obligation.player;
        let available = self.payment_mana_units(player);
        let spent = payment
            .units
            .iter()
            .map(|index| available[*index])
            .collect::<Vec<_>>();
        self.players[player.index()].mana = available
            .into_iter()
            .enumerate()
            .filter_map(|(index, mana)| (!payment.units.contains(&index)).then_some(mana))
            .collect();
        for mana in &spent {
            self.players[player.index()]
                .mana_pool
                .remove_color(mana.color, 1);
        }
        Some(spent)
    }
}

impl Game {
    pub(super) fn concrete_mana_activation(
        &self,
        player: PlayerId,
        source: super::GameObjectId,
        ability: super::AbilityOrigin,
        color: super::ManaColor,
        choices: &super::ManaActivationChoices,
    ) -> Option<super::ManaAbilityActivation> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| self.mana_ability_activation(permanent, ability, color, choices))
            .or_else(|| {
                self.hand_mana_ability_activations(player)
                    .into_iter()
                    .chain(self.ongoing_mana_ability_activations(player))
                    .find(|activation| {
                        activation.source == source
                            && activation.ability == ability
                            && activation.color == color
                            && activation.counters_removed == choices.counters_removed
                            && activation.cost_object == choices.cost_object
                            && activation.combination == choices.combination
                    })
            })
    }

    pub(super) fn mana_activation_for_action(
        &self,
        player: PlayerId,
        action: &super::Action,
    ) -> Option<super::ManaAbilityActivation> {
        let super::Action::ActivateManaAbility {
            source,
            ability,
            color,
            counters_removed,
            cost_object,
            combination,
            triggered_mana,
        } = action
        else {
            return None;
        };
        self.concrete_mana_activation(
            player,
            *source,
            *ability,
            *color,
            &super::ManaActivationChoices::new(
                *counters_removed,
                *cost_object,
                *combination,
                triggered_mana.clone(),
            ),
        )
    }
}

pub(in crate::game) fn payment_action_object(
    action: &super::Action,
) -> Option<super::GameObjectId> {
    match action {
        super::Action::CastSpell { card, .. } => Some(*card),
        super::Action::ActivateAbility { source, .. }
        | super::Action::ActivateManaAbility { source, .. } => Some(*source),
        _ => None,
    }
}

pub(in crate::game) fn mana_ability_payment_purpose(
    source: super::GameObjectId,
    costs: &[super::CostDef],
) -> ManaPaymentPurpose {
    ManaPaymentPurpose::Ability {
        source,
        taps_source: costs.contains(&super::CostDef::TapSource),
        leaves_source: costs.iter().any(|cost| {
            matches!(
                cost,
                super::CostDef::SacrificeSource
                    | super::CostDef::ExileSource
                    | super::CostDef::ReturnSourceToHand
            )
        }),
    }
}
