//! Mana abilities may be used when a resolving procedure requests mana. The
//! pending effect retains its continuation; opening this window grants no priority.
use super::super::{
    Action, DecisionContinuation, Game, ManaCost, ManaPaymentPurpose, PendingDecision, PlayerId,
    ResolvedEffectPayment,
};
use super::{
    ManaPaymentObligation,
    state::{PaymentDecision, PaymentTarget},
};

pub(in crate::game) fn requested_payment(
    pending: &PendingDecision,
) -> Option<&ResolvedEffectPayment> {
    match &pending.continuation {
        DecisionContinuation::PayOr { payment, .. }
        | DecisionContinuation::PaySpecialAction { payment, .. } => Some(payment),
        _ => None,
    }
}

pub(in crate::game) fn contains_mana(payment: &ResolvedEffectPayment) -> bool {
    match payment {
        ResolvedEffectPayment::Mana(_)
        | ResolvedEffectPayment::LabeledMana { .. }
        | ResolvedEffectPayment::SnowMana { .. }
        | ResolvedEffectPayment::ChosenGenericMana => true,
        ResolvedEffectPayment::All(parts) | ResolvedEffectPayment::Choice(parts) => {
            parts.iter().any(contains_mana)
        }
        _ => false,
    }
}

impl Game {
    pub(in crate::game) fn payment_mana_window(&self) -> bool {
        self.pending_decisions
            .first()
            .and_then(requested_payment)
            .is_some_and(contains_mana)
    }

    pub(in crate::game) fn add_payment_mana_actions(
        &self,
        player: PlayerId,
        actions: &mut Vec<Action>,
    ) {
        let mut mana = Vec::new();
        self.add_mana_actions(player, &mut mana);
        actions.extend(mana.into_iter().filter(|action| {
            self.mana_activation_for_action(player, action)
                .is_some_and(|activation| !activation.only_as_instant)
        }));
    }

    pub(in crate::game) fn explicit_effect_options(
        &self,
        pending: &PendingDecision,
    ) -> Vec<super::super::DecisionOption> {
        let Some(payment) = requested_payment(pending) else {
            return Vec::new();
        };
        let mut view = self.clone();
        view.payment_query = super::query::PaymentQuery::announcement();
        view.explicit_mana_payment = None;
        view.explicit_mana_payment_tail.clear();
        view.payment_options(pending.observation.player, payment.clone(), true, "Decline")
    }

    pub(in crate::game) fn effect_payment_obligation(
        &self,
        pending: &PendingDecision,
        chosen: u32,
    ) -> Option<ManaPaymentObligation> {
        self.effect_payment_obligations(pending, chosen)?
            .into_iter()
            .next()
    }

    pub(in crate::game) fn effect_payment_obligations(
        &self,
        pending: &PendingDecision,
        chosen: u32,
    ) -> Option<Vec<ManaPaymentObligation>> {
        if chosen == 0 {
            return None;
        }
        let payment = requested_payment(pending)?;
        let player = pending.observation.player;
        let (cost, purpose) = match payment {
            ResolvedEffectPayment::Mana(cost) => (*cost, ManaPaymentPurpose::Other),
            ResolvedEffectPayment::LabeledMana {
                source,
                label,
                cost,
            } => (
                *cost,
                ManaPaymentPurpose::Payment {
                    source: *source,
                    label: Some(*label),
                    snow: false,
                },
            ),
            ResolvedEffectPayment::SnowMana {
                source,
                label,
                amount,
            } => (
                ManaCost::new(*amount, 0),
                ManaPaymentPurpose::Payment {
                    source: *source,
                    label: *label,
                    snow: true,
                },
            ),
            ResolvedEffectPayment::ChosenGenericMana => (
                ManaCost::new(u16::try_from(chosen).ok()?, 0),
                ManaPaymentPurpose::Other,
            ),
            ResolvedEffectPayment::All(_) | ResolvedEffectPayment::Choice(_) => {
                let mut view = self.clone();
                view.payment_query = super::query::PaymentQuery::announcement();
                return view.cost_list_mana_obligations(player, payment, chosen);
            }
            _ => return None,
        };
        Some(vec![
            self.mana_payment_obligation(player, cost, 0, &purpose),
        ])
    }

    /// Funding can make a previously unavailable payment available. Refresh
    /// its exact choices and invalidate the old decision id when they change.
    pub(in crate::game) fn refresh_payment_offer(&mut self) {
        let Some(pending) = self.pending_decisions.first() else {
            return;
        };
        let Some(payment) = requested_payment(pending).cloned() else {
            return;
        };
        let player = pending.observation.player;
        let decline = pending
            .observation
            .options
            .first()
            .map_or("Decline", |o| o.label.as_str());
        let options = self.payment_options(
            player,
            payment.clone(),
            self.can_pay_effect_payment(player, payment),
            decline,
        );
        if options != pending.observation.options {
            let pending = &mut self.pending_decisions[0];
            pending.observation.options = options;
            pending.observation.id = self.next_decision_id;
            self.next_decision_id = self.next_decision_id.saturating_add(1);
        }
    }

    pub(in crate::game) fn cancel_explicit_payment(&mut self, payment: PaymentDecision) {
        let resume = match payment {
            PaymentDecision::Funding(draft)
            | PaymentDecision::Mana {
                target: PaymentTarget::Draft(draft) | PaymentTarget::Funding { draft, .. },
                ..
            } => draft.resume,
            PaymentDecision::Operation { resume, .. }
            | PaymentDecision::Mana {
                target: PaymentTarget::Action { resume, .. },
                ..
            } => resume,
            PaymentDecision::Mana {
                target: PaymentTarget::Effect { pending, .. },
                ..
            } => Some(pending),
        };
        if let Some(pending) = resume {
            self.pending_decisions.insert(0, *pending);
        }
    }
}
