// Object costs committed while casting a spell.
//
// Included textually into `casting.rs`, so the imports here are the parent
// module's.

impl Game {
    pub(super) fn continue_spell_cast(
        &mut self,
        stack_object: StackObject,
        targets: Vec<Target>,
        remaining_sacrifices: Vec<crate::game::cost_payment::CostPaymentStep>,
    ) {
        let Some((stack_object, targets)) =
            self.pay_spell_object_costs(stack_object, targets, remaining_sacrifices)
        else {
            return;
        };
        self.complete_spell_cast(stack_object, targets);
    }

    // Keep the ordered payment dispatch together, including named-action
    // completion markers that must follow the actions they label.
    #[allow(clippy::too_many_lines)]
    fn pay_spell_object_costs(
        &mut self,
        mut stack_object: StackObject,
        targets: Vec<Target>,
        mut remaining_sacrifices: Vec<crate::game::cost_payment::CostPaymentStep>,
    ) -> Option<(StackObject, Vec<Target>)> {
        // The action carries object choices in the same order as their
        // additional-cost clauses. Process one at a time so a mandatory
        // return/exile cost and an optional sacrifice cost retain distinct
        // semantic actions even when both were selected for the same cast.
        while let Some(step) = remaining_sacrifices.first().copied() {
            remaining_sacrifices.remove(0);
            let (spent, cost) = match step {
                crate::game::cost_payment::CostPaymentStep::Object(spent, cost) => (spent, cost),
                crate::game::cost_payment::CostPaymentStep::EndAction => continue,
                crate::game::cost_payment::CostPaymentStep::CompleteMechanic(mechanic) => {
                    self.capture_battlefield_triggers(&CommittedTriggerEvent::MechanicPerformed {
                        mechanics: vec![mechanic],
                        player: stack_object.controller,
                        object: None,
                    });
                    continue;
                }
            };
            if !stack_object.chosen_permanents.contains(&spent) {
                stack_object.chosen_permanents.push(spent);
            }
            match cost {
                CostDef::Sacrifice { .. } => {
                    let mut batch = vec![spent];
                    while let Some(crate::game::cost_payment::CostPaymentStep::Object(next, next_cost)) = remaining_sacrifices.first().copied() {
                        if next_cost != cost { break; }
                        remaining_sacrifices.remove(0);
                        batch.push(next);
                        stack_object.chosen_permanents.push(next);
                    }
                    self.capture_sacrifices(&batch);
                    self.move_permanents_to_graveyard_then(
                        &batch,
                        Some(BattlefieldExitCompletion::CompleteSpellCast {
                            object: Box::new(stack_object),
                            targets,
                            remaining_sacrifices,
                        }),
                    );
                    return None;
                }
                CostDef::ReturnToHand { .. } => {
                    self.move_target_to_zone(
                        Target::Permanent(spent),
                        ZoneKind::Hand,
                        ZoneMoveCause::Effect {
                            controller: stack_object.controller,
                        },
                        None,
                        ZonePlacement::Top,
                    );
                    continue;
                }
                CostDef::Tap { .. } => {
                    if !self
                        .battlefield
                        .iter()
                        .any(|permanent| permanent.card.id == spent && !permanent.tapped)
                    {
                        return None;
                    }
                    self.tap_permanent(spent)?;
                    continue;
                }
                CostDef::Exile {
                    from: ZoneKind::Battlefield,
                    ..
                } => {
                    self.move_target_to_zone(
                        Target::Permanent(spent),
                        ZoneKind::Exile,
                        ZoneMoveCause::Effect {
                            controller: stack_object.controller,
                        },
                        None,
                        ZonePlacement::Top,
                    );
                    continue;
                }
                CostDef::Discard { .. }
                | CostDef::Exile { .. } => {}
                CostDef::ManaTimes { .. }
                | CostDef::Mana(_)
                | CostDef::PayLife(_)
                | CostDef::PayLifeTimes(_)
                | CostDef::All(_)
                | CostDef::Choice(_) => {
                    unreachable!("scalar and composite costs do not name individual objects")
                }
                _ => unreachable!("unsupported spell costs are not advertised"),
            }

            let exiled_payment_cards = self.pay_nonbattlefield_spell_object_cost(
                stack_object.controller,
                spent,
                cost,
                &mut remaining_sacrifices,
                &mut stack_object.chosen_permanents,
            );
            stack_object
                .cast
                .as_mut()
                .expect("a cast spell retains its context through payment")
                .exiled_payment_cards
                .extend(exiled_payment_cards);
        }

        Some((stack_object, targets))
    }

    fn pay_nonbattlefield_spell_object_cost(
        &mut self,
        controller: PlayerId,
        spent: GameObjectId,
        cost: CostDef,
        remaining_payments: &mut Vec<crate::game::cost_payment::CostPaymentStep>,
        paid_objects: &mut Vec<GameObjectId>,
    ) -> Vec<GameObjectId> {
        let mut batch = vec![spent];
        // EndAction separates identical adjacent costs too: one semantic
        // action must not absorb the next action's objects or completion tag.
        while let Some(crate::game::cost_payment::CostPaymentStep::Object(next, next_cost)) =
            remaining_payments.first().copied()
        {
            if next_cost != cost { break; }
            remaining_payments.remove(0);
            batch.push(next);
            paid_objects.push(next);
        }
        self.pay_object_card_cost(controller, cost, &batch)
    }
}
