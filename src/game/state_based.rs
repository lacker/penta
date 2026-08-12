use super::{
    CardSupertype, CardType, CounterKind, Game, GameObjectId, GameResult, PlayerId,
    TriggerEventDef, WinReason,
};

impl Game {
    pub(super) fn check_state_based_actions(&mut self) {
        self.close_stale_miracle_window();
        if self.check_player_loss_conditions() {
            return;
        }
        loop {
            let battlefield_len = self.battlefield.len();
            let mut regenerate = Vec::new();
            let mut die = Vec::new();
            for permanent in &self.battlefield {
                // 704.5m: an Aura attached to nothing, or to something that is
                // no longer a legal host, is put into its owner's graveyard.
                if self.is_aura_permanent(permanent)
                    && permanent
                        .attached_to
                        .is_none_or(|host| !self.is_legal_aura_host(permanent, host))
                {
                    die.push(permanent.card.id);
                    continue;
                }
                if self
                    .permanent_types(permanent)
                    .is_some_and(|types| types.contains(CardType::Planeswalker))
                    && permanent.counters(CounterKind::Loyalty) == 0
                {
                    die.push(permanent.card.id);
                    continue;
                }
                let Some(toughness) = self.toughness(permanent) else {
                    continue;
                };
                let zero_toughness = toughness <= 0;
                let lethal_damage = i32::from(permanent.damage) >= i32::from(toughness)
                    || (permanent.damage > 0 && permanent.deathtouch_damage);
                if zero_toughness {
                    die.push(permanent.card.id);
                    continue;
                }
                if !lethal_damage || self.has_indestructible(permanent) {
                    continue;
                }
                if permanent.regeneration_shields > 0 {
                    regenerate.push(permanent.card.id);
                } else {
                    die.push(permanent.card.id);
                }
            }
            for id in regenerate {
                self.regenerate_permanent(id);
            }
            self.move_permanents_to_graveyard(&die);
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                return;
            }
            self.apply_legend_rule();
            if self.battlefield.len() == battlefield_len {
                break;
            }
        }
        self.capture_state_triggers();
    }

    /// CR 704.5a-b: zero life and trying to draw from an empty library are
    /// state-based loss conditions. Read both players and both conditions in
    /// one pass so simultaneous losses end the two-player game in a draw.
    fn check_player_loss_conditions(&mut self) -> bool {
        let tried_to_draw_from_empty = self
            .players
            .each_mut()
            .map(|player| std::mem::take(&mut player.tried_to_draw_from_empty_library));
        let lost = [
            self.players[0].life <= 0 || tried_to_draw_from_empty[0],
            self.players[1].life <= 0 || tried_to_draw_from_empty[1],
        ];

        let result = match lost {
            [true, true] => Some(GameResult::Draw),
            [true, false] => Some(GameResult::Winner {
                winner: PlayerId::Two,
                reason: if tried_to_draw_from_empty[0] {
                    WinReason::OpponentTriedToDrawFromEmptyLibrary
                } else {
                    WinReason::OpponentLostAllLife
                },
            }),
            [false, true] => Some(GameResult::Winner {
                winner: PlayerId::One,
                reason: if tried_to_draw_from_empty[1] {
                    WinReason::OpponentTriedToDrawFromEmptyLibrary
                } else {
                    WinReason::OpponentLostAllLife
                },
            }),
            [false, false] => None,
        };
        if let Some(result) = result {
            self.finish(result);
            true
        } else {
            false
        }
    }

    /// CR 603.8: a state trigger triggers whenever its condition is true, and
    /// does not trigger again while it is already waiting or on the stack.
    /// State-based actions are checked whenever anything could have changed,
    /// which is exactly when such a condition could have become true.
    pub(super) fn capture_state_triggers(&mut self) {
        let listeners = self
            .battlefield_trigger_listeners()
            .into_iter()
            .filter(|listener| {
                listener.uses_stack && listener.event == TriggerEventDef::StateCondition
            })
            .filter(|listener| {
                let source = listener.capture.source;
                let waiting = self
                    .pending_triggers
                    .iter()
                    .any(|pending| pending.source == source);
                let on_stack = self.stack.iter().any(|object| {
                    object.source == Some(source.object)
                        && object
                            .ability
                            .as_ref()
                            .is_some_and(|ability| ability.origin == source.ability)
                });
                !waiting && !on_stack
            })
            .collect::<Vec<_>>();
        for listener in listeners {
            self.capture_trigger(&listener.capture);
        }
    }

    /// The legend rule as a state-based action: a player controlling two or
    /// more legendary permanents with the same name keeps one and puts the
    /// rest into the graveyard. The rules let the controller choose; with
    /// identical names the copies differ only in tap and damage state, so the
    /// strictly best one — untapped over tapped, then newest — is kept
    /// without asking.
    pub(super) fn apply_legend_rule(&mut self) {
        loop {
            let mut extra: Option<GameObjectId> = None;
            'search: for permanent in &self.battlefield {
                if !self
                    .effective_rules(permanent)
                    .is_some_and(|rules| rules.has_supertype(CardSupertype::Legendary))
                {
                    continue;
                }
                let name_source = Self::effective_rules_source(permanent);
                for other in &self.battlefield {
                    if other.card.id == permanent.card.id
                        || other.controller != permanent.controller
                        || Self::effective_rules_source(other) != name_source
                    {
                        continue;
                    }
                    let permanent_wins = (!permanent.tapped && other.tapped)
                        || (permanent.tapped == other.tapped
                            && permanent.card.id.0 > other.card.id.0);
                    extra = Some(if permanent_wins {
                        other.card.id
                    } else {
                        permanent.card.id
                    });
                    break 'search;
                }
            }
            let Some(extra) = extra else {
                return;
            };
            self.move_permanents_to_graveyard(&[extra]);
            if !self.pending_decisions.is_empty() || !self.pending_events.is_empty() {
                return;
            }
        }
    }
}
