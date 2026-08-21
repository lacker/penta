use super::*;

#[test]
fn a_scripted_game_runs_to_a_result_through_json_alone() {
    let game = BotGame::new("Sligh", "The Deck", Opponent::Handcrafted, PlayerId::Two, 7)
        .expect("game starts");
    let result = finish(game, |_, observation| pass_bot(observation));
    assert!(matches!(
        result,
        GameResult::Winner { .. } | GameResult::Draw
    ));
}

#[test]
fn an_external_game_lets_one_loop_drive_both_seats() {
    let game = BotGame::new("Goblins", "Sligh", Opponent::External, PlayerId::Two, 11)
        .expect("game starts");
    let result = finish(game, |_, observation| pass_bot(observation));
    assert!(matches!(
        result,
        GameResult::Winner { .. } | GameResult::Draw
    ));
}

#[test]
fn the_same_seed_produces_the_same_bytes() {
    let make = || {
        BotGame::new("Sligh", "Goblins", Opponent::Random, PlayerId::Two, 99).expect("game starts")
    };
    let (mut first, mut second) = (make(), make());
    for _ in 0..40 {
        if first.result().is_some() {
            break;
        }
        let seat = first.decision_seat().expect("still running");
        assert_eq!(first.observe_json(seat), second.observe_json(seat));
        first.act(0).expect("index 0 is legal");
        second.act(0).expect("index 0 is legal");
    }
}

#[test]
fn a_clone_replays_identically_and_diverges_independently() {
    let mut game = BotGame::new("Sligh", "The Deck", Opponent::Handcrafted, PlayerId::Two, 7)
        .expect("game starts");
    // Reach a mid-game state with real board state on both sides.
    for _ in 0..30 {
        let seat = game.decision_seat().expect("game is still running");
        let observation: Value =
            serde_json::from_str(&game.observe_json(seat)).expect("valid JSON");
        game.act(pass_bot(&observation)).expect("legal index");
    }
    let seat = game.decision_seat().expect("game is still running");
    let mut replay = game.clone();
    assert_eq!(game.observe_json(seat), replay.observe_json(seat));

    // Determinism: the same indices drive both copies — the scripted
    // opponent's state included — to byte-identical observations.
    for _ in 0..20 {
        let seat = game.decision_seat().expect("game is still running");
        let observation: Value =
            serde_json::from_str(&game.observe_json(seat)).expect("valid JSON");
        let choice = pass_bot(&observation);
        game.act(choice).expect("legal in the original");
        replay.act(choice).expect("legal in the clone");
        assert_eq!(game.observe_json(seat), replay.observe_json(seat));
    }

    // Independence: the fork plays a different legal action than the
    // original, the two games stop matching, and the original never
    // notices. Walk to a decision with at least two options first.
    let (seat, choice, other) = loop {
        let seat = game.decision_seat().expect("game is still running");
        let observation: Value =
            serde_json::from_str(&game.observe_json(seat)).expect("valid JSON");
        let count = observation["legalActions"].as_array().expect("array").len();
        if count >= 2 {
            let choice = pass_bot(&observation);
            break (seat, choice, (choice + 1) % count);
        }
        game.act(0).expect("legal index");
    };
    let before = game.observe_json(seat);
    let mut fork = game.clone();
    fork.act(other).expect("legal in the fork");
    assert_eq!(game.observe_json(seat), before, "the original is untouched");
    game.act(choice).expect("legal in the original");
    assert_ne!(
        game.observe_json(seat),
        fork.observe_json(seat),
        "different actions, different games",
    );

    // A fork is a live game, not a snapshot: it plays on by itself.
    for _ in 0..10 {
        if fork.result().is_some() {
            break;
        }
        let seat = fork.decision_seat().expect("fork is still running");
        let observation: Value =
            serde_json::from_str(&fork.observe_json(seat)).expect("valid JSON");
        fork.act(pass_bot(&observation)).expect("legal in the fork");
    }
}

#[test]
fn observations_carry_indexed_legal_actions_and_no_hidden_cards() {
    let game = BotGame::new("Sligh", "The Deck", Opponent::Handcrafted, PlayerId::Two, 3)
        .expect("game starts");
    let seat = game.decision_seat().expect("mulligan decision");
    let observation: Value = serde_json::from_str(&game.observe_json(seat)).expect("valid JSON");
    assert_eq!(observation["seat"], "p1");
    assert_eq!(observation["pregame"], true);
    let actions = observation["legalActions"].as_array().expect("array");
    for (index, action) in actions.iter().enumerate() {
        assert_eq!(action["index"], index, "indices match positions");
        assert!(action["type"].is_string(), "every action is tagged");
    }
    // The opponent's hand is a count, never a list of cards.
    assert!(observation["opponentHandSize"].is_u64());
    assert_eq!(observation["hand"].as_array().expect("hand").len(), 7);
}

#[test]
fn an_observation_rebuilds_with_separate_hidden_hypotheses_and_fresh_rng() {
    let game = BotGame::new("Sligh", "The Deck", Opponent::External, PlayerId::Two, 73)
        .expect("game starts");
    let seat = game.decision_seat().expect("mulligan decision");
    assert_eq!(seat, PlayerId::One);
    let observation = game.observe_json(seat);
    let definitions = |json: String| {
        serde_json::from_str::<Value>(&json)
            .expect("zone JSON")
            .as_array()
            .expect("zone array")
            .iter()
            .map(|card| card["definition"].as_u64().expect("definition"))
            .collect::<Vec<_>>()
    };
    let hidden = json!({
        "hands": {
            "p2": definitions(game.hand_json(PlayerId::Two)),
        },
        "libraries": {
            "p1": definitions(game.library_json(PlayerId::One)),
            "p2": definitions(game.library_json(PlayerId::Two)),
        },
        "outsideGame": {"p1": [], "p2": []},
    });

    let before: Value = serde_json::from_str(&observation).expect("observation JSON");
    assert_eq!(before["checkpoint"]["version"], CHECKPOINT_VERSION);
    assert_eq!(
        before["checkpoint"]["simulationFingerprint"],
        SIMULATION_FINGERPRINT
    );

    let mut additive = before.clone();
    additive["futureTopLevel"] = json!({ "ignored": true });
    additive["checkpoint"]["futureBookkeeping"] = json!([1, 2, 3]);
    additive["legalActions"][0]["futureActionMetadata"] = json!({ "ignored": true });
    additive["protocolCapabilities"]
        .as_array_mut()
        .expect("capabilities array")
        .push(json!("future.optional.v9"));
    additive["engineVersion"] = json!("99.0.0-diagnostic-only");
    BotGame::from_observation_json(&additive.to_string(), &hidden.to_string(), 998)
        .expect("open objects and package provenance do not block reconstruction");

    let mut inconsistent = before.clone();
    inconsistent["activeTurn"] = json!(99);
    let public_error =
        BotGame::from_observation_json(&inconsistent.to_string(), &hidden.to_string(), 992)
            .err()
            .expect("known public state must still match the checkpoint");
    assert!(public_error.contains("public observation field: activeTurn"));

    let mut wrong_fingerprint = before.clone();
    wrong_fingerprint["simulationFingerprint"] = json!("sha256-wrong");
    let fingerprint_error =
        BotGame::from_observation_json(&wrong_fingerprint.to_string(), &hidden.to_string(), 997)
            .err()
            .expect("wrong simulation identity");
    assert!(fingerprint_error.contains("simulation fingerprint"));

    let mut wrong_protocol = before.clone();
    wrong_protocol["protocolVersion"] = json!(PROTOCOL_VERSION + 1);
    let protocol_error =
        BotGame::from_observation_json(&wrong_protocol.to_string(), &hidden.to_string(), 994)
            .err()
            .expect("wrong bot-wire epoch");
    assert!(protocol_error.contains("protocol version"));

    let mut wrong_checkpoint = before.clone();
    wrong_checkpoint["checkpoint"]["version"] = json!(CHECKPOINT_VERSION + 1);
    wrong_checkpoint["checkpoint"]
        .as_object_mut()
        .expect("checkpoint object")
        .remove("turnsStarted");
    let checkpoint_error =
        BotGame::from_observation_json(&wrong_checkpoint.to_string(), &hidden.to_string(), 996)
            .err()
            .expect("wrong checkpoint format");
    assert!(checkpoint_error.contains("checkpoint version"));

    let mut wrong_checkpoint_fingerprint = before.clone();
    wrong_checkpoint_fingerprint["checkpoint"]["simulationFingerprint"] = json!("sha256-wrong");
    let checkpoint_fingerprint_error = BotGame::from_observation_json(
        &wrong_checkpoint_fingerprint.to_string(),
        &hidden.to_string(),
        993,
    )
    .err()
    .expect("wrong checkpoint simulation identity");
    assert!(checkpoint_fingerprint_error.contains("checkpoint simulation fingerprint"));

    let mut rebuilt = BotGame::from_observation_json(&observation, &hidden.to_string(), 999)
        .expect("checkpoint reconstructs");
    let after: Value = serde_json::from_str(&rebuilt.observe_json(seat)).expect("rebuilt JSON");
    assert_eq!(
        after, before,
        "the complete public observation is preserved"
    );
    assert_eq!(after["hand"], before["hand"], "public ids are preserved");
    assert!(after.get("seed").is_none());
    assert!(after["checkpoint"].get("rng").is_none());

    rebuilt.act(0).expect("the reconstructed game is live");
    assert_eq!(rebuilt.decision_seat(), Some(PlayerId::Two));
}

#[test]
fn an_ordinary_spell_on_the_stack_rebuilds_as_a_response_window() {
    let mut game = BotGame::new("Sligh", "The Deck", Opponent::External, PlayerId::Two, 7)
        .expect("game starts");
    let (viewer, observation) = loop {
        let viewer = game.decision_seat().expect("game continues");
        let observation: Value =
            serde_json::from_str(&game.observe_json(viewer)).expect("observation JSON");
        let actions = observation["legalActions"].as_array().expect("actions");
        if !observation["stack"].as_array().expect("stack").is_empty() {
            break (viewer, observation);
        }
        let preferred = ["KeepHand", "PlayLand", "CastSpell", "PassPriority"];
        let index = preferred
            .iter()
            .find_map(|kind| {
                actions
                    .iter()
                    .position(|action| action["type"].as_str() == Some(kind))
            })
            .unwrap_or_else(|| pass_bot(&observation));
        game.act(index).expect("selected action is legal");
    };
    assert_eq!(observation["stack"][0]["kind"], "Spell");

    let zone_definitions = |json: String| {
        serde_json::from_str::<Value>(&json)
            .expect("zone JSON")
            .as_array()
            .expect("zone array")
            .iter()
            .map(|card| card["definition"].as_u64().expect("definition"))
            .collect::<Vec<_>>()
    };
    let opponent_key = if viewer.opponent() == PlayerId::One {
        "p1"
    } else {
        "p2"
    };
    let hidden = json!({
        "hands": {
            (opponent_key): zone_definitions(game.hand_json(viewer.opponent())),
        },
        "libraries": {
            "p1": zone_definitions(game.library_json(PlayerId::One)),
            "p2": zone_definitions(game.library_json(PlayerId::Two)),
        },
        "outsideGame": {"p1": [], "p2": []},
    });
    let rebuilt =
        BotGame::from_observation_json(&observation.to_string(), &hidden.to_string(), 123)
            .expect("response window reconstructs");
    let rebuilt_observation: Value =
        serde_json::from_str(&rebuilt.observe_json(viewer)).expect("rebuilt observation");
    assert_eq!(rebuilt_observation["stack"], observation["stack"]);
    assert_eq!(
        rebuilt_observation["legalActions"],
        observation["legalActions"]
    );
}

#[test]
fn an_activated_ability_on_the_stack_rebuilds_as_a_response_window() {
    let mut game = BotGame::new(
        "The Deck",
        "The Deck",
        Opponent::External,
        PlayerId::Two,
        29,
    )
    .expect("game starts");
    let (viewer, observation) = (0..1_200)
        .find_map(|_| {
            let viewer = game.decision_seat().expect("game continues");
            let observation: Value =
                serde_json::from_str(&game.observe_json(viewer)).expect("observation JSON");
            if observation["stack"].as_array().is_some_and(|stack| {
                stack
                    .iter()
                    .any(|object| object["kind"] == "ActivatedAbility")
            }) {
                return Some((viewer, observation));
            }
            let actions = observation["legalActions"].as_array().expect("actions");
            let factory_activation = actions.iter().position(|action| {
                action["type"] == "ActivateAbility"
                    && observation["battlefield"]
                        .as_array()
                        .is_some_and(|battlefield| {
                            battlefield.iter().any(|permanent| {
                                permanent["objectId"] == action["source"]
                                    && permanent["name"] == "Mishra's Factory"
                            })
                        })
            });
            let index = factory_activation.unwrap_or_else(|| advancing_action(&observation));
            game.act(index).expect("selected action is legal");
            None
        })
        .expect("a Factory activation reaches the stack");

    let hidden = hidden_hypothesis(&game, viewer);
    let rebuilt =
        BotGame::from_observation_json(&observation.to_string(), &hidden.to_string(), 1_001)
            .expect("activated response window reconstructs");
    let rebuilt_observation: Value =
        serde_json::from_str(&rebuilt.observe_json(viewer)).expect("rebuilt observation");
    assert_eq!(rebuilt_observation["stack"], observation["stack"]);
    assert_eq!(
        rebuilt_observation["legalActions"],
        observation["legalActions"]
    );
}

#[test]
fn a_triggered_ability_on_the_stack_rebuilds_with_its_event_context() {
    let mut game = BotGame::new(
        "The Deck",
        "The Deck",
        Opponent::External,
        PlayerId::Two,
        31,
    )
    .expect("game starts");
    let (viewer, observation) = (0..1_200)
        .find_map(|_| {
            let viewer = game.decision_seat().expect("game continues");
            let observation: Value =
                serde_json::from_str(&game.observe_json(viewer)).expect("observation JSON");
            if observation["stack"].as_array().is_some_and(|stack| {
                stack
                    .iter()
                    .any(|object| object["kind"] == "TriggeredAbility")
            }) {
                return Some((viewer, observation));
            }
            game.act(advancing_action(&observation))
                .expect("selected action is legal");
            None
        })
        .expect("a triggered ability reaches the stack");

    let hidden = hidden_hypothesis(&game, viewer);
    let rebuilt =
        BotGame::from_observation_json(&observation.to_string(), &hidden.to_string(), 1_003)
            .expect("triggered response window reconstructs");
    let rebuilt_observation: Value =
        serde_json::from_str(&rebuilt.observe_json(viewer)).expect("rebuilt observation");
    assert_eq!(rebuilt_observation["stack"], observation["stack"]);
    assert_eq!(
        rebuilt_observation["legalActions"],
        observation["legalActions"]
    );
}

fn advancing_action(observation: &Value) -> usize {
    let actions = observation["legalActions"].as_array().expect("actions");
    [
        "KeepHand",
        "PlayLand",
        "CastSpell",
        "DeclareAttacker",
        "FinishDeclaringAttackers",
        "DeclareBlocker",
        "FinishDeclaringBlockers",
        "PassPriority",
    ]
    .iter()
    .find_map(|kind| {
        actions
            .iter()
            .position(|action| action["type"].as_str() == Some(kind))
    })
    .unwrap_or_else(|| pass_bot(observation))
}

fn hidden_hypothesis(game: &BotGame, viewer: PlayerId) -> Value {
    let definitions = |json: String| {
        serde_json::from_str::<Value>(&json)
            .expect("zone JSON")
            .as_array()
            .expect("zone array")
            .iter()
            .map(|card| card["definition"].as_u64().expect("definition"))
            .collect::<Vec<_>>()
    };
    let opponent_key = if viewer.opponent() == PlayerId::One {
        "p1"
    } else {
        "p2"
    };
    json!({
        "hands": {
            (opponent_key): definitions(game.hand_json(viewer.opponent())),
        },
        "libraries": {
            "p1": definitions(game.library_json(PlayerId::One)),
            "p2": definitions(game.library_json(PlayerId::Two)),
        },
        "outsideGame": {"p1": [], "p2": []},
    })
}

#[test]
#[allow(clippy::too_many_lines)]
fn protocol_reincarnates_public_object_identity_across_cast_zones() {
    let mut game = BotGame::new("Goblins", "Goblins", Opponent::External, PlayerId::Two, 0)
        .expect("game starts");

    let (casting_seat, hand_id, definition_id) = (0..600)
        .find_map(|_| {
            let seat = game.decision_seat().expect("game has not ended");
            let observation: Value =
                serde_json::from_str(&game.observe_json(seat)).expect("valid observation JSON");
            assert_no_physical_lineage_keys(&observation);
            let actions = observation["legalActions"].as_array().expect("actions");

            let permanent_cast = actions.iter().find_map(|action| {
                if action["type"] != "CastSpell" {
                    return None;
                }
                let hand_raw = action["card"].as_u64()?;
                let hand_card = observation["hand"]
                    .as_array()
                    .expect("hand")
                    .iter()
                    .find(|card| card["objectId"].as_u64() == Some(hand_raw))?;
                let definition_raw = hand_card["definition"].as_u64()?;
                let definition = crate::CardDefinitionId(u16::try_from(definition_raw).ok()?);
                if !game
                    .catalog
                    .get(definition)
                    .is_some_and(|card| card.rules.types().is_permanent())
                {
                    return None;
                }
                Some((
                    usize::try_from(action["index"].as_u64()?).ok()?,
                    GameObjectId(u32::try_from(hand_raw).ok()?),
                    definition,
                ))
            });
            if let Some((index, hand_id, definition)) = permanent_cast {
                let hand_card = observation["hand"]
                    .as_array()
                    .expect("hand")
                    .iter()
                    .find(|card| card["objectId"].as_u64() == Some(u64::from(hand_id.0)))
                    .expect("cast card was public in hand");
                assert_eq!(hand_card["instance"], hand_id.0);
                game.act(index).expect("cast action is legal");
                return Some((seat, hand_id, definition));
            }

            let find_action = |kind: &str| {
                actions
                    .iter()
                    .find(|action| action["type"] == kind)
                    .and_then(|action| action["index"].as_u64())
                    .and_then(|index| usize::try_from(index).ok())
            };
            let main_phase = matches!(
                observation["step"].as_str(),
                Some("PrecombatMain" | "PostcombatMain")
            );
            let mut selected = find_action("KeepHand");
            if selected.is_none() && main_phase {
                selected = find_action("PlayLand");
            }
            if selected.is_none() && main_phase {
                selected = actions
                    .iter()
                    .find(|action| {
                        action["type"] == "ActivateManaAbility" && action["color"] == "red"
                    })
                    .or_else(|| {
                        actions
                            .iter()
                            .find(|action| action["type"] == "ActivateManaAbility")
                    })
                    .and_then(|action| action["index"].as_u64())
                    .and_then(|index| usize::try_from(index).ok());
            }
            for kind in [
                "BottomCards",
                "DiscardCards",
                "ChooseDecision",
                "ChooseUntap",
                "FinishDeclaringAttackers",
                "FinishDeclaringBlockers",
                "AssignCombatDamage",
                "PassPriority",
            ] {
                if selected.is_none() {
                    selected = find_action(kind);
                }
            }
            game.act(selected.expect("the protocol always offers progress"))
                .expect("selected protocol action is legal");
            None
        })
        .expect("the seeded game reaches a castable permanent");

    let stack_observation: Value =
        serde_json::from_str(&game.observe_json(casting_seat)).expect("valid stack observation");
    assert_no_physical_lineage_keys(&stack_observation);
    assert!(
        stack_observation["hand"]
            .as_array()
            .expect("hand")
            .iter()
            .all(|card| card["objectId"].as_u64() != Some(u64::from(hand_id.0))),
        "the hand object ceased to exist when the card changed zones"
    );
    let spell = stack_observation["stack"]
        .as_array()
        .expect("stack")
        .iter()
        .find(|object| object["definition"].as_u64() == Some(u64::from(definition_id.0)))
        .expect("cast spell is public on the stack");
    assert_eq!(spell["kind"], "Spell");
    assert!(spell["sourceObjectId"].is_null());
    assert!(spell["source"].is_null());
    assert!(spell["signature"].is_object());
    let spell_id = GameObjectId(
        u32::try_from(spell["objectId"].as_u64().expect("stack object ID")).expect("ID fits"),
    );
    assert_ne!(spell_id, hand_id);

    for _ in 0..2 {
        let seat = game.decision_seat().expect("priority decision");
        let observation: Value =
            serde_json::from_str(&game.observe_json(seat)).expect("valid priority observation");
        assert_no_physical_lineage_keys(&observation);
        let pass = observation["legalActions"]
            .as_array()
            .expect("actions")
            .iter()
            .find(|action| action["type"] == "PassPriority")
            .and_then(|action| action["index"].as_u64())
            .and_then(|index| usize::try_from(index).ok())
            .expect("priority can be passed");
        game.act(pass).expect("priority pass is legal");
    }

    let battlefield_observation: Value = serde_json::from_str(&game.observe_json(casting_seat))
        .expect("valid battlefield observation");
    assert_no_physical_lineage_keys(&battlefield_observation);
    let permanent = battlefield_observation["battlefield"]
        .as_array()
        .expect("battlefield")
        .iter()
        .find(|object| object["definition"].as_u64() == Some(u64::from(definition_id.0)))
        .expect("resolved permanent is public on the battlefield");
    let permanent_id = GameObjectId(
        u32::try_from(permanent["objectId"].as_u64().expect("permanent object ID"))
            .expect("ID fits"),
    );
    assert_ne!(permanent_id, hand_id);
    assert_ne!(permanent_id, spell_id);
    assert_eq!(permanent["instance"], permanent_id.0);
    assert!(
        battlefield_observation["stack"]
            .as_array()
            .expect("stack")
            .iter()
            .all(|object| object["objectId"].as_u64() != Some(u64::from(spell_id.0))),
        "the stack object ceased to exist when the permanent was created"
    );
}

#[test]
#[ignore = "slow simulation sweep"]
fn bots_are_never_offered_the_chance_to_resign() {
    // Conceding is legal in every state and strictly dominated, so it is
    // not in the bot's list at all. That makes picking blindly — index
    // zero, or uniform random — a weak bot rather than an instant loss,
    // which is what a random baseline has to be worth measuring against.
    let decks = deck_names();
    let mut observations = 0_u32;
    let mut rng = 12_345_u64;
    for index in 0..12 {
        let mut game = BotGame::new(
            decks[index % decks.len()],
            decks[(index * 7 + 3) % decks.len()],
            Opponent::External,
            PlayerId::Two,
            index as u64 * 101,
        )
        .expect("game starts");
        for _ in 0..1_500 {
            if game.result().is_some() {
                break;
            }
            let seat = game.decision_seat().expect("still running");
            let observation: Value =
                serde_json::from_str(&game.observe_json(seat)).expect("valid JSON");
            let actions = observation["legalActions"].as_array().expect("array");
            assert!(
                !actions.is_empty(),
                "removing Concede never empties the list",
            );
            for action in actions {
                assert_ne!(action["type"], "Concede", "no way to resign by index");
            }
            // Uniform random over the whole list, the baseline a new bot
            // author measures against.
            rng = rng.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            let pick = usize::try_from(rng >> 33).unwrap_or(0) % actions.len();
            game.act(pick).expect("legal index");
            observations += 1;
        }
    }
    assert!(
        observations > 2_000,
        "played enough to be meaningful, saw {observations}",
    );
}

#[test]
fn decisions_reach_bots_as_concrete_indexed_actions() {
    // The engine exposes one empty-options template and expects an ordinary
    // caller to fill it from the decision schema. Bots can act by index alone,
    // so the protocol expands that template into one legal action per option.
    let game = BotGame::new("Sligh", "The Deck", Opponent::External, PlayerId::Two, 0)
        .expect("game starts");
    let seat = game.decision_seat().expect("mulligan decision");
    let mut observation = game.game.observe(seat);
    let decision_id = 41;
    observation.decision = Some(DecisionObservation {
        id: decision_id,
        player: seat,
        kind: DecisionKind::Choice,
        order_semantics: None,
        prompt: "Choose one".into(),
        visibility: crate::game::DecisionVisibility::Public,
        preference: crate::game::DecisionPreference::Neutral,
        minimum: 1,
        maximum: 1,
        cancellable: false,
        options: vec![
            crate::game::DecisionOption {
                id: 7,
                label: "First".into(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: crate::game::DecisionZone::None,
            },
            crate::game::DecisionOption {
                id: 9,
                label: "Second".into(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: crate::game::DecisionZone::None,
            },
        ],
    });
    observation.legal_actions = vec![Action::ChooseDecision {
        decision: decision_id,
        options: Vec::new(),
    }];

    let actions = protocol_actions(&observation);
    assert_eq!(
        actions,
        vec![
            Action::ChooseDecision {
                decision: decision_id,
                options: vec![7],
            },
            Action::ChooseDecision {
                decision: decision_id,
                options: vec![9],
            },
        ],
    );

    let wire = observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.game.in_pregame(),
        &actions,
    );
    let legal = wire["legalActions"].as_array().expect("legal action array");
    assert_eq!(legal[0]["index"], 0);
    assert_eq!(legal[0]["type"], "ChooseDecision");
    assert_eq!(legal[0]["options"], json!([7]));
    assert_eq!(legal[1]["index"], 1);
    assert_eq!(legal[1]["type"], "ChooseDecision");
    assert_eq!(legal[1]["options"], json!([9]));
}

#[test]
fn optional_single_choices_expose_decline_and_each_acceptance_by_index() {
    let game = BotGame::new("Sligh", "The Deck", Opponent::External, PlayerId::Two, 0)
        .expect("game starts");
    let seat = game.decision_seat().expect("mulligan decision");
    let mut observation = game.game.observe(seat);
    let decision_id = 42;
    observation.decision = Some(DecisionObservation {
        id: decision_id,
        player: seat,
        kind: DecisionKind::Choice,
        order_semantics: None,
        prompt: "Take up to one action".into(),
        visibility: crate::game::DecisionVisibility::Private,
        preference: crate::game::DecisionPreference::Neutral,
        minimum: 0,
        maximum: 1,
        cancellable: false,
        options: vec![crate::game::DecisionOption {
            id: 7,
            label: "Reveal".into(),
            card: None,
            members: Vec::new(),
            ability_text: None,
            zone: crate::game::DecisionZone::None,
        }],
    });
    observation.legal_actions = vec![Action::ChooseDecision {
        decision: decision_id,
        options: Vec::new(),
    }];

    assert_eq!(
        protocol_actions(&observation),
        vec![
            Action::ChooseDecision {
                decision: decision_id,
                options: Vec::new(),
            },
            Action::ChooseDecision {
                decision: decision_id,
                options: vec![7],
            },
        ]
    );
}
