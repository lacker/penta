use super::*;

fn host(mode: &str) -> Host {
    Host::with_seed(
        Config {
            format: "old-school-93-94".into(),
            p1_deck: "Sligh".into(),
            p2_deck: "The Deck".into(),
            match_mode: mode.into(),
        },
        u64::MAX,
    )
    .unwrap()
}

fn capabilities(host: &mut Host) -> (String, [String; 2]) {
    let launch = host.launch_info("http://127.0.0.1:1234");
    let organizer = launch["organizerSecret"].as_str().unwrap().to_owned();
    let seats = host.request(&organizer, "seats", &json!({})).unwrap();
    (
        organizer,
        ["p1", "p2"].map(|seat| seats["seats"][seat]["secret"].as_str().unwrap().to_owned()),
    )
}

fn discover(host: &mut Host, secret: &str) -> Value {
    // These legacy ticket tests explicitly opt into action discovery.
    host.request(secret, "discover", &json!({})).unwrap()
}

fn choose_args(view: &Value, kind: &str, id: &str) -> Value {
    let action = view["observation"]["legalActions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|action| action["type"] == kind)
        .unwrap();
    json!({
        "ticket": format!("{}:a{}", view["revision"].as_str().unwrap(), action["index"]),
        "requestId": id,
    })
}

#[test]
fn capabilities_isolate_organizer_seats_and_private_transcripts() {
    let mut host = host("one-conclusion");
    let (organizer, seats) = capabilities(&mut host);
    assert_ne!(seats[0], seats[1]);
    assert_ne!(organizer, seats[0]);
    assert_eq!(
        host.request("unknown", "observe", &json!({}))
            .unwrap_err()
            .status,
        403
    );
    for method in [
        "observe",
        "wait",
        "choose",
        "play",
        "concede",
        "yield",
        "cancelYield",
        "receipt",
        "transcript",
        "note",
        "catalog",
    ] {
        assert_eq!(
            host.request(&organizer, method, &json!({}))
                .unwrap_err()
                .status,
            403
        );
    }
    for method in ["seats", "status", "export", "close"] {
        for seat in &seats {
            assert_eq!(
                host.request(seat, method, &json!({})).unwrap_err().status,
                403
            );
        }
    }
    host.request(&seats[0], "note", &json!({"text":"p1 private reasoning"}))
        .unwrap();
    host.request(&seats[1], "note", &json!({"text":"p2 private reasoning"}))
        .unwrap();
    for (index, seat) in seats.iter().enumerate() {
        let own = host.request(seat, "transcript", &json!({})).unwrap();
        assert!(
            own.to_string()
                .contains(&format!("p{} private reasoning", index + 1))
        );
        assert!(
            !own.to_string()
                .contains(&format!("p{} private reasoning", 2 - index))
        );
        for secret in [&organizer, &seats[0], &seats[1]] {
            assert!(!own.to_string().contains(secret));
            assert!(!discover(&mut host, seat).to_string().contains(secret));
        }
    }
    let waiting = host
        .request(&seats[1], "wait", &json!({"waitMs":0}))
        .unwrap();
    assert_eq!(waiting["status"], "waiting");
    assert!(waiting.get("observation").is_none());
    assert!(waiting.get("revision").is_none());
    assert!(
        host.request(&seats[0], "observe", &json!({"seat":"p2"}))
            .is_err()
    );
}

#[test]
fn reconnect_and_old_request_receipts_do_not_apply_a_move_twice() {
    let mut host = host("one-conclusion");
    let (_, seats) = capabilities(&mut host);
    let first = discover(&mut host, &seats[0]);
    let args = choose_args(&first, "KeepHand", "first-keep");
    let accepted = host.request(&seats[0], "choose", &args).unwrap();
    assert_eq!(accepted["receipt"]["accepted"], 1);
    let second = discover(&mut host, &seats[1]);
    let second_args = choose_args(&second, "KeepHand", "second-keep");
    host.request(&seats[1], "choose", &second_args).unwrap();
    let commands = host.journal.len();
    let retry = host.request(&seats[0].clone(), "choose", &args).unwrap();
    assert_eq!(retry["receipt"], accepted["receipt"]);
    assert_eq!(host.journal.len(), commands);
    let receipt = host
        .request(&seats[0], "receipt", &json!({"requestId":"first-keep"}))
        .unwrap();
    assert_eq!(receipt["receipt"], accepted["receipt"]);
    assert!(
        host.request(&seats[1], "receipt", &json!({"requestId":"first-keep"}))
            .unwrap()["receipt"]
            .is_null()
    );
    let mut changed = args.clone();
    changed["ticket"] = json!("different:a0");
    assert_eq!(
        host.request(&seats[0], "choose", &changed)
            .unwrap_err()
            .status,
        409
    );
    let mut stale = args.clone();
    stale["requestId"] = json!("new-stale-request");
    assert_eq!(
        host.request(&seats[0], "choose", &stale)
            .unwrap_err()
            .status,
        409
    );
    assert_eq!(host.journal.len(), commands);
}

#[test]
fn exact_batch_reports_and_retains_only_the_accepted_prefix() {
    let mut host = host("one-conclusion");
    let (_, seats) = capabilities(&mut host);
    let initial = discover(&mut host, &seats[0]);
    let index_batch = json!({"revision":initial["revision"],"requestId":"indices",
        "choices":[{"index":0},{"index":0}]});
    assert!(host.request(&seats[0], "play", &index_batch).is_err());
    let too_many = json!({"revision":initial["revision"],"requestId":"too-many",
        "choices":vec![json!({"action":{"type":"KeepHand"}});65]});
    assert!(host.request(&seats[0], "play", &too_many).is_err());
    assert!(host.journal.is_empty());
    let batch = json!({"revision":initial["revision"],"requestId":"prefix",
        "choices":[{"action":{"type":"KeepHand"}},{"action":{"type":"TakeMulligan"}}]});
    let accepted = host.request(&seats[0], "play", &batch).unwrap();
    assert_eq!(accepted["receipt"]["accepted"], 1);
    assert_eq!(discover(&mut host, &seats[1])["status"], "ready");
    assert_eq!(host.journal.len(), 1);
    let retry = host.request(&seats[0], "play", &batch).unwrap();
    assert_eq!(retry["receipt"], accepted["receipt"]);
    assert_eq!(host.journal.len(), 1);
}

#[test]
fn explicit_decision_tickets_require_options_and_caller_request_ids() {
    let mut host = host("first-to-two-wins");
    let (_, seats) = capabilities(&mut host);
    let view = discover(&mut host, &seats[0]);
    let mut args = choose_args(&view, "ChooseDecision", "play-first");
    assert!(host.request(&seats[0], "choose", &args).is_err());
    args["options"] = json!([0]);
    let mut missing_id = args.clone();
    missing_id.as_object_mut().unwrap().remove("requestId");
    assert!(host.request(&seats[0], "choose", &missing_id).is_err());
    assert!(host.journal.is_empty());
    let accepted = host.request(&seats[0], "choose", &args).unwrap();
    assert_eq!(accepted["receipt"]["accepted"], 1);
    assert_eq!(
        discover(&mut host, &seats[0])["observation"]["match"]["mode"],
        "first-to-two-wins"
    );
    host.request(
        &seats[1],
        "concede",
        &json!({"requestId":"game-one-concession"}),
    )
    .unwrap();
    assert!(!host.session.is_finished());
    let seat = host.session.decision_seat().unwrap().index();
    let sideboarding = discover(&mut host, &seats[seat]);
    assert_eq!(
        sideboarding["observation"]["match"]["stage"],
        "sideboarding"
    );
    assert_eq!(sideboarding["observation"]["match"]["wins"], json!([1, 0]));
    let decision = &sideboarding["observation"]["decision"];
    let options: Vec<_> = decision["options"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|option| option["zone"] == "Library")
        .map(|option| option["id"].clone())
        .collect();
    let selection = json!({"revision":sideboarding["revision"],"requestId":"sideboard",
        "choices":[{"decision":decision["id"],"options":options}]});
    assert_eq!(
        host.request(&seats[seat], "play", &selection).unwrap()["receipt"]["accepted"],
        1
    );
}

#[test]
fn completed_match_remains_readable_and_export_replays_until_explicit_close() {
    let mut host = host("one-conclusion");
    let (organizer, seats) = capabilities(&mut host);
    let initial = discover(&mut host, &seats[0]);
    let keep = choose_args(&initial, "KeepHand", "keep");
    host.request(&seats[0], "choose", &keep).unwrap();
    host.request(&seats[0], "note", &json!({"text":"remember this position"}))
        .unwrap();
    host.request(&seats[0], "concede", &json!({"requestId":"concede"}))
        .unwrap();
    for seat in &seats {
        assert_eq!(discover(&mut host, seat)["status"], "complete");
        assert!(host.request(seat, "transcript", &json!({})).is_ok());
    }
    assert!(!host.is_closed());
    let bundle = host.request(&organizer, "export", &json!({})).unwrap();
    assert_eq!(bundle["seed"], u64::MAX.to_string());
    assert_eq!(bundle["nativeReplayVersion"], 3);
    for field in [
        "config",
        "commands",
        "observations",
        "transcripts",
        "receipts",
        "diagnostics",
    ] {
        assert!(bundle.get(field).is_some(), "missing export field {field}");
    }
    for secret in [&organizer, &seats[0], &seats[1]] {
        assert!(!bundle.to_string().contains(secret));
    }
    let replayed = replay_bundle(&bundle).unwrap();
    assert_eq!(replayed["status"], "complete");
    assert_eq!(replayed["observations"], bundle["observations"]);
    let mut wrong_engine = bundle.clone();
    wrong_engine["simulationFingerprint"] = json!("different-engine");
    assert!(replay_bundle(&wrong_engine).is_err());
    let mut corrupt = bundle.clone();
    corrupt["observations"]["p1"]["seat"] = json!("p2");
    assert!(replay_bundle(&corrupt).is_err());
    host.request(&organizer, "close", &json!({})).unwrap();
    assert!(host.is_closed());
    assert_eq!(
        host.request(&seats[0], "observe", &json!({}))
            .unwrap_err()
            .status,
        410
    );
}

#[test]
fn batch_receipt_retains_all_accepted_updates_across_retry_and_handover() {
    let mut host = host("one-conclusion");
    let (_, seats) = capabilities(&mut host);
    for secret in &seats {
        let view = discover(&mut host, secret);
        let args = choose_args(&view, "KeepHand", "keep");
        host.request(secret, "choose", &args).unwrap();
    }
    for index in 0..20 {
        let seat = host.session.decision_seat().unwrap();
        if host.observations[seat.index()]["step"] == "PrecombatMain" {
            break;
        }
        let args = json!({"revision":host.revision,"requestId":format!("setup-pass-{index}"),
            "choices":[{"proposal":{"action":{"type":"PassPriority"}}}]});
        assert_eq!(
            host.request(&seats[seat.index()], "play", &args).unwrap()["receipt"]["accepted"],
            1
        );
    }
    let view = discover(&mut host, &seats[0]);
    let mut land = view["observation"]["legalActions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|action| action["type"] == "PlayLand")
        .unwrap()
        .clone();
    land.as_object_mut().unwrap().remove("index");
    let choices = vec![
        json!({"action":land}),
        json!({"action":{"type":"PassPriority"}}),
    ];
    let mut expected = Vec::new();
    let mut simulated = host.session.clone();
    for choice in &choices {
        simulated.apply_choice(PlayerId::One, choice).unwrap();
        simulated.advance_forced().unwrap();
        let updates = simulated.observe(PlayerId::One)["updates"]
            .as_array()
            .unwrap()
            .clone();
        if expected.is_empty() {
            assert!(!updates.is_empty(), "first move must reveal information");
        }
        expected.extend(updates);
    }
    let args = json!({"revision":view["revision"],"choices":choices,"requestId":"batch"});
    let result = host.request(&seats[0], "play", &args).unwrap();
    assert_eq!(result["receipt"]["accepted"], 2);
    assert_eq!(result["receipt"]["updates"], json!(expected));
    let retried = host.request(&seats[0], "play", &args).unwrap();
    assert_eq!(retried["receipt"], result["receipt"]);
    assert_eq!(
        host.request(&seats[0], "receipt", &json!({"requestId":"batch"}))
            .unwrap()["receipt"],
        result["receipt"]
    );
}
