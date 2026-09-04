mod action_view;
mod autopass;
mod hosted;
mod labels;
mod pacing;
mod presentation;
mod session;
mod snapshot;

use penta::card;
use penta::game::{DecisionKind, DecisionOrderSemantics};
use penta::{
    AbilityOrigin, Action, BattlefieldExit, CardArtPreference, CardCatalog, CardDefinitionId,
    CardInstanceId, Format, Game, GameEvent, GameResult, HandcraftedPolicy, ModeId,
    ObjectCharacteristics, PlayOptionId, PlayerId, PlayerObservation, Policy, RandomPolicy, Step,
    Target,
};
use presentation::deck_by_name;
use serde_json::{Value, json};
use session::{Checkpoint, LocalSession};
use wasm_bindgen::prelude::*;

#[cfg(test)]
use action_view::{
    action_ability_origin, action_card, action_target_card, action_target_cards,
    action_target_player, action_target_players, action_target_stack, action_target_stacks,
    cast_signature_value, should_animate_action,
};
#[cfg(test)]
use autopass::{
    AutoPassContext, automatic_human_action, automatic_human_action_for_context,
    automatic_human_action_with_blockers,
};
#[cfg(test)]
use presentation::{
    StackCardPresentation, card_art_value, hand_mana_cost_value, object_presentation,
    stack_card_presentation,
};

const BOT_ACTION_LIMIT: usize = 50_000;

/// Version of the browser/host command-journal envelope. Changes to command
/// encoding or interpretation move this independently from the bot wire and
/// core simulation fingerprint.
const REPLAY_VERSION: u32 = 2;

/// What a host says when its clock simply expired. Journaled verbatim like
/// any other reason, and recognised here so that the ordinary ending keeps
/// the win-reason table's seat-aware wording rather than this bare phrase.
const DEFAULT_TIMEOUT_REASON: &str = "ran out of time";

fn parse_art_preference(value: Option<&str>) -> Result<CardArtPreference, JsValue> {
    match value.unwrap_or("debut") {
        "debut" => Ok(CardArtPreference::Debut),
        "format-matching" => Ok(CardArtPreference::FormatMatching),
        other => Err(js_error(format!("unknown card art preference {other:?}"))),
    }
}

const fn art_preference_slug(preference: CardArtPreference) -> &'static str {
    match preference {
        CardArtPreference::Debut => "debut",
        CardArtPreference::FormatMatching => "format-matching",
    }
}

fn required_json_field<'a>(
    object: &'a serde_json::Map<String, Value>,
    context: &str,
    name: &str,
) -> Result<&'a Value, JsValue> {
    object
        .get(name)
        .ok_or_else(|| js_error(format!("{context}.{name} is required")))
}

fn required_json_object<'a>(
    object: &'a serde_json::Map<String, Value>,
    context: &str,
    name: &str,
) -> Result<&'a serde_json::Map<String, Value>, JsValue> {
    required_json_field(object, context, name)?
        .as_object()
        .ok_or_else(|| js_error(format!("{context}.{name} must be an object")))
}

fn required_json_string<'a>(
    object: &'a serde_json::Map<String, Value>,
    context: &str,
    name: &str,
) -> Result<&'a str, JsValue> {
    required_json_field(object, context, name)?
        .as_str()
        .ok_or_else(|| js_error(format!("{context}.{name} must be a string")))
}

fn required_json_bool(
    object: &serde_json::Map<String, Value>,
    context: &str,
    name: &str,
) -> Result<bool, JsValue> {
    required_json_field(object, context, name)?
        .as_bool()
        .ok_or_else(|| js_error(format!("{context}.{name} must be boolean")))
}

fn required_json_u32(
    object: &serde_json::Map<String, Value>,
    context: &str,
    name: &str,
) -> Result<u32, JsValue> {
    required_json_field(object, context, name)?
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| {
            js_error(format!(
                "{context}.{name} must be an unsigned 32-bit integer"
            ))
        })
}

fn required_json_usize(
    object: &serde_json::Map<String, Value>,
    context: &str,
    name: &str,
) -> Result<usize, JsValue> {
    required_json_field(object, context, name)?
        .as_u64()
        .and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| {
            js_error(format!(
                "{context}.{name} must be an unsigned integer index"
            ))
        })
}

fn required_json_array<'a>(
    object: &'a serde_json::Map<String, Value>,
    context: &str,
    name: &str,
) -> Result<&'a [Value], JsValue> {
    required_json_field(object, context, name)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| js_error(format!("{context}.{name} must be an array")))
}

fn required_json_u32_array(
    object: &serde_json::Map<String, Value>,
    context: &str,
    name: &str,
) -> Result<Vec<u32>, JsValue> {
    required_json_array(object, context, name)?
        .iter()
        .enumerate()
        .map(|(index, value)| {
            value
                .as_u64()
                .and_then(|value| u32::try_from(value).ok())
                .ok_or_else(|| {
                    js_error(format!(
                        "{context}.{name}[{index}] must be an unsigned 32-bit integer"
                    ))
                })
        })
        .collect()
}

enum BotPolicy {
    Random(RandomPolicy),
    Handcrafted(HandcraftedPolicy),
    /// No policy at all: the opponent seat is driven from outside, one
    /// protocol action index at a time, the way a bot on a socket plays.
    External,
}

impl BotPolicy {
    fn choose_action(&mut self, observation: &PlayerObservation) -> Option<Action> {
        match self {
            Self::Random(policy) => policy.choose_action(observation),
            Self::Handcrafted(policy) => policy.choose_action(observation),
            Self::External => None,
        }
    }
}

/// Browser-owned game facade. JavaScript only selects legal action indexes;
/// rules and bot decisions remain inside the Rust engine.
#[wasm_bindgen]
pub struct WebGame {
    session: LocalSession,
    /// How this game was dealt, kept verbatim so the journal below can be
    /// replayed by anyone -- a game room, a bug report, a native harness.
    replay_config: Value,
    /// Every command applied through the public surface, in order. With the
    /// config this is the whole game; the two together are what a bug report
    /// attaches.
    journal: Vec<Value>,
    catalog: CardCatalog,
    art_preference: CardArtPreference,
    human: PlayerId,
    bot: BotPolicy,
    opponent_actions: Vec<Value>,
    pending_opponent_mana: Vec<String>,
    mana_undo_history: Vec<Checkpoint>,
    phase_stops: Vec<String>,
    autopass_enabled: bool,
    attack_undo: Option<Checkpoint>,
    /// The turn the presentation has already announced, so a turn nobody acts
    /// on still gets its banner instead of being skipped over in silence.
    announced_turn: Option<u32>,
    /// The board the moment your own action landed, before the game answered.
    human_action_state: Option<Value>,
    /// Why a host's clock ended this game, when the host said something more
    /// specific than [`DEFAULT_TIMEOUT_REASON`]. `None` is the ordinary
    /// expired-clock ending, which the win-reason table already describes.
    timeout_reason: Option<String>,
}

#[wasm_bindgen]
impl WebGame {
    /// Creates a mirror-format game and advances until the human must decide.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when a deck or policy name is unknown, game
    /// construction fails, or the bot cannot reach a human decision.
    #[allow(clippy::needless_pass_by_value)] // wasm-bindgen owns optional strings at the ABI.
    #[wasm_bindgen(constructor)]
    pub fn new(
        human_deck: &str,
        bot_deck: &str,
        bot_policy: &str,
        human_first: bool,
        seed: u32,
        format: Option<String>,
        art_preference: Option<String>,
    ) -> Result<WebGame, JsValue> {
        let format = penta::protocol::parse_format_slug(
            format.as_deref().unwrap_or(Format::OldSchool9394.slug()),
        )
        .map_err(js_error)?;
        let art_preference = parse_art_preference(art_preference.as_deref())?;
        // The names as asked for, before resolution: a replay hands these
        // same strings back to this same constructor.
        let replay_config = json!({
            "format": format.slug(),
            "artPreference": art_preference_slug(art_preference),
            "humanDeck": human_deck,
            "botDeck": bot_deck,
            "botPolicy": bot_policy.to_ascii_lowercase(),
            "humanFirst": human_first,
            "seed": seed,
        });
        let catalog = card::catalog().map_err(js_error)?;
        let human_deck = deck_by_name(format, human_deck)?;
        let bot_deck = deck_by_name(format, bot_deck)?;
        let human = if human_first {
            PlayerId::One
        } else {
            PlayerId::Two
        };
        let decks = match human {
            PlayerId::One => [human_deck, bot_deck],
            PlayerId::Two => [bot_deck, human_deck],
        };
        let game = Game::new_with_format(format, catalog.clone(), decks, u64::from(seed))
            .map_err(js_error)?;
        let bot = match bot_policy.to_ascii_lowercase().as_str() {
            "random" => BotPolicy::Random(RandomPolicy::new(u64::from(seed) ^ 0x00b0_7b07)),
            "handcrafted" => BotPolicy::Handcrafted(HandcraftedPolicy::new(catalog.clone())),
            "external" => BotPolicy::External,
            _ => return Err(JsValue::from_str("unknown bot policy")),
        };
        let mut web_game = Self {
            session: LocalSession::new(game),
            replay_config,
            journal: Vec::new(),
            catalog,
            art_preference,
            human,
            bot,
            opponent_actions: Vec::new(),
            pending_opponent_mana: Vec::new(),
            mana_undo_history: Vec::new(),
            phase_stops: Vec::new(),
            autopass_enabled: true,
            attack_undo: None,
            // The opening turn arrives with the board, not as a change to it.
            announced_turn: Some(1),
            human_action_state: None,
            timeout_reason: None,
        };
        web_game.advance_until_human_choice()?;
        Ok(web_game)
    }

    /// Applies one action from the current state's action list.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when the game is not waiting for the human,
    /// the index is stale, the action is rejected, or the bot cannot finish.
    pub fn act(&mut self, action_index: usize) -> Result<(), JsValue> {
        if self.session.decision_seat() != Some(self.human) {
            return Err(JsValue::from_str("the game is not waiting for the human"));
        }
        let observation = self.session.observe(self.human);
        let action = observation
            .legal_actions
            .get(action_index)
            .cloned()
            .ok_or_else(|| JsValue::from_str("unknown legal action"))?;
        self.apply_human_action(action)?;
        // Only a command that took effect belongs in the journal: a replay
        // reapplies these, and a rejected one would halt it.
        self.journal
            .push(json!({ "t": "act", "index": action_index }));
        Ok(())
    }

    /// Submits the selected option IDs for the current generic decision.
    ///
    /// The selection is validated by the engine, so the browser does not need
    /// to receive an eagerly-expanded action for every possible combination.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when the game is not waiting for the human,
    /// the JSON is malformed, or the engine rejects the selection.
    pub fn choose_decision(&mut self, decision: u32, options_json: &str) -> Result<(), JsValue> {
        if self.session.decision_seat() != Some(self.human) {
            return Err(JsValue::from_str("the game is not waiting for the human"));
        }
        let options: Vec<u32> = serde_json::from_str(options_json).map_err(js_error)?;
        self.apply_human_action(Action::ChooseDecision {
            decision,
            options: options.clone(),
        })?;
        self.journal
            .push(json!({ "t": "choose", "decision": decision, "options": options }));
        Ok(())
    }

    fn apply_human_action(&mut self, action: Action) -> Result<(), JsValue> {
        let mana_checkpoint =
            matches!(action, Action::ActivateManaAbility { .. }).then(|| self.session.checkpoint());
        if mana_checkpoint.is_none() {
            self.mana_undo_history.clear();
        }
        // The first declaration of the combat is the point a cancel returns to.
        if matches!(action, Action::DeclareAttacker { .. }) && self.attack_undo.is_none() {
            self.attack_undo = Some(self.session.checkpoint());
        }
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        let event_start = self.session.event_cursor();
        self.session.apply(self.human, action).map_err(js_error)?;
        // What you just did, before anything the game does in response. The
        // replay is told from here, so a land you played is on the board
        // before the turn it ended is announced.
        self.human_action_state = Some(self.snapshot_value(false));
        // Yielding is how combat damage happens, and ending your own turn hands
        // one to the opponent. Both need showing every bit as much as the
        // actions the bot takes on its own.
        self.record_combat_damage(event_start);
        self.record_draw_step(event_start);
        self.record_turn_change(event_start);
        // Committing the attack invalidates the checkpoint immediately, so no
        // snapshot taken while the turn plays out still offers the cancel.
        self.forget_attack_undo_unless_still_declaring();
        self.advance_until_human_choice()?;
        self.forget_attack_undo_unless_still_declaring();
        if let Some(checkpoint) = mana_checkpoint {
            let before = checkpoint.observed_by(self.human);
            let after = self.session.observe(self.human);
            if self.session.decision_seat() == Some(self.human)
                && before.turn == after.turn
                && before.step == after.step
                && before.active_player == after.active_player
                && before.stack == after.stack
            {
                self.mana_undo_history.push(checkpoint);
            } else {
                self.mana_undo_history.clear();
            }
        }
        Ok(())
    }

    /// Declares every currently legal attacker, then finishes attacker declaration.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error unless the human is declaring attackers or
    /// the engine rejects one of the otherwise-legal actions.
    pub fn attack_all(&mut self) -> Result<(), JsValue> {
        if self.session.decision_seat() != Some(self.human)
            || self.session.observe(self.human).step != Step::DeclareAttackers
        {
            return Err(JsValue::from_str("the human is not declaring attackers"));
        }
        self.mana_undo_history.clear();
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        if self.attack_undo.is_none() {
            self.attack_undo = Some(self.session.checkpoint());
        }
        loop {
            let action = self
                .session
                .observe(self.human)
                .legal_actions
                .into_iter()
                .find(|action| matches!(action, Action::DeclareAttacker { .. }));
            let Some(action) = action else {
                break;
            };
            self.session.apply(self.human, action).map_err(js_error)?;
        }
        if let Some(finish) = self
            .session
            .observe(self.human)
            .legal_actions
            .into_iter()
            .find(|action| matches!(action, Action::FinishDeclaringAttackers))
        {
            self.session.apply(self.human, finish).map_err(js_error)?;
        }
        self.forget_attack_undo_unless_still_declaring();
        self.advance_until_human_choice()?;
        self.forget_attack_undo_unless_still_declaring();
        self.journal.push(json!({ "t": "attackAll" }));
        Ok(())
    }

    /// Takes back every attacker declared so far this combat.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when the attack has already been committed.
    pub fn cancel_attackers(&mut self) -> Result<(), JsValue> {
        let previous = self
            .attack_undo
            .take()
            .ok_or_else(|| JsValue::from_str("there are no declared attackers to take back"))?;
        self.session.restore(previous);
        self.mana_undo_history.clear();
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        self.journal.push(json!({ "t": "cancelAttackers" }));
        Ok(())
    }

    /// A cancel is only offered while the attack is still being assembled;
    /// once it is committed the declaration is part of the game.
    fn forget_attack_undo_unless_still_declaring(&mut self) {
        if self.attack_undo.is_none() {
            return;
        }
        let still_declaring = self.session.decision_seat() == Some(self.human)
            && self
                .session
                .observe(self.human)
                .legal_actions
                .iter()
                .any(|action| matches!(action, Action::FinishDeclaringAttackers));
        if !still_declaring {
            self.attack_undo = None;
        }
    }

    /// Commits a complete set of blocker assignments selected by the browser UI.
    ///
    /// Assignments are encoded as JSON pairs of `[blocker_id, attacker_id]` so
    /// the UI can rearrange arrows freely before making one atomic submission.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error unless the human is declaring blockers or an
    /// assignment is duplicated, malformed, or no longer legal.
    pub fn finalize_blocks(&mut self, assignments_json: &str) -> Result<(), JsValue> {
        if self.session.decision_seat() != Some(self.human)
            || self.session.observe(self.human).step != Step::DeclareBlockers
        {
            return Err(JsValue::from_str("the human is not declaring blockers"));
        }
        let assignments: Vec<[u32; 2]> =
            serde_json::from_str(assignments_json).map_err(js_error)?;
        let mut used_blockers = Vec::with_capacity(assignments.len());
        let legal_actions = self.session.observe(self.human).legal_actions;
        let mut block_actions = Vec::with_capacity(assignments.len());
        for [blocker, attacker] in assignments {
            let blocker = CardInstanceId(blocker);
            if used_blockers.contains(&blocker) {
                return Err(JsValue::from_str("a blocker can only block one attacker"));
            }
            used_blockers.push(blocker);
            let action = Action::DeclareBlocker {
                blocker,
                attacker: CardInstanceId(attacker),
            };
            if !legal_actions.contains(&action) {
                return Err(JsValue::from_str("a blocker assignment is no longer legal"));
            }
            block_actions.push(action);
        }
        self.mana_undo_history.clear();
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        for action in block_actions {
            self.session.apply(self.human, action).map_err(js_error)?;
        }
        self.session
            .apply(self.human, Action::FinishDeclaringBlockers)
            .map_err(js_error)?;
        self.advance_until_human_choice()?;
        self.journal
            .push(json!({ "t": "blocks", "assignments": assignments_json }));
        Ok(())
    }

    /// Rewinds the most recent manual mana ability while it is still safe to do so.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when there is no reversible mana activation.
    pub fn undo_mana(&mut self) -> Result<(), JsValue> {
        let previous = self
            .mana_undo_history
            .pop()
            .ok_or_else(|| JsValue::from_str("there is no mana ability to undo"))?;
        self.session.restore(previous);
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        self.journal.push(json!({ "t": "undoMana" }));
        Ok(())
    }

    /// Enables or disables a human-interface stop for one displayed phase.
    /// The rules engine still exposes every individual step.
    /// Sets or clears a UI phase stop.
    ///
    /// # Errors
    ///
    /// Returns an error if advancing the facade encounters an invalid engine action.
    pub fn set_phase_stop(&mut self, phase: &str, enabled: bool) -> Result<(), JsValue> {
        if !matches!(
            phase,
            "Beginning" | "Main 1" | "Combat" | "Main 2" | "Ending"
        ) {
            return Err(JsValue::from_str("unknown displayed phase"));
        }
        self.phase_stops.retain(|candidate| candidate != phase);
        if enabled {
            self.phase_stops.push(phase.into());
        }
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        self.journal
            .push(json!({ "t": "phaseStop", "phase": phase, "enabled": enabled }));
        Ok(())
    }

    /// Enables or disables the browser's smooth automatic priority yields.
    /// Enables or disables routine UI priority passing.
    ///
    /// # Errors
    ///
    /// Returns an error if advancing the facade encounters an invalid engine action.
    pub fn set_autopass(&mut self, enabled: bool) -> Result<(), JsValue> {
        self.autopass_enabled = enabled;
        self.opponent_actions.clear();
        self.pending_opponent_mana.clear();
        if enabled {
            self.advance_until_human_choice()?;
        }
        self.journal
            .push(json!({ "t": "autopass", "enabled": enabled }));
        Ok(())
    }

    /// Whether the opponent seat is driven from outside rather than by a
    /// built-in policy. The snapshot uses this to keep the seed out of a
    /// game whose opponent is real.
    pub(crate) fn opponent_is_externally_driven(&self) -> bool {
        matches!(self.bot, BotPolicy::External)
    }

    /// Whether the engine is waiting on the externally driven opponent seat.
    /// Always false for a built-in policy, which never leaves the engine
    /// waiting between calls.
    #[wasm_bindgen(js_name = opponentIsDeciding)]
    #[must_use]
    pub fn opponent_is_deciding(&self) -> bool {
        matches!(self.bot, BotPolicy::External)
            && self.session.decision_seat() == Some(self.human.opponent())
    }

    /// Whether the game has ended, without building the state to find out.
    ///
    /// A host asks this constantly -- on every bot poll, and again on every
    /// applied command -- and the answer is one `Option` on the session. The
    /// alternative it replaces, serializing the whole human-visible state
    /// and parsing back one field, costs a payload that grows with the game
    /// and is discarded immediately.
    #[wasm_bindgen(js_name = isFinished)]
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.session.result().is_some()
    }

    /// The finished game's result as the human sees it, or `None` while the
    /// game is live. The same object `state_json`'s `result` member carries,
    /// for a caller that wants only that.
    #[wasm_bindgen(js_name = resultJson)]
    #[must_use]
    pub fn result_json(&self) -> Option<String> {
        self.result_value().map(|result| result.to_string())
    }

    /// The opponent seat's redacted view, in the same protocol JSON a bot on
    /// a socket already reads. Only an external opponent has a driver to
    /// show it to.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error unless the opponent is externally driven.
    #[wasm_bindgen(js_name = opponentObserveJson)]
    pub fn opponent_observe_json(&self) -> Result<String, JsValue> {
        if !matches!(self.bot, BotPolicy::External) {
            return Err(js_error(
                "the opponent is played by a built-in policy, not a driver",
            ));
        }
        let observation = self.session.observe(self.human.opponent());
        let actions = penta::protocol::protocol_actions(&observation);
        Ok(penta::protocol::observation_json_for_format(
            &self.catalog,
            self.session.format(),
            &observation,
            self.session.in_pregame(),
            &actions,
        )
        .to_string())
    }

    /// Applies the external opponent's chosen action by its index in the
    /// protocol action list, with the same presentation bookkeeping a
    /// built-in opponent gets, then advances until the human holds a real
    /// choice again. Beats accumulate across a run of these, so the human
    /// watches a remote opponent's turn exactly as they would a local one's.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error unless the opponent is externally driven
    /// and currently holds the decision, or when the index is out of range.
    #[wasm_bindgen(js_name = opponentAct)]
    pub fn opponent_act(&mut self, index: u32) -> Result<(), JsValue> {
        if !matches!(self.bot, BotPolicy::External) {
            return Err(js_error(
                "the opponent is played by a built-in policy, not a driver",
            ));
        }
        let opponent = self.human.opponent();
        if self.session.decision_seat() != Some(opponent) {
            return Err(js_error("the opponent does not hold the decision"));
        }
        let observation = self.session.observe(opponent);
        let actions = penta::protocol::protocol_actions(&observation);
        let action = actions
            .get(index as usize)
            .cloned()
            .ok_or_else(|| js_error("action index out of range"))?;
        self.apply_advancing_action(opponent, &observation, action)?;
        self.advance_until_human_choice()?;
        self.journal.push(json!({ "t": "botAct", "index": index }));
        Ok(())
    }

    /// Ends the game because one seat, `"human"` or `"bot"`, ran out of
    /// time. This is how a room enforces its clock.
    ///
    /// Unlike the ordinary verbs it does not require that seat to hold the
    /// decision: a player who has stopped answering is exactly the player who
    /// is not going to take their turn. And unlike conceding, nobody chose
    /// it, which is why the result says so.
    ///
    /// `reason` is the host's own account of the ending, and defaults to
    /// [`DEFAULT_TIMEOUT_REASON`]. A host that knows more than "the clock
    /// expired" -- that the opponent's process stopped answering, say --
    /// should say so, because a seat that is merely slow and a seat that is
    /// gone are different things to the player waiting on it. Anything other
    /// than the default is shown to the human in place of the generic
    /// wording, so phrase it from that seat's point of view and name its own
    /// subject: "Fizzbot stopped answering", not "stopped answering".
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error for an unknown seat, or when the game is
    /// already over.
    #[allow(clippy::needless_pass_by_value)] // wasm-bindgen owns optional strings at the ABI.
    #[wasm_bindgen(js_name = loseOnTime)]
    pub fn lose_on_time(&mut self, seat: &str, reason: Option<String>) -> Result<(), JsValue> {
        self.lose_on_time_with_reason(seat, reason.as_deref().unwrap_or(DEFAULT_TIMEOUT_REASON))
    }

    fn lose_on_time_with_reason(&mut self, seat: &str, reason: &str) -> Result<(), JsValue> {
        let (player, replay_seat) = match seat {
            "human" => (self.human, "human"),
            "bot" | "opponent" => (self.human.opponent(), "bot"),
            other => return Err(js_error(format!("unknown seat {other:?}"))),
        };
        if self.session.result().is_some() {
            return Err(js_error("the game is already over"));
        }
        self.mana_undo_history.clear();
        self.attack_undo = None;
        // Only a reason that says more than the default is worth carrying:
        // the win-reason table already words the plain expired clock, and it
        // words it per seat, which a single stored string cannot.
        self.timeout_reason = (reason != DEFAULT_TIMEOUT_REASON).then(|| reason.to_owned());
        self.session.lose_on_time(player);
        // The ending is the whole remaining story, so the human sees it as a
        // beat rather than as a board that silently stopped.
        self.human_action_state = None;
        self.journal.push(json!({
            "t": "loseOnTime",
            "seat": replay_seat,
            "reason": reason,
        }));
        Ok(())
    }

    /// The whole game as a portable record: how it was dealt, and every
    /// command applied since, in order. Deterministic replay is what makes
    /// this a bug report's attachment -- the same JSON rebuilds the same
    /// board in a game room, a browser, or a native harness.
    #[wasm_bindgen(js_name = replayJson)]
    #[must_use]
    pub fn replay_json(&self) -> String {
        json!({
            "config": self.replay_config,
            "commands": self.journal,
            "replayVersion": REPLAY_VERSION,
            "simulationFingerprint": penta::protocol::SIMULATION_FINGERPRINT,
            "engineVersion": penta::protocol::ENGINE_VERSION,
            "protocolVersion": penta::protocol::PROTOCOL_VERSION,
        })
        .to_string()
    }

    /// Rebuilds a game from [`Self::replay_json`] output, refusing an unknown
    /// journal format or simulation fingerprint before replaying commands.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error for malformed JSON, a version mismatch, or
    /// a command that no longer applies at its position.
    #[wasm_bindgen(js_name = fromReplayJson)]
    pub fn from_replay_json(replay: &str) -> Result<WebGame, JsValue> {
        let replay: Value = serde_json::from_str(replay).map_err(js_error)?;
        let envelope = replay
            .as_object()
            .ok_or_else(|| js_error("replay must be an object"))?;
        let version = required_json_u32(envelope, "replay", "replayVersion")?;
        if version != REPLAY_VERSION {
            return Err(js_error(format!(
                "replay version {version} does not match {REPLAY_VERSION}",
            )));
        }
        let fingerprint = required_json_string(envelope, "replay", "simulationFingerprint")?;
        if fingerprint != penta::protocol::SIMULATION_FINGERPRINT {
            return Err(js_error(format!(
                "replay simulation fingerprint {fingerprint:?} does not match {}",
                penta::protocol::SIMULATION_FINGERPRINT,
            )));
        }
        // These are diagnostic provenance rather than replay gates, but a v2
        // envelope always carries them and malformed values must not pass as a
        // valid artifact.
        let _engine_version = required_json_string(envelope, "replay", "engineVersion")?;
        let _protocol_version = required_json_u32(envelope, "replay", "protocolVersion")?;
        let config = required_json_object(envelope, "replay", "config")?;
        let format = required_json_string(config, "replay.config", "format")?;
        let art_preference = config
            .get("artPreference")
            .map(|value| {
                value
                    .as_str()
                    .ok_or_else(|| js_error("replay.config.artPreference must be a string"))
            })
            .transpose()?;
        let human_deck = required_json_string(config, "replay.config", "humanDeck")?;
        let bot_deck = required_json_string(config, "replay.config", "botDeck")?;
        let bot_policy = required_json_string(config, "replay.config", "botPolicy")?;
        let human_first = required_json_bool(config, "replay.config", "humanFirst")?;
        let seed = required_json_u32(config, "replay.config", "seed")?;
        let commands = required_json_array(envelope, "replay", "commands")?;
        let mut game = Self::new(
            human_deck,
            bot_deck,
            bot_policy,
            human_first,
            seed,
            Some(format.to_owned()),
            art_preference.map(str::to_owned),
        )?;
        let total = commands.len();
        for (position, command) in commands.iter().enumerate() {
            game.apply_replay_command(command).map_err(|error| {
                js_error(format!(
                    "command {position} of {total} ({}) no longer applies: {}",
                    command["t"].as_str().unwrap_or("?"),
                    js_value_message(&error),
                ))
            })?;
        }
        Ok(game)
    }

    /// One journaled command, reapplied. The journal records only commands
    /// that took effect, so an error here means the replay does not match
    /// this engine.
    fn apply_replay_command(&mut self, command: &Value) -> Result<(), JsValue> {
        let command = command
            .as_object()
            .ok_or_else(|| js_error("command must be an object"))?;
        let tag = required_json_string(command, "command", "t")?;
        match tag {
            "act" => self.act(required_json_usize(command, "command", "index")?),
            "choose" => {
                let decision = required_json_u32(command, "command", "decision")?;
                let options = required_json_u32_array(command, "command", "options")?;
                self.choose_decision(
                    decision,
                    &serde_json::to_string(&options).map_err(js_error)?,
                )
            }
            "attackAll" => self.attack_all(),
            "cancelAttackers" => self.cancel_attackers(),
            "blocks" => {
                self.finalize_blocks(required_json_string(command, "command", "assignments")?)
            }
            "undoMana" => self.undo_mana(),
            "phaseStop" => self.set_phase_stop(
                required_json_string(command, "command", "phase")?,
                required_json_bool(command, "command", "enabled")?,
            ),
            "autopass" => self.set_autopass(required_json_bool(command, "command", "enabled")?),
            "botAct" => self.opponent_act(required_json_u32(command, "command", "index")?),
            "loseOnTime" => {
                let seat = required_json_string(command, "command", "seat")?;
                if !matches!(seat, "human" | "bot") {
                    return Err(js_error(format!(
                        "command.seat must be \"human\" or \"bot\", got {seat:?}"
                    )));
                }
                let reason = required_json_string(command, "command", "reason")?;
                self.lose_on_time_with_reason(seat, reason)
            }
            other => Err(js_error(format!("unknown journal command {other:?}"))),
        }
    }

    /// Returns the human-visible game state as JSON.
    #[must_use]
    pub fn state_json(&self) -> String {
        self.snapshot().to_string()
    }

    /// Puts a named card onto a seat's battlefield, for reaching a board state
    /// without playing toward one.
    ///
    /// Compiled only with the `dev-cheats` feature, which the production web
    /// build never enables. `seat` is `"human"` or `"bot"`.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when the seat name is unknown, no card has
    /// that name, or the game cannot take another object.
    #[cfg(feature = "dev-cheats")]
    pub fn dev_put_onto_battlefield(&mut self, seat: &str, card_name: &str) -> Result<(), JsValue> {
        let player = match seat {
            "human" => self.human,
            "bot" => self.human.opponent(),
            other => {
                return Err(js_error(format!(
                    "seat must be \"human\" or \"bot\", got {other:?}"
                )));
            }
        };
        let definition = self
            .catalog
            .find_by_name(card_name)
            .ok_or_else(|| js_error(format!("no card named {card_name:?}")))?;
        self.session
            .put_onto_battlefield(player, definition)
            .map_err(|error| js_error(error.to_string()))?;
        Ok(())
    }

    /// Puts a named card straight into a seat's graveyard, for testing zones
    /// the browser cannot otherwise reach. Compiled only with `dev-cheats`.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when the seat name is unknown, no card has
    /// that name, or the game cannot take another object.
    #[cfg(feature = "dev-cheats")]
    pub fn dev_put_into_graveyard(&mut self, seat: &str, card_name: &str) -> Result<(), JsValue> {
        let player = match seat {
            "human" => self.human,
            "bot" => self.human.opponent(),
            other => {
                return Err(js_error(format!(
                    "seat must be \"human\" or \"bot\", got {other:?}"
                )));
            }
        };
        let definition = self
            .catalog
            .find_by_name(card_name)
            .ok_or_else(|| js_error(format!("no card named {card_name:?}")))?;
        self.session
            .put_into_graveyard(player, definition)
            .map_err(|error| js_error(error.to_string()))?;
        Ok(())
    }
}

fn readable_debug(value: impl std::fmt::Debug) -> String {
    let source = format!("{value:?}");
    let mut output = String::with_capacity(source.len() + 4);
    for (index, character) in source.chars().enumerate() {
        if index > 0 && character.is_ascii_uppercase() {
            output.push(' ');
        }
        output.push(character);
    }
    output
}

fn js_error(error: impl std::fmt::Display) -> JsValue {
    #[cfg(target_arch = "wasm32")]
    {
        JsValue::from_str(&error.to_string())
    }
    // `JsValue::from_str` calls into the wasm-bindgen host and panics on a
    // native target, which native tests are. NULL is a plain constant, so an
    // error path can at least run there; the message only exists in a browser.
    #[cfg(not(target_arch = "wasm32"))]
    {
        // The message cannot ride in a native JsValue, and a silent NULL made
        // the replay harness useless for diagnosis. Stderr is the next best
        // carrier; intentional-error tests just say what they meant to.
        eprintln!("engine error: {error}");
        JsValue::NULL
    }
}

fn js_value_message(error: &JsValue) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        error.as_string().unwrap_or_default()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = error;
        String::new()
    }
}

#[cfg(test)]
mod tests;
