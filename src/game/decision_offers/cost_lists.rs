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
        if payments.len() == 1 {
            return payments.into_iter().next().expect("one payment");
        }
        let mut flat = Vec::new();
        let mut mana: Option<ManaCost> = None;
        let mut labeled_mana: Vec<(GameObjectId, crate::card::AbilityLabel, ManaCost)> = Vec::new();
        let mut snow_mana: Vec<(GameObjectId, Option<crate::card::AbilityLabel>, u16)> = Vec::new();
        let mut life = 0_u16;
        let mut energy = 0_u16;
        for payment in payments {
            append_payment(payment, &mut flat);
        }
        flat.retain(|payment| match payment {
            Self::Mana(cost) => {
                mana = Some(mana.unwrap_or_default().plus(*cost));
                false
            }
            Self::LabeledMana {
                source,
                label,
                cost,
            } => {
                if let Some((_, _, total)) = labeled_mana
                    .iter_mut()
                    .find(|(id, purpose, _)| id == source && purpose == label)
                {
                    *total = total.plus(*cost);
                } else {
                    labeled_mana.push((*source, *label, *cost));
                }
                false
            }
            Self::SnowMana {
                source,
                label,
                amount,
            } => {
                if let Some((_, _, total)) = snow_mana
                    .iter_mut()
                    .find(|(id, purpose, _)| id == source && purpose == label)
                {
                    *total = total.saturating_add(*amount);
                } else {
                    snow_mana.push((*source, *label, *amount));
                }
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
        flat.extend(
            labeled_mana
                .into_iter()
                .map(|(source, label, cost)| Self::LabeledMana {
                    source,
                    label,
                    cost,
                }),
        );
        flat.extend(
            snow_mana
                .into_iter()
                .map(|(source, label, amount)| Self::SnowMana {
                    source,
                    label,
                    amount,
                }),
        );
        flat.sort_by_key(|payment| {
            !matches!(payment, Self::LabeledMana { .. } | Self::SnowMana { .. })
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
        let mut distinct = Vec::new();
        for payments in expanded_payment_lists(payments) {
            let normalized = ResolvedEffectPayment::all(payments);
            let payments = match normalized {
                ResolvedEffectPayment::All(payments) => payments,
                payment => vec![payment],
            };
            if !distinct.contains(&payments) {
                distinct.push(payments);
            }
        }
        distinct
            .into_iter()
            .flat_map(|payments| self.flat_cost_list_payment_plans(player, &payments))
            .collect()
    }

    fn flat_cost_list_payment_plans(
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
                ResolvedEffectPayment::LabeledMana {
                    source,
                    label,
                    cost,
                } => Some((
                    cost,
                    super::ManaPaymentPurpose::Payment {
                        label: Some(label),
                        source,
                        snow: false,
                    },
                )),
                ResolvedEffectPayment::SnowMana {
                    source,
                    label,
                    amount,
                } => Some((
                    ManaCost::new(amount, 0),
                    super::ManaPaymentPurpose::Payment {
                        label,
                        source,
                        snow: true,
                    },
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
            result.object_bindings.extend(paid.object_bindings);
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
                            .map(|step| match step.payment {
                                ResolvedEffectPayment::Mana(cost)
                                | ResolvedEffectPayment::LabeledMana { cost, .. } => {
                                    format!("Pay {cost}")
                                }
                                _ => step.option.label.clone(),
                            })
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
        offered: Option<&[DecisionOption]>,
    ) -> Option<SettledEffectPayment> {
        if let Some(offered) = offered {
            let current = self.cost_list_payment_options(player, payments);
            if current.iter().find(|option| option.id == chosen)
                != offered.iter().find(|option| option.id == chosen)
            {
                return None;
            }
        }
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

// Choice is an alternative obligation, not an optional component of a list.
fn expanded_payment_lists(payments: &[ResolvedEffectPayment]) -> Vec<Vec<ResolvedEffectPayment>> {
    let mut lists = vec![Vec::new()];
    for payment in payments {
        let alternatives = match payment {
            ResolvedEffectPayment::All(payments) => expanded_payment_lists(payments),
            ResolvedEffectPayment::Choice(choices) => choices
                .iter()
                .flat_map(|choice| expanded_payment_lists(std::slice::from_ref(choice)))
                .collect(),
            payment => vec![vec![payment.clone()]],
        };
        let mut next = Vec::new();
        for list in &lists {
            for alternative in &alternatives {
                let mut combined = list.clone();
                combined.extend(alternative.iter().cloned());
                if !next.contains(&combined) {
                    next.push(combined);
                }
            }
        }
        lists = next;
    }
    lists
}

fn append_payment(payment: ResolvedEffectPayment, flat: &mut Vec<ResolvedEffectPayment>) {
    match payment {
        ResolvedEffectPayment::All(payments) => {
            for payment in payments {
                append_payment(payment, flat);
            }
        }
        // In a complete payment, name discards before committing any part.
        ResolvedEffectPayment::Discard(n) => {
            flat.push(ResolvedEffectPayment::DiscardCards(n));
        }
        payment => flat.push(payment),
    }
}
