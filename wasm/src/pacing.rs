use super::action_view::{action_card, animated_action_kind, should_animate_action};
use super::{
    Action, BOT_ACTION_LIMIT, CardInstanceId, GameEvent, JsValue, PlayerId, Step, Value, WebGame,
    js_error, json,
};

impl WebGame {
    pub(super) fn advance_until_human_choice(&mut self) -> Result<(), JsValue> {
        for _ in 0..BOT_ACTION_LIMIT {
            let Some(player) = self.session.decision_seat() else {
                return Ok(());
            };
            let observation = self.session.observe(player);
            let action = if player == self.human {
                let automatic_action = self.automatic_human_action_for(&observation);
                let Some(action) = automatic_action else {
                    return Ok(());
                };
                action
            } else {
                let Some(action) = self.bot.choose_action(&observation) else {
                    // An external opponent has no policy here. The engine
                    // waits, and the host feeds the seat's next action in
                    // through `opponent_act`.
                    return Ok(());
                };
                action
            };
            self.apply_advancing_action(player, &observation, action)?;
        }
        Err(JsValue::from_str(
            "game exceeded its automatic action limit",
        ))
    }

    /// Applies one action for either seat with the full presentation
    /// bookkeeping: opponent beats, mana grouping, resolution and combat
    /// records, the turn banner, and the board your own click left behind.
    /// The advance loop and an externally driven opponent both come through
    /// here, so a remote bot's play is watched exactly like a local one's.
    pub(super) fn apply_advancing_action(
        &mut self,
        player: PlayerId,
        observation: &super::PlayerObservation,
        action: Action,
    ) -> Result<(), JsValue> {
        let mut pending_animation = None;
        if player != self.human {
            if let Action::ActivateManaAbility { source, .. } = &action {
                self.pending_opponent_mana
                    .push(self.instance_name(observation, *source));
            } else if should_animate_action(&action) {
                let mana_sources = if matches!(
                    action,
                    Action::CastSpell { .. }
                        | Action::ActivateAbility { .. }
                        | Action::TakeSpecialAction { .. }
                ) {
                    std::mem::take(&mut self.pending_opponent_mana)
                } else {
                    Vec::new()
                };
                let label = self.opponent_action_label(observation, &action);
                let kind = animated_action_kind(&action);
                let card_id = action_card(&action);
                let card = card_id.map(|id| self.instance_name(observation, id));
                pending_animation = Some(json!({
                    "label": label,
                    "kind": kind,
                    "card": card,
                    "cardId": card_id.map(|id| id.0),
                    "manaSources": mana_sources,
                }));
            } else {
                self.pending_opponent_mana.clear();
            }
        }
        // Who owns each object on the stack, read before it leaves: a
        // resolution event names the card but the object is gone by then.
        let stack_owners: Vec<(CardInstanceId, PlayerId)> = observation
            .stack
            .iter()
            .map(|object| (object.id, object.controller))
            .collect();
        let event_start = self.session.event_cursor();
        self.session.apply(player, action).map_err(js_error)?;
        if pending_animation.is_none() {
            self.record_resolutions(event_start, &stack_owners);
        }
        self.record_combat_damage(event_start);
        self.record_draw_step(event_start);
        if let Some(mut animation) = pending_animation.take() {
            let caused = self.session.events_for_since(self.human, event_start);
            let mana_sources = caused
                .iter()
                .filter_map(|event| match event {
                    GameEvent::ManaAdded {
                        player: producer,
                        source,
                    } if *producer == player => Some(*source),
                    _ => None,
                })
                .map(|source| json!(self.instance_name(observation, source)))
                .collect::<Vec<_>>();
            if let Some(existing) = animation["manaSources"].as_array_mut() {
                existing.extend(mana_sources);
            }
            animation["state"] = self.snapshot_value(false);
            self.opponent_actions.push(animation);
        }
        // Last, so the beat that ended the turn is still watched before the
        // next turn is announced.
        self.record_turn_change(event_start);
        // Your click is not finished until your own spell has left the
        // stack: the yields that resolve it are automatic and produce no
        // beat, so they belong to what you did rather than to the replay.
        // The moment anything worth watching happens, this stops moving
        // and the replay starts from there.
        if player == self.human && self.opponent_actions.is_empty() {
            self.human_action_state = Some(self.snapshot_value(false));
        }
        Ok(())
    }

    /// Gives anything that resolved off the stack its own beat.
    ///
    /// A pass that completes a round resolves the top of the stack, an event
    /// no one clicked. Without a beat the object would blink out between
    /// frames — and a turn banner could show while a spell everyone watched
    /// resolve still sat on the stack.
    ///
    /// Your own spell is the exception: its resolution is the rest of the
    /// click you just made, and replaying it locks the board for a beat you
    /// did not need to watch. A fizzle is always shown, whoever cast it, since
    /// it is the only explanation for a spell that did nothing.
    pub(super) fn record_resolutions(
        &mut self,
        event_start: usize,
        stack_owners: &[(CardInstanceId, PlayerId)],
    ) {
        let caused = self.session.events_for_since(self.human, event_start);
        let resolved: Vec<_> = caused
            .iter()
            .filter_map(|event| match event {
                GameEvent::SpellResolved { card, definition } => Some((*card, *definition, false)),
                GameEvent::AbilityResolved {
                    object, definition, ..
                }
                | GameEvent::TriggeredAbilityResolved {
                    object, definition, ..
                } => Some((*object, *definition, false)),
                GameEvent::SpellFizzled { card, definition } => Some((*card, *definition, true)),
                GameEvent::AbilityFizzled {
                    object, definition, ..
                }
                | GameEvent::TriggeredAbilityFizzled {
                    object, definition, ..
                } => Some((*object, *definition, true)),
                _ => None,
            })
            .collect();
        for (card, definition, fizzled) in resolved {
            let yours = stack_owners
                .iter()
                .any(|(object, controller)| *object == card && *controller == self.human);
            if yours && !fizzled {
                continue;
            }
            let name = self.card_name(definition);
            self.opponent_actions.push(json!({
                "label": if fizzled {
                    format!("{name} fizzles")
                } else {
                    format!("{name} resolves")
                },
                "kind": "spell",
                "card": name,
                "cardId": card.0,
                "manaSources": Vec::<String>::new(),
                "state": self.snapshot_value(false),
            }));
        }
    }

    /// Gives the turn's draw its own beat.
    ///
    /// The draw step is over in the same yield that entered it, so without a
    /// beat the card arrives in a frame the board already labels "first main".
    /// Holding it here draws the card where the phase strip says it happens.
    pub(super) fn record_draw_step(&mut self, event_start: usize) {
        let events = self.session.events_for_since(self.human, event_start);
        let drew = events
            .iter()
            .any(|event| matches!(event, GameEvent::CardDrawn { .. }));
        let in_draw_step = events.iter().any(|event| {
            matches!(
                event,
                GameEvent::StepChanged {
                    step: Step::Draw,
                    ..
                }
            )
        });
        if !drew || !in_draw_step {
            return;
        }
        self.opponent_actions.push(json!({
            "label": "Draw",
            "kind": "draw",
            "card": Value::Null,
            "cardId": Value::Null,
            "manaSources": Vec::<String>::new(),
            "state": self.snapshot_value(false),
        }));
    }

    /// Gives combat damage its own beat.
    ///
    /// Nobody clicks damage into happening, and yielding through the step is
    /// now the normal way an unblocked attack ends. Without a beat the life
    /// totals and the dead creatures would change between frames.
    pub(super) fn record_combat_damage(&mut self, event_start: usize) {
        let events = self.session.events_for_since(self.human, event_start);
        let entered_damage = events.iter().any(|event| {
            matches!(
                event,
                GameEvent::StepChanged {
                    step: Step::CombatDamage,
                    ..
                }
            )
        });
        if !entered_damage {
            return;
        }
        let landed = events.iter().any(|event| {
            matches!(
                event,
                GameEvent::DamageDealt { .. }
                    | GameEvent::LifeLost { .. }
                    | GameEvent::PermanentLeftBattlefield { .. }
            )
        });
        if !landed {
            return;
        }
        self.opponent_actions.push(json!({
            "label": "Combat damage",
            "kind": "combat",
            "card": Value::Null,
            "cardId": Value::Null,
            "manaSources": Vec::<String>::new(),
            "state": self.snapshot_value(false),
        }));
    }

    /// Gives a turn that just began its own presentation beat.
    ///
    /// Turn banners are otherwise inferred from the beats around them, so an
    /// opponent who draws and passes would slide by without ever being
    /// announced. This beat carries no action of its own — the client shows
    /// the banner and moves on.
    pub(super) fn record_turn_change(&mut self, event_start: usize) {
        let caused = self.session.events_for_since(self.human, event_start);
        let Some(turn) = caused
            .iter()
            .filter_map(|event| match event {
                GameEvent::StepChanged { turn, .. } => Some(*turn),
                _ => None,
            })
            .next_back()
        else {
            return;
        };
        if self.announced_turn == Some(turn) {
            return;
        }
        self.announced_turn = Some(turn);
        self.opponent_actions.push(json!({
            "label": "New turn",
            "kind": "turn",
            "card": Value::Null,
            "cardId": Value::Null,
            "manaSources": Vec::<String>::new(),
            "state": self.snapshot_value(false),
        }));
    }
}
