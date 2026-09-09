use super::{JsValue, Value, WebGame, js_error, json, wasm_bindgen};

#[wasm_bindgen]
impl WebGame {
    /// Public match stage, safe even while the other seat chooses privately.
    #[wasm_bindgen(js_name = matchStage)]
    #[must_use]
    pub fn match_stage(&self) -> String {
        self.session.match_json(self.human)["stage"]
            .as_str()
            .unwrap_or("playing")
            .to_owned()
    }

    /// Starts a first-to-two-wins match before the human submits any commands.
    /// # Errors
    /// Rejects mode changes after commands have been submitted.
    pub fn enable_match(&mut self) -> Result<(), JsValue> {
        if !self.journal.is_empty() {
            return Err(js_error("select match mode before the first action"));
        }
        let mut config = self.replay_config.clone();
        config["matchMode"] = json!("first-to-two-wins");
        *self = Self::build(config)?;
        Ok(())
    }
}

impl WebGame {
    pub(super) fn match_value(&self) -> Value {
        let mut value = self.session.match_json(self.human);
        let wins = value["wins"].as_array().expect("two match scores").clone();
        value["wins"] = json!([
            wins[self.human.index()],
            wins[self.human.opponent().index()]
        ]);
        value["finished"] = json!(value["stage"] == "complete");
        value["canChoose"] = json!(self.session.decision_seat() == Some(self.human));
        let card = |id: &Value| json!({ "id": id, "name": id.as_u64().and_then(penta::CardDefinitionId::try_new).and_then(|id| self.catalog.get(id)).map(|card| &card.name) });
        for zone in ["main", "sideboard"] {
            value[zone] = json!(
                value[zone]
                    .as_array()
                    .map(|ids| ids.iter().map(card).collect::<Vec<_>>())
                    .unwrap_or_default()
            );
        }
        value
    }
}
