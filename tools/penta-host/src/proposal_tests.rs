use super::*;

fn setup() -> (Host, String, [String; 2]) {
    setup_mode("one-conclusion")
}

fn setup_mode(mode: &str) -> (Host, String, [String; 2]) {
    let mut host = Host::with_seed(
        Config {
            format: "old-school-93-94".into(),
            p1_deck: "Sligh".into(),
            p2_deck: "The Deck".into(),
            match_mode: mode.into(),
        },
        u64::MAX,
    )
    .unwrap();
    let launch = host.launch_info("http://127.0.0.1:1234");
    let organizer = launch["organizerSecret"].as_str().unwrap().to_owned();
    let seats = host.request(&organizer, "seats", &json!({})).unwrap();
    let secrets =
        ["p1", "p2"].map(|seat| seats["seats"][seat]["secret"].as_str().unwrap().to_owned());
    (host, organizer, secrets)
}

fn observe(host: &mut Host, secret: &str) -> Value {
    host.request(secret, "observe", &json!({})).unwrap()
}

fn submit_private_sideboard_swap(
    host: &mut Host,
    seats: &[String; 2],
    game: usize,
    submission: usize,
    wins: [u32; 2],
) {
    let seat = host.session.decision_seat().unwrap().index();
    let view = observe(host, &seats[seat]);
    let observation = &view["observation"];
    assert_eq!(observation["match"]["stage"], "sideboarding");
    assert_eq!(observation["match"]["wins"], json!(wins));
    assert!(host.observations[1 - seat]["decision"].is_null());
    let opponent = observe(host, &seats[1 - seat]);
    assert_eq!(opponent["status"], "waiting");
    assert!(opponent.get("observation").is_none());
    let decision = &observation["decision"];
    let options = decision["options"].as_array().unwrap();
    let main = observation["match"]["main"].as_array().unwrap();
    let outside = options
        .iter()
        .find(|option| option["zone"] == "OutsideGame")
        .unwrap();
    let incoming = observation["match"]["sideboard"][0].clone();
    let outgoing = main.iter().position(|card| *card != incoming).unwrap();
    let mut selection: Vec<_> = options
        .iter()
        .filter(|option| option["zone"] == "Library" && option["id"] != outgoing)
        .map(|option| option["id"].clone())
        .collect();
    selection.push(outside["id"].clone());
    let args = json!({"requestId":format!("sideboard-{game}-{submission}"),
        "revision":view["revision"],
        "choices":[{"proposal":{"action":{"type":"ChooseDecision",
            "decision":decision["id"],"options":selection}}}]});
    let mut invalid = args.clone();
    invalid["requestId"] = json!(format!("invalid-{game}-{submission}"));
    invalid["choices"][0]["proposal"]["action"]["options"] = json!([]);
    let rejected = host.request(&seats[seat], "interact", &invalid).unwrap();
    assert_eq!(rejected["receipt"]["accepted"], 0);
    assert_eq!(observe(host, &seats[seat])["observation"], *observation);
    let accepted = host.request(&seats[seat], "interact", &args).unwrap();
    assert_eq!(accepted["receipt"]["accepted"], 1);
    let count = host.journal.len();
    assert_eq!(
        host.request(&seats[seat], "interact", &args).unwrap()["receipt"],
        accepted["receipt"]
    );
    assert_eq!(host.journal.len(), count);
    let mut expected_main = main.clone();
    let removed = expected_main.remove(outgoing);
    expected_main.push(incoming);
    let mut expected_sideboard = observation["match"]["sideboard"]
        .as_array()
        .unwrap()
        .clone();
    expected_sideboard.remove(0);
    expected_sideboard.insert(0, removed);
    assert_eq!(
        host.observations[seat]["match"]["main"],
        json!(expected_main)
    );
    assert_eq!(
        host.observations[seat]["match"]["sideboard"],
        json!(expected_sideboard)
    );
}

#[test]
fn proposal_first_to_two_match_sideboards_and_replays_three_games() {
    let (mut host, organizer, seats) = setup_mode("first-to-two-wins");
    let mut chooser = 0;
    let mut wins = [0, 0];
    for (game, loser) in [1, 0, 1].into_iter().enumerate() {
        let opening = observe(&mut host, &seats[chooser]);
        assert_eq!(opening["apiVersion"], 2);
        assert_eq!(opening["observation"]["actionInterface"], "proposal-v1");
        assert!(opening["observation"].get("legalActions").is_none());
        assert_eq!(opening["observation"]["match"]["stage"], "play-draw");
        assert_eq!(opening["observation"]["match"]["wins"], json!(wins));
        // Exercise both play and draw; the previous loser owns this choice.
        let draw = game % 2;
        let args = json!({"requestId":format!("start-{game}"),
            "revision":opening["revision"],
            "choices":[{"proposal":{"action":{"type":"ChooseDecision",
                "decision":opening["observation"]["decision"]["id"],"options":[draw]}}}]});
        let accepted = host.request(&seats[chooser], "interact", &args).unwrap();
        assert_eq!(accepted["receipt"]["accepted"], 1);
        let starting = if draw == 0 { chooser } else { 1 - chooser };
        for seat in 0..2 {
            let observation = &host.observations[seat];
            assert_eq!(observation["seat"], ["p1", "p2"][seat]);
            assert_eq!(observation["activeSeat"], ["p1", "p2"][starting]);
            assert_eq!(observation["match"]["game"], game + 1);
            assert_eq!(observation["match"]["stage"], "playing");
        }
        // Accept the opening hands through the same proposal interface.
        for keep in 0..2 {
            let seat = host.session.decision_seat().unwrap().index();
            let view = observe(&mut host, &seats[seat]);
            let accepted = host
                .request(
                    &seats[seat],
                    "interact",
                    &json!({
                        "requestId":format!("keep-{game}-{keep}"),"revision":view["revision"],
                        "choices":[{"proposal":{"action":{"type":"KeepHand"}}}]
                    }),
                )
                .unwrap();
            assert_eq!(accepted["receipt"]["accepted"], 1);
        }
        if game == 0 {
            let view = observe(&mut host, &seats[starting]);
            let yielded = host
                .request(
                    &seats[starting],
                    "yield",
                    &json!({
                        "requestId":"across-game-boundary", "revision":view["revision"],
                        "until":[{"type":"nextStep","step":"End"}]
                    }),
                )
                .unwrap();
            assert_eq!(yielded["yield"]["status"], "active");
        }
        host.request(
            &seats[loser],
            "concede",
            &json!({
                "requestId":format!("concede-{game}")
            }),
        )
        .unwrap();
        if game == 0 {
            let view = observe(&mut host, &seats[starting]);
            assert_eq!(view["yield"]["status"], "stopped");
            assert_eq!(view["yield"]["stopReason"], "gameEnded");
        }
        wins[1 - loser] += 1;
        if game == 2 {
            break;
        }
        assert!(!host.session.is_finished());
        for submission in 0..2 {
            submit_private_sideboard_swap(&mut host, &seats, game, submission, wins);
        }
        chooser = loser;
        assert_eq!(host.session.decision_seat().unwrap().index(), chooser);
    }
    assert_eq!(wins, [2, 1]);
    for (seat, secret) in seats.iter().enumerate() {
        let view = observe(&mut host, secret);
        assert_eq!(view["status"], "complete");
        assert_eq!(view["observation"]["seat"], ["p1", "p2"][seat]);
        assert_eq!(view["observation"]["match"]["stage"], "complete");
        assert_eq!(view["observation"]["match"]["wins"], json!([2, 1]));
    }
    let bundle = host.request(&organizer, "export", &json!({})).unwrap();
    assert_eq!(bundle["nativeReplayVersion"], 3);
    let replay = replay_bundle(&bundle).unwrap();
    assert_eq!(replay["status"], "complete");
    assert_eq!(replay["observations"], bundle["observations"]);
}

#[test]
fn proposal_observation_requires_explicit_discovery_for_legal_menus() {
    let (mut host, organizer, seats) = setup();
    let view = observe(&mut host, &seats[0]);
    assert_eq!(view["apiVersion"], 2);
    assert_eq!(view["observation"]["actionInterface"], "proposal-v1");
    assert!(view["observation"].get("legalActions").is_none());
    assert!(view["observation"]["hand"].is_array());
    assert!(host.request(&organizer, "discover", &json!({})).is_err());
    let discovered = host.request(&seats[0], "discover", &json!({})).unwrap();
    assert_eq!(discovered["apiVersion"], 1);
    assert_eq!(discovered["revision"], view["revision"]);
    assert!(
        discovered["observation"]["legalActions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|action| action["type"] == "KeepHand")
    );
    let refreshed = observe(&mut host, &seats[0]);
    assert!(refreshed["observation"].get("legalActions").is_none());
    assert_eq!(refreshed["revision"], view["revision"]);
}

#[test]
fn proposal_interaction_is_revision_checked_idempotent_and_replayable() {
    let (mut host, organizer, seats) = setup();
    let initial = observe(&mut host, &seats[0]);
    let args = json!({"requestId":"proposal-keep", "revision":initial["revision"],
        "choices":[{"proposal":{"action":{"type":"KeepHand"}}}]});
    let accepted = host.request(&seats[0], "interact", &args).unwrap();
    assert_eq!(accepted["receipt"]["accepted"], 1);
    let count = host.journal.len();
    let retry = host.request(&seats[0], "interact", &args).unwrap();
    assert_eq!(retry["receipt"], accepted["receipt"]);
    assert_eq!(host.journal.len(), count);
    let mut stale = args.clone();
    stale["requestId"] = json!("proposal-stale");
    assert_eq!(
        host.request(&seats[0], "interact", &stale)
            .unwrap_err()
            .status,
        409
    );
    let bundle = host.request(&organizer, "export", &json!({})).unwrap();
    assert_eq!(bundle["nativeReplayVersion"], 3);
    let replay = replay_bundle(&bundle).unwrap();
    assert_eq!(replay["observations"], bundle["observations"]);
}

#[test]
fn malformed_proposal_returns_safe_rejection_without_changing_state() {
    let (mut host, _, seats) = setup();
    let initial = observe(&mut host, &seats[0]);
    let rejected = host
        .request(
            &seats[0],
            "interact",
            &json!({
                "requestId":"bad-proposal", "revision":initial["revision"],
                "choices":[{"proposal":{"action":{"type":"CastSpell","card":1,
                    "choices":{"playOptionId":0,"targetSelections":[{"slotId":0,
                        "targets":[{"type":"player","seat":"p2","hidden":"forged"}]}]}}}}]
            }),
        )
        .unwrap();
    assert_eq!(rejected["receipt"]["accepted"], 0);
    let refusal = &rejected["receipt"]["rejection"];
    assert_eq!(refusal["code"], "invalidField");
    assert!(refusal["path"].as_str().unwrap().contains("hidden"));
    assert!(refusal["message"].is_string());
    assert!(host.journal.is_empty());
    let after = observe(&mut host, &seats[0]);
    assert_eq!(after["observation"], initial["observation"]);
}

#[test]
fn proposal_play_retains_typed_refusal_and_retry_receipt() {
    let (mut host, _, seats) = setup();
    let initial = observe(&mut host, &seats[0]);
    let args = json!({"requestId":"refused-play","revision":initial["revision"],
        "choices":[{"proposal":{"action":{"type":"PassPriority"}}}]});
    let rejected = host.request(&seats[0], "play", &args).unwrap();
    assert_eq!(rejected["receipt"]["accepted"], 0);
    assert_eq!(rejected["receipt"]["rejection"]["code"], "illegalAction");
    assert_eq!(
        host.request(&seats[0], "play", &args).unwrap()["receipt"],
        rejected["receipt"]
    );
    assert_eq!(
        observe(&mut host, &seats[0])["observation"],
        initial["observation"]
    );
}

#[test]
fn proposal_complete_and_waiting_contexts_do_not_insert_null_offer_arrays() {
    let (mut host, _, seats) = setup();
    assert_eq!(host.observations[1]["actionContext"]["kind"], "waiting");
    assert!(
        host.observations[1]["actionContext"]
            .get("castOffers")
            .is_none()
    );
    host.request(&seats[0], "concede", &json!({"requestId":"end-game"}))
        .unwrap();
    for seat in seats {
        let view = observe(&mut host, &seat);
        assert_eq!(view["status"], "complete");
        assert_eq!(view["observation"]["actionContext"]["kind"], "complete");
        assert!(
            view["observation"]["actionContext"]
                .get("castOffers")
                .is_none()
        );
    }
}
