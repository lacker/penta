// Zone move publication retains the simultaneous instruction that caused it.

impl Game {
    /// Moves a card into its owner's graveyard from `from`, honouring a
    /// replacement the card itself carries about that move. The caller has
    /// already taken the card out of `from`, so this path owns the remaining
    /// identity change, replacement, destination, and arrival publication.
    /// A countered spell and a milled card both reach the graveyard through
    /// this path instead of rebuilding those pieces independently.
    ///
    /// Audit: unsupported -- the move is read as a rules move, so a clause that
    /// asks *whose* effect moved the card ([`ZoneMoveCauseDef::EffectControlledBy`])
    /// is not answered here. Nothing in the catalog writes one of those about
    /// a graveyard move it makes from the stack or the library.
    pub(super) fn put_card_into_graveyard_replacing(
        &mut self,
        owner: PlayerId,
        card: CardInstance,
        from: ZoneKind,
    ) -> Option<CardInstance> {
        let mut events = Vec::new();
        let result =
            self.put_card_into_graveyard_replacing_collecting(owner, card, from, &mut events);
        self.capture_zone_move_events(&events);
        result
    }

    pub(super) fn put_card_into_graveyard_replacing_collecting(
        &mut self,
        owner: PlayerId,
        card: CardInstance,
        from: ZoneKind,
        events: &mut Vec<CommittedTriggerEvent>,
    ) -> Option<CardInstance> {
        let before_move = card.clone();
        let program = self.zone_move_replacement_program(
            &card,
            from,
            ZoneKind::Graveyard,
            ZoneMoveCause::Rules,
        );
        let (card, _zone_change) = self.zone_change_card(card);
        let destination = program
            .and_then(Self::replacement_move_destination)
            .unwrap_or(ZoneKind::Graveyard);
        match destination {
            ZoneKind::Library => {
                self.players[owner.index()].library.push(card);
                if program.is_some_and(Self::replacement_shuffles_library) {
                    self.rng.shuffle(&mut self.players[owner.index()].library);
                }
                None
            }
            ZoneKind::Hand => {
                self.players[owner.index()].hand.push(card);
                None
            }
            ZoneKind::Exile => {
                self.players[owner.index()].exile.push(card.clone());
                events.extend(self.cards_exiled_events(std::iter::once((&card, from))));
                None
            }
            // A replacement that names the graveyard, the battlefield, or a
            // zone this arrival cannot build is left to the ordinary path.
            ZoneKind::Graveyard | ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {
                self.put_card_into_graveyard(owner, card.clone());
                if self.players[owner.index()]
                    .graveyard
                    .iter()
                    .any(|candidate| candidate.id == card.id)
                {
                    if let Some(event) =
                        self.nonbattlefield_graveyard_arrival(&before_move, &card, from)
                    {
                        events.push(event);
                    }
                    Some(card)
                } else {
                    if self.players[owner.index()]
                        .exile
                        .iter()
                        .any(|exiled| exiled.id == card.id)
                    {
                        events.extend(self.cards_exiled_events(std::iter::once((&card, from))));
                    }
                    None
                }
            }
        }
    }

    /// Moves a card between non-stack zones after applying replacement
    /// abilities printed on that card. The replacement is selected before the
    /// old object leaves its source zone, so its source-zone characteristics
    /// remain available while matching the proposed move.
    pub(super) fn move_card_from_nonbattlefield_zone(
        &mut self,
        id: GameObjectId,
        expected_from: ZoneKind,
        requested_to: ZoneKind,
        cause: ZoneMoveCause,
        arrival: Option<BattlefieldArrival>,
    ) -> Option<(CardInstance, ZoneKind)> {
        let mut events = Vec::new();
        let result = self.move_card_from_nonbattlefield_zone_collecting(
            id,
            expected_from,
            requested_to,
            cause,
            arrival,
            &mut events,
        );
        self.capture_zone_move_events(&events);
        result
    }

    pub(super) fn move_card_from_nonbattlefield_zone_collecting(
        &mut self,
        id: GameObjectId,
        expected_from: ZoneKind,
        requested_to: ZoneKind,
        cause: ZoneMoveCause,
        // How the permanent arrives, when the destination is the battlefield.
        // Reanimation that steals names a controller; a fetch land names
        // tapped. Everything else leaves this empty.
        arrival: Option<BattlefieldArrival>,
        events: &mut Vec<CommittedTriggerEvent>,
    ) -> Option<(CardInstance, ZoneKind)> {
        self.move_card_with_exile_visibility_collecting(
            id, expected_from, requested_to, cause, arrival, false, events,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn move_card_with_exile_visibility_collecting(
        &mut self,
        id: GameObjectId,
        expected_from: ZoneKind,
        requested_to: ZoneKind,
        cause: ZoneMoveCause,
        arrival: Option<BattlefieldArrival>,
        exile_face_down: bool,
        events: &mut Vec<CommittedTriggerEvent>,
    ) -> Option<(CardInstance, ZoneKind)> {
        let (from, card) = self
            .card_in_nonbattlefield_zone(id)
            .map(|(zone, card)| (zone, card.clone()))?;
        if from != expected_from {
            return None;
        }
        let destination = self
            .zone_move_replacement_destination(&card, from, requested_to, cause)
            .unwrap_or(requested_to);
        let destination = self.commander_hidden_move_destination(
            Target::Card(id),
            destination,
            cause,
            ZonePlacement::Top,
        )?;
        let shuffles = self.zone_move_replacement_shuffles(&card, from, requested_to, cause);
        if destination == ZoneKind::Stack {
            return None;
        }

        let owner = card.owner;
        let before_move = card.clone();
        // A battlefield arrival reuses the card's object ID for its entering
        // permanent, so recording a retired entry here would shadow that live
        // object with an entry that has no successor. The checkpoint then
        // reads the entry as a hidden-zone reference it cannot rebind and
        // fails the decision closed, as a library-to-battlefield shockland
        // entry did. Other destinations mint a fresh ID with a successor, so
        // only they keep last-known information here.
        if destination != ZoneKind::Battlefield {
            self.remember_card_characteristics(&card, Some(from));
        }
        let cards = match from {
            ZoneKind::Library => &mut self.players[owner.index()].library,
            ZoneKind::Hand => &mut self.players[owner.index()].hand,
            ZoneKind::Graveyard => &mut self.players[owner.index()].graveyard,
            ZoneKind::Exile => &mut self.players[owner.index()].exile,
            ZoneKind::Command => &mut self.players[owner.index()].command,
            ZoneKind::Battlefield | ZoneKind::Stack => return None,
        };
        let card = remove_card(cards, id)?;
        let card = if destination == ZoneKind::Battlefield {
            self.put_card_onto_battlefield_from(
                card,
                from,
                arrival.unwrap_or_else(|| BattlefieldArrival::under(owner)),
                None,
            )?
        } else {
            let (card, _zone_change) = self.zone_change_card(card);
            match destination {
                ZoneKind::Library => self.players[owner.index()].library.push(card.clone()),
                ZoneKind::Hand => self.players[owner.index()].hand.push(card.clone()),
                ZoneKind::Graveyard => self.put_card_into_graveyard(owner, card.clone()),
                ZoneKind::Exile => {
                    self.players[owner.index()].exile.push(card.clone());
                    if exile_face_down {
                        self.hide_from_everyone_while_exiled(card.id, owner);
                    }
                }
                ZoneKind::Command => self.players[owner.index()].command.push(card.clone()),
                ZoneKind::Battlefield | ZoneKind::Stack => {
                    unreachable!("unsupported destinations returned before removing the card")
                }
            }
            card
        };
        if shuffles && destination == ZoneKind::Library {
            self.rng.shuffle(&mut self.players[owner.index()].library);
        }
        if destination == ZoneKind::Graveyard
            && let Some(event) = self.nonbattlefield_graveyard_arrival(&before_move, &card, from)
        {
            events.push(event);
        }
        if self.players[owner.index()]
            .exile
            .iter()
            .any(|exiled| exiled.id == card.id)
        {
            events.extend(self.cards_exiled_events(std::iter::once((&card, from))));
        }
        if from == ZoneKind::Graveyard {
            self.note_card_left_graveyard(owner);
        }
        Some((card, destination))
    }

    /// "When this is put into a graveyard from anywhere" for the halves that
    /// are not a permanent dying: discarded from a hand, milled from a
    /// library, exiled and then returned.
    ///
    /// Raised after the card has landed, which is what lets the graveyard
    /// walk find the listener at all -- it reads the cards lying there. A
    /// battlefield departure uses the batched exit path instead, which keeps
    /// its pre-move LKI and installs the destination object before publishing.
    pub(super) fn nonbattlefield_graveyard_arrival(
        &self,
        before: &CardInstance,
        after: &CardInstance,
        from: ZoneKind,
    ) -> Option<CommittedTriggerEvent> {
        if from == ZoneKind::Graveyard
            || !self.players[after.owner.index()]
                .graveyard
                .iter()
                .any(|card| card.id == after.id)
        {
            return None;
        }
        let before = match from {
            ZoneKind::Stack => self
                .retired_stack_object(before.id)
                .and_then(|spell| self.stack_object_event_object(&spell)),
            ZoneKind::Battlefield => return None,
            _ => {
                let source_context = match from {
                    ZoneKind::Library => CharacteristicContext::Library,
                    ZoneKind::Hand => CharacteristicContext::Hand,
                    ZoneKind::Graveyard => CharacteristicContext::Graveyard,
                    ZoneKind::Exile => CharacteristicContext::Exile,
                    ZoneKind::Command => CharacteristicContext::Command,
                    ZoneKind::Stack | ZoneKind::Battlefield => unreachable!(),
                };
                self.printed_trigger_event_object(
                    before.id,
                    before.definition,
                    before.owner,
                    &source_context,
                )
            }
        };
        let after = self.printed_trigger_event_object(
            after.id,
            after.definition,
            after.owner,
            &CharacteristicContext::Graveyard,
        )?;
        Some(CommittedTriggerEvent::ZoneChanged {
            before,
            after: Some(after),
            from,
            to: ZoneKind::Graveyard,
            damage_sources: Vec::new(),
        })
    }

    /// Publish only after every card in a simultaneous move has arrived.
    pub(super) fn capture_zone_move_events(&mut self, events: &[CommittedTriggerEvent]) {
        if events.is_empty() {
            return;
        }
        let mut combined = Vec::new();
        for event in events {
            if let CommittedTriggerEvent::CardsExiled { cards, from, owner } = event
                && let Some(CommittedTriggerEvent::CardsExiled {
                    cards: previous_cards,
                    from: previous_from,
                    ..
                }) = combined.iter_mut().find(|candidate| {
                    matches!(candidate, CommittedTriggerEvent::CardsExiled { owner: previous, .. } if previous == owner)
                })
            {
                previous_cards.extend(cards.iter().cloned());
                for zone in from {
                    if !previous_from.contains(zone) {
                        previous_from.push(*zone);
                    }
                }
            } else {
                combined.push(event.clone());
            }
        }
        let listeners = self.battlefield_trigger_listeners();
        self.capture_battlefield_trigger_batch_from_snapshot(&listeners, &combined);
    }
}
