use super::{
    BotPolicy, CardDefinitionId, GameResult, JsValue, PlayerId, Value, WebGame, js_error, json,
    wasm_bindgen,
};
use penta::match_play::BestOfThree;

pub(super) struct DeckLists {
    main: Vec<CardDefinitionId>,
    sideboard: Vec<CardDefinitionId>,
}

impl DeckLists {
    pub(super) fn parse(value: &Value) -> Result<Self, JsValue> {
        Ok(Self {
            main: serde_json::from_value(value["main"].clone()).map_err(js_error)?,
            sideboard: serde_json::from_value(value["sideboard"].clone()).map_err(js_error)?,
        })
    }

    pub(super) fn into_deck(self) -> penta::Deck {
        penta::Deck {
            main: self.main,
            sideboard: self.sideboard,
        }
    }
}

#[wasm_bindgen]
impl WebGame {
    /// Fix both registrations for a local first-to-two match.
    /// # Errors
    /// Rejects enabling a match after human commands or for an external bot.
    pub fn enable_match(&mut self) -> Result<(), JsValue> {
        if !self.journal.is_empty()
            || self.series.is_some()
            || matches!(self.bot, BotPolicy::External)
        {
            return Err(js_error("start a new local game to enable best of three"));
        }
        self.series = Some(BestOfThree::new(self.decks.clone(), PlayerId::One));
        Ok(())
    }

    /// Start the next game with a validated sideboard and play/draw choice.
    /// # Errors
    /// Rejects unfinished games, completed matches, illegal lists and choices.
    pub fn next_match_game(
        &mut self,
        deck_json: &str,
        human_first: bool,
        seed: u32,
    ) -> Result<(), JsValue> {
        if self.session.result().is_none() {
            return Err(js_error("finish this game before sideboarding"));
        }
        let series = self
            .completed_series()
            .ok_or_else(|| js_error("no match in progress"))?;
        if series.winner().is_some() {
            return Err(js_error("the match is finished"));
        }
        if series.chooser() == PlayerId::Two && human_first {
            return Err(js_error("the opponent chooses to play first"));
        }
        let deck = DeckLists::parse(&serde_json::from_str::<Value>(deck_json).map_err(js_error)?)?
            .into_deck();
        series
            .validate_sideboard(PlayerId::One, &deck, &self.catalog, self.session.format())
            .map_err(js_error)?;
        let decks = [deck, self.decks[1].clone()];
        let mut config = self.replay_config.clone();
        config["humanFirst"] = json!(human_first);
        config["seed"] = json!(seed);
        config["decks"] = json!(
            decks
                .iter()
                .map(|deck| json!({ "main": deck.main, "sideboard": deck.sideboard }))
                .collect::<Vec<_>>()
        );
        let mut replacement = Self::build(config)?;
        replacement.series = Some(series);
        *self = replacement;
        Ok(())
    }
}

impl WebGame {
    fn completed_series(&self) -> Option<BestOfThree> {
        let mut series = self.series.clone()?;
        if let Some(result) = self.session.result() {
            let winner = match result {
                GameResult::Draw => None,
                GameResult::Winner { winner, .. } => Some(if winner == self.human {
                    PlayerId::One
                } else {
                    PlayerId::Two
                }),
            };
            series
                .record(winner)
                .expect("current game belongs to an unfinished match");
        }
        Some(series)
    }

    pub(super) fn match_value(&self) -> Value {
        let Some(series) = self.completed_series() else {
            return Value::Null;
        };
        let card = |id: &CardDefinitionId| json!({ "id": id, "name": self.catalog.get(*id).map(|card| &card.name) });
        json!({
            "wins": series.wins(),
            "game": self.series.as_ref().map_or(1, |series| series.games() + 1),
            "finished": series.winner().is_some(),
            "humanChooses": series.chooser() == PlayerId::One,
            "main": self.decks[0].main.iter().map(card).collect::<Vec<_>>(),
            "sideboard": self.decks[0].sideboard.iter().map(card).collect::<Vec<_>>(),
        })
    }
}
