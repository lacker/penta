//! Bound funding programs are validated by executing their selected mana
//! abilities on a prepared game. Automatic source search is not consulted.
use super::super::{Action, Game, ManaActivationChoices, PlayerId};
use super::{BoundManaPayment, preview::PaymentFrame};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::game) struct FundingStep {
    pub(in crate::game) action: Action,
    pub(in crate::game) mana: Option<BoundManaPayment>,
    /// Ordered answers to choices made while this mana ability pays its costs.
    pub(in crate::game) answers: Vec<Vec<u32>>,
}

#[derive(Clone, Debug)]
pub(in crate::game) struct PaymentDraft {
    pub(in crate::game) player: PlayerId,
    pub(in crate::game) action: Box<Action>,
    pub(in crate::game) funding: Vec<FundingStep>,
    pub(in crate::game) announcements: Vec<Vec<u32>>,
    pub(in crate::game) contributions: Vec<super::contributions::BoundContribution>,
    pub(in crate::game) resume: Option<Box<super::super::PendingDecision>>,
}

impl Game {
    pub(in crate::game) fn apply_funding_step(
        &mut self,
        player: PlayerId,
        step: &FundingStep,
    ) -> Option<()> {
        let activation = self.mana_activation_for_action(player, &step.action)?;
        if activation.only_as_instant {
            return None;
        }
        if let Some(mana) = &step.mana {
            let obligation = self.explicit_payment_obligation(player, &step.action)?;
            if !self.validate_mana_payment(&obligation, mana) {
                return None;
            }
        }
        let Action::ActivateManaAbility {
            source,
            ability,
            color,
            counters_removed,
            cost_object,
            combination,
            triggered_mana,
        } = &step.action
        else {
            return None;
        };
        self.explicit_mana_payment.clone_from(&step.mana);
        self.activate_mana_source(
            player,
            *source,
            *ability,
            *color,
            &ManaActivationChoices::new(
                *counters_removed,
                *cost_object,
                *combination,
                triggered_mana.clone(),
            ),
        );
        if self.explicit_mana_payment.is_some() {
            return None;
        }
        for answer in &step.answers {
            let pending = self.pending_decisions.first()?;
            let chooser = pending.observation.player;
            let decision = pending.observation.id;
            if !self.is_legal_action(
                chooser,
                &Action::ChooseDecision {
                    decision,
                    options: answer.clone(),
                },
            ) {
                return None;
            }
            self.choose_decision(chooser, decision, answer);
        }
        Some(())
    }

    pub(in crate::game) fn preview_funding(
        &self,
        draft: &PaymentDraft,
    ) -> Option<(Self, PaymentFrame)> {
        let (mut preview, frame) = self.prepare_payment_draft(draft)?;
        let mut frame = frame?;
        if !frame.allows_mana_abilities && !draft.funding.is_empty() {
            return None;
        }
        for (index, step) in draft.funding.iter().enumerate() {
            // CR 605.3: an unresolved mana ability cannot fund itself.
            if let (
                Action::ActivateManaAbility {
                    source, ability, ..
                },
                Action::ActivateManaAbility {
                    source: next_source,
                    ability: next_ability,
                    ..
                },
            ) = (draft.action.as_ref(), &step.action)
                && source == next_source
                && ability == next_ability
            {
                return None;
            }
            preview.apply_funding_step(draft.player, step)?;
            if !preview.pending_decisions.is_empty() && index + 1 != draft.funding.len() {
                return None;
            }
            if !preview.payment_reservations_hold(draft.player, &frame.reserved) {
                return None;
            }
        }
        preview.apply_contribution_bindings(&mut frame, &draft.contributions)?;
        Some((preview, frame))
    }

    pub(in crate::game) fn run_explicit_funding(&mut self, player: PlayerId) {
        let Some(funding) = self.explicit_funding.take() else {
            return;
        };
        self.payment_query = super::query::PaymentQuery::default();
        let allocation = self.explicit_mana_payment.take();
        for step in funding {
            self.apply_funding_step(player, &step)
                .expect("the bound funding program was validated at this payment boundary");
            assert!(
                self.pending_decisions.is_empty(),
                "a committed funding step has all its answers"
            );
        }
        self.explicit_mana_payment = allocation;
    }

    pub(in crate::game) fn commit_payment_draft(
        &mut self,
        draft: &PaymentDraft,
        mana: &BoundManaPayment,
    ) -> Option<()> {
        let (preview, frame) = self.preview_funding(draft)?;
        if !preview.pending_decisions.is_empty()
            || !preview.validate_mana_payment(&frame.obligation, mana)
        {
            return None;
        }
        let mut committed = self.clone();
        let queued = std::mem::take(&mut committed.pending_decisions);
        if let Some(pending) = &draft.resume {
            committed.pending_decisions.push(*pending.clone());
        }
        committed.explicit_mana_payment = Some(mana.clone());
        committed.explicit_funding = Some(draft.funding.clone());
        if matches!(draft.action.as_ref(), Action::CastSpell { .. }) {
            committed.explicit_cast_contributions = Some(Self::bind_cast_contributions(
                draft,
                frame.obligation.clone(),
            ));
        }
        committed.payment_query = super::query::PaymentQuery::announcement();
        committed.start_payment_operation(draft.player, &draft.action)?;
        committed.answer_payment_announcements(&draft.announcements)?;
        committed.pending_decisions.extend(queued);
        committed.finish_rules_procedure();
        debug_assert!(committed.explicit_mana_payment.is_none());
        debug_assert!(committed.explicit_funding.is_none());
        debug_assert!(committed.explicit_cast_contributions.is_none());
        committed.payment_query = super::query::PaymentQuery::default();
        *self = committed;
        Some(())
    }
}
