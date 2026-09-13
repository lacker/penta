//! Explicit allocations are proposals until the complete payment is submitted.
use super::super::{
    Action, DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility,
    DecisionZone, Game, ManaCost, PendingDecision, PlayerId,
};
use super::{
    BoundManaPayment, ManaPaymentObligation,
    funding::{FundingStep, PaymentDraft},
    state::{PaymentDecision, PaymentTarget},
};

impl Game {
    /// Avoid opening an empty editor in a forced priority window. This only
    /// proves that there is no initial payment resource; it never asks the
    /// automatic planner to find a complete payment.
    pub(in crate::game) fn explicit_payment_available(
        &self,
        player: PlayerId,
        actions: &[Action],
    ) -> bool {
        let initial_mana = self.players[player.index()].mana_pool.total() > 0
            || actions
                .iter()
                .any(|action| matches!(action, Action::ActivateManaAbility { .. }));
        let mut view = self.clone();
        view.payment_query = super::query::PaymentQuery::minimum_announcement();
        view.announced_payment_actions(player, None)
            .iter()
            .any(|action| {
                let Some(bill) = view.explicit_payment_obligation(player, action) else {
                    return false;
                };
                if initial_mana || bill.cost.mana_value() == 0 {
                    return true;
                }
                let kinds = self.payment_contributions(&bill.purpose);
                (kinds.delve && !self.players[player.index()].graveyard.is_empty())
                    || self
                        .battlefield
                        .iter()
                        .filter(|p| p.controller == player)
                        .any(|p| !self.permanent_contribution_outputs(p, kinds).is_empty())
            })
    }

    #[cfg(test)]
    pub(in crate::game) fn manual_payment_actions(&self, player: PlayerId) -> Vec<Action> {
        let resume =
            self.pending_decisions
                .first()
                .and_then(|pending| match &pending.continuation {
                    DecisionContinuation::Payment(PaymentDecision::Operation {
                        resume, ..
                    }) => resume.as_deref(),
                    _ => (super::effects::requested_payment(pending).is_some()
                        || pending.continuation.cast_offer().is_some())
                    .then_some(pending),
                });
        let x = self
            .pending_decisions
            .first()
            .and_then(|pending| match pending.continuation {
                DecisionContinuation::Payment(PaymentDecision::Operation { x, .. }) => Some(x),
                _ => None,
            })
            .unwrap_or(0);
        self.manual_payment_actions_at_x(player, resume, x)
    }

    pub(in crate::game) fn manual_payment_actions_in(
        &self,
        player: PlayerId,
        resume: Option<&PendingDecision>,
    ) -> Vec<Action> {
        self.manual_payment_actions_at_x(player, resume, 0)
    }

    pub(in crate::game) fn manual_payment_actions_at_x(
        &self,
        player: PlayerId,
        resume: Option<&PendingDecision>,
        x: u16,
    ) -> Vec<Action> {
        let mut view = self.clone();
        view.payment_query = super::query::PaymentQuery::with_x(x);
        if let Some(pending) = resume {
            view.pending_decisions = vec![pending.clone()];
        }
        view.announced_payment_actions(player, resume)
    }

    fn announced_payment_actions(
        &self,
        player: PlayerId,
        resume: Option<&PendingDecision>,
    ) -> Vec<Action> {
        let mut actions = Vec::new();
        if let Some(offer) = resume.and_then(|pending| pending.continuation.cast_offer()) {
            self.add_offered_cast_actions(offer, &mut actions);
        } else if resume.is_some() {
            self.add_payment_mana_actions(player, &mut actions);
        } else {
            self.add_spell_actions(player, &mut actions);
            self.add_ability_actions(player, &mut actions);
            self.add_mana_actions(player, &mut actions);
        }
        actions.retain(|action| {
            self.explicit_payment_obligation(player, action)
                .is_some_and(|obligation| {
                    resume.is_none_or(|pending| pending.continuation.cast_offer().is_some())
                        || self.pool_can_pay_obligation(&obligation)
                })
        });
        actions
    }

    pub(in crate::game) fn pool_can_pay_obligation(
        &self,
        obligation: &ManaPaymentObligation,
    ) -> bool {
        let pool = self.eligible_mana_pool(obligation.player, &obligation.purpose);
        super::super::mana_planning::payment_remainder(
            pool,
            obligation.cost,
            obligation.x,
            &|_| 0,
            &super::super::ManaColor::ALL,
            false,
        )
        .is_some()
    }

    pub(in crate::game) fn explicit_payment_obligation(
        &self,
        player: PlayerId,
        action: &Action,
    ) -> Option<ManaPaymentObligation> {
        if let Some(activation) = self.mana_activation_for_action(player, action) {
            let mut cost = ManaCost::default();
            let mut has_mana = false;
            for payment in &activation.costs {
                if let super::super::CostDef::Mana(mana) = payment {
                    cost = super::super::add_mana_cost(cost, *mana);
                    has_mana = true;
                }
            }
            return has_mana.then(|| {
                self.mana_payment_obligation(
                    player,
                    cost,
                    0,
                    &super::mana_ability_payment_purpose(activation.source, &activation.costs),
                )
            });
        }
        let (cost, x, _, purpose) = self.mana_requirement(player, action)?;
        Some(self.mana_payment_obligation(player, cost, x, &purpose))
    }

    pub(in crate::game) fn explicit_effect_choices(&self, pending: &PendingDecision) -> Vec<u32> {
        self.explicit_effect_options(pending)
            .iter()
            .filter(|option| {
                self.effect_payment_obligation(pending, option.id)
                    .is_some_and(|obligation| self.pool_can_pay_obligation(&obligation))
            })
            .map(|option| option.id)
            .collect()
    }

    pub(in crate::game) fn begin_explicit_payment(&mut self, player: PlayerId) {
        self.begin_explicit_payment_at_x(player, 0);
    }

    pub(in crate::game) fn begin_explicit_payment_at_x(&mut self, player: PlayerId, x: u16) {
        let resume = (self.payment_mana_window()
            || self
                .pending_decisions
                .first()
                .is_some_and(|pending| pending.continuation.cast_offer().is_some()))
        .then(|| Box::new(self.pending_decisions.remove(0)));
        let mut options: Vec<_> = self
            .manual_payment_actions_at_x(player, resume.as_deref(), x)
            .iter()
            .enumerate()
            .map(|(index, action)| self.payment_operation_option(player, index, action))
            .collect();
        if let Some(pending) = &resume {
            let offered_options = self.explicit_effect_options(pending);
            for choice in self.explicit_effect_choices(pending) {
                let offered = offered_options
                    .iter()
                    .find(|option| option.id == choice)
                    .expect("existing choice");
                let mut option = offered.clone();
                option.id = u32::try_from(options.len()).expect("payment choice fits u32");
                option.label = format!("Choose mana: {}", option.label);
                options.push(option);
            }
        }
        if resume.is_none()
            || resume
                .as_ref()
                .is_some_and(|pending| pending.continuation.cast_offer().is_some())
        {
            for (index, delta) in [1u16, 10, 100, 1000].into_iter().enumerate() {
                if let Some(next) = x.checked_add(delta) {
                    options.push(payment_option(
                        (u32::MAX - u32::try_from(index).expect("option index fits u32")) as usize,
                        format!("Use X = {next}"),
                    ));
                }
                if let Some(next) = x.checked_sub(delta) {
                    options.push(payment_option(
                        (u32::MAX - 4 - u32::try_from(index).expect("option index fits u32"))
                            as usize,
                        format!("Use X = {next}"),
                    ));
                }
            }
        }
        self.queue_decision(
            player,
            if x == 0 {
                "Choose what to pay for".into()
            } else {
                format!("Choose what to pay for (X = {x})")
            },
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            true,
            options,
            DecisionContinuation::Payment(PaymentDecision::Operation { player, x, resume }),
        );
    }

    pub(in crate::game) fn queue_exact_mana(
        &mut self,
        target: PaymentTarget,
        obligation: ManaPaymentObligation,
        selected: Vec<usize>,
    ) {
        let player = obligation.player;
        let view = self
            .exact_payment_view(&target)
            .expect("a selected funding proposal remains valid");
        let mut options = Vec::new();
        if view.validate_mana_payment(
            &obligation,
            &BoundManaPayment {
                units: selected.clone(),
            },
        ) {
            options.push(payment_option(0, "Pay with the selected mana".into()));
        }
        for (index, mana) in view.payment_mana_units(player).iter().enumerate() {
            let mut next = selected.clone();
            next.push(index);
            if view.mana_selection_can_complete(&obligation, &next) {
                options.push(view.payment_unit_option(player, index, *mana));
            }
        }
        if !selected.is_empty() {
            options.push(payment_option(
                u32::MAX as usize,
                "Undo the last mana selection".into(),
            ));
        }
        self.queue_decision(
            player,
            format!(
                "Choose mana to pay {} ({} selected)",
                obligation.cost,
                selected.len()
            ),
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            true,
            options,
            DecisionContinuation::Payment(PaymentDecision::Mana {
                target,
                obligation,
                selected,
            }),
        );
    }

    pub(in crate::game) fn resolve_explicit_payment(
        &mut self,
        payment: PaymentDecision,
        selected: &[u32],
    ) {
        match payment {
            PaymentDecision::Funding(draft) => self.resolve_funding_choice(draft, selected),
            PaymentDecision::Operation { player, x, resume } => {
                if (resume.is_none()
                    || resume
                        .as_ref()
                        .is_some_and(|pending| pending.continuation.cast_offer().is_some()))
                    && let Some(choice) = selected.first().filter(|choice| **choice >= u32::MAX - 7)
                {
                    let control = (u32::MAX - choice) as usize;
                    let delta = [1u16, 10, 100, 1000][control % 4];
                    let next = if control < 4 {
                        x.checked_add(delta)
                    } else {
                        x.checked_sub(delta)
                    }
                    .expect("an offered X adjustment fits");
                    if let Some(pending) = resume {
                        self.pending_decisions.insert(0, *pending);
                    }
                    self.begin_explicit_payment_at_x(player, next);
                    return;
                }
                let actions = self.manual_payment_actions_at_x(player, resume.as_deref(), x);
                let Some(index) = selected.first().map(|index| *index as usize) else {
                    self.cancel_explicit_payment(PaymentDecision::Operation { player, x, resume });
                    return;
                };
                if let Some(action) = actions.get(index).cloned() {
                    if resume.is_none()
                        || resume
                            .as_ref()
                            .is_some_and(|pending| pending.continuation.cast_offer().is_some())
                    {
                        self.queue_funding(PaymentDraft {
                            player,
                            action: Box::new(action),
                            funding: Vec::new(),
                            contributions: Vec::new(),
                            announcements: Vec::new(),
                            resume,
                        });
                    } else {
                        let obligation = self
                            .explicit_payment_obligation(player, &action)
                            .expect("offered payment has a cost");
                        self.queue_exact_mana(
                            PaymentTarget::Action {
                                action: Box::new(action),
                                resume,
                            },
                            obligation,
                            Vec::new(),
                        );
                    }
                } else if let Some(pending) = resume {
                    let chosen = self.explicit_effect_choices(&pending)[index - actions.len()];
                    let obligation = self
                        .effect_payment_obligation(&pending, chosen)
                        .expect("offered effect has mana");
                    self.queue_exact_mana(
                        PaymentTarget::Effect {
                            pending,
                            answered: vec![chosen],
                            allocations: Vec::new(),
                        },
                        obligation,
                        Vec::new(),
                    );
                }
            }
            PaymentDecision::Mana {
                target,
                obligation,
                selected: units,
            } => {
                self.resolve_exact_mana(target, obligation, units, selected);
            }
        }
    }
    fn resolve_exact_mana(
        &mut self,
        target: PaymentTarget,
        obligation: ManaPaymentObligation,
        mut units: Vec<usize>,
        selected: &[u32],
    ) {
        let choice = selected.first().copied().expect("one payment choice");
        if choice != 0 {
            if choice == u32::MAX {
                units.pop();
            } else {
                units.push(choice as usize - 1);
            }
            self.queue_exact_mana(target, obligation, units);
            return;
        }
        let payment = BoundManaPayment { units };
        let view = self
            .exact_payment_view(&target)
            .expect("the submitted proposal remains valid");
        assert!(
            view.validate_mana_payment(&obligation, &payment),
            "the submitted allocation was validated"
        );
        match target {
            PaymentTarget::Draft(draft) => {
                self.commit_payment_draft(&draft, &payment)
                    .expect("the full payment was validated");
            }
            PaymentTarget::Funding { mut draft, action } => {
                draft.funding.push(FundingStep {
                    action: *action,
                    mana: Some(payment),
                    answers: Vec::new(),
                });
                self.queue_funding(draft);
            }
            PaymentTarget::Effect {
                pending,
                answered,
                mut allocations,
            } => {
                allocations.push(payment);
                let obligations = self
                    .effect_payment_obligations(&pending, answered[0])
                    .expect("an offered complete cost");
                if let Some(next) = obligations.get(allocations.len()).cloned() {
                    self.queue_exact_mana(
                        PaymentTarget::Effect {
                            pending,
                            answered,
                            allocations,
                        },
                        next,
                        Vec::new(),
                    );
                    return;
                }
                let mut pending = pending;
                pending.observation.options = self.explicit_effect_options(&pending);
                self.explicit_mana_payment_tail = allocations.into();
                self.explicit_mana_payment = self.explicit_mana_payment_tail.pop_front();
                let decision = pending.observation.id;
                self.pending_decisions.insert(0, *pending);
                self.choose_decision(obligation.player, decision, &answered);
                debug_assert!(
                    self.explicit_mana_payment.is_none()
                        && self.explicit_mana_payment_tail.is_empty()
                );
            }
            PaymentTarget::Action { action, resume } => {
                self.explicit_mana_payment = Some(payment);
                if let Some(pending) = resume {
                    self.pending_decisions.insert(0, *pending);
                }
                self.apply_legal_action(obligation.player, *action);
                debug_assert!(
                    self.explicit_mana_payment.is_none(),
                    "the operation consumed its allocation"
                );
            }
        }
    }
}

pub(in crate::game::payment) fn payment_option(index: usize, label: String) -> DecisionOption {
    DecisionOption {
        id: u32::try_from(index).expect("payment option fits u32"),
        label,
        card: None,
        members: Vec::new(),
        ability_text: None,
        zone: DecisionZone::None,
    }
}
