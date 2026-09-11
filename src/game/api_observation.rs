// Seat observation assembly shares api.rs imports.
impl Game {
    #[allow(clippy::too_many_lines)]
    pub fn observe(&self, viewer: PlayerId) -> PlayerObservation {
        let _land_types = self.hold_land_type_query_memo();
        let _listeners = self.hold_board_read_memo();
        let player = &self.players[viewer.index()];
        let opponent = &self.players[viewer.opponent().index()];
        PlayerObservation {
            viewer,
            turn: self.turn,
            active_turn: self.turns_started[self.active_player.index()],
            active_player: self.active_player,
            priority: self.priority,
            step: self.step,
            regular_combat_damage_pending: self.regular_combat_damage_pending(),
            life_totals: [self.players[0].life, self.players[1].life],
            poison_counters: [
                self.players[0].counters.count(CounterKind::Poison),
                self.players[1].counters.count(CounterKind::Poison),
            ],
            energy_counters: [
                self.players[0].counters.count(CounterKind::named("energy")),
                self.players[1].counters.count(CounterKind::named("energy")),
            ],
            counters: [PlayerId::One, PlayerId::Two].map(|player| {
                self.players[player.index()]
                    .counters
                    .iter()
                    .map(|(kind, count)| CounterObservation {
                        name: kind.name().to_owned(),
                        count,
                    })
                    .collect()
            }),
            monarch: self.monarch,
            mana_pools: [self.players[0].mana_pool, self.players[1].mana_pool],
            hand: player
                .hand
                .iter()
                .map(|card| (card.id, card.definition))
                .collect(),
            opponent_hand_size: opponent.hand.len(),
            last_seen_hand: self.last_seen_hands[viewer.index()].clone(),
            public_reveals: self.events.iter().filter_map(|event| match event {
                GameEvent::CardRevealed { player, card, definition } => Some((*player, *card, *definition)),
                _ => None,
            }).collect(),
            library_sizes: [self.players[0].library.len(), self.players[1].library.len()],
            companions: self.observed_companions(viewer),
            revealed_library_top: self.observed_library_top(viewer, viewer),
            opponent_revealed_library_top: self.observed_library_top(viewer, viewer.opponent()),
            command_zones: [public_cards(&self.players[0].command), public_cards(&self.players[1].command)],
            commanders: self.commanders(viewer),
            graveyards: [
                public_cards(&self.players[0].graveyard),
                public_cards(&self.players[1].graveyard),
            ],
            // A foretold card lies face down, so the opponent's is left out
            // of the list entirely and counted instead -- the same way a
            // hand is a size rather than a list. Your own are listed: you
            // know what you exiled.
            exiles: [
                self.observed_exile(PlayerId::One, viewer),
                self.observed_exile(PlayerId::Two, viewer),
            ],
            face_down_exile_sizes: [
                self.face_down_exile_size(PlayerId::One),
                self.face_down_exile_size(PlayerId::Two),
            ],
            card_counters: self.observed_card_counters(viewer),
            // Phased-out permanents come last and carry a flag: they are
            // visible to both players, and only the rules treat them as
            // absent. Reconstruction relies on this order.
            battlefield: self
                .battlefield
                .iter()
                .chain(self.phased_out.iter())
                .map(|permanent| self.observe_permanent(permanent, viewer))
                .collect(),
            emblems: self.observed_emblems(),
            stack: self
                .stack
                .iter()
                .map(|object| StackObservation {
                    id: object.id,
                    kind: object.kind,
                    source: object.source,
                    ability: object.ability_origin(),
                    ability_text: object.ability_text().map(str::to_owned),
                    // A spell cast face down has its mechanism-owned public
                    // values; only its controller knows which card it is.
                    characteristics: match object.face_down {
                        Some(face_down) if object.controller != viewer => {
                            ObjectCharacteristics::face_down(face_down)
                        }
                        Some(_) | None => object.presentation(),
                    },
                    controller: object.controller,
                    counterable: self.can_be_countered(object),
                    signature: (object.face_down.is_none() || object.controller == viewer)
                        .then(|| object.signature.clone())
                        .flatten(),
                    targets: object.declared_targets(),
                    chosen_permanents: object.chosen_permanents.clone(),
                    x: object.x(),
                })
                .collect(),
            decision: self.match_decision().filter(|decision| decision.player == viewer).or_else(|| (!self.between_games()).then(|| self.pending_decisions.first()).flatten().and_then(|decision| {
                decision.observation.for_viewer(viewer)
            })),
            result: self.result(),
            legal_actions: self.legal_actions(viewer),
            checkpoint: self.checkpoint_json(viewer),
        }
    }

}
