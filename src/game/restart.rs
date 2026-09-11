//! A rules restart preserves participating cards, not the old rules state.
use super::{
    BattlefieldArrival, Deck, Game, GameObjectId, ObjectBacking, PlayerId, StackObject, Target,
    ZoneMoveCause,
};
use crate::card::{ObjectPredicateDef, ZoneKind, ZonePlacement};

#[derive(Clone, Debug)]
pub(super) struct RestartRequest {
    controller: PlayerId,
    retained: Vec<GameObjectId>,
}

#[derive(Clone, Debug)]
pub(super) struct RestartArrivals {
    pub controller: PlayerId,
    pub retained: Vec<GameObjectId>,
    pub entering: bool,
    pub ready: Vec<super::PendingEvent>,
}

impl Game {
    pub(super) fn request_restart(&mut self, predicate: ObjectPredicateDef, object: &StackObject) {
        let retained = self
            .matching_linked_exiles(predicate, object)
            .into_iter()
            .filter_map(|target| {
                if let Target::Card(id) = target {
                    Some(id)
                } else {
                    None
                }
            })
            .collect();
        self.pending_restart = Some(RestartRequest {
            controller: object.controller,
            retained,
        });
    }

    pub(super) fn perform_restart(&mut self) {
        let request = self.pending_restart.take().expect("restart was requested");
        let retained = request
            .retained
            .iter()
            .filter_map(|id| {
                self.card_in_nonbattlefield_zone(*id)
                    .map(|(_, card)| card.clone())
            })
            .collect::<Vec<_>>();
        let outside = self
            .players
            .iter()
            .flat_map(|player| &player.outside_game)
            .collect::<Vec<_>>();
        let omitted = retained
            .iter()
            .chain(outside.iter().copied())
            .flat_map(|card| super::backing_cards(&card.backing))
            .collect::<Vec<_>>();
        let decks = [PlayerId::One, PlayerId::Two].map(|seat| Deck {
            commanders: self
                .commanders
                .iter()
                .filter(|c| c.owner == seat && !omitted.contains(&c.physical))
                .map(|c| c.definition)
                .collect(),
            main: self
                .physical_cards
                .iter()
                .filter(|physical| {
                    physical.owner == seat
                        && !omitted.contains(&physical.id)
                        && !self.commanders.iter().any(|c| c.physical == physical.id)
                })
                .map(|physical| physical.definition)
                .collect(),
            sideboard: self.players[seat.index()]
                .outside_game
                .iter()
                .map(|card| card.definition)
                .collect(),
        });
        let seed = self.rng.next_u64();
        let mut restarted = Self::new_from_decks(
            self.format,
            self.catalog.clone(),
            decks,
            seed,
            request.controller,
            false,
            self.next_object_id,
        )
        .expect("participating cards are known to the catalog");
        let mut ids = Vec::new();
        for card in retained {
            if let ObjectBacking::Cards(physical) = card.backing {
                for id in physical {
                    let physical = self
                        .physical_cards
                        .iter()
                        .find(|card| card.id == id)
                        .expect("physical card");
                    let mut cards = restarted
                        .build_zone(physical.owner, &[physical.definition])
                        .expect("registered physical card");
                    let card = cards.pop().expect("one card");
                    if self
                        .commanders
                        .iter()
                        .any(|commander| commander.physical == id)
                    {
                        let backing = super::backing_cards(&card.backing)[0];
                        restarted
                            .commanders
                            .push(super::commander::CommanderState::new(
                                backing,
                                physical.definition,
                                physical.owner,
                            ));
                    }
                    ids.push(card.id);
                    restarted.players[physical.owner.index()].exile.push(card);
                }
            }
        }
        restarted.set_prepared_engine_enabled(self.prepared_engine_enabled());
        restarted.match_context = self.match_context.take();
        restarted.restart_count = self.restart_count + 1;
        restarted.next_decision_id = self.next_decision_id;
        restarted.restart_arrivals = Some(RestartArrivals {
            controller: request.controller,
            retained: ids,
            entering: false,
            ready: Vec::new(),
        });
        let mut events = std::mem::take(&mut self.events);
        events.append(&mut restarted.events);
        restarted.events = events;
        *self = restarted;
    }

    /// Returns true while restart startup still owns the rules procedure.
    pub(super) fn continue_restart_startup(&mut self) -> bool {
        if self.pregame.is_some() {
            return false;
        }
        let Some(mut arrivals) = self.restart_arrivals.take() else {
            return false;
        };
        if !arrivals.entering {
            // These entries happen before the first turn. Creatures therefore
            // become eligible to attack when that turn starts normally.
            self.turn = 0;
            self.turns_started = [0, 0];
            arrivals.entering = true;
            let retained = arrivals.retained.clone();
            let controller = arrivals.controller;
            self.restart_arrivals = Some(arrivals);
            for id in retained {
                self.move_target_to_zone(
                    Target::Card(id),
                    ZoneKind::Battlefield,
                    ZoneMoveCause::Effect { controller },
                    Some(BattlefieldArrival::under(controller)),
                    ZonePlacement::Top,
                );
            }
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                return true;
            }
            arrivals = self.restart_arrivals.take().expect("restart startup");
        }
        let controller = arrivals.controller;
        let ready = std::mem::take(&mut arrivals.ready);
        arrivals.entering = false;
        self.restart_arrivals = Some(arrivals);
        // Resolve every entry replacement against the pre-entry battlefield,
        // then commit the arrivals together so they all see each other's ETBs.
        self.entering_together(|game| {
            for pending in ready {
                game.commit_pending_event(pending);
            }
        });
        self.restart_arrivals = None;
        self.commit_next_turn(controller, Vec::new());
        false
    }
}

impl Game {
    /// Checkpoint hypotheses describe live card objects, so assign physical
    /// identities once on import. Tokens, copies and ability objects have none.
    pub(super) fn restore_physical_cards(&mut self) {
        fn assign(
            cards: &mut Vec<super::PhysicalCard>,
            definition: crate::CardDefinitionId,
            owner: PlayerId,
            backing: &mut ObjectBacking,
        ) {
            let id = crate::PhysicalCardId(
                u32::try_from(cards.len()).expect("physical card count fits"),
            );
            cards.push(super::PhysicalCard {
                id,
                definition,
                owner,
            });
            *backing = ObjectBacking::Cards(vec![id]);
        }
        for player in &mut self.players {
            for card in player
                .library
                .iter_mut()
                .chain(&mut player.hand)
                .chain(&mut player.graveyard)
                .chain(&mut player.exile)
                .chain(&mut player.command)
                .chain(&mut player.outside_game)
            {
                assign(
                    &mut self.physical_cards,
                    card.definition,
                    card.owner,
                    &mut card.backing,
                );
            }
        }
        for permanent in self.battlefield.iter_mut().chain(&mut self.phased_out) {
            if let Some(definition) = permanent.card.definition.card_definition() {
                assign(
                    &mut self.physical_cards,
                    definition,
                    permanent.card.owner,
                    &mut permanent.card.backing,
                );
            }
        }
        for pending in self.pending_events.iter_mut().chain(
            self.restart_arrivals
                .iter_mut()
                .flat_map(|state| &mut state.ready),
        ) {
            let super::ReplaceableEvent::BattlefieldEntry(entry) = &mut pending.event;
            if let Some(definition) = entry.permanent.card.definition.card_definition() {
                assign(
                    &mut self.physical_cards,
                    definition,
                    entry.permanent.card.owner,
                    &mut entry.permanent.card.backing,
                );
            }
        }
        for object in self.stack.iter_mut() {
            if object.kind == super::StackObjectKind::Spell
                && !object.is_copy
                && let Some(definition) = object.card.definition.card_definition()
            {
                assign(
                    &mut self.physical_cards,
                    definition,
                    object.card.owner,
                    &mut object.card.backing,
                );
            }
        }
    }
}

#[cfg(test)]
#[path = "restart_tests.rs"]
mod tests;
