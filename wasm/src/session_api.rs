//! A transport-neutral, exact-action view of the same match the browser renders.

use super::{BotPolicy, JsValue, PlayerId, WebGame, js_error, json, wasm_bindgen};

impl WebGame {
    pub(super) fn session_api_enabled(&self) -> bool {
        self.replay_config["sessionApi"].as_bool() == Some(true)
    }

    fn session_player(&self, role: &str) -> Result<PlayerId, JsValue> {
        match role {
            "human" => Ok(self.human),
            "bot" => Ok(self.human.opponent()),
            _ => Err(js_error("role must be human or bot")),
        }
    }
}

#[wasm_bindgen]
impl WebGame {
    /// Public setup choices from the engine's registered formats and decks.
    #[must_use]
    #[wasm_bindgen(js_name = sessionOptionsJson)]
    pub fn session_options_json() -> String {
        json!({ "apiVersion": 1, "formats": penta::Format::ALL.iter().map(|format| {
            json!({ "id": format.slug(), "name": format.display_name(),
                "decks": penta::protocol::deck_names_for_format(*format) })
        }).collect::<Vec<_>>() })
        .to_string()
    }

    /// Public catalog data for the session's format; no seat state is included.
    #[must_use]
    #[wasm_bindgen(js_name = sessionCatalogJson)]
    pub fn session_catalog_json(&self) -> String {
        penta::protocol::catalog_json_for_format(&self.catalog, self.session.format()).to_string()
    }

    /// Enables externally controlled seats before the first submitted command.
    /// Neither seat is advanced by a policy or by browser auto-pass settings.
    /// # Errors
    /// Rejects a running game or a built-in opponent.
    #[wasm_bindgen(js_name = enableSessionApi)]
    pub fn enable_session_api(&mut self) -> Result<(), JsValue> {
        if !self.journal.is_empty() || !matches!(self.bot, BotPolicy::External) {
            return Err(js_error(
                "session API requires a new externally controlled match",
            ));
        }
        self.replay_config["sessionApi"] = json!(true);
        self.autopass_enabled = false;
        Ok(())
    }

    /// The connection role holding the next decision, or none after completion.
    #[must_use]
    #[wasm_bindgen(js_name = sessionDecisionRole)]
    pub fn session_decision_role(&self) -> Option<String> {
        self.session
            .decision_seat()
            .map(|seat| if seat == self.human { "human" } else { "bot" }.to_string())
    }

    /// The canonical, redacted bot observation for either connection role.
    /// # Errors
    /// Rejects an unknown role or a game without external session control.
    #[wasm_bindgen(js_name = sessionObserveJson)]
    pub fn session_observe_json(&self, role: &str) -> Result<String, JsValue> {
        if !self.session_api_enabled() {
            return Err(js_error("this match does not use the session API"));
        }
        let seat = self.session_player(role)?;
        let observation = self.session.observe(seat);
        let actions = penta::protocol::protocol_actions(&observation);
        let mut value = penta::protocol::observation_json_for_format(
            &self.catalog,
            self.session.format(),
            &observation,
            self.session.in_pregame(),
            &actions,
        );
        value["match"] = self.session.match_json(seat);
        Ok(value.to_string())
    }

    /// Applies exactly one canonical indexed action and preserves browser beats.
    /// # Errors
    /// Rejects an invalid role, out-of-turn request, or illegal action.
    #[wasm_bindgen(js_name = sessionAct)]
    pub fn session_act(&mut self, role: &str, index: u32) -> Result<(), JsValue> {
        if !self.session_api_enabled() {
            return Err(js_error("this match does not use the session API"));
        }
        let seat = self.session_player(role)?;
        if self.session.decision_seat() != Some(seat) {
            return Err(js_error("the seat does not hold the decision"));
        }
        if seat != self.human {
            return self.opponent_act(index);
        }
        let observation = self.session.observe(seat);
        let action = penta::protocol::protocol_actions(&observation)
            .get(index as usize)
            .cloned()
            .ok_or_else(|| js_error("action index out of range"))?;
        self.apply_human_action(action)?;
        self.journal
            .push(json!({"t": "sessionAct", "role": role, "index": index}));
        Ok(())
    }
}
