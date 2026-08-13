//! The engine as a host runs it, rather than as a browser plays it.
//!
//! [`WebGame`](crate::WebGame) is a seat's client: it owns presentation, undo,
//! and beats, and it assumes the engine is local. A host has the opposite job.
//! It owns the engine, drives both seats, and hands each of them only what
//! `observe_json` allows. That is the shape a Cloudflare Durable Object needs,
//! and it is the shape a bot on the other end of a socket already speaks.
//!
//! A game is worth a few hundred bytes at rest: the format, the two decks, the
//! seed, and the action indices taken so far. [`HostedGame::replay`] rebuilds
//! from exactly that, because the engine is deterministic -- see
//! `the_same_seed_produces_the_same_bytes` in the protocol tests. Nothing here
//! stores engine state, so nothing here has to migrate it.

use penta::PlayerId;
use penta::protocol::{
    BotGame, ENGINE_VERSION, LEGACY_UNDECLARED_PROTOCOL_VERSION, Opponent, PROTOCOL_CAPABILITIES,
    PROTOCOL_VERSION, REQUIRED_BOT_CAPABILITIES, SIMULATION_FINGERPRINT, parse_format_slug,
};
use wasm_bindgen::prelude::*;

fn js_error(message: impl Into<String>) -> JsValue {
    JsValue::from_str(&message.into())
}

fn seat_name(seat: PlayerId) -> &'static str {
    if seat == PlayerId::One { "p1" } else { "p2" }
}

/// One game, owned by whoever is hosting it.
#[wasm_bindgen]
pub struct HostedGame {
    game: BotGame,
    /// Every action index applied, in order. With the seed and the decks this
    /// is the whole game, which is what gets written down.
    history: Vec<u32>,
}

#[wasm_bindgen]
impl HostedGame {
    /// Starts a game the host drives on both sides.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error for an unknown format or a deck that does
    /// not belong to it.
    #[wasm_bindgen(constructor)]
    pub fn new(
        format: &str,
        p1_deck: &str,
        p2_deck: &str,
        seed: &str,
    ) -> Result<HostedGame, JsValue> {
        let format = parse_format_slug(format).map_err(js_error)?;
        // The seed arrives as text, not a number. A host writes it down as
        // JSON to rebuild the game later, and a JS number cannot hold the
        // whole u64 range without rounding -- a seed that changes in storage
        // is a game that cannot be replayed.
        let seed: u64 = seed
            .parse()
            .map_err(|_| js_error(format!("seed must be a whole number, got {seed:?}")))?;
        let game = BotGame::new_with_format(
            format,
            p1_deck,
            p2_deck,
            Opponent::External,
            PlayerId::Two,
            seed,
        )
        .map_err(js_error)?;
        Ok(Self {
            game,
            history: Vec::new(),
        })
    }

    /// Rebuilds a game from what was written down. The actions are replayed
    /// through the ordinary path, so an index that is no longer legal fails
    /// here rather than producing a quietly different game.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when the game cannot be started or an action
    /// no longer applies under the selected simulation.
    pub fn replay(
        format: &str,
        p1_deck: &str,
        p2_deck: &str,
        seed: &str,
        history: &[u32],
    ) -> Result<HostedGame, JsValue> {
        let mut game = Self::new(format, p1_deck, p2_deck, seed)?;
        for (position, index) in history.iter().enumerate() {
            game.act(*index).map_err(|error| {
                js_error(format!(
                    "action {position} of {} no longer applies: {}",
                    history.len(),
                    error.as_string().unwrap_or_default()
                ))
            })?;
        }
        Ok(game)
    }

    /// Which seat the engine is waiting on, or nothing once the game is over.
    #[wasm_bindgen(js_name = decisionSeat)]
    #[must_use]
    pub fn decision_seat(&self) -> Option<String> {
        self.game
            .decision_seat()
            .map(|seat| seat_name(seat).to_owned())
    }

    /// What one seat may see. This is the redacted view; there is no
    /// unredacted one on this type.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error for a seat name other than `p1` or `p2`.
    #[wasm_bindgen(js_name = observeJson)]
    pub fn observe_json(&self, seat: &str) -> Result<String, JsValue> {
        let seat = match seat {
            "p1" => PlayerId::One,
            "p2" => PlayerId::Two,
            other => return Err(js_error(format!("unknown seat {other:?}"))),
        };
        Ok(self.game.observe_json(seat))
    }

    /// Applies one action by its index in the deciding seat's `legalActions`.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when the index is out of range or the game
    /// has already finished.
    pub fn act(&mut self, index: u32) -> Result<(), JsValue> {
        self.game.act(index as usize).map_err(js_error)?;
        self.history.push(index);
        Ok(())
    }

    /// Every action so far, which with the seed and decks is the whole game.
    #[wasm_bindgen(js_name = historyJson)]
    #[must_use]
    pub fn history_json(&self) -> String {
        serde_json::to_string(&self.history).unwrap_or_else(|_| "[]".to_owned())
    }

    /// The outcome once there is one.
    #[wasm_bindgen(js_name = resultJson)]
    #[must_use]
    pub fn result_json(&self) -> Option<String> {
        self.game.result().map(|result| {
            serde_json::json!({
                "winner": match result {
                    penta::GameResult::Winner { winner, .. } => Some(seat_name(winner)),
                    penta::GameResult::Draw => None,
                },
                "draw": matches!(result, penta::GameResult::Draw),
            })
            .to_string()
        })
    }

    /// Package-release provenance retained alongside stored state.
    #[wasm_bindgen(js_name = engineVersion)]
    #[must_use]
    pub fn engine_version() -> String {
        ENGINE_VERSION.to_owned()
    }

    #[wasm_bindgen(js_name = protocolVersion)]
    #[must_use]
    pub fn protocol_version() -> u32 {
        PROTOCOL_VERSION
    }

    /// Conservative identity used to guard deterministic replays and stored games.
    #[wasm_bindgen(js_name = simulationFingerprint)]
    #[must_use]
    pub fn simulation_fingerprint() -> String {
        SIMULATION_FINGERPRINT.to_owned()
    }

    /// The command-journal envelope version, independent of the bot wire.
    #[wasm_bindgen(js_name = replayVersion)]
    #[must_use]
    pub fn replay_version() -> u32 {
        crate::REPLAY_VERSION
    }

    /// The authoritative compatibility manifest for hosted bot negotiation.
    #[wasm_bindgen(js_name = botCompatibilityJson)]
    #[must_use]
    pub fn bot_compatibility_json() -> String {
        serde_json::json!({
            "protocolVersion": PROTOCOL_VERSION,
            "capabilities": PROTOCOL_CAPABILITIES,
            "requiredCapabilities": REQUIRED_BOT_CAPABILITIES,
            "simulationFingerprint": SIMULATION_FINGERPRINT,
            "legacyUndeclaredProtocolVersion": LEGACY_UNDECLARED_PROTOCOL_VERSION,
        })
        .to_string()
    }
}
