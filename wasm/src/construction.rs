use super::{
    BotPolicy, Format, Game, HandcraftedPolicy, JsValue, LocalSession, PlayerId, RandomPolicy,
    Value, WebGame, art_preference_slug, card, deck_by_name, js_error, json, parse_art_preference,
    wasm_bindgen,
};

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
    ) -> Result<WebGame, JsValue> {
        Self::new_with_art_preference(
            human_deck,
            bot_deck,
            bot_policy,
            human_first,
            seed,
            format,
            None,
        )
    }

    /// Creates a game with an explicit card-art selection policy.
    ///
    /// The ordinary constructor remains compatible with older browser and
    /// host callers and defaults to debut art.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when a deck, policy, format, or art
    /// preference is unknown, or game construction fails.
    #[allow(clippy::needless_pass_by_value)] // wasm-bindgen owns optional strings at the ABI.
    #[wasm_bindgen(js_name = withArtPreference)]
    pub fn new_with_art_preference(
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
        Self::build(replay_config)
    }
}

impl WebGame {
    pub(super) fn build(mut replay_config: Value) -> Result<Self, JsValue> {
        let format = penta::protocol::parse_format_slug(
            replay_config["format"]
                .as_str()
                .ok_or_else(|| js_error("missing format"))?,
        )
        .map_err(js_error)?;
        let art_preference = parse_art_preference(
            replay_config
                .get("artPreference")
                .map(|value| {
                    value
                        .as_str()
                        .ok_or_else(|| js_error("artPreference must be a string"))
                })
                .transpose()?,
        )?;
        replay_config["artPreference"] = json!(art_preference_slug(art_preference));
        let human_deck = replay_config["humanDeck"]
            .as_str()
            .ok_or_else(|| js_error("missing human deck"))?;
        let bot_deck = replay_config["botDeck"]
            .as_str()
            .ok_or_else(|| js_error("missing bot deck"))?;
        let bot_policy = replay_config["botPolicy"]
            .as_str()
            .ok_or_else(|| js_error("missing policy"))?;
        let human_first = replay_config["humanFirst"]
            .as_bool()
            .ok_or_else(|| js_error("missing starting player"))?;
        let seed = replay_config["seed"]
            .as_u64()
            .and_then(|seed| u32::try_from(seed).ok())
            .ok_or_else(|| js_error("invalid seed"))?;
        let catalog = card::catalog().map_err(js_error)?;
        let human_deck = deck_by_name(format, human_deck)?;
        let bot_deck = deck_by_name(format, bot_deck)?;
        let human = if human_first {
            PlayerId::One
        } else {
            PlayerId::Two
        };
        let ordered_decks = match human {
            PlayerId::One => [human_deck, bot_deck],
            PlayerId::Two => [bot_deck, human_deck],
        };
        let mut game =
            Game::new_with_format(format, catalog.clone(), ordered_decks, u64::from(seed))
                .map_err(js_error)?;
        let mode = penta::match_play::MatchMode::parse(
            replay_config
                .get("matchMode")
                .map_or(Ok("one-conclusion"), |value| {
                    value.as_str().ok_or("matchMode must be a string")
                })
                .map_err(js_error)?,
        )
        .map_err(js_error)?;
        game.set_match_mode(mode).map_err(js_error)?;
        let bot = match bot_policy.to_ascii_lowercase().as_str() {
            "random" => BotPolicy::Random(RandomPolicy::new(u64::from(seed) ^ 0x00b0_7b07)),
            "handcrafted" => BotPolicy::Handcrafted(HandcraftedPolicy::new(catalog.clone())),
            "external" => BotPolicy::External,
            _ => return Err(JsValue::from_str("unknown bot policy")),
        };
        let session_api = replay_config.get("sessionApi").map_or(Ok(false), |value| {
            value
                .as_bool()
                .ok_or_else(|| js_error("sessionApi must be boolean"))
        })?;
        if session_api && !matches!(bot, BotPolicy::External) {
            return Err(js_error("sessionApi requires an external opponent"));
        }
        let autopass_enabled = !session_api;
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
            autopass_enabled,
            attack_undo: None,
            // The opening turn arrives with the board, not as a change to it.
            announced_turn: Some(1),
            human_action_state: None,
            timeout_reason: None,
        };
        if session_api {
            web_game.session.track_updates();
        }
        web_game.advance_until_human_choice()?;
        Ok(web_game)
    }
}
