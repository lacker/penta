// Object costs committed while casting a spell.
//
// Included textually into `casting.rs`, so the imports here are the parent
// module's.

impl Game {
    pub(super) fn continue_spell_cast(
        &mut self,
        stack_object: StackObject,
        targets: Vec<Target>,
        remaining_sacrifices: Vec<(GameObjectId, SpendModeDef)>,
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
        mut remaining_sacrifices: Vec<(GameObjectId, SpendModeDef)>,
    ) -> Option<(StackObject, Vec<Target>)> {
        // The action carries object choices in the same order as their
        // additional-cost clauses. Process one at a time so a mandatory
        // return/exile cost and an optional sacrifice cost retain distinct
        // spend operations even when both were selected for the same cast.
        while let Some((spent, spend)) = remaining_sacrifices.first().copied() {
            remaining_sacrifices.remove(0);
            if !stack_object.chosen_permanents.contains(&spent) {
                stack_object.chosen_permanents.push(spent);
            }
            if self
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == spent)
            {
                match spend {
                    SpendModeDef::ByZone => {
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
                    SpendModeDef::Exile | SpendModeDef::ReturnToHand => {
                        let destination =
                            Self::additional_cost_destination(spend, ZoneKind::Battlefield);
                        self.move_target_to_zone(
                            Target::Permanent(spent),
                            destination,
                            ZoneMoveCause::Effect {
                                controller: stack_object.controller,
                            },
                            None,
                            ZonePlacement::Top,
                        );
                    }
                }
                continue;
            }

            let Some((from, card)) = self
                .card_in_nonbattlefield_zone(spent)
                .map(|(zone, card)| (zone, card.clone()))
            else {
                continue;
            };
            let destination = Self::additional_cost_destination(spend, from);
            let owner = card.owner;
            // "One or more cards" exiled from a graveyard by one payment is
            // one move and therefore one trigger event. Keep that upstream
            // batching while retaining each object's own spend provenance:
            // a following return-to-hand object is not part of this batch.
            if from == ZoneKind::Graveyard && destination == ZoneKind::Exile {
                self.exile_graveyard_payment_batch(
                    owner,
                    spent,
                    &mut remaining_sacrifices,
                    &mut stack_object.chosen_permanents,
                );
                continue;
            }
            let moved = self.move_card_from_nonbattlefield_zone(
                spent,
                from,
                destination,
                ZoneMoveCause::Effect {
                    controller: stack_object.controller,
                },
                None,
            );
            if from == ZoneKind::Hand
                && destination == ZoneKind::Graveyard
                && let Some((card, actual_destination)) = moved
                && actual_destination == ZoneKind::Graveyard
            {
                self.events.push(GameEvent::CardsDiscarded {
                    player: owner,
                    cards: vec![(card.id, card.definition)],
                });
                let discarded = self.printed_trigger_event_object(
                    card.id,
                    card.definition,
                    owner,
                    &CharacteristicContext::Graveyard,
                );
                self.capture_battlefield_triggers(&CommittedTriggerEvent::Discarded {
                    player: owner,
                    card: discarded,
                });
                self.capture_battlefield_triggers(&CommittedTriggerEvent::CardsDiscarded {
                    player: owner,
                });
            }
        }

        Some((stack_object, targets))
    }

    const fn additional_cost_destination(spend: SpendModeDef, from: ZoneKind) -> ZoneKind {
        match spend {
            SpendModeDef::ReturnToHand => ZoneKind::Hand,
            SpendModeDef::ByZone if matches!(from, ZoneKind::Hand | ZoneKind::Battlefield) => {
                ZoneKind::Graveyard
            }
            SpendModeDef::Exile | SpendModeDef::ByZone => ZoneKind::Exile,
        }
    }

    fn exile_graveyard_payment_batch(
        &mut self,
        owner: PlayerId,
        spent: GameObjectId,
        remaining_sacrifices: &mut Vec<(GameObjectId, SpendModeDef)>,
        paid_objects: &mut Vec<GameObjectId>,
    ) {
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
                    .and_then(|(candidate, candidate_spend)| {
                        let (candidate_zone, candidate_card) =
                            self.card_in_nonbattlefield_zone(candidate)?;
                        let candidate_destination =
                            Self::additional_cost_destination(candidate_spend, candidate_zone);
                        (candidate_zone == ZoneKind::Graveyard
                            && candidate_destination == ZoneKind::Exile
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
    }
}
