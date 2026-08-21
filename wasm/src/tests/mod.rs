use super::*;
use crate::hosted::HostedGame;

mod actions;
mod autopass;
mod draw_windows;
mod snapshots;

fn assert_nested_card_art(card: &Value) {
    assert!(card.get("scryfallId").is_none());
    assert!(card.get("artist").is_none());

    let art = card["art"].as_object().expect("card art is an object");
    assert_eq!(art.len(), 2);
    assert!(art["scryfallId"].as_str().is_some_and(|id| id.len() == 36));
    assert!(
        art["artist"]
            .as_str()
            .is_some_and(|artist| !artist.is_empty())
    );
}

#[test]
fn hosted_exports_one_authoritative_compatibility_manifest() {
    let manifest: Value = serde_json::from_str(&HostedGame::bot_compatibility_json())
        .expect("hosted compatibility manifest is JSON");
    assert_eq!(
        manifest["protocolVersion"],
        penta::protocol::PROTOCOL_VERSION
    );
    assert_eq!(
        manifest["capabilities"],
        serde_json::json!(penta::protocol::PROTOCOL_CAPABILITIES)
    );
    assert_eq!(
        manifest["requiredCapabilities"],
        serde_json::json!(penta::protocol::REQUIRED_BOT_CAPABILITIES)
    );
    assert_eq!(
        manifest["simulationFingerprint"],
        penta::protocol::SIMULATION_FINGERPRINT
    );
    assert_eq!(
        manifest["legacyUndeclaredProtocolVersion"],
        penta::protocol::LEGACY_UNDECLARED_PROTOCOL_VERSION
    );
    assert_eq!(
        HostedGame::simulation_fingerprint(),
        penta::protocol::SIMULATION_FINGERPRINT
    );
    assert_eq!(HostedGame::replay_version(), REPLAY_VERSION);
}

fn act_matching(game: &mut WebGame, predicate: impl Fn(&Action) -> bool) {
    let action_index = game
        .session
        .observe(game.human)
        .legal_actions
        .iter()
        .position(predicate)
        .expect("matching legal action");
    game.act(action_index).expect("legal action succeeds");
}

fn apply_engine_action(game: &mut Game, predicate: impl Fn(&Action) -> bool) {
    let player = game.decision_player().expect("game has a decision player");
    let action = game
        .observe(player)
        .legal_actions
        .into_iter()
        .find(predicate)
        .expect("matching engine action");
    game.apply(player, action).expect("engine action succeeds");
}

fn advance_engine_quietly_until(game: &mut Game, stop: impl Fn(&PlayerObservation) -> bool) {
    for _ in 0..200 {
        let player = game.decision_player().expect("game remains in progress");
        let observation = game.observe(player);
        if stop(&observation) {
            return;
        }
        let action = observation
            .legal_actions
            .into_iter()
            .find(|action| {
                matches!(
                    action,
                    Action::PassPriority
                        | Action::FinishDeclaringAttackers
                        | Action::FinishDeclaringBlockers
                        | Action::DiscardCards { .. }
                        | Action::ChooseUntap { .. }
                )
            })
            .expect("a quiet action advances the test game");
        game.apply(player, action).expect("quiet action succeeds");
    }
    panic!("test game did not reach the requested state");
}

fn choices_targeting(target: Target) -> penta::CastChoices {
    penta::CastChoices::default().with_targets(vec![penta::TargetSelection::single(
        penta::TargetSlotId(0),
        target,
    )])
}

mod external_opponent {
    use super::*;

    fn hosted_external() -> WebGame {
        WebGame::new("Sligh", "Goblins", "External", true, 77, None).expect("game starts")
    }

    fn parsed(json: &str) -> serde_json::Value {
        serde_json::from_str(json).expect("observation is JSON")
    }

    /// Picks the first action the driver would consider a real play, the way
    /// the socket bots in the protocol tests do.
    fn driver_index(observation: &serde_json::Value) -> u32 {
        let actions = observation["legalActions"]
            .as_array()
            .expect("legalActions is an array");
        assert!(!actions.is_empty(), "the driver has something to do");
        u32::try_from(
            actions
                .iter()
                .position(|action| {
                    !matches!(action["type"].as_str(), Some("PassPriority" | "Concede"))
                })
                .unwrap_or(0),
        )
        .expect("index fits")
    }

    #[test]
    fn an_external_game_waits_for_the_driver_instead_of_inventing_a_policy() {
        let mut game = hosted_external();
        // The human settles their hand first.
        game.act(
            parsed(&game.state_json())["actions"]
                .as_array()
                .unwrap()
                .iter()
                .position(|action| action["label"] == "Keep this hand")
                .expect("keep is offered"),
        )
        .expect("keep applies");
        // Now the engine is parked on the opponent's mulligan, not playing it.
        assert!(game.opponent_is_deciding(), "the driver's seat is waiting");
        let observation = parsed(&game.opponent_observe_json().expect("external"));
        assert!(
            observation["legalActions"]
                .as_array()
                .is_some_and(|actions| !actions.is_empty()),
            "the driver sees its legal actions"
        );
    }

    #[test]
    fn the_driver_plays_by_protocol_index_and_the_human_gets_the_beats() {
        // The opponent goes first, so their whole first turn is a driver
        // window: mulligan, land, spells. The human should watch it happen
        // the way they watch a built-in opponent -- as beats and log lines --
        // even though every choice arrived from outside by index.
        let mut game =
            WebGame::new("Sligh", "Goblins", "External", false, 77, None).expect("game starts");
        let mut saw_beats = false;
        for _ in 0..4_000 {
            if game.opponent_is_deciding() {
                let observation = parsed(&game.opponent_observe_json().expect("external"));
                assert!(
                    !observation["decision"].as_object().is_some_and(|decision| {
                        decision["minimum"] == 0
                            && decision["options"].as_array().is_some_and(Vec::is_empty)
                    }),
                    "the external driver is never prompted for an empty action window",
                );
                game.opponent_act(driver_index(&observation))
                    .expect("the driver's index is legal");
                continue;
            }
            let state = parsed(&game.state_json());
            saw_beats |= state["opponentActions"]
                .as_array()
                .is_some_and(|beats| !beats.is_empty());
            if state["events"].as_array().is_some_and(|events| {
                events.iter().any(|line| {
                    line.as_str()
                        .is_some_and(|line| line.starts_with("Opponent played"))
                })
            }) {
                break;
            }
            let actions = state["actions"].as_array().expect("actions").clone();
            let index = actions
                .iter()
                .position(|action| action["label"] == "Keep this hand")
                .or_else(|| actions.iter().position(|action| action["kind"] == "pass"))
                .unwrap_or(0);
            game.act(index).expect("the human's index is legal");
        }
        let state = parsed(&game.state_json());
        assert!(
            state["events"].as_array().is_some_and(|events| {
                events.iter().any(|line| {
                    line.as_str()
                        .is_some_and(|line| line.starts_with("Opponent played"))
                })
            }),
            "the driver's land shows up in the human's log"
        );
        assert!(saw_beats, "and the window produced beats to watch");
    }

    #[test]
    fn a_built_in_opponent_refuses_the_driver_entry_points() {
        let mut game =
            WebGame::new("Sligh", "Goblins", "Handcrafted", true, 77, None).expect("game starts");
        assert!(!game.opponent_is_deciding());
        assert!(game.opponent_observe_json().is_err());
        assert!(game.opponent_act(0).is_err());
    }

    #[test]
    fn the_driver_cannot_act_out_of_turn() {
        let mut game = hosted_external();
        // The human has not kept yet, so the opponent holds nothing.
        assert!(!game.opponent_is_deciding());
        assert!(game.opponent_act(0).is_err());
    }
}

#[test]
fn an_external_game_never_prints_the_seed() {
    // The seed reconstructs both libraries, and in an external game the
    // opponent is real. The built-in-policy snapshot keeps its courtesy line;
    // the external one must not have it anywhere.
    let external =
        WebGame::new("Sligh", "Goblins", "External", true, 4242, None).expect("game starts");
    assert!(
        !external.state_json().contains("seed"),
        "an external game's snapshot mentions no seed"
    );
    let local =
        WebGame::new("Sligh", "Goblins", "Handcrafted", true, 4242, None).expect("game starts");
    assert!(
        local.state_json().contains("Game started · seed 4242"),
        "a built-in game still shows its courtesy line"
    );
}

mod replay_journal {
    use super::*;

    fn assert_replay_is_rejected(replay: &Value, reason: &str) {
        assert!(
            WebGame::from_replay_json(&replay.to_string()).is_err(),
            "{reason}: {replay}"
        );
    }

    /// Plays a stretch of real game through the public surface, then rebuilds
    /// from the journal and expects the same board to the byte. This is the
    /// property a bug report's attachment depends on.
    #[test]
    fn a_journal_replays_to_an_identical_snapshot() {
        let mut game = WebGame::new("Sligh", "Goblins", "Handcrafted", true, 4_242, None)
            .expect("game starts");
        let mut acted = 0;
        for _ in 0..400 {
            let state: serde_json::Value =
                serde_json::from_str(&game.state_json()).expect("snapshot is JSON");
            if state["result"].is_object() {
                break;
            }
            if let Some(decision) = state["decision"].as_object() {
                let id = u32::try_from(decision["id"].as_u64().expect("id")).expect("fits");
                let minimum = decision["minimum"].as_u64().unwrap_or(0).max(1);
                let options: Vec<u64> = decision["options"]
                    .as_array()
                    .expect("options")
                    .iter()
                    .take(usize::try_from(minimum).expect("fits"))
                    .map(|option| option["id"].as_u64().expect("option id"))
                    .collect();
                game.choose_decision(id, &serde_json::to_string(&options).expect("encodes"))
                    .expect("decision applies");
            } else {
                let actions = state["actions"].as_array().expect("actions");
                if actions.is_empty() {
                    break;
                }
                let index = actions
                    .iter()
                    .position(|action| {
                        action["label"].as_str().is_some_and(|label| {
                            label.starts_with("Keep") || label.starts_with("Play ")
                        })
                    })
                    .or_else(|| actions.iter().position(|action| action["kind"] == "pass"))
                    .unwrap_or(0);
                game.act(index).expect("action applies");
            }
            acted += 1;
            if acted >= 25 {
                break;
            }
        }
        // A phase-stop toggle steers the autopass path, so it has to replay too.
        game.set_phase_stop("Combat", true).expect("stop applies");

        let rebuilt = WebGame::from_replay_json(&game.replay_json()).expect("journal replays");
        assert!(acted > 5, "the drive did real work: {acted} commands");
        assert_eq!(
            rebuilt.state_json(),
            game.state_json(),
            "the journal rebuilds the same board"
        );
        assert_eq!(rebuilt.replay_json(), game.replay_json());
    }

    #[test]
    fn replay_format_and_simulation_identity_are_independent_guards() {
        let game =
            WebGame::new("Sligh", "Goblins", "Handcrafted", true, 7, None).expect("game starts");
        let replay: serde_json::Value =
            serde_json::from_str(&game.replay_json()).expect("replay is JSON");
        assert_eq!(replay["replayVersion"], REPLAY_VERSION);
        assert_eq!(
            replay["simulationFingerprint"],
            penta::protocol::SIMULATION_FINGERPRINT
        );

        let mut wrong_format = replay.clone();
        wrong_format["replayVersion"] = serde_json::json!(REPLAY_VERSION + 1);
        assert!(
            WebGame::from_replay_json(&wrong_format.to_string()).is_err(),
            "an unknown journal format is refused"
        );

        let mut wrong_simulation = replay.clone();
        wrong_simulation["simulationFingerprint"] = serde_json::json!("sha256-wrong");
        assert!(
            WebGame::from_replay_json(&wrong_simulation.to_string()).is_err(),
            "different rules are refused before commands apply"
        );

        let mut diagnostic_changes = replay;
        diagnostic_changes["engineVersion"] = serde_json::json!("99.0.0");
        diagnostic_changes["protocolVersion"] = serde_json::json!(1);
        diagnostic_changes["futureEnvelopeField"] = serde_json::json!(true);
        diagnostic_changes["config"]["futureConfigField"] = serde_json::json!([1, 2, 3]);
        WebGame::from_replay_json(&diagnostic_changes.to_string())
            .expect("package, bot-wire, and additive metadata do not gate replay");
    }

    #[test]
    fn replay_v2_requires_its_envelope_and_config_fields_with_their_declared_types() {
        let game =
            WebGame::new("Sligh", "Goblins", "Handcrafted", true, 11, None).expect("game starts");
        let replay: Value = serde_json::from_str(&game.replay_json()).expect("replay is JSON");

        assert_replay_is_rejected(&serde_json::json!([]), "the envelope must be an object");

        for field in [
            "replayVersion",
            "simulationFingerprint",
            "engineVersion",
            "protocolVersion",
            "config",
            "commands",
        ] {
            let mut malformed = replay.clone();
            malformed
                .as_object_mut()
                .expect("fixture is an object")
                .remove(field);
            assert_replay_is_rejected(&malformed, &format!("missing top-level field {field}"));
        }

        for (field, wrong_type) in [
            ("replayVersion", serde_json::json!("1")),
            ("simulationFingerprint", serde_json::json!(1)),
            ("engineVersion", serde_json::json!(1)),
            ("protocolVersion", serde_json::json!("21")),
            ("config", serde_json::json!([])),
            ("commands", serde_json::json!({})),
        ] {
            let mut malformed = replay.clone();
            malformed[field] = wrong_type;
            assert_replay_is_rejected(&malformed, &format!("wrong type for {field}"));
        }

        for field in [
            "format",
            "humanDeck",
            "botDeck",
            "botPolicy",
            "humanFirst",
            "seed",
        ] {
            let mut malformed = replay.clone();
            malformed["config"]
                .as_object_mut()
                .expect("fixture config is an object")
                .remove(field);
            assert_replay_is_rejected(&malformed, &format!("missing config field {field}"));
        }

        for (field, wrong_type) in [
            ("format", serde_json::json!(false)),
            ("humanDeck", serde_json::json!(false)),
            ("botDeck", serde_json::json!(false)),
            ("botPolicy", serde_json::json!(false)),
            ("humanFirst", serde_json::json!("true")),
            ("seed", serde_json::json!("11")),
        ] {
            let mut malformed = replay.clone();
            malformed["config"][field] = wrong_type;
            assert_replay_is_rejected(&malformed, &format!("wrong type for config.{field}"));
        }

        let mut oversized_seed = replay;
        oversized_seed["config"]["seed"] = serde_json::json!(u64::from(u32::MAX) + 1);
        assert_replay_is_rejected(&oversized_seed, "the seed must fit the replay-v2 type");
    }

    #[test]
    #[allow(clippy::too_many_lines)] // One table documents every replay-v2 command field.
    fn replay_v2_commands_require_the_fields_and_types_declared_by_their_tag() {
        let game =
            WebGame::new("Sligh", "Goblins", "Handcrafted", true, 12, None).expect("game starts");
        let replay: Value = serde_json::from_str(&game.replay_json()).expect("replay is JSON");

        for (reason, command) in [
            ("a command must be an object", serde_json::json!("autopass")),
            ("a command needs a tag", serde_json::json!({})),
            (
                "a command tag must be a string",
                serde_json::json!({ "t": false }),
            ),
            ("act needs an index", serde_json::json!({ "t": "act" })),
            (
                "an act index must be an integer",
                serde_json::json!({ "t": "act", "index": "0" }),
            ),
            (
                "choose needs a decision",
                serde_json::json!({ "t": "choose", "options": [] }),
            ),
            (
                "a decision must be an integer",
                serde_json::json!({ "t": "choose", "decision": "0", "options": [] }),
            ),
            (
                "a decision must fit u32",
                serde_json::json!({
                    "t": "choose",
                    "decision": u64::from(u32::MAX) + 1,
                    "options": [],
                }),
            ),
            (
                "choose needs options",
                serde_json::json!({ "t": "choose", "decision": 0 }),
            ),
            (
                "choose options must be an array",
                serde_json::json!({ "t": "choose", "decision": 0, "options": {} }),
            ),
            (
                "each choose option must be an integer",
                serde_json::json!({ "t": "choose", "decision": 0, "options": ["1"] }),
            ),
            (
                "each choose option must fit u32",
                serde_json::json!({
                    "t": "choose",
                    "decision": 0,
                    "options": [u64::from(u32::MAX) + 1],
                }),
            ),
            (
                "blocks needs assignments",
                serde_json::json!({ "t": "blocks" }),
            ),
            (
                "block assignments must be a string",
                serde_json::json!({ "t": "blocks", "assignments": [] }),
            ),
            (
                "phaseStop needs a phase",
                serde_json::json!({ "t": "phaseStop", "enabled": true }),
            ),
            (
                "a phase must be a string",
                serde_json::json!({ "t": "phaseStop", "phase": 1, "enabled": true }),
            ),
            (
                "phaseStop needs enabled",
                serde_json::json!({ "t": "phaseStop", "phase": "Combat" }),
            ),
            (
                "phaseStop enabled must be boolean",
                serde_json::json!({ "t": "phaseStop", "phase": "Combat", "enabled": 0 }),
            ),
            (
                "autopass needs enabled",
                serde_json::json!({ "t": "autopass" }),
            ),
            (
                "autopass enabled must be boolean",
                serde_json::json!({ "t": "autopass", "enabled": 0 }),
            ),
            (
                "botAct needs an index",
                serde_json::json!({ "t": "botAct" }),
            ),
            (
                "a botAct index must be an integer",
                serde_json::json!({ "t": "botAct", "index": "0" }),
            ),
            (
                "a botAct index must fit u32",
                serde_json::json!({ "t": "botAct", "index": u64::from(u32::MAX) + 1 }),
            ),
            (
                "loseOnTime needs a seat",
                serde_json::json!({ "t": "loseOnTime", "reason": "clock expired" }),
            ),
            (
                "a timeout seat must be a string",
                serde_json::json!({ "t": "loseOnTime", "seat": 1, "reason": "clock expired" }),
            ),
            (
                "a timeout seat must use the canonical vocabulary",
                serde_json::json!({
                    "t": "loseOnTime",
                    "seat": "opponent",
                    "reason": "clock expired",
                }),
            ),
            (
                "loseOnTime needs a reason",
                serde_json::json!({ "t": "loseOnTime", "seat": "bot" }),
            ),
            (
                "a timeout reason must be a string",
                serde_json::json!({ "t": "loseOnTime", "seat": "bot", "reason": 1 }),
            ),
            (
                "unknown command tags remain invalid",
                serde_json::json!({ "t": "futureCommand" }),
            ),
        ] {
            let mut malformed = replay.clone();
            malformed["commands"] = serde_json::json!([command]);
            assert_replay_is_rejected(&malformed, reason);
        }
    }

    #[test]
    fn replay_v2_ignores_unknown_command_members_and_preserves_timeout_reason() {
        let game =
            WebGame::new("Sligh", "Goblins", "Handcrafted", true, 13, None).expect("game starts");
        let mut replay: Value = serde_json::from_str(&game.replay_json()).expect("replay is JSON");
        replay["commands"] = serde_json::json!([
            { "t": "autopass", "enabled": false, "future": [1, 2, 3] },
            { "t": "phaseStop", "phase": "Combat", "enabled": true, "future": {} },
            {
                "t": "loseOnTime",
                "seat": "bot",
                "reason": "the remote clock expired",
                "future": true,
            },
        ]);

        let rebuilt =
            WebGame::from_replay_json(&replay.to_string()).expect("additive members are ignored");
        let rebuilt_replay: Value =
            serde_json::from_str(&rebuilt.replay_json()).expect("rebuilt replay is JSON");
        assert_eq!(
            rebuilt_replay["commands"][2]["reason"], "the remote clock expired",
            "diagnostic timeout text survives a replay round trip"
        );
    }
}

/// Losing on time. A room enforces its clock this way, and the result says
/// what actually happened rather than blaming the player for a concession
/// they never made.
mod lose_on_time {
    use super::*;

    fn parsed(json: &str) -> serde_json::Value {
        serde_json::from_str(json).expect("state is JSON")
    }

    #[test]
    fn a_bot_that_runs_out_of_time_hands_the_game_to_the_human() {
        let mut game =
            WebGame::new("Sligh", "Goblins", "External", true, 9, None).expect("game starts");
        assert!(
            parsed(&game.state_json())["result"].is_null(),
            "game is live"
        );

        game.lose_on_time("bot").expect("the bot loses on time");

        let result = parsed(&game.state_json());
        assert_eq!(result["result"]["outcome"], "win", "{result}");
        assert!(
            game.lose_on_time("bot").is_err(),
            "a finished game cannot run out of time again"
        );
    }

    #[test]
    fn a_seat_can_lose_on_time_without_holding_the_decision() {
        // The human holds the opening decision, so this forfeits the seat
        // that is *not* being waited on -- which is the whole point: a player
        // who stopped answering is not going to take their turn either.
        let mut game =
            WebGame::new("Sligh", "Goblins", "External", true, 9, None).expect("game starts");
        assert!(
            !game.opponent_is_deciding(),
            "the human is the one on the clock here"
        );
        game.lose_on_time("bot")
            .expect("the clock does not need the turn");
        assert_eq!(parsed(&game.state_json())["result"]["outcome"], "win");
    }

    #[test]
    fn a_human_who_runs_out_of_time_loses_the_game() {
        let mut game =
            WebGame::new("Sligh", "Goblins", "External", true, 9, None).expect("game starts");
        game.lose_on_time("human").expect("the human loses on time");
        assert_eq!(parsed(&game.state_json())["result"]["outcome"], "loss");
    }

    /// The wart this replaced: a player who walked away was told they
    /// conceded, which is a thing they never did.
    #[test]
    fn the_result_says_time_rather_than_concession() {
        let mut game =
            WebGame::new("Sligh", "Goblins", "External", true, 9, None).expect("game starts");
        game.lose_on_time("human").expect("the human loses on time");
        let message = parsed(&game.state_json())["result"]["message"]
            .as_str()
            .expect("a finished game explains itself")
            .to_string();
        assert!(message.contains("ran out of time"), "{message}");
        assert!(!message.contains("conceded"), "{message}");
    }

    #[test]
    fn an_unknown_seat_is_refused() {
        let mut game =
            WebGame::new("Sligh", "Goblins", "External", true, 9, None).expect("game starts");
        assert!(game.lose_on_time("nobody").is_err());
        assert!(
            parsed(&game.state_json())["result"].is_null(),
            "a refused timeout leaves the game alone"
        );
    }

    #[test]
    fn the_timeout_journal_uses_the_canonical_v1_seat_and_reason() {
        let mut game =
            WebGame::new("Sligh", "Goblins", "External", true, 9, None).expect("game starts");
        game.lose_on_time("opponent")
            .expect("the public alias remains accepted");

        let replay = parsed(&game.replay_json());
        assert_eq!(replay["commands"][0]["seat"], "bot");
        assert_eq!(replay["commands"][0]["reason"], "ran out of time");
    }

    #[test]
    fn a_timeout_replays_like_any_other_command() {
        let mut game =
            WebGame::new("Sligh", "Goblins", "External", true, 9, None).expect("game starts");
        game.act(
            parsed(&game.state_json())["actions"]
                .as_array()
                .expect("actions")
                .iter()
                .position(|action| action["label"] == "Keep this hand")
                .expect("keep is offered"),
        )
        .expect("keep applies");
        game.lose_on_time("bot").expect("the bot loses on time");

        let rebuilt = WebGame::from_replay_json(&game.replay_json()).expect("replay rebuilds");
        assert_eq!(
            rebuilt.state_json(),
            game.state_json(),
            "the journal carries the timeout"
        );
    }
}
