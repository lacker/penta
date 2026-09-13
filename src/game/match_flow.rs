//! Tournament decisions share the ordinary action/observation boundary.
use super::{Game, PlayerId};
use crate::game::{
    DecisionKind, DecisionObservation, DecisionOption, DecisionPreference, DecisionVisibility,
    DecisionZone,
};
use crate::match_play::{MatchContext, MatchMode};
use crate::{Action, ActionError, Deck, GameResult};
use serde_json::{Value, json};

impl Game {
    /// Select the match stopping condition before the first player acts.
    /// # Errors
    /// Rejects reconfiguration after play has started.
    pub fn set_match_mode(&mut self, mode: MatchMode) -> Result<(), String> {
        if !matches!(self.pregame, Some(super::Pregame::Mulligan(player)) if player == self.starting_player)
            || self.events.len() != 1
            || self.mulligans != [0, 0]
            || self.result.is_some()
            || self.match_context.is_some()
        {
            return Err("select match mode before the first action".into());
        }
        self.match_context = if mode == MatchMode::FirstToTwoWins {
            let decks = [PlayerId::One, PlayerId::Two].map(|seat| {
                let outside = &self.players[seat.index()].outside_game;
                let outside_ids = outside
                    .iter()
                    .flat_map(|card| super::backing_cards(&card.backing))
                    .collect::<std::collections::BTreeSet<_>>();
                Deck {
                    commanders: self
                        .commanders
                        .iter()
                        .filter(|c| c.owner == seat)
                        .map(|c| c.definition)
                        .collect(),
                    main: self
                        .physical_cards
                        .iter()
                        .filter(|physical| {
                            physical.owner == seat
                                && !outside_ids.contains(&physical.id)
                                && !self.commanders.iter().any(|c| c.physical == physical.id)
                        })
                        .map(|physical| physical.definition)
                        .collect(),
                    sideboard: outside.iter().map(|card| card.definition).collect(),
                }
            });
            Some(Box::new(MatchContext {
                initial_choice: true,
                registered: decks.clone(),
                decks,
                wins: [0, 0],
                draws: 0,
                chooser: self.starting_player,
                submitted: [true; 2],
                seed: self.seed,
            }))
        } else {
            None
        };
        if self.match_context.is_some() {
            // Play/draw is chosen before either player sees an opening hand.
            for player in &mut self.players {
                player.library.append(&mut player.hand);
            }
        }
        Ok(())
    }

    /// The current game's conclusion, distinct from the match's result.
    #[must_use]
    pub const fn current_game_result(&self) -> Option<GameResult> {
        self.result
    }

    pub(super) fn match_result(&self) -> Option<GameResult> {
        if let Some(context) = &self.match_context {
            let (wins, _) = context.score(self.result);
            if wins.iter().all(|wins| *wins < 2) {
                return None;
            }
        }
        self.result
    }

    pub(super) fn between_games(&self) -> bool {
        self.match_context.as_ref().is_some_and(|context| {
            context.initial_choice || (self.result.is_some() && self.match_result().is_none())
        })
    }

    // Keep the three between-game choices beside their shared observation shape.
    #[allow(clippy::too_many_lines)]
    pub(super) fn match_decision(&self) -> Option<DecisionObservation> {
        if !self.between_games() {
            return None;
        }
        let context = self.match_context.as_ref()?;
        let sideboarding = context.submitted.iter().any(|submitted| !submitted);
        let player = if sideboarding {
            if context.submitted[0] {
                PlayerId::Two
            } else {
                PlayerId::One
            }
        } else {
            context.chooser_after(self.result)
        };
        let deck = &context.decks[player.index()];
        let swapping = self
            .format
            .commander_definition()
            .is_some_and(|rules| rules.commander_swapping);
        let (prompt, minimum, maximum, options) = if sideboarding && swapping {
            let options = deck
                .commanders
                .iter()
                .chain(&deck.main)
                .enumerate()
                .map(|(index, id)| DecisionOption {
                    id: u32::try_from(index).expect("deck size fits IDs"),
                    label: self.catalog.get(*id).expect("registered card").name.clone(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: if index < deck.commanders.len() {
                        DecisionZone::Command
                    } else {
                        DecisionZone::Library
                    },
                })
                .collect::<Vec<_>>();
            (
                "Choose commanders for the next game",
                1,
                2.min(options.len()),
                options,
            )
        } else if sideboarding {
            let options = deck
                .main
                .iter()
                .chain(&deck.sideboard)
                .enumerate()
                .map(|(index, id)| DecisionOption {
                    id: u32::try_from(index).expect("deck size fits option IDs"),
                    label: self.catalog.get(*id).expect("registered card").name.clone(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: if index < deck.main.len() {
                        DecisionZone::Library
                    } else {
                        DecisionZone::OutsideGame
                    },
                })
                .collect::<Vec<_>>();
            let minimum = self.format.rules().minimum_main_deck_size.max(
                options
                    .len()
                    .saturating_sub(self.format.rules().maximum_sideboard_size),
            );
            (
                "Sideboard: select the cards for your next main deck",
                minimum,
                options.len(),
                options,
            )
        } else {
            let options = ["Play first", "Draw first"]
                .into_iter()
                .enumerate()
                .map(|(index, label)| DecisionOption {
                    id: u32::try_from(index).expect("two options"),
                    label: label.into(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                })
                .collect();
            ("Choose play or draw for the next game", 1, 1, options)
        };
        Some(DecisionObservation {
            id: self.next_decision_id,
            player,
            kind: DecisionKind::Choice,
            order_semantics: None,
            source: None,
            prompt: prompt.into(),
            visibility: DecisionVisibility::Private,
            preference: DecisionPreference::Neutral,
            minimum,
            maximum,
            cancellable: false,
            options,
        })
    }

    pub(super) fn match_action_is_legal(&self, player: PlayerId, action: &Action) -> bool {
        let Some(offered) = self.match_decision() else {
            return false;
        };
        let Action::ChooseDecision { decision, options } = action else {
            return false;
        };
        if player != offered.player
            || *decision != offered.id
            || options.len() < offered.minimum
            || options.len() > offered.maximum
        {
            return false;
        }
        let unique = options
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        if unique.len() != options.len()
            || unique
                .iter()
                .any(|id| !offered.options.iter().any(|option| option.id == *id))
        {
            return false;
        }
        if self
            .match_context
            .as_ref()
            .is_some_and(|context| !context.submitted[player.index()])
        {
            let deck = self.sideboard_selection(player, options);
            return self.match_context.as_ref().is_some_and(|context| {
                context.valid_deck(player, &deck, &self.catalog, self.format)
            });
        }
        true
    }

    fn sideboard_selection(&self, player: PlayerId, options: &[u32]) -> Deck {
        let deck = &self.match_context.as_ref().expect("match context").decks[player.index()];
        if self
            .format
            .commander_definition()
            .is_some_and(|rules| rules.commander_swapping)
        {
            let mut selected = Deck {
                commanders: Vec::new(),
                main: Vec::new(),
                sideboard: deck.sideboard.clone(),
            };
            for (index, id) in deck.commanders.iter().chain(&deck.main).enumerate() {
                if options.contains(&u32::try_from(index).expect("deck size fits IDs")) {
                    selected.commanders.push(*id);
                } else {
                    selected.main.push(*id);
                }
            }
            return selected;
        }
        let mut selected = Deck {
            commanders: deck.commanders.clone(),
            main: Vec::new(),
            sideboard: Vec::new(),
        };
        for (index, id) in deck.main.iter().chain(&deck.sideboard).enumerate() {
            if options.contains(&u32::try_from(index).expect("deck size fits IDs")) {
                selected.main.push(*id);
            } else {
                selected.sideboard.push(*id);
            }
        }
        selected
    }

    pub(super) fn apply_match_action(
        &mut self,
        player: PlayerId,
        action: Action,
    ) -> Result<(), ActionError> {
        if !self.match_action_is_legal(player, &action) {
            return Err(ActionError::NotLegal {
                player,
                action: Box::new(action),
            });
        }
        let Action::ChooseDecision { options, .. } = action else {
            unreachable!()
        };
        if !self.match_context.as_ref().expect("context").submitted[player.index()] {
            let deck = self.sideboard_selection(player, &options);
            let context = self.match_context.as_mut().expect("context");
            context.decks[player.index()] = deck;
            context.submitted[player.index()] = true;
            self.next_decision_id += 1;
            self.forget_enumeration();
            return Ok(());
        }
        let starting = if options[0] == 0 {
            player
        } else {
            player.opponent()
        };
        let mut context = self.match_context.as_ref().expect("context").clone();
        let (wins, draws) = context.score(self.result);
        context.initial_choice = false;
        context.wins = wins;
        context.draws = draws;
        context.chooser = player;
        context.submitted = [false; 2];
        let conclusions = u64::from(wins[0] + wins[1] + draws);
        let next_seed = context
            .seed
            .wrapping_add(conclusions.wrapping_mul(0x9e37_79b9_7f4a_7c15));
        let mut replacement = Self::new_from_decks(
            self.format,
            self.catalog.clone(),
            context.decks.clone(),
            next_seed,
            starting,
            true,
            self.next_object_id,
        )
        .expect("validated registered decks");
        replacement.next_decision_id = self.next_decision_id + 1;
        replacement.set_prepared_engine_enabled(self.prepared_engine_enabled());
        replacement.match_context = Some(context);
        // Preserve a monotonic journal cursor across game boundaries.
        let mut events = std::mem::take(&mut self.events);
        events.append(&mut replacement.events);
        replacement.events = events;
        *self = replacement;
        Ok(())
    }

    /// Hidden-safe match summary. Each seat sees only its own submitted deck.
    #[must_use]
    pub fn match_json(&self, viewer: PlayerId) -> Value {
        let mode = if self.match_context.is_some() {
            MatchMode::FirstToTwoWins
        } else {
            MatchMode::OneConclusion
        };
        let (wins, draws) = self.match_context.as_ref().map_or_else(
            || {
                let mut wins = [0, 0];
                let mut draws = 0;
                match self.result {
                    Some(GameResult::Winner { winner, .. }) => wins[winner.index()] = 1,
                    Some(GameResult::Draw) => draws = 1,
                    None => (),
                }
                (wins, draws)
            },
            |context| context.score(self.result),
        );
        let stage = if self.result().is_some() {
            "complete"
        } else if self.between_games() {
            if self
                .match_context
                .as_ref()
                .is_some_and(|context| context.submitted.iter().all(|value| *value))
            {
                "play-draw"
            } else {
                "sideboarding"
            }
        } else {
            "playing"
        };
        let game = self.match_context.as_ref().map_or(1, |context| {
            context.wins[0] + context.wins[1] + context.draws + 1
        });
        let deck = self
            .match_context
            .as_ref()
            .map(|context| &context.decks[viewer.index()]);
        json!({ "mode": mode.slug(), "wins": wins, "draws": draws, "game": game, "stage": stage, "restarts": self.restart_count,
            "main": deck.map(|deck| &deck.main), "sideboard": deck.map(|deck| &deck.sideboard),
            "submitted": self.match_context.as_ref().map(|context| context.submitted),
        })
    }
}

impl Game {
    pub(super) fn match_checkpoint(&self, viewer: PlayerId) -> Option<Value> {
        let context = self.match_context.as_ref()?;
        let mut value = serde_json::to_value(context).expect("match context serializes");
        // A determinization supplies the opposing lists separately. The match
        // seed is private randomness, replaced with the rollout seed on import.
        value["seed"] = json!(0);
        value["registered"][viewer.opponent().index()] = Value::Null;
        value["decks"][viewer.opponent().index()] = Value::Null;
        let opposing = &context.registered[viewer.opponent().index()];
        value["opponentCardCount"] = json!(opposing.main.len() + opposing.sideboard.len());
        Some(value)
    }

    pub(super) fn restore_match_checkpoint(
        &mut self,
        value: Option<&Value>,
        hidden: &Value,
        viewer: PlayerId,
        seed: u64,
    ) -> Result<(), String> {
        let Some(value) = value else {
            return Ok(());
        };
        let mut state = value.clone();
        let opposing = viewer.opponent().index();
        let hypothesis = hidden.get("matchDecks").and_then(|decks| decks.get(opposing)).ok_or("match checkpoints require the opponent's registered and current lists in hidden.matchDecks")?;
        state["registered"][opposing] = hypothesis["registered"].clone();
        state["decks"][opposing] = hypothesis["current"].clone();
        state["seed"] = json!(seed);
        let context: MatchContext =
            serde_json::from_value(state).map_err(|error| error.to_string())?;
        if context.wins.iter().any(|wins| *wins >= 2)
            || context.draws > u32::MAX - 5
            || (context.initial_choice
                && (context.wins != [0, 0]
                    || context.draws != 0
                    || self.result.is_some()
                    || context.submitted != [true; 2]))
            || (!context.initial_choice && self.result.is_none() && context.submitted != [false; 2])
        {
            return Err("inconsistent match checkpoint".into());
        }
        let deck = &context.registered[opposing];
        if value["opponentCardCount"].as_u64()
            != Some((deck.main.len() + deck.sideboard.len()) as u64)
        {
            return Err("opponent registration size does not match the checkpoint".into());
        }
        for seat in [PlayerId::One, PlayerId::Two] {
            if !context.valid_deck(
                seat,
                &context.decks[seat.index()],
                &self.catalog,
                self.format,
            ) {
                return Err("invalid match deck hypothesis".into());
            }
        }
        self.match_context = Some(Box::new(context));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Format, WinReason,
        card::{self, cards},
    };

    fn game() -> Game {
        let decks = [0, 1].map(|_| Deck {
            commanders: Vec::new(),
            main: vec![cards::MOUNTAIN; 60],
            sideboard: vec![cards::FOREST; 15],
        });
        let mut game =
            Game::new_with_format(Format::OldSchool9394, card::catalog().unwrap(), decks, 17)
                .unwrap();
        game.set_match_mode(MatchMode::FirstToTwoWins).unwrap();
        let decision = game.match_decision().unwrap();
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![0],
            },
        )
        .unwrap();
        game
    }

    fn next(game: &mut Game, first: PlayerId) {
        for _ in 0..2 {
            let player = game.decision_player().unwrap();
            let decision = game.observe(player).decision.unwrap();
            let options = decision
                .options
                .iter()
                .filter(|option| option.zone == DecisionZone::Library)
                .map(|option| option.id)
                .collect();
            game.apply(
                player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
        }
        let chooser = game.decision_player().unwrap();
        let decision = game.observe(chooser).decision.unwrap();
        game.apply(
            chooser,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![u32::from(first != chooser)],
            },
        )
        .unwrap();
    }

    #[test]
    fn tournament_match_draws_and_sideboarding_use_shared_actions() {
        let mut game = game();
        game.result = Some(GameResult::Draw);
        assert_eq!(game.result(), None);
        assert_eq!(game.match_json(PlayerId::One)["draws"], 1);
        assert!(game.observe(PlayerId::Two).decision.is_none());
        next(&mut game, PlayerId::Two);
        assert_eq!(game.active_player, PlayerId::Two);
        assert_eq!(game.decision_player(), Some(PlayerId::Two));
        for winner in [PlayerId::One, PlayerId::Two, PlayerId::One] {
            game.result = Some(GameResult::Winner {
                winner,
                reason: WinReason::OpponentConceded,
            });
            if game.result().is_none() {
                next(&mut game, winner.opponent());
            }
        }
        assert_eq!(game.match_json(PlayerId::One)["wins"], json!([2, 1]));
        assert_eq!(game.match_json(PlayerId::One)["draws"], 1);
        assert!(game.decision_player().is_none());
        assert!(game.legal_actions(PlayerId::One).is_empty());
    }

    #[test]
    fn tournament_match_checkpoint_round_trips_sideboarding_privately() {
        let mut game = game();
        game.pregame = None;
        game.finish(GameResult::Draw);
        let (wire, mut hidden) = crate::game::tests::checkpoint_fixture(&game, PlayerId::One);
        let context = game.match_context.as_ref().unwrap();
        hidden["matchDecks"] =
            json!([null, {"registered": context.registered[1], "current": context.decks[1]}]);
        assert!(wire["checkpoint"]["matchState"]["registered"][1].is_null());
        assert!(wire["checkpoint"]["matchState"]["decks"][1].is_null());
        let mut rebuilt = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            90,
        )
        .unwrap();
        assert_eq!(
            game.match_json(PlayerId::One),
            rebuilt.match_json(PlayerId::One)
        );
        assert_eq!(
            game.observe(PlayerId::One).decision,
            rebuilt.observe(PlayerId::One).decision
        );
        next(&mut rebuilt, PlayerId::Two);
        assert_eq!(rebuilt.match_json(PlayerId::One)["draws"], 1);
        assert_eq!(rebuilt.active_player, PlayerId::Two);
    }

    #[test]
    fn tournament_match_rejects_duplicate_and_undersized_sideboards_without_mutation() {
        let mut game = game();
        game.result = Some(GameResult::Draw);
        let decision = game.observe(PlayerId::One).decision.unwrap();
        for options in [vec![0; 60], (0..59).collect(), vec![100; 60]] {
            assert!(
                game.apply(
                    PlayerId::One,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options
                    }
                )
                .is_err()
            );
            assert_eq!(
                game.observe(PlayerId::One).decision.as_ref(),
                Some(&decision)
            );
        }
    }
}
