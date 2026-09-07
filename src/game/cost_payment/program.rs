//! Expand authored cost composition into a semantic plan without paying it.
//! Answers belong to a single payment window, never to prepared catalog data.

use super::{CostDef, CostPaymentWindow, GameObjectId, MechanicId};
use crate::card::{CostQuantityDef, EffectDef, ZoneKind};
use crate::game::{Game, Mana, ResolvedEffectPayment};
use std::collections::VecDeque;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::game) enum PaymentAnswer {
    Choice(usize),
    Objects(Vec<GameObjectId>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::game) struct PaymentPart {
    pub cost: CostDef,
    pub times: u16,
    pub objects: Vec<GameObjectId>,
    pub mechanics: Vec<MechanicId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::game) struct NamedPayment {
    pub mechanic: MechanicId,
    pub repetitions: u16,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(in crate::game) struct PaymentPlan {
    pub parts: VecDeque<PaymentPart>,
    pub named: Vec<NamedPayment>,
    pub mana_spent: Vec<Mana>,
}

#[derive(Clone, Debug)]
pub(in crate::game) enum PaymentQuestion {
    Choice(&'static [CostDef]),
    Objects(PaymentPart),
    Confirm,
}

pub(in crate::game) struct PaymentEvaluation {
    pub plan: PaymentPlan,
    pub question: PaymentQuestion,
}

struct Builder<'a> {
    game: &'a Game,
    window: &'a CostPaymentWindow,
    answers: std::slice::Iter<'a, PaymentAnswer>,
    plan: PaymentPlan,
    pending: Option<PaymentQuestion>,
    mechanics: Vec<MechanicId>,
    validate_selection: bool,
}

impl Game {
    pub(in crate::game) fn evaluate_payment_window(
        &self,
        window: &CostPaymentWindow,
    ) -> Option<PaymentEvaluation> {
        self.evaluate_payment_program(window, true)
    }

    // Committed checkpoints validate the authored shape, not affordability:
    // earlier selected objects may already have left their original zones.
    pub(in crate::game) fn evaluate_payment_program(
        &self,
        window: &CostPaymentWindow,
        validate_selection: bool,
    ) -> Option<PaymentEvaluation> {
        let EffectDef::PayOr(definition) = window.definition.effect else {
            return None;
        };
        let mut builder = Builder {
            game: self,
            window,
            answers: window.answers.iter(),
            plan: PaymentPlan::default(),
            pending: None,
            mechanics: Vec::new(),
            validate_selection,
        };
        builder.visit(definition.payment.cost, 1)?;
        if builder.answers.next().is_some() {
            return None;
        }
        Some(PaymentEvaluation {
            plan: builder.plan,
            question: builder.pending.unwrap_or(PaymentQuestion::Confirm),
        })
    }

    pub(in crate::game) fn payment_part_candidates(
        &self,
        window: &CostPaymentWindow,
        part: &PaymentPart,
        reserved: &[GameObjectId],
    ) -> Vec<GameObjectId> {
        let source = window.object.source.unwrap_or(window.object.id);
        let candidates = match part.cost {
            CostDef::Action(effect) => self.action_program_candidates(window, *effect),
            cost => self.object_cost_candidates(window.player, source, cost),
        };
        candidates
            .into_iter()
            .filter(|id| !reserved.contains(id))
            .collect()
    }

    pub(in crate::game) fn payment_part_quantity(
        &self,
        window: &CostPaymentWindow,
        part: &PaymentPart,
    ) -> Option<(ZoneKind, CostQuantityDef)> {
        let (zone, quantity) = if let CostDef::Action(EffectDef::ChooseExact(choice)) = part.cost {
            (
                ZoneKind::Battlefield,
                CostQuantityDef::Fixed(
                    u16::try_from(
                        self.effect_value(
                            choice.amount,
                            &window.object,
                            &window.context,
                            window.definition,
                        )
                        .max(0),
                    )
                    .ok()?,
                ),
            )
        } else {
            let (_, zone, quantity) = part.cost.object_selection()?;
            (zone, quantity)
        };
        let quantity = match quantity {
            CostQuantityDef::Fixed(count) => CostQuantityDef::Fixed(count.checked_mul(part.times)?),
            quantity if part.times == 1 => quantity,
            _ => return None,
        };
        Some((zone, quantity))
    }

    pub(in crate::game) fn payment_part_is_valid(
        &self,
        window: &CostPaymentWindow,
        part: &PaymentPart,
        reserved: &[GameObjectId],
    ) -> bool {
        let Some((_, quantity)) = self.payment_part_quantity(window, part) else {
            return false;
        };
        self.object_selection_is_valid(
            &self.payment_part_candidates(window, part, reserved),
            &part.objects,
            quantity,
        )
    }

    pub(in crate::game) fn payment_scalar(
        window: &CostPaymentWindow,
        part: &PaymentPart,
    ) -> Option<ResolvedEffectPayment> {
        let source = window.object.source.unwrap_or(window.object.id);
        Self::resolve_repeated_scalar_cost(part.cost, source, part.times)
    }
}

impl Builder<'_> {
    fn visit(&mut self, cost: CostDef, times: u16) -> Option<()> {
        if self.pending.is_some() {
            return Some(());
        }
        match cost {
            CostDef::Named { mechanic, cost } => {
                let repetitions = if let CostDef::Repeat { times: value, .. } = cost {
                    self.amount(*value)?.checked_mul(times)?
                } else {
                    times
                };
                self.plan.named.push(NamedPayment {
                    mechanic,
                    repetitions,
                });
                self.mechanics.push(mechanic);
                let result = self.visit(*cost, times);
                self.mechanics.pop();
                result
            }
            CostDef::Repeat { cost, times: value } => {
                self.visit(*cost, self.amount(value)?.checked_mul(times)?)
            }
            CostDef::All(costs) => {
                for _ in 0..times {
                    for cost in costs {
                        self.visit(*cost, 1)?;
                    }
                }
                Some(())
            }
            CostDef::Choice(costs) => {
                for _ in 0..times {
                    if self.pending.is_some() {
                        break;
                    }
                    match self.answers.next() {
                        Some(PaymentAnswer::Choice(index)) => self.visit(*costs.get(*index)?, 1)?,
                        Some(PaymentAnswer::Objects(_)) => return None,
                        None => self.pending = Some(PaymentQuestion::Choice(costs)),
                    }
                }
                Some(())
            }
            cost => {
                let mut part = PaymentPart {
                    cost,
                    times,
                    objects: Vec::new(),
                    mechanics: self.mechanics.clone(),
                };
                if self
                    .game
                    .payment_part_quantity(self.window, &part)
                    .is_some()
                {
                    match self.answers.next() {
                        Some(PaymentAnswer::Objects(objects)) => part.objects.clone_from(objects),
                        Some(PaymentAnswer::Choice(_)) => return None,
                        None => {
                            self.pending = Some(PaymentQuestion::Objects(part));
                            return Some(());
                        }
                    }
                    let reserved = selected_objects(&self.plan);
                    if self.validate_selection {
                        if !self
                            .game
                            .payment_part_is_valid(self.window, &part, &reserved)
                        {
                            return None;
                        }
                    } else {
                        let (_, quantity) = self.game.payment_part_quantity(self.window, &part)?;
                        if usize::from(quantity.fixed_value()?) != part.objects.len()
                            || part.objects.iter().enumerate().any(|(index, id)| {
                                reserved.contains(id) || part.objects[..index].contains(id)
                            })
                        {
                            return None;
                        }
                    }
                } else {
                    Game::payment_scalar(self.window, &part)?;
                }
                self.plan.parts.push_back(part);
                Some(())
            }
        }
    }

    fn amount(&self, value: crate::card::ValueDef) -> Option<u16> {
        u16::try_from(
            self.game
                .effect_value(
                    value,
                    &self.window.object,
                    &self.window.context,
                    self.window.definition,
                )
                .max(0),
        )
        .ok()
    }
}

pub(in crate::game) fn selected_objects(plan: &PaymentPlan) -> Vec<GameObjectId> {
    plan.parts
        .iter()
        .flat_map(|part| part.objects.iter().copied())
        .collect()
}
