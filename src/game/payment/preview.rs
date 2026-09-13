//! A proposal is evaluated at the caller's real payment boundary. Preparing a
//! clone freezes costs and removes a proposed spell from its former zone, while
//! leaving mana abilities, state-based actions, and priority unadvanced.
use super::super::{Action, ActivationChoices, Game, ManaCost, ManaPaymentPurpose, PlayerId};
use super::{ManaPaymentObligation, resources::PaymentReservation};

#[derive(Clone, Debug)]
pub(in crate::game) enum PaymentProbe {
    Requested,
    Ready(PaymentFrame),
}

#[derive(Clone, Debug)]
pub(in crate::game) struct PaymentFrame {
    pub(in crate::game) obligation: ManaPaymentObligation,
    pub(in crate::game) reserved: Vec<PaymentReservation>,
    pub(in crate::game) allows_mana_abilities: bool,
}

impl Game {
    pub(in crate::game) fn capture_payment_probe(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
        reserved: Vec<PaymentReservation>,
        allows_mana_abilities: bool,
    ) -> bool {
        if self.payment_probe.is_none() {
            return false;
        }
        self.payment_probe = Some(PaymentProbe::Ready(PaymentFrame {
            obligation: self.mana_payment_obligation(player, cost, x, purpose),
            reserved,
            allows_mana_abilities,
        }));
        true
    }

    pub(in crate::game) fn add_announced_cast_actions(
        &self,
        offer: super::super::CastOffer,
        actions: &mut Vec<Action>,
    ) {
        self.add_offered_cast_actions(offer, actions);
        let mut view = self.clone();
        view.payment_query = super::query::PaymentQuery::minimum_announcement();
        let mut announced = Vec::new();
        view.add_offered_cast_actions(offer, &mut announced);
        for action in announced {
            if !actions.contains(&action) {
                actions.push(action);
            }
        }
    }

    #[cfg(test)]
    pub(in crate::game) fn preview_payment(
        &self,
        player: PlayerId,
        action: &Action,
    ) -> Option<(Self, PaymentFrame)> {
        let (view, frame) = self.prepare_payment(player, action, &[])?;
        Some((view, frame?))
    }

    pub(in crate::game) fn prepare_payment(
        &self,
        player: PlayerId,
        action: &Action,
        answers: &[Vec<u32>],
    ) -> Option<(Self, Option<PaymentFrame>)> {
        let mut preview = self.clone();
        preview
            .pending_decisions
            .retain(|pending| pending.continuation.cast_offer().is_some());
        preview.explicit_mana_payment = None;
        preview.payment_query = super::query::PaymentQuery::announcement();
        preview.payment_probe = Some(PaymentProbe::Requested);
        preview.start_payment_operation(player, action)?;
        preview.answer_payment_announcements(answers)?;
        let frame = match preview.payment_probe.take()? {
            PaymentProbe::Ready(frame) => Some(frame),
            PaymentProbe::Requested if !preview.pending_decisions.is_empty() => None,
            PaymentProbe::Requested => return None,
        };
        preview.payment_query = super::query::PaymentQuery::default();
        Some((preview, frame))
    }

    pub(in crate::game) fn prepare_payment_draft(
        &self,
        draft: &super::funding::PaymentDraft,
    ) -> Option<(Self, Option<PaymentFrame>)> {
        let mut base = self.clone();
        base.pending_decisions.clear();
        if let Some(pending) = &draft.resume {
            base.pending_decisions.push(*pending.clone());
        }
        base.prepare_payment(draft.player, &draft.action, &draft.announcements)
    }

    pub(in crate::game) fn answer_payment_announcements(
        &mut self,
        answers: &[Vec<u32>],
    ) -> Option<()> {
        for options in answers {
            let observation = &self.pending_decisions.first()?.observation;
            let player = observation.player;
            let decision = observation.id;
            if !self.is_legal_action(
                player,
                &Action::ChooseDecision {
                    decision,
                    options: options.clone(),
                },
            ) {
                return None;
            }
            self.choose_decision(player, decision, options);
        }
        Some(())
    }

    pub(in crate::game) fn start_payment_operation(
        &mut self,
        player: PlayerId,
        action: &Action,
    ) -> Option<()> {
        match action.clone() {
            Action::CastSpell {
                card,
                choices,
                sacrifices,
            } => self.cast_spell(player, card, &choices, &sacrifices),
            Action::ActivateAbility {
                source,
                ability,
                targets,
                cost_objects,
                x,
                modes,
                mana_payment,
            } => self.activate_ability(
                player,
                source,
                ability,
                ActivationChoices {
                    targets,
                    cost_objects: &cost_objects,
                    x,
                    modes: &modes,
                    mana_payment: mana_payment.as_deref(),
                },
            ),
            Action::ActivateManaAbility {
                source,
                ability,
                color,
                counters_removed,
                cost_object,
                combination,
                triggered_mana,
            } => {
                self.mana_activation_for_action(player, action)?;
                self.activate_mana_source(
                    player,
                    source,
                    ability,
                    color,
                    &super::super::ManaActivationChoices::new(
                        counters_removed,
                        cost_object,
                        combination,
                        triggered_mana,
                    ),
                );
            }
            _ => return None,
        }
        Some(())
    }
}
