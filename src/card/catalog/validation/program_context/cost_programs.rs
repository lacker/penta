// A parameter is valid only beneath its lexical supplier. Keep this check
// separate from effect traversal so copied/granted programs get the same rules.
fn validate_cost_program(
    effect: EffectDef,
    parameter: Option<&'static [CostDef]>,
) -> Result<(), &'static str> {
    if let EffectDef::WithCosts { costs, effect } = effect {
        if costs.iter().any(|cost| contains_cost_parameter(*cost)) {
            return Err("WithCosts cannot bind an unresolved parameter");
        }
        return validate_cost_program(*effect, Some(costs));
    }
    if let EffectDef::PayOr(payment) = effect {
        if payment
            .payment
            .costs
            .iter()
            .any(|cost| matches!(cost, CostDef::Named { .. }))
        {
            match payment.payment.costs {
                [cost @ CostDef::Named { .. }]
                    if cost.named_choices().is_some() && payment.label.is_none() => {}
                _ => {
                    return Err(
                        "named object costs currently require their own unlabeled payment window",
                    );
                }
            }
        } else {
            for cost in payment.payment.costs {
                validate_program_cost(*cost, parameter, payment.label.is_some(), false)?;
            }
        }
    }
    for child in crate::card::child_effects(effect) {
        validate_cost_program(child, parameter)?;
    }
    Ok(())
}

fn validate_program_cost(
    cost: CostDef,
    parameter: Option<&'static [CostDef]>,
    repeated_or_labeled: bool,
    scalar_only: bool,
) -> Result<(), &'static str> {
    let (costs, repeated_or_labeled, scalar_only) = match cost {
        CostDef::Named { .. } => return Err("nested named object payment is not supported"),
        CostDef::Parameter => (
            parameter.ok_or("unbound cost parameter")?,
            repeated_or_labeled,
            scalar_only,
        ),
        CostDef::All(costs) => (costs, repeated_or_labeled, scalar_only),
        CostDef::Repeated { costs, .. } => (costs, true, scalar_only),
        CostDef::Choice(costs) => (costs, repeated_or_labeled, true),
        CostDef::Perform(program) if !scalar_only => {
            return if program.payment_program_supported() {
                Ok(())
            } else {
                Err("unsupported action payment program")
            };
        }
        _ if scalar_only && !scalar_batch_cost(cost) => {
            return Err("unsupported non-scalar cost choice");
        }
        _ if repeated_or_labeled && !repeatable_payment_cost(cost) => {
            return Err("unsupported repeated or labeled payment cost");
        }
        _ => return Ok(()),
    };
    for cost in costs {
        validate_program_cost(*cost, parameter, repeated_or_labeled, scalar_only)?;
    }
    Ok(())
}

fn contains_cost_parameter(cost: CostDef) -> bool {
    match cost {
        CostDef::Parameter => true,
        CostDef::Named { cost, .. } => contains_cost_parameter(*cost),
        CostDef::All(costs) | CostDef::Choice(costs) | CostDef::Repeated { costs, .. } => {
            costs.iter().any(|cost| contains_cost_parameter(*cost))
        }
        _ => false,
    }
}

fn repeatable_payment_cost(cost: CostDef) -> bool {
    match cost {
        CostDef::All(costs) => costs.iter().all(|cost| repeatable_payment_cost(*cost)),
        CostDef::Choice(_) => scalar_batch_cost(cost),
        CostDef::Mana(cost) => !cost.variable_x,
        CostDef::SnowMana(_)
        | CostDef::PayLife(_)
        | CostDef::Energy(_)
        | CostDef::MillCards(_)
        | CostDef::DrawCards(_)
        | CostDef::DiscardCards(_)
        | CostDef::ExileTopCards(_)
        | CostDef::FlipCoins(_)
        | CostDef::PutCountersOnSource { .. }
        | CostDef::SacrificePermanents {
            controller: PlayerRelation::You,
            ..
        }
        | CostDef::GainLife {
            player: PlayerRelation::Opponent,
            ..
        }
        | CostDef::CreateTokens {
            player: PlayerRelation::Opponent,
            ..
        } => true,
        CostDef::AddMana(effect) => {
            matches!(
                effect.mana,
                crate::card::ManaSelectionDef::One(crate::card::ManaTypeDef::Fixed(_))
            ) && effect.also.is_none()
                && effect.variable_amount.is_none()
                && effect.amount_override.is_none()
                && effect.damage_to_controller == 0
                && effect.sacrifice_source_when_out_of.is_none()
                && effect.restrictions.is_empty()
                && effect.spend_effects.is_empty()
        }
        _ => false,
    }
}

fn scalar_batch_cost(cost: CostDef) -> bool {
    match cost {
        CostDef::Mana(cost) => !cost.variable_x,
        CostDef::PayLife(_) => true,
        CostDef::All(costs) | CostDef::Choice(costs) => {
            costs.iter().all(|cost| scalar_batch_cost(*cost))
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{AbilityLabel, PayOrDef, actions};

    const SCALAR_CHOICES: &[CostDef] = &[
        CostDef::PayLife(1),
        CostDef::Mana(crate::ManaCost::new(1, 0)),
    ];

    static PAY_PARAMETER: EffectDef = EffectDef::PayOr(
        PayOrDef::optional(
            &[CostDef::repeated(
                &[CostDef::Parameter],
                &ValueDef::Constant(2),
            )],
            &EffectDef::None,
        )
        .labeled(AbilityLabel::from_name("test:purpose"))
        .with_visibility(crate::card::ChoiceVisibilityDef::Public),
    );

    #[test]
    fn named_costs_reject_unplannable_composition() {
        const ID: crate::card::MechanicId = crate::card::MechanicId::from_name("test:selection");
        const NAMED: CostDef = CostDef::named(
            ID,
            &CostDef::Sacrifice {
                object: crate::card::ObjectPredicateDef::Any,
                quantity: crate::card::CostQuantityDef::Fixed(2),
            },
        );
        assert!(
            validate_cost_program(
                EffectDef::PayOr(PayOrDef::optional(&[NAMED], &EffectDef::None)),
                None
            )
            .is_ok()
        );
        for costs in [
            &[CostDef::All(&[NAMED])][..],
            &[NAMED, CostDef::PayLife(1)][..],
        ] {
            let costs = Box::leak(costs.to_vec().into_boxed_slice());
            assert!(
                validate_cost_program(
                    EffectDef::PayOr(PayOrDef::optional(costs, &EffectDef::None)),
                    None
                )
                .is_err()
            );
        }
        assert!(
            validate_cost_program(
                EffectDef::PayOr(PayOrDef::optional(&[NAMED], &EffectDef::None).labeled(ID)),
                None
            )
            .is_err()
        );
        assert!(contains_cost_parameter(CostDef::named(
            ID,
            &CostDef::Parameter
        )));
    }

    #[test]
    fn game_action_programs_reject_unplannable_payments() {
        use crate::card::{EffectRecipientDef, GameActionDef};
        static DISCARD: GameActionDef = actions::choose_discard(3);
        static CHOICE: [CostDef; 1] = [DISCARD.as_cost()];
        const OPPONENT_CHOOSES: CostDef = actions::choose_discard(3)
            .with_chooser(crate::card::PlayerRefDef::Opponent)
            .as_cost();
        assert!(validate_program_cost(DISCARD.as_cost(), None, true, false).is_ok());
        assert!(validate_program_cost(CostDef::Choice(&CHOICE), None, false, false).is_err());
        assert!(
            validate_program_cost(
                GameActionDef::SacrificeYours {
                    object: EffectRecipientDef::Source,
                }
                .as_cost(),
                None,
                false,
                false
            )
            .is_err(),
            "an arbitrary action body has no complete payment plan"
        );
        let GameActionDef::Choose(mut choice) = DISCARD else {
            unreachable!()
        };
        choice.then = &GameActionDef::DiscardCards {
            object: EffectRecipientDef::Source,
        };
        let invalid = Box::leak(Box::new(GameActionDef::Choose(choice)));
        assert!(
            validate_program_cost(invalid.as_cost(), None, false, false).is_err(),
            "payment must execute the exact objects it selects"
        );
        assert!(
            validate_program_cost(OPPONENT_CHOOSES, None, false, false,).is_err(),
            "cost conversion preserves the payment planner's chooser boundary"
        );
    }

    #[test]
    fn composed_mechanic_programs_reject_unbound_and_unpayable_programs() {
        assert!(validate_cost_program(PAY_PARAMETER, None).is_err());
        assert!(
            validate_cost_program(
                EffectDef::WithCosts {
                    costs: &[CostDef::Choice(&[
                        CostDef::PayLife(1),
                        CostDef::DrawCards(1)
                    ])],
                    effect: &PAY_PARAMETER,
                },
                None
            )
            .is_err()
        );
        assert!(
            validate_cost_program(
                EffectDef::WithCosts {
                    costs: &[CostDef::Choice(SCALAR_CHOICES)],
                    effect: &PAY_PARAMETER,
                },
                None
            )
            .is_ok()
        );
        assert!(
            validate_cost_program(
                EffectDef::WithCosts {
                    costs: &[CostDef::PayLife(1)],
                    effect: &PAY_PARAMETER
                },
                None
            )
            .is_ok()
        );
        assert!(
            validate_cost_program(
                EffectDef::WithCosts {
                    costs: &[CostDef::Special("unsupported")],
                    effect: &PAY_PARAMETER
                },
                None
            )
            .is_err()
        );
    }
}
