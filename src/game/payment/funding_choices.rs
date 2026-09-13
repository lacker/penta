//! The interactive editor for a complete payment program. Only the prepared
//! projection changes while the player selects funding abilities and mana.
use super::super::{Action, DecisionContinuation, DecisionPreference, DecisionVisibility, Game};
use super::{
    funding::{FundingStep, PaymentDraft},
    state::{PaymentDecision, PaymentTarget},
};

impl Game {
    pub(in crate::game) fn exact_payment_view(&self, target: &PaymentTarget) -> Option<Self> {
        match target {
            PaymentTarget::Draft(draft) | PaymentTarget::Funding { draft, .. } => {
                self.preview_funding(draft).map(|(game, _)| game)
            }
            PaymentTarget::Action { .. } => Some(self.clone()),
            PaymentTarget::Effect {
                pending,
                answered,
                allocations,
            } => {
                let obligations = self.effect_payment_obligations(pending, *answered.first()?)?;
                if allocations.len() >= obligations.len() {
                    return None;
                }
                let mut view = self.clone();
                for (obligation, allocation) in obligations.iter().zip(allocations) {
                    view.commit_mana_payment(obligation, allocation)?;
                }
                Some(view)
            }
        }
    }

    pub(in crate::game) fn funding_candidates(&self, draft: &PaymentDraft) -> Option<Vec<Action>> {
        let (preview, frame) = self.preview_funding(draft)?;
        if !frame.allows_mana_abilities || !preview.pending_decisions.is_empty() {
            return Some(Vec::new());
        }
        let mut actions = Vec::new();
        preview.add_payment_mana_actions(draft.player, &mut actions);
        actions.retain(|action| {
            if preview
                .explicit_payment_obligation(draft.player, action)
                .is_some_and(|obligation| !preview.pool_can_pay_obligation(&obligation))
            {
                return false;
            }
            let mut candidate = draft.clone();
            candidate.funding.push(FundingStep {
                action: action.clone(),
                mana: None,
                answers: Vec::new(),
            });
            self.preview_funding(&candidate).is_some()
        });
        Some(actions)
    }

    pub(in crate::game) fn queue_funding(&mut self, draft: PaymentDraft) {
        let (announcement, frame) = self
            .prepare_payment_draft(&draft)
            .expect("an offered operation can be announced");
        if frame.is_none() {
            self.queue_payment_program_choice(
                draft,
                &announcement.pending_decisions[0].observation,
            );
            return;
        }
        let (preview, frame) = self
            .preview_funding(&draft)
            .expect("an offered payment program has valid steps");
        if let Some(pending) = preview.pending_decisions.first() {
            self.queue_payment_program_choice(draft, &pending.observation);
            return;
        }
        let mut options = Vec::new();
        if preview.pool_can_pay_obligation(&frame.obligation) {
            options.push(super::choices::payment_option(
                0,
                "Choose mana for the payment".into(),
            ));
        }
        let abilities = self
            .funding_candidates(&draft)
            .expect("valid funding projection");
        for (index, action) in abilities.iter().enumerate() {
            options.push(preview.payment_operation_option(draft.player, index + 1, action));
        }
        for (index, contribution) in self
            .contribution_candidates(&draft)
            .expect("valid contributions")
            .iter()
            .enumerate()
        {
            options.push(preview.payment_contribution_option(
                draft.player,
                abilities.len() + index + 1,
                *contribution,
            ));
        }
        if !draft.contributions.is_empty() {
            options.push(super::choices::payment_option(
                (u32::MAX - 1) as usize,
                "Undo the last direct contribution".into(),
            ));
        }
        if !draft.funding.is_empty() {
            options.push(super::choices::payment_option(
                u32::MAX as usize,
                "Undo the last mana ability".into(),
            ));
        }
        self.queue_decision(
            draft.player,
            format!(
                "Plan funding for {} ({} abilities selected)",
                frame.obligation.cost,
                draft.funding.len()
            ),
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            true,
            options,
            DecisionContinuation::Payment(PaymentDecision::Funding(draft)),
        );
    }

    fn queue_payment_program_choice(
        &mut self,
        draft: PaymentDraft,
        observation: &super::super::DecisionObservation,
    ) {
        let mut observation = observation.clone();
        observation.id = self.next_decision_id;
        self.next_decision_id = self.next_decision_id.saturating_add(1);
        observation.cancellable = observation.player == draft.player;
        self.pending_decisions.push(super::super::PendingDecision {
            observation,
            continuation: DecisionContinuation::Payment(PaymentDecision::Funding(draft)),
        });
    }

    pub(in crate::game) fn resolve_funding_choice(
        &mut self,
        mut draft: PaymentDraft,
        options: &[u32],
    ) {
        if self
            .prepare_payment_draft(&draft)
            .is_some_and(|(_, frame)| frame.is_none())
        {
            draft.announcements.push(options.to_vec());
            self.queue_funding(draft);
            return;
        }
        if self
            .preview_funding(&draft)
            .is_some_and(|(view, _)| !view.pending_decisions.is_empty())
        {
            draft
                .funding
                .last_mut()
                .expect("a funding choice belongs to a selected ability")
                .answers
                .push(options.to_vec());
            if self.preview_funding(&draft).is_some() {
                self.queue_funding(draft);
            } else {
                // A replacement answer can make the proposed payment impossible.
                // Roll back the uncommitted proposal instead of spending a prefix.
                self.cancel_explicit_payment(PaymentDecision::Funding(draft));
            }
            return;
        }
        let Some(choice) = options.first().copied() else {
            return;
        };
        if choice == 0 {
            let (_, frame) = self.preview_funding(&draft).expect("valid funding program");
            self.queue_exact_mana(PaymentTarget::Draft(draft), frame.obligation, Vec::new());
        } else if choice == u32::MAX {
            draft.funding.pop();
            // Later contributions may depend on the removed ability.
            draft.contributions.clear();
            self.queue_funding(draft);
        } else if choice == u32::MAX - 1 {
            draft.contributions.pop();
            self.queue_funding(draft);
        } else {
            let abilities = self
                .funding_candidates(&draft)
                .expect("valid funding program");
            let index = choice as usize - 1;
            if index >= abilities.len() {
                draft.contributions.push(
                    self.contribution_candidates(&draft)
                        .expect("valid contributions")[index - abilities.len()],
                );
                self.queue_funding(draft);
                return;
            }
            let action = abilities[index].clone();
            let (preview, _) = self.preview_funding(&draft).expect("valid funding program");
            if let Some(obligation) = preview.explicit_payment_obligation(draft.player, &action) {
                self.queue_exact_mana(
                    PaymentTarget::Funding {
                        draft,
                        action: Box::new(action),
                    },
                    obligation,
                    Vec::new(),
                );
            } else {
                draft.funding.push(FundingStep {
                    action,
                    mana: None,
                    answers: Vec::new(),
                });
                self.queue_funding(draft);
            }
        }
    }
}
