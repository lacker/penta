//! Whole-payment resource validation and commitment. This phase is entered
//! only after selection; cancellation cannot run once an action is committed.

use super::{CostDef, CostPaymentWindow, MechanicId, PaymentPlan, selected_objects};
use crate::card::{EffectDef, ManaCost, ZoneKind};
use crate::game::{
    BattlefieldExitCompletion, CommittedTriggerEvent, Game, ManaPaymentPurpose, ManaPlanOptions,
    ResolvedEffectPayment,
};

impl Game {
    fn payment_mana_requirement(
        window: &CostPaymentWindow,
        plan: &PaymentPlan,
    ) -> Option<(ManaCost, ManaPaymentPurpose)> {
        let mut mana = ManaCost::default();
        let mut life = 0_u16;
        let mut snow = false;
        let mut ordinary = false;
        let mut mechanics: Option<Vec<MechanicId>> = None;
        for part in &plan.parts {
            if matches!(part.cost, CostDef::Mana(_) | CostDef::SnowMana(_)) {
                if let Some(common) = &mut mechanics {
                    common.retain(|mechanic| part.mechanics.contains(mechanic));
                } else {
                    mechanics = Some(part.mechanics.clone());
                }
            }
            match Self::payment_scalar(window, part) {
                Some(ResolvedEffectPayment::Mana(cost)) => {
                    ordinary = true;
                    mana = crate::game::mana_planning::add_mana_cost(mana, cost);
                }
                Some(ResolvedEffectPayment::SnowMana { amount, .. }) => {
                    snow = true;
                    mana =
                        crate::game::mana_planning::add_mana_cost(mana, ManaCost::new(amount, 0));
                }
                Some(ResolvedEffectPayment::Life(amount)) => life = life.checked_add(amount)?,
                _ => {}
            }
        }
        // Mixed snow/ordinary allocations need independently constrained mana
        // slots. No supported declaration claims that joint shape yet.
        if snow && ordinary {
            return None;
        }
        Some((
            mana,
            ManaPaymentPurpose::Resolving {
                source: window.object.source.unwrap_or(window.object.id),
                mechanics: mechanics.unwrap_or_default(),
                reserved_life_payment: life,
                snow,
            },
        ))
    }

    pub(in crate::game) fn payment_plan_is_executable(
        &self,
        window: &CostPaymentWindow,
        plan: &PaymentPlan,
    ) -> bool {
        let mut reserved = Vec::new();
        let mut life = 0_u16;
        let mut energy = 0_u16;
        let mut exile_top = 0_u16;
        for part in &plan.parts {
            if self.payment_part_quantity(window, part).is_some() {
                if !self.payment_part_is_valid(window, part, &reserved) {
                    return false;
                }
                reserved.extend(&part.objects);
                continue;
            }
            let Some(scalar) = Self::payment_scalar(window, part) else {
                return false;
            };
            match scalar {
                ResolvedEffectPayment::Mana(_) | ResolvedEffectPayment::SnowMana { .. } => {}
                ResolvedEffectPayment::Life(amount) => {
                    let Some(total) = life.checked_add(amount) else {
                        return false;
                    };
                    life = total;
                }
                ResolvedEffectPayment::Energy(amount) => {
                    let Some(total) = energy.checked_add(amount) else {
                        return false;
                    };
                    energy = total;
                }
                ResolvedEffectPayment::ExileTopCards(amount) => {
                    let Some(total) = exile_top.checked_add(amount) else {
                        return false;
                    };
                    exile_top = total;
                }
                payment if !self.can_pay_effect_payment(window.player, payment) => return false,
                _ => {}
            }
        }
        if !self.can_pay_life(window.player, life)
            || !self.can_pay_effect_payment(window.player, ResolvedEffectPayment::Energy(energy))
            || !self.can_pay_effect_payment(
                window.player,
                ResolvedEffectPayment::ExileTopCards(exile_top),
            )
        {
            return false;
        }
        let Some((mana, purpose)) = Self::payment_mana_requirement(window, plan) else {
            return false;
        };
        self.plan_mana_activations_for_reserving(window.player, mana, 0, None, &purpose, &reserved)
            .is_some()
    }

    pub(in crate::game) fn start_payment_commit(
        &mut self,
        mut window: CostPaymentWindow,
        mut plan: PaymentPlan,
    ) {
        let (mana, purpose) =
            Self::payment_mana_requirement(&window, &plan).expect("validated mana assignments");
        let reserved = selected_objects(&plan);
        let (mana, x) = self.activate_mana_for_cost_with_options_reserving_for(
            window.player,
            mana,
            0,
            ManaPlanOptions::default(),
            &purpose,
            &reserved,
        );
        plan.mana_spent = self.pay_player_cost_for(window.player, mana, x, &purpose);
        self.order_payment_actions(&window, &mut plan);
        window.committing = Some(plan);
        self.continue_payment_commit(window);
    }

    pub(in crate::game) fn order_payment_actions(
        &self,
        window: &CostPaymentWindow,
        plan: &mut PaymentPlan,
    ) {
        plan.parts
            .retain(|part| !matches!(part.cost, CostDef::Mana(_) | CostDef::SnowMana(_)));
        // Pay scalar resource costs before battlefield exits can alter life
        // totals or remove their sources. The entire order was validated above.
        let mut objects = std::collections::VecDeque::new();
        let mut scalars = std::collections::VecDeque::new();
        for part in std::mem::take(&mut plan.parts) {
            if self.payment_part_quantity(window, &part).is_some() {
                objects.push_back(part);
            } else {
                scalars.push_back(part);
            }
        }
        scalars.extend(objects);
        plan.parts = scalars;
    }

    pub(in crate::game) fn continue_payment_commit(&mut self, mut window: CostPaymentWindow) {
        let mut later = std::mem::take(&mut self.pending_procedures);
        while let Some(mut part) = window
            .committing
            .as_mut()
            .expect("committed payment")
            .parts
            .pop_front()
        {
            if part.times == 0 {
                continue;
            }
            if part.times > 1 {
                let mut remaining = part.clone();
                let unit_count = part.objects.len() / usize::from(part.times);
                remaining.objects = part.objects.split_off(unit_count);
                remaining.times -= 1;
                window
                    .committing
                    .as_mut()
                    .expect("committed payment")
                    .parts
                    .push_front(remaining);
            }
            part.times = 1;
            match part.cost {
                CostDef::Sacrifice { .. } => {
                    self.capture_sacrifices(&part.objects);
                    self.move_permanents_to_graveyard_then(
                        &part.objects,
                        Some(BattlefieldExitCompletion::CompleteResolvingCost(Box::new(
                            window,
                        ))),
                    );
                    self.pending_procedures.append(&mut later);
                    return;
                }
                CostDef::Discard { .. }
                | CostDef::Exile {
                    from: ZoneKind::Hand | ZoneKind::Graveyard,
                    ..
                } => {
                    self.pay_object_card_cost(window.player, part.cost, &part.objects);
                }
                CostDef::Action(effect) => {
                    self.commit_action_program(&window, *effect, &part.objects);
                }
                _ => {
                    let scalar =
                        Self::payment_scalar(&window, &part).expect("validated scalar cost");
                    assert!(
                        self.pay_effect_payment(window.player, scalar),
                        "preflighted scalar cost remains executable"
                    );
                }
            }
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                self.pending_procedures
                    .push_back(crate::game::PendingProcedure::CommitPayment(Box::new(
                        window,
                    )));
                self.pending_procedures.append(&mut later);
                return;
            }
        }
        self.finish_cost_payment_window(window, true);
        self.pending_procedures.append(&mut later);
    }

    pub(in crate::game) fn finish_cost_payment_window(
        &mut self,
        mut window: CostPaymentWindow,
        paid: bool,
    ) {
        let plan = window
            .committing
            .take()
            .or_else(|| {
                self.evaluate_payment_window(&window)
                    .map(|evaluation| evaluation.plan)
            })
            .unwrap_or_default();
        for named in plan.named.into_iter().rev() {
            let source = window
                .object
                .source
                .and_then(|id| {
                    self.battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == id)
                        .or_else(|| match self.retired_objects.get(&id) {
                            Some(crate::game::RetiredObject::Permanent { permanent, .. }) => {
                                Some(permanent)
                            }
                            _ => None,
                        })
                })
                .map(|permanent| self.trigger_event_object(permanent));
            if let Some(source) = source {
                self.capture_battlefield_triggers(&CommittedTriggerEvent::MechanicPayment {
                    mechanic: named.mechanic,
                    paid,
                    object: source,
                    player: window.player,
                    repetitions: named.repetitions,
                    mana_spent: plan.mana_spent.iter().map(|mana| mana.color).collect(),
                });
            }
            if paid {
                self.capture_battlefield_triggers(&CommittedTriggerEvent::MechanicPerformed {
                    mechanics: vec![named.mechanic],
                    player: window.player,
                    object: None,
                });
            }
        }
        if paid {
            self.capture_optional_effect_taken(&window.object);
        }
        let EffectDef::PayOr(definition) = window.definition.effect else {
            unreachable!()
        };
        if let Some(effect) = if paid {
            definition.if_paid
        } else {
            definition.otherwise
        } {
            let mut context = window.context;
            context.paid_amount = paid.then_some(0);
            self.resolve_nested_effect_before_later(
                window.definition.with_effect(*effect),
                &window.object,
                context,
            );
        }
    }
}
