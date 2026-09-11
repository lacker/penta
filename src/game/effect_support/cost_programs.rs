impl Game {
    pub(in crate::game) fn resolve_payment_offer(
        &self,
        definition: crate::card::PayOrDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> (
        super::ResolvedEffectPayment,
        Option<super::PaymentProvenance>,
    ) {
        let payment = super::ResolvedEffectPayment::all(
            definition
                .payment
                .costs
                .iter()
                .map(|cost| {
                    self.resolved_program_cost(
                        *cost,
                        object,
                        context,
                        scoped,
                        None,
                        definition.label,
                    )
                })
                .collect(),
        );
        (
            payment,
            definition
                .label
                .map(|label| super::PaymentProvenance { label }),
        )
    }

    fn payment_repetition_count(
        &self,
        count: crate::card::ValueDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> u16 {
        u16::try_from(self.effect_value(count, object, context, scoped).max(0)).unwrap_or(u16::MAX)
    }

    /// Interpret composition before handing concrete costs to the shared
    /// planner. A repeated node scales only its own children, never siblings.
    fn resolved_program_cost(
        &self,
        cost: crate::card::CostDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
        times: Option<u16>,
        label: Option<crate::card::AbilityLabel>,
    ) -> super::ResolvedEffectPayment {
        use super::ResolvedEffectPayment as Resolved;
        use crate::card::CostDef as Cost;
        if times == Some(0) {
            return Resolved::all(Vec::new());
        }
        let (costs, times) = match cost {
            Cost::Perform(program) => {
                return self.resolve_action_payment(
                    *program,
                    object,
                    context,
                    scoped,
                    times.unwrap_or(1),
                );
            }
            Cost::All(costs) => (costs, times),
            Cost::Parameter => (
                scoped
                    .cost_parameter
                    .expect("validated lexical cost parameter"),
                times,
            ),
            Cost::Repeated { costs, count } => (
                costs,
                Some(times.unwrap_or(1).saturating_mul(
                    self.payment_repetition_count(*count, object, context, scoped),
                )),
            ),
            Cost::Choice(_) => {
                return Resolved::Choice(
                    self.resolved_scalar_cost_plans(
                        cost,
                        times.unwrap_or(1),
                        object,
                        context,
                        scoped,
                    )
                    .expect("validated scalar alternatives")
                    .into_iter()
                    .map(|plan| {
                        let mut payments = Vec::new();
                        if plan.includes_mana_payment {
                            payments.push(Self::resolved_repeated_payment(
                                Cost::Mana(plan.mana),
                                object.source.unwrap_or(object.id),
                                1,
                                label,
                            ));
                        }
                        if plan.life > 0 {
                            payments.push(Resolved::Life(plan.life));
                        }
                        Resolved::all(payments)
                    })
                    .collect(),
                );
            }
            _ if times.is_some() || label.is_some() => {
                return Self::resolved_repeated_payment(
                    cost,
                    object.source.unwrap_or(object.id),
                    times.unwrap_or(1),
                    label,
                );
            }
            _ => return self.resolved_effect_payment(cost, object, context, scoped),
        };
        Resolved::all(
            costs
                .iter()
                .map(|cost| {
                    self.resolved_program_cost(*cost, object, context, scoped, times, label)
                })
                .collect(),
        )
    }

    fn resolved_scalar_cost_plans(
        &self,
        cost: crate::card::CostDef,
        times: u16,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<Vec<super::cost_planning::ScalarCostPlan>> {
        use crate::card::CostDef as Cost;
        super::cost_planning::scalar_cost_plans_with(cost, times, &|cost| {
            let (costs, times) = match cost {
                Cost::Parameter => (scoped.cost_parameter?, 1),
                Cost::Repeated { costs, count } => (
                    costs,
                    self.payment_repetition_count(*count, object, context, scoped),
                ),
                _ => return None,
            };
            self.resolved_scalar_cost_plans(Cost::All(costs), times, object, context, scoped)
        })
    }
}
