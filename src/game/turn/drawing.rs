//! Drawing cards: the ordinary one, the replacements that can interrupt it,
//! the miracle reveal that rides on the turn's first, and the simultaneous
//! opening draw. Split from the turn structure that schedules them because
//! a draw is one instruction whose interruptions are its own.

use super::super::{
    AlternativeCastKindDef, CardDefinitionId, CardPartId, CommittedTriggerEvent,
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    DeclarativeAbilityDef, Game, GameEvent, GameObjectId, GameResult, ObjectCharacteristics,
    PendingProcedure, PlayerId, Step, WinReason,
};

impl Game {
    pub(in crate::game) fn draw_card(&mut self, player: PlayerId) -> Option<GameObjectId> {
        if self.draw_replacements[player.index()].len() > 1 {
            self.queue_draw_replacement_choice(player);
            return None;
        }
        if let Some(replacement) = self.draw_replacements[player.index()].pop_front() {
            self.resolve_effect_def(replacement.effect, &replacement.object, replacement.context);
            return None;
        }
        let Some(card) = self.players[player.index()].library.pop() else {
            // Jace, Wielder of Mysteries turns the loss into a win. The draw
            // is replaced, so the flag that would end the game is never set.
            if self.player_wins_on_empty_library_draw(player) {
                self.finish(GameResult::Winner {
                    winner: player,
                    reason: WinReason::OpponentLostToAnEffect,
                });
                return None;
            }
            self.players[player.index()].tried_to_draw_from_empty_library = true;
            return None;
        };
        let (card, _zone_change) = self.zone_change_card(card);
        let card_id = card.id;
        let definition = card.definition;
        self.players[player.index()].hand.push(card);
        self.events.push(GameEvent::CardDrawn {
            player,
            card: card_id,
        });
        let drawn = &mut self.cards_drawn_this_turn[player.index()];
        *drawn = drawn.saturating_add(1);
        self.drawn_this_turn[player.index()].push(card_id);
        if self.cards_drawn_this_turn[player.index()] == 1 && self.has_miracle(definition) {
            self.queue_miracle_reveal(player, card_id);
        }
        // Raised where the card actually reaches the hand: a draw that was
        // replaced above never happened, so nothing watching for one fires.
        // Asked before the flag is set, so the draw that claims the
        // exemption is the one that reports having it.
        let first_in_draw_step = self.step == Step::Draw
            && self.active_player == player
            && !self.draw_step_draw_taken[player.index()];
        if first_in_draw_step {
            self.draw_step_draw_taken[player.index()] = true;
        }
        self.capture_battlefield_triggers(&CommittedTriggerEvent::DrewCard {
            player,
            first_in_draw_step,
        });
        Some(card_id)
    }

    fn queue_draw_replacement_choice(&mut self, player: PlayerId) {
        let replacements = self.draw_replacements[player.index()]
            .drain(..)
            .collect::<Vec<_>>();
        let options = replacements
            .iter()
            .enumerate()
            .map(|(index, replacement)| {
                let name = self
                    .presentation_name(replacement.object.presentation())
                    .unwrap_or_else(|| "Draw replacement".into());
                DecisionOption {
                    id: u32::try_from(index).unwrap_or(u32::MAX),
                    label: replacement
                        .object
                        .ability_text()
                        .map_or_else(|| name.to_string(), |text| format!("{name} — {text}")),
                    card: None,
                    members: Vec::new(),
                    ability_text: replacement.object.ability_text().map(str::to_owned),
                    zone: DecisionZone::None,
                }
            })
            .collect();
        self.queue_decision(
            player,
            "Choose which effect replaces this draw",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::DrawReplacement {
                player,
                replacements,
            },
        );
    }

    /// Whether a card offers a miracle cost at all.
    pub(in crate::game) fn has_miracle(&self, definition: CardDefinitionId) -> bool {
        self.catalog.get(definition).is_some_and(|definition| {
            definition.parts.iter().any(|part| {
                part.rules.ability_clauses().iter().any(|ability| {
                    ability.is_executable()
                        && matches!(
                            ability.definition,
                            DeclarativeAbilityDef::AlternativeCast(alternative)
                                if alternative.kind == AlternativeCastKindDef::Miracle
                        )
                })
            })
        })
    }

    /// Offers the reveal that opens a miracle window. Revealing is the whole
    /// choice: whether to then pay the cost is the ordinary cast decision,
    /// and declining to cast simply lets the window close.
    pub(in crate::game) fn queue_miracle_reveal(&mut self, player: PlayerId, card: GameObjectId) {
        let definition = self.players[player.index()]
            .hand
            .iter()
            .find(|held| held.id == card)
            .map(|held| held.definition);
        let name = definition
            .and_then(|definition| self.catalog.get(definition))
            .map_or_else(
                || "that card".to_string(),
                |definition| definition.name.clone(),
            );
        self.queue_decision(
            player,
            format!("Reveal {name} for its miracle cost?"),
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            false,
            vec![
                DecisionOption {
                    id: 0,
                    label: "Keep it hidden".into(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                },
                DecisionOption {
                    id: 1,
                    label: format!("Reveal {name}"),
                    card: definition.map(|definition| {
                        (
                            card,
                            ObjectCharacteristics::card(definition, CardPartId::PRIMARY),
                        )
                    }),
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::Hand,
                },
            ],
            DecisionContinuation::MiracleReveal { card },
        );
    }

    pub(in crate::game) fn draw_cards(&mut self, player: PlayerId, count: u16) {
        if count == 0 {
            return;
        }
        if !self.pending_decisions.is_empty()
            || !self.pending_events.is_empty()
            || !self.pending_procedures.is_empty()
        {
            self.pending_procedures
                .push_back(PendingProcedure::DrawCards {
                    player,
                    remaining: count,
                });
            return;
        }
        let mut remaining = count;
        while remaining > 0 {
            if self.result.is_some() {
                break;
            }
            remaining -= 1;
            let _ = self.draw_card(player);
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                if remaining > 0 {
                    self.pending_procedures
                        .push_back(PendingProcedure::DrawCards { player, remaining });
                }
                break;
            }
        }
    }

    /// Draws every card for the active player first, then every card for the
    /// other player. Each player's draws still happen one at a time so draw
    /// replacements can suspend the instruction. One spell can deck both
    /// players, so empty-library losses remain deferred until the complete
    /// simultaneous instruction finishes. Empty-library loss is recorded on
    /// each player and settled at the next state-based-action check.
    #[cfg(test)]
    pub(in crate::game) fn draw_cards_simultaneously(&mut self, counts: [u16; 2]) {
        let was_deferred = self.defer_empty_library_loss;
        self.defer_empty_library_loss = true;
        self.continue_simultaneous_draws(counts, self.active_player, was_deferred);
    }

    pub(in crate::game) fn continue_simultaneous_draws(
        &mut self,
        mut remaining: [u16; 2],
        mut next: PlayerId,
        was_deferred: bool,
    ) {
        while remaining.iter().any(|count| *count > 0) && self.result.is_none() {
            let player = next;
            if remaining[player.index()] == 0 {
                next = player.opponent();
                continue;
            }
            remaining[player.index()] -= 1;
            let _ = self.draw_card(player);
            if remaining[player.index()] == 0 {
                next = player.opponent();
            }
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                self.pending_procedures
                    .push_back(PendingProcedure::SimultaneousDraws {
                        remaining,
                        next,
                        was_deferred,
                    });
                return;
            }
        }
        self.defer_empty_library_loss = was_deferred;
    }
}
