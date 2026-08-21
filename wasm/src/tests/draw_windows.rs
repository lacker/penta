use super::*;

fn quiet_human_action(game: &WebGame) -> usize {
    game.session
        .observe(game.human)
        .legal_actions
        .iter()
        .position(|action| {
            matches!(
                action,
                Action::PassPriority
                    | Action::FinishDeclaringAttackers
                    | Action::FinishDeclaringBlockers
                    | Action::DiscardCards { .. }
                    | Action::ChooseUntap { .. }
            )
        })
        .expect("the quiet test game has an advancing human action")
}

fn reach_next_human_draw(definition: CardDefinitionId, autopass: bool) -> WebGame {
    let mut game = WebGame::new(
        "Briksza Naya Midrange",
        "Greer G/R Aggro",
        "Handcrafted",
        true,
        2_013,
        Some("isd-dgm-standard".into()),
    )
    .expect("the standard game starts");
    act_matching(&mut game, |action| matches!(action, Action::KeepHand));

    let human = game.human;
    game.session
        .engine_mut()
        .set_hand(human, &[])
        .expect("an empty test hand is valid");
    game.session
        .engine_mut()
        .set_hand(human.opponent(), &[])
        .expect("an empty opponent hand is valid");
    game.session
        .engine_mut()
        .set_library(human, &[definition])
        .expect("the drawn card is cataloged");
    game.set_autopass(autopass)
        .expect("auto-pass setting applies");

    for _ in 0..32 {
        if game
            .session
            .observe(human)
            .hand
            .iter()
            .any(|(_, drawn)| *drawn == definition)
        {
            return game;
        }
        let index = quiet_human_action(&game);
        game.act(index).expect("quiet human action succeeds");
    }
    panic!("the game did not reach the configured draw");
}

fn external_action_index(game: &WebGame, predicate: impl Fn(&Action) -> bool) -> u32 {
    let observation = game.session.observe(game.human.opponent());
    let index = penta::protocol::protocol_actions(&observation)
        .iter()
        .position(predicate)
        .expect("the external opponent has the requested action");
    u32::try_from(index).expect("the protocol action index fits")
}

fn reach_external_opponent_miracle() -> WebGame {
    let mut game = WebGame::new(
        "Briksza Naya Midrange",
        "Greer G/R Aggro",
        "External",
        true,
        2_013,
        Some("isd-dgm-standard".into()),
    )
    .expect("the hosted standard game starts");
    act_matching(&mut game, |action| matches!(action, Action::KeepHand));
    let keep = external_action_index(&game, |action| matches!(action, Action::KeepHand));
    game.opponent_act(keep)
        .expect("the external opponent keeps its hand");

    let human = game.human;
    let opponent = human.opponent();
    game.session
        .engine_mut()
        .set_hand(human, &[])
        .expect("an empty human hand is valid");
    game.session
        .engine_mut()
        .set_hand(opponent, &[])
        .expect("an empty opponent hand is valid");
    game.session
        .engine_mut()
        .set_library(opponent, &[penta::card::cards::TERMINUS])
        .expect("Terminus is cataloged");

    for _ in 0..64 {
        if game.opponent_is_deciding() {
            let observation = game.session.observe(opponent);
            if observation.decision.as_ref().is_some_and(|decision| {
                decision.visibility == penta::DecisionVisibility::Private
                    && decision.options.iter().any(|option| {
                        option.card.is_some_and(|(_, card)| {
                            card.card_definition() == Some(penta::card::cards::TERMINUS)
                        })
                    })
            }) {
                return game;
            }
            let index = external_action_index(&game, |action| {
                matches!(
                    action,
                    Action::PassPriority
                        | Action::FinishDeclaringAttackers
                        | Action::FinishDeclaringBlockers
                        | Action::DiscardCards { .. }
                        | Action::ChooseUntap { .. }
                )
            });
            game.opponent_act(index)
                .expect("the external opponent's quiet action succeeds");
        } else {
            let index = quiet_human_action(&game);
            game.act(index).expect("the human's quiet action succeeds");
        }
    }
    panic!("the external opponent did not reach the configured Miracle draw");
}

#[allow(clippy::too_many_lines)]
fn resolve_external_wheel(first_opponent_draw: CardDefinitionId) -> WebGame {
    let mut game = WebGame::new("Sligh", "Goblins", "External", false, 7_777, None)
        .expect("the hosted old-school game starts");
    let keep = external_action_index(&game, |action| matches!(action, Action::KeepHand));
    game.opponent_act(keep)
        .expect("the external opponent keeps its hand");
    act_matching(&mut game, |action| matches!(action, Action::KeepHand));

    let human = game.human;
    let opponent = human.opponent();
    game.session
        .engine_mut()
        .set_hand(human, &[])
        .expect("an empty human hand is valid");
    game.session
        .engine_mut()
        .set_hand(opponent, &[penta::card::cards::WHEEL_OF_FORTUNE])
        .expect("Wheel of Fortune is cataloged");
    game.session
        .engine_mut()
        .set_library(human, &[penta::card::cards::PLAINS; 7])
        .expect("the human library is valid");
    let mut opponent_library = [penta::card::cards::PLAINS; 7];
    opponent_library[6] = first_opponent_draw;
    game.session
        .engine_mut()
        .set_library(opponent, &opponent_library)
        .expect("the opponent library is valid");
    for _ in 0..3 {
        game.session
            .engine_mut()
            .put_onto_battlefield(opponent, penta::card::cards::MOUNTAIN)
            .expect("Wheel's mana source is cataloged");
    }

    let mut wheel_cast = false;
    for _ in 0..96 {
        if game.opponent_is_deciding() {
            let observation = game.session.observe(opponent);
            if let Some(decision) = observation.decision.as_ref()
                && decision.visibility == penta::DecisionVisibility::Private
                && decision.options.iter().any(|option| {
                    option.card.is_some_and(|(_, card)| {
                        card.card_definition() == Some(penta::card::cards::TERMINUS)
                    })
                })
            {
                assert_eq!(first_opponent_draw, penta::card::cards::TERMINUS);
                assert_eq!(
                    (
                        observation.hand.len(),
                        observation.library_sizes[opponent.index()]
                    ),
                    (1, 6),
                    "Wheel is suspended immediately after the private first draw",
                );
                assert_eq!(
                    game.human_action_state
                        .as_ref()
                        .and_then(|state| state["opponent"]["handSize"].as_u64()),
                    Some(1),
                    "Wheel really paused after only the private first draw",
                );
                let decline = external_action_index(
                    &game,
                    |action| matches!(action, Action::ChooseDecision { options, .. } if options.is_empty()),
                );
                game.opponent_act(decline)
                    .expect("the external opponent declines Miracle");
                continue;
            }

            let actions = penta::protocol::protocol_actions(&observation);
            if !wheel_cast
                && let Some(index) = actions.iter().position(|action| {
                    matches!(action, Action::CastSpell { card, .. }
                    if observation.hand.iter().any(|(held, definition)| {
                        held == card && *definition == penta::card::cards::WHEEL_OF_FORTUNE
                    }))
                })
            {
                game.opponent_act(u32::try_from(index).expect("the cast index fits"))
                    .expect("the external opponent casts Wheel of Fortune");
                wheel_cast = true;
                continue;
            }

            if wheel_cast
                && observation.stack.is_empty()
                && observation.hand.len() == 7
                && observation.opponent_hand_size == 7
            {
                return game;
            }
            let index = external_action_index(&game, |action| {
                matches!(
                    action,
                    Action::PassPriority
                        | Action::FinishDeclaringAttackers
                        | Action::FinishDeclaringBlockers
                )
            });
            game.opponent_act(index)
                .expect("the external opponent's quiet action succeeds");
        } else {
            let index = quiet_human_action(&game);
            game.act(index).expect("the human's quiet action succeeds");
        }
    }
    panic!("Wheel of Fortune did not finish resolving");
}

fn assert_no_embedded_decision(state: &Value) {
    assert!(
        state["decision"].is_null(),
        "the live state has no empty decision"
    );
    if let Some(after) = state["afterYourAction"].as_object() {
        assert!(
            after.get("decision").is_none_or(Value::is_null),
            "the post-click snapshot has no empty decision",
        );
    }
    for beat in state["opponentActions"].as_array().into_iter().flatten() {
        assert!(
            beat["state"]["decision"].is_null(),
            "no presentation beat captures the transient empty window: {beat}",
        );
    }
}

#[test]
fn an_ordinary_draw_never_reaches_the_ui_even_with_autopass_disabled() {
    let game = reach_next_human_draw(penta::card::cards::PLAINS, false);
    let state = game.snapshot();

    assert_eq!(state["step"], "Draw", "UI auto-pass is genuinely disabled");
    assert_no_embedded_decision(&state);
    assert!(
        state["actions"]
            .as_array()
            .is_some_and(|actions| actions.iter().any(|action| action["kind"] == "pass"))
    );
}

#[test]
fn a_real_miracle_action_still_reaches_the_ui() {
    let mut game = reach_next_human_draw(penta::card::cards::TERMINUS, false);
    let state = game.snapshot();
    let decision = state["decision"]
        .as_object()
        .expect("Miracle leaves a real private decision");

    assert_eq!(state["step"], "Draw");
    assert_eq!(decision["minimum"], 0);
    assert_eq!(decision["maximum"], 1);
    assert_eq!(decision["visibility"], "Private");
    assert_eq!(decision["options"].as_array().map(Vec::len), Some(1));
    assert_eq!(decision["options"][0]["label"], "Reveal Terminus");

    let id = u32::try_from(decision["id"].as_u64().expect("decision id")).expect("id fits");
    game.choose_decision(id, "[]")
        .expect("declining Miracle with the empty selection succeeds");
    assert!(game.snapshot()["decision"].is_null());
    let replay: Value = serde_json::from_str(&game.replay_json()).expect("replay is JSON");
    let last = replay["commands"]
        .as_array()
        .and_then(|commands| commands.last());
    assert_eq!(
        last.and_then(|command| command["t"].as_str()),
        Some("choose")
    );
    assert_eq!(last.map(|command| &command["options"]), Some(&json!([])));
}

#[test]
fn an_external_opponents_private_miracle_decline_has_no_choice_beat() {
    let mut game = reach_external_opponent_miracle();
    let opponent = game.human.opponent();
    let observation = game.session.observe(opponent);
    let decision = observation
        .decision
        .as_ref()
        .expect("the opponent is considering the private Miracle action");
    assert_eq!(decision.visibility, penta::DecisionVisibility::Private);

    let decline = external_action_index(
        &game,
        |action| matches!(action, Action::ChooseDecision { options, .. } if options.is_empty()),
    );
    let beats_before = game.opponent_actions.clone();
    let commands_before = game.journal.len();
    game.opponent_act(decline)
        .expect("the external opponent can decline Miracle");

    assert_eq!(
        game.opponent_actions, beats_before,
        "an empty private choice adds no human-visible animation"
    );
    assert!(
        game.session.observe(opponent).decision.is_none(),
        "the private decision was still applied"
    );
    assert_eq!(game.journal.len(), commands_before + 1);
    assert_eq!(
        game.journal.last(),
        Some(&json!({ "t": "botAct", "index": decline })),
        "the hidden decline remains in the authoritative replay"
    );
}

#[test]
fn an_opponent_wheel_is_identical_after_an_ordinary_draw_or_declined_miracle() {
    let ordinary = resolve_external_wheel(penta::card::cards::ISLAND);
    let miracle = resolve_external_wheel(penta::card::cards::TERMINUS);
    let ordinary_state = ordinary.snapshot();
    let miracle_state = miracle.snapshot();

    assert!(
        ordinary_state["afterYourAction"].is_object(),
        "the regression includes the post-click replay frame",
    );
    assert!(
        ordinary_state["opponentActions"]
            .as_array()
            .is_some_and(|beats| beats.iter().any(|beat| beat["state"].is_object())),
        "the regression includes nested opponent-action snapshots",
    );
    assert_eq!(
        miracle_state, ordinary_state,
        "no published or nested snapshot reveals the declined Miracle",
    );
}

#[test]
fn an_ordinary_draw_adds_no_synthetic_command_and_replays() {
    let mut game = WebGame::new("Sligh", "Goblins", "Handcrafted", true, 4_242, None)
        .expect("the old-school game starts");
    act_matching(&mut game, |action| matches!(action, Action::KeepHand));
    let library_before = game.snapshot()["human"]["library"]
        .as_u64()
        .expect("library size");

    for _ in 0..12 {
        if game.snapshot()["human"]["library"].as_u64() < Some(library_before) {
            break;
        }
        let index = quiet_human_action(&game);
        game.act(index).expect("quiet human action succeeds");
    }
    let state = game.snapshot();
    assert!(state["human"]["library"].as_u64() < Some(library_before));
    assert_no_embedded_decision(&state);

    let replay: Value = serde_json::from_str(&game.replay_json()).expect("replay is JSON");
    assert!(
        replay["commands"]
            .as_array()
            .is_some_and(|commands| commands.iter().all(|command| command["t"] != "choose")),
        "the atomic empty answer is internal rather than a replay command",
    );
    let rebuilt = WebGame::from_replay_json(&replay.to_string()).expect("journal replays");
    assert_eq!(rebuilt.state_json(), game.state_json());
    assert_eq!(rebuilt.replay_json(), game.replay_json());
}
