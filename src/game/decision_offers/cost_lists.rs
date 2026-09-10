// Complete resolving payments. Choices name a whole payment, so a later
// component cannot fail after an earlier one has already been spent.
#[derive(Clone)]
struct PaymentStep {
    payment: ResolvedEffectPayment,
    option: DecisionOption,
    options: Vec<DecisionOption>,
}

impl ResolvedEffectPayment {
    pub(super) fn all(payments: Vec<Self>) -> Self {
        fn append(payment: ResolvedEffectPayment, flat: &mut Vec<ResolvedEffectPayment>) {
            match payment {
                ResolvedEffectPayment::All(payments) => {
                    for payment in payments {
                        append(payment, flat);
                    }
                }
                // In a complete payment, name discards before committing any part.
                ResolvedEffectPayment::Discard(n) => {
                    flat.push(ResolvedEffectPayment::DiscardCards(n));
                }
                payment => flat.push(payment),
            }
        }
        if payments.len() == 1 {
            return payments.into_iter().next().expect("one payment");
        }
        let mut flat = Vec::new();
        let mut mana: Option<ManaCost> = None;
        let mut life = 0_u16;
        let mut energy = 0_u16;
        for payment in payments {
            append(payment, &mut flat);
        }
        flat.retain(|payment| match payment {
            Self::Mana(cost) => {
                mana = Some(mana.unwrap_or_default().plus(*cost));
                false
            }
            Self::Life(amount) => {
                life = life.saturating_add(*amount);
                false
            }
            Self::Energy(amount) => {
                energy = energy.saturating_add(*amount);
                false
            }
            _ => true,
        });
        flat.sort_by_key(|payment| {
            !matches!(payment, Self::CumulativeMana { .. } | Self::SnowMana { .. })
        });
        // All mana is raised before other payments. Retain Some({0}).
        if let Some(mana) = mana {
            flat.insert(0, Self::Mana(mana));
        }
        if life > 0 {
            flat.push(Self::Life(life));
        }
        if energy > 0 {
            flat.push(Self::Energy(energy));
        }
        Self::All(flat)
    }
}

impl Game {
    fn cost_list_payment_plans(
        &self,
        player: PlayerId,
        payments: &[ResolvedEffectPayment],
    ) -> Vec<Vec<PaymentStep>> {
        let mut plans = vec![Vec::<PaymentStep>::new()];
        for payment in payments {
            let can_pay = self.can_pay_effect_payment(player, payment.clone());
            let options = self.payment_options(player, payment.clone(), can_pay, "Decline");
            let mut next = Vec::new();
            for plan in &plans {
                for option in options.iter().filter(|option| option.id != 0) {
                    let ids = |option: &DecisionOption| -> Vec<GameObjectId> {
                        option
                            .card
                            .iter()
                            .map(|(id, _)| *id)
                            .chain(option.members.iter().map(|(id, _)| *id))
                            .collect()
                    };
                    let selected = ids(option);
                    if plan
                        .iter()
                        .any(|step| ids(&step.option).iter().any(|id| selected.contains(id)))
                    {
                        continue;
                    }
                    let mut candidate = plan.clone();
                    candidate.push(PaymentStep {
                        payment: payment.clone(),
                        option: option.clone(),
                        options: options.clone(),
                    });
                    next.push(candidate);
                }
            }
            plans = next;
        }
        plans
            .into_iter()
            .filter(|plan| {
                self.clone()
                    .commit_cost_list_payment(player, plan)
                    .is_some()
            })
            .collect()
    }

    fn commit_cost_list_payment(
        &mut self,
        player: PlayerId,
        plan: &[PaymentStep],
    ) -> Option<SettledEffectPayment> {
        let reserved = plan
            .iter()
            .flat_map(|step| {
                step.option
                    .card
                    .iter()
                    .map(|(id, _)| *id)
                    .chain(step.option.members.iter().map(|(id, _)| *id))
            })
            .collect::<Vec<_>>();
        let life = plan
            .iter()
            .filter_map(|step| match step.payment {
                ResolvedEffectPayment::Life(amount) => Some(amount),
                _ => None,
            })
            .fold(0_u16, u16::saturating_add);
        let mut result = SettledEffectPayment::without_mana(0);
        for step in plan {
            let mana = match step.payment {
                ResolvedEffectPayment::Mana(cost) => Some((cost, super::ManaPaymentPurpose::Other)),
                ResolvedEffectPayment::CumulativeMana { source, cost } => Some((
                    cost,
                    super::ManaPaymentPurpose::CumulativeUpkeep {
                        source,
                        snow: false,
                    },
                )),
                ResolvedEffectPayment::SnowMana { source, amount } => Some((
                    ManaCost::new(amount, 0),
                    super::ManaPaymentPurpose::CumulativeUpkeep { source, snow: true },
                )),
                _ => None,
            };
            let paid = if let Some((cost, purpose)) = mana {
                let life_available = self.life_available_after_payment(player, life)?;
                self.pay_resolving_mana_cost(player, cost, &reserved, life_available, &purpose)?
            } else {
                self.settle_payment_decision(
                    player,
                    step.payment.clone(),
                    &[step.option.id],
                    &step.options,
                )?
            };
            result.paid_amount = result.paid_amount.saturating_add(paid.paid_amount);
            result.mana_spent.extend(paid.mana_spent);
        }
        Some(result)
    }

    fn cost_list_payment_options(
        &self,
        player: PlayerId,
        payments: &[ResolvedEffectPayment],
    ) -> Vec<DecisionOption> {
        self.cost_list_payment_plans(player, payments)
            .into_iter()
            .enumerate()
            .map(|(index, plan)| {
                let mut members = Vec::new();
                for step in &plan {
                    members.extend(step.option.card);
                    members.extend(step.option.members.iter().copied());
                }
                DecisionOption {
                    id: u32::try_from(index + 1).expect("payment options fit decision ids"),
                    label: if plan.is_empty() {
                        "Pay the cost".into()
                    } else {
                        plan.iter()
                            .map(|step| step.option.label.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    },
                    card: None,
                    members,
                    ability_text: None,
                    zone: DecisionZone::None,
                }
            })
            .collect()
    }

    fn settle_cost_list_payment(
        &mut self,
        player: PlayerId,
        payments: &[ResolvedEffectPayment],
        chosen: u32,
    ) -> Option<SettledEffectPayment> {
        let index = usize::try_from(chosen.checked_sub(1)?).ok()?;
        let plan = self
            .cost_list_payment_plans(player, payments)
            .into_iter()
            .nth(index)?;
        let mut committed = self.clone();
        let result = committed.commit_cost_list_payment(player, &plan)?;
        *self = committed;
        Some(result)
    }
}
