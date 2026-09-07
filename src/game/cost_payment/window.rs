//! Collect every choice before committing any part of one payment.

use super::{
    CostDef, CostPaymentWindow, GameObjectId, PaymentAnswer, PaymentPlan, PaymentQuestion,
    selected_objects,
};
use crate::card::{CostQuantityDef, EffectDef, ZoneKind};
use crate::game::{
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    Game,
};

pub(in crate::game) struct CostPaymentOffer {
    pub options: Vec<DecisionOption>,
    pub count: usize,
    pub cancellable: bool,
}

impl Game {
    pub(in crate::game) fn cost_payment_offer(
        &self,
        window: &CostPaymentWindow,
    ) -> Option<CostPaymentOffer> {
        let evaluation = self.evaluate_payment_window(window)?;
        match evaluation.question {
            PaymentQuestion::Choice(choices) => {
                if !window.chosen.is_empty() {
                    return None;
                }
                let mut options = vec![plain_option(0, "Decline".into())];
                for (index, choice) in choices.iter().enumerate() {
                    let mut trial = window.clone();
                    trial.answers.push(PaymentAnswer::Choice(index));
                    if self.payment_window_can_continue(&trial) {
                        options.push(plain_option(
                            u32::try_from(index + 1).ok()?,
                            action_cost_label(*choice),
                        ));
                    }
                }
                if !window.answers.is_empty() {
                    options.push(plain_option(
                        u32::try_from(choices.len() + 1).ok()?,
                        "Start selections over".into(),
                    ));
                }
                Some(CostPaymentOffer {
                    options,
                    count: 1,
                    cancellable: false,
                })
            }
            PaymentQuestion::Objects(part) => {
                let (zone, quantity) = self.payment_part_quantity(window, &part)?;
                let candidates = self.payment_part_candidates(
                    window,
                    &part,
                    &selected_objects(&evaluation.plan),
                );
                if !self.object_selection_is_payable(&candidates, quantity) {
                    return (!window.answers.is_empty()).then(|| CostPaymentOffer {
                        options: vec![
                            plain_option(0, "Decline".into()),
                            plain_option(2, "Start selections over".into()),
                        ],
                        count: 1,
                        cancellable: false,
                    });
                }
                if window
                    .chosen
                    .iter()
                    .enumerate()
                    .any(|(i, id)| !candidates.contains(id) || window.chosen[..i].contains(id))
                {
                    return None;
                }
                let threshold = matches!(quantity, CostQuantityDef::ObjectSetValueAtLeast(_));
                if !threshold && !window.chosen.is_empty() {
                    return None;
                }
                let mut options = Vec::new();
                if threshold
                    && self.object_selection_is_valid(&candidates, &window.chosen, quantity)
                {
                    options.push(plain_option(0, "Pay selected objects".into()));
                }
                options.extend(
                    self.object_cost_options(&candidates, zone)
                        .into_iter()
                        .filter(|option| {
                            option
                                .card
                                .is_none_or(|(id, _)| !window.chosen.contains(&id))
                        }),
                );
                Some(CostPaymentOffer {
                    options,
                    count: if threshold {
                        1
                    } else {
                        usize::from(quantity.fixed_value()?)
                    },
                    cancellable: true,
                })
            }
            PaymentQuestion::Confirm => self.payment_confirmation_offer(window, &evaluation.plan),
        }
    }

    fn payment_confirmation_offer(
        &self,
        window: &CostPaymentWindow,
        plan: &PaymentPlan,
    ) -> Option<CostPaymentOffer> {
        if !window.chosen.is_empty() {
            return None;
        }
        let mut options = vec![plain_option(0, "Decline".into())];
        if self.payment_plan_is_executable(window, plan) {
            let label = if plan.parts.len() == 1 {
                plan.parts
                    .front()
                    .and_then(|part| Self::payment_scalar(window, part))
                    .map_or_else(|| "Pay the cost".into(), Self::effect_payment_label)
            } else {
                "Pay the cost".into()
            };
            options.push(plain_option(1, label));
        }
        if !window.answers.is_empty() {
            options.push(plain_option(2, "Start selections over".into()));
        }
        Some(CostPaymentOffer {
            options,
            count: 1,
            cancellable: false,
        })
    }

    fn payment_window_can_continue(&self, window: &CostPaymentWindow) -> bool {
        let Some(evaluation) = self.evaluate_payment_window(window) else {
            return false;
        };
        match evaluation.question {
            PaymentQuestion::Objects(part) => self
                .payment_part_quantity(window, &part)
                .is_some_and(|(_, quantity)| {
                    self.object_selection_is_payable(
                        &self.payment_part_candidates(
                            window,
                            &part,
                            &selected_objects(&evaluation.plan),
                        ),
                        quantity,
                    )
                }),
            PaymentQuestion::Choice(_) => true,
            PaymentQuestion::Confirm => self.payment_plan_is_executable(window, &evaluation.plan),
        }
    }

    pub(in crate::game) fn queue_cost_payment_window(&mut self, window: CostPaymentWindow) {
        if window.answers.is_empty()
            && self
                .evaluate_payment_window(&window)
                .is_some_and(|evaluation| {
                    matches!(evaluation.question, PaymentQuestion::Confirm)
                        && !self.payment_plan_is_executable(&window, &evaluation.plan)
                })
        {
            self.finish_cost_payment_window(window, false);
            return;
        }
        let Some(offer) = self.cost_payment_offer(&window) else {
            self.finish_cost_payment_window(window, false);
            return;
        };
        if !offer.cancellable && offer.options.len() == 1 && offer.options[0].id == 0 {
            self.finish_cost_payment_window(window, false);
            return;
        }
        let visibility = window.visibility();
        let source = window.object.source;
        self.queue_decision(
            window.player,
            window.object.ability_text().unwrap_or("Pay the cost?"),
            visibility,
            DecisionPreference::Neutral,
            offer.count..=offer.count,
            offer.cancellable,
            offer.options,
            DecisionContinuation::CostPayment(Box::new(window)),
        );
        if let Some(decision) = self.pending_decisions.last_mut() {
            decision.observation.source = source;
        }
    }

    pub(in crate::game) fn resolve_cost_payment_window(
        &mut self,
        mut window: CostPaymentWindow,
        selected: &[u32],
        options: &[DecisionOption],
    ) {
        let Some(evaluation) = self.evaluate_payment_window(&window) else {
            return;
        };
        match evaluation.question {
            PaymentQuestion::Choice(choices) => {
                let Some(index) = selected
                    .first()
                    .and_then(|id| id.checked_sub(1))
                    .and_then(|id| usize::try_from(id).ok())
                else {
                    self.finish_cost_payment_window(window, false);
                    return;
                };
                if selected.len() == 1 && index == choices.len() && !window.answers.is_empty() {
                    window.answers.clear();
                    self.queue_cost_payment_window(window);
                    return;
                }
                if selected.len() != 1 || index >= choices.len() {
                    return;
                }
                window.answers.push(PaymentAnswer::Choice(index));
                self.queue_cost_payment_window(window);
            }
            PaymentQuestion::Objects(part) => {
                let Some((_, quantity)) = self.payment_part_quantity(&window, &part) else {
                    return;
                };
                let candidates = self.payment_part_candidates(
                    &window,
                    &part,
                    &selected_objects(&evaluation.plan),
                );
                if !self.object_selection_is_payable(&candidates, quantity) {
                    if selected == [2] && !window.answers.is_empty() {
                        window.answers.clear();
                        window.chosen.clear();
                        self.queue_cost_payment_window(window);
                    } else {
                        self.finish_cost_payment_window(window, false);
                    }
                    return;
                }
                let mut members = selected
                    .iter()
                    .filter_map(|id| {
                        options
                            .iter()
                            .find(|option| option.id == *id)
                            .and_then(|option| option.card.map(|(id, _)| id))
                    })
                    .collect::<Vec<_>>();
                if matches!(quantity, CostQuantityDef::ObjectSetValueAtLeast(_)) {
                    if selected == [0] {
                        members.clone_from(&window.chosen);
                    } else if let [id] = members.as_slice()
                        && selected.len() == 1
                        && candidates.contains(id)
                        && !window.chosen.contains(id)
                    {
                        window.chosen.push(*id);
                        self.queue_cost_payment_window(window);
                        return;
                    } else {
                        return;
                    }
                } else if selected.len() != members.len() {
                    return;
                }
                if !self.object_selection_is_valid(&candidates, &members, quantity) {
                    return;
                }
                window.answers.push(PaymentAnswer::Objects(members));
                window.chosen.clear();
                let Some(next) = self.evaluate_payment_window(&window) else {
                    return;
                };
                if matches!(next.question, PaymentQuestion::Confirm)
                    && self.payment_plan_is_executable(&window, &next.plan)
                {
                    self.start_payment_commit(window, next.plan);
                } else {
                    self.queue_cost_payment_window(window);
                }
            }
            PaymentQuestion::Confirm => match selected {
                [1] if self.payment_plan_is_executable(&window, &evaluation.plan) => {
                    self.start_payment_commit(window, evaluation.plan);
                }
                [2] if !window.answers.is_empty() => {
                    window.answers.clear();
                    self.queue_cost_payment_window(window);
                }
                _ => self.finish_cost_payment_window(window, false),
            },
        }
    }

    pub(in crate::game) fn object_cost_options(
        &self,
        candidates: &[GameObjectId],
        from: ZoneKind,
    ) -> Vec<DecisionOption> {
        let zone = match from {
            ZoneKind::Hand => DecisionZone::Hand,
            ZoneKind::Graveyard => DecisionZone::Graveyard,
            ZoneKind::Battlefield => DecisionZone::Battlefield,
            _ => return Vec::new(),
        };
        candidates
            .iter()
            .enumerate()
            .filter_map(|(index, id)| {
                let characteristics = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == *id)
                    .map(Self::effective_rules_source)
                    .or_else(|| {
                        self.card_in_nonbattlefield_zone(*id).map(|(_, card)| {
                            crate::game::ObjectCharacteristics::card(
                                card.definition,
                                crate::game::CardPartId::PRIMARY,
                            )
                        })
                    })?;
                Some(DecisionOption {
                    id: u32::try_from(index + 1).ok()?,
                    label: self.characteristics_name(characteristics)?.into_owned(),
                    card: Some((*id, characteristics)),
                    members: Vec::new(),
                    ability_text: None,
                    zone,
                })
            })
            .collect()
    }
}

impl CostPaymentWindow {
    pub(in crate::game) fn visibility(&self) -> DecisionVisibility {
        let EffectDef::PayOr(definition) = self.definition.effect else {
            unreachable!()
        };
        if private_selection(definition.payment.cost) {
            DecisionVisibility::Private
        } else {
            crate::game::decision_offers::effect_choice_visibility(definition.visibility)
        }
    }
}

fn private_selection(cost: CostDef) -> bool {
    match cost {
        CostDef::Named { cost, .. } | CostDef::Repeat { cost, .. } => private_selection(*cost),
        CostDef::Choice(costs) | CostDef::All(costs) => {
            costs.iter().copied().any(private_selection)
        }
        cost => cost
            .object_selection()
            .is_some_and(|(_, zone, _)| zone == ZoneKind::Hand),
    }
}

fn plain_option(id: u32, label: String) -> DecisionOption {
    DecisionOption {
        id,
        label,
        card: None,
        members: Vec::new(),
        ability_text: None,
        zone: DecisionZone::None,
    }
}

fn action_cost_label(cost: CostDef) -> String {
    match cost {
        CostDef::Named { cost, .. } | CostDef::Repeat { cost, .. } => action_cost_label(*cost),
        CostDef::Sacrifice {
            quantity: CostQuantityDef::Fixed(count),
            ..
        } => format!("Sacrifice {count} permanent(s)"),
        CostDef::Exile {
            from: ZoneKind::Graveyard,
            quantity: CostQuantityDef::Fixed(count),
            ..
        } => format!("Exile {count} card(s) from your graveyard"),
        CostDef::Sacrifice { .. } => "Sacrifice matching permanents".into(),
        CostDef::Discard { .. } => "Discard matching cards".into(),
        CostDef::Exile { .. } => "Exile matching cards".into(),
        CostDef::Action(_) => "Perform the action".into(),
        CostDef::Choice(_) => "Choose a payment".into(),
        _ => "Pay the cost".into(),
    }
}
