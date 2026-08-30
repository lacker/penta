// Object costs committed while casting a spell.
//
// Included textually into `casting.rs`, so the imports here are the parent
// module's.

impl Game {
    pub(super) fn continue_spell_cast(
        &mut self,
        stack_object: StackObject,
        targets: Vec<Target>,
        remaining_sacrifices: Vec<(GameObjectId, SpellAdditionalCostDef)>,
    ) {
        let Some((stack_object, targets)) =
            self.pay_spell_object_costs(stack_object, targets, remaining_sacrifices)
        else {
            return;
        };
        self.complete_spell_cast(stack_object, targets);
    }

    fn pay_spell_object_costs(
        &mut self,
        mut stack_object: StackObject,
        targets: Vec<Target>,
        mut remaining_sacrifices: Vec<(GameObjectId, SpellAdditionalCostDef)>,
    ) -> Option<(StackObject, Vec<Target>)> {
        // The action carries object choices in the same order as their
        // additional-cost clauses. Process one at a time so a mandatory
        // return/exile cost and an optional sacrifice cost retain distinct
        // semantic actions even when both were selected for the same cast.
        while let Some((spent, cost)) = remaining_sacrifices.first().copied() {
            remaining_sacrifices.remove(0);
            if !stack_object.chosen_permanents.contains(&spent) {
                stack_object.chosen_permanents.push(spent);
            }
            match cost {
                SpellAdditionalCostDef::Sacrifice { .. } => {
                    self.capture_sacrifices(&[spent]);
                    self.move_permanents_to_graveyard_then(
                        &[spent],
                        Some(BattlefieldExitCompletion::CompleteSpellCast {
                            object: Box::new(stack_object),
                            targets,
                            remaining_sacrifices,
                        }),
                    );
                    return None;
                }
                SpellAdditionalCostDef::ReturnToHand { .. } => {
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
                SpellAdditionalCostDef::Exile {
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
                SpellAdditionalCostDef::Discard { .. }
                | SpellAdditionalCostDef::Exile { .. } => {}
                SpellAdditionalCostDef::PayMana(_)
                | SpellAdditionalCostDef::PayLife(_)
                | SpellAdditionalCostDef::Forage
                | SpellAdditionalCostDef::All(_)
                | SpellAdditionalCostDef::Choice(_) => {
                    unreachable!("scalar and composite costs do not name individual objects")
                }
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
        cost: SpellAdditionalCostDef,
        remaining_payments: &mut Vec<(GameObjectId, SpellAdditionalCostDef)>,
        paid_objects: &mut Vec<GameObjectId>,
    ) -> Vec<GameObjectId> {
        let Some((from, card)) = self
            .card_in_nonbattlefield_zone(spent)
            .map(|(zone, card)| (zone, card.clone()))
        else {
            return Vec::new();
        };
        let destination = match cost {
            SpellAdditionalCostDef::Discard { .. } => ZoneKind::Graveyard,
            SpellAdditionalCostDef::Exile { .. } => ZoneKind::Exile,
            _ => unreachable!("battlefield costs were handled above"),
        };
        let owner = card.owner;
        // "One or more cards" exiled from a graveyard by one payment is one
        // move and therefore one trigger event. A following payment action is
        // not part of this batch.
        if matches!(
            cost,
            SpellAdditionalCostDef::Exile {
                from: ZoneKind::Graveyard,
                ..
            }
        ) {
            return self.exile_graveyard_payment_batch(
                owner,
                spent,
                remaining_payments,
                paid_objects,
            );
        }
        let discarded = if matches!(cost, SpellAdditionalCostDef::Discard { .. }) {
            self.printed_trigger_event_object(
                card.id,
                card.definition,
                owner,
                &CharacteristicContext::Hand,
            )
        } else {
            None
        };
        let moved = self.move_card_from_nonbattlefield_zone(
            spent,
            from,
            destination,
            ZoneMoveCause::Effect { controller },
            None,
        );
        if let (Some(discarded), Some((card, _actual_destination))) = (discarded, moved.as_ref()) {
            self.events.push(GameEvent::CardsDiscarded {
                player: owner,
                cards: vec![(card.id, card.definition)],
            });
            self.capture_battlefield_triggers(&CommittedTriggerEvent::Discarded {
                player: owner,
                card: Some(discarded),
            });
            self.capture_battlefield_triggers(&CommittedTriggerEvent::CardsDiscarded {
                player: owner,
            });
        }
        moved
            .filter(|(_, actual_destination)| *actual_destination == ZoneKind::Exile)
            .map_or_else(Vec::new, |(card, _)| vec![card.id])
    }

    fn exile_graveyard_payment_batch(
        &mut self,
        owner: PlayerId,
        spent: GameObjectId,
        remaining_sacrifices: &mut Vec<(GameObjectId, SpellAdditionalCostDef)>,
        paid_objects: &mut Vec<GameObjectId>,
    ) -> Vec<GameObjectId> {
        let mut exiled = Vec::new();
        let mut next = Some(spent);
        while let Some(id) = next.take() {
            if !paid_objects.contains(&id) {
                paid_objects.push(id);
            }
            if let Some(card) = remove_card(&mut self.players[owner.index()].graveyard, id) {
                let (card, _zone_change) = self.zone_change_card(card);
                self.players[owner.index()].exile.push(card.clone());
                exiled.push(card);
            }
            next =
                remaining_sacrifices
                    .first()
                    .copied()
                    .and_then(|(candidate, candidate_cost)| {
                        let (candidate_zone, candidate_card) =
                            self.card_in_nonbattlefield_zone(candidate)?;
                        (candidate_zone == ZoneKind::Graveyard
                            && matches!(
                                candidate_cost,
                                SpellAdditionalCostDef::Exile {
                                    from: ZoneKind::Graveyard,
                                    ..
                                }
                            )
                            && candidate_card.owner == owner)
                            .then_some(candidate)
                    });
            if next.is_some() {
                remaining_sacrifices.remove(0);
            }
        }
        if !exiled.is_empty() {
            self.capture_cards_exiled(&exiled, ZoneKind::Graveyard);
            self.note_card_left_graveyard(owner);
        }
        exiled.into_iter().map(|card| card.id).collect()
    }
}
