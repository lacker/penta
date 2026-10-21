use super::*;
use crate::{Config, replay_bundle};

fn ready_host() -> Host {
    let mut host = Host::with_seed(
        Config {
            format: "old-school-93-94".into(),
            p1_deck: "Sligh".into(),
            p2_deck: "The Deck".into(),
            match_mode: "one-conclusion".into(),
        },
        1,
    )
    .unwrap();
    for seat in [PlayerId::One, PlayerId::Two] {
        let args = json!({"requestId":format!("keep-{}",seat.index()),
            "revision":host.revision,"choices":[{"action":{"type":"KeepHand"}}]});
        host.play(seat, "play", &args).unwrap();
    }
    host
}

fn plan() -> Value {
    json!({"until":[{"type":"nextStep","step":"End"}],"maxPasses":64})
}

fn args(host: &Host, choices: Value) -> Value {
    let mut request = json!({"requestId":"interaction","revision":host.revision,"yield":plan()});
    request["choices"] = choices;
    request
}

#[test]
fn interaction_combines_action_and_yield_and_retries_without_reapplying() {
    let mut host = ready_host();
    let seat = host.session.decision_seat().unwrap();
    let request = args(&host, json!([{"action":{"type":"PassPriority"}}]));
    let view = host.play(seat, "interact", &request).unwrap();
    assert_eq!(view["receipt"]["accepted"], 1);
    assert_eq!(view["receipt"]["yieldInstallation"]["status"], "installed");
    assert_eq!(view["receipt"]["yield"]["passes"], 0);
    let journal = host.journal.clone();
    let retry = host.play(seat, "interact", &request).unwrap();
    assert_eq!(retry["receipt"], view["receipt"]);
    assert_eq!(host.journal, journal);
    replay_bundle(&host.export()).unwrap();
}

#[test]
fn interaction_empty_batch_installs_yield_only_at_ordinary_priority() {
    let mut host = ready_host();
    let seat = host.session.decision_seat().unwrap();
    let request = args(&host, json!([]));
    let view = host.play(seat, "interact", &request).unwrap();
    assert_eq!(view["receipt"]["accepted"], 0);
    assert_eq!(view["receipt"]["yieldInstallation"]["status"], "installed");
    assert!(
        host.journal
            .iter()
            .any(|entry| entry["choice"] == json!({"autoPass":true}))
    );
    replay_bundle(&host.export()).unwrap();
}

#[test]
fn interaction_partial_prefix_never_installs_requested_yield() {
    let mut host = ready_host();
    let seat = host.session.decision_seat().unwrap();
    let request = args(
        &host,
        json!([
            {"action":{"type":"PassPriority"}}, {"action":{"type":"KeepHand"}}
        ]),
    );
    let view = host.play(seat, "interact", &request).unwrap();
    assert_eq!(view["receipt"]["accepted"], 1);
    assert_eq!(
        view["receipt"]["yieldInstallation"]["reason"],
        "partialBatch"
    );
    assert!(host.yields[seat.index()].is_none());
    replay_bundle(&host.export()).unwrap();
}

#[test]
fn interaction_validates_entire_plan_and_choice_shape_before_mutation() {
    for invalid in [
        json!({"until":[]}),
        json!({"until":[{"type":"unknown"}]}),
        json!({"until":[{"type":"opponentActs"}],"maxPasses":0}),
    ] {
        let mut host = ready_host();
        let seat = host.session.decision_seat().unwrap();
        let mut request = args(&host, json!([{"action":{"type":"PassPriority"}}]));
        request["yield"] = invalid;
        let before = host.journal.clone();
        let revision = host.revision.clone();
        assert_eq!(
            host.play(seat, "interact", &request).unwrap_err().status,
            400
        );
        assert_eq!(host.journal, before);
        assert_eq!(host.revision, revision);
    }
    let mut host = ready_host();
    let seat = host.session.decision_seat().unwrap();
    let request = args(
        &host,
        json!([{"action":{"type":"PassPriority"}},{"index":0}]),
    );
    let before = host.journal.clone();
    assert_eq!(
        host.play(seat, "interact", &request).unwrap_err().status,
        400
    );
    assert_eq!(host.journal, before);
}

#[test]
fn interaction_installation_respects_decision_interruptions_and_other_seats() {
    let mut host = ready_host();
    let seat = host.session.decision_seat().unwrap();
    host.observations[seat.index()]["decision"] = json!({"id":1});
    host.observations[seat.index()]["actionContext"]["ordinaryPriority"] = json!(false);
    let receipt = host.interaction_yield(
        seat,
        "own",
        Some(YieldOptions::parse(&plan()).unwrap()),
        true,
    );
    assert_eq!(receipt["reason"], "decision");
    assert!(host.yields[seat.index()].is_none());
    let opponent = seat.opponent();
    let receipt = host.interaction_yield(
        opponent,
        "other",
        Some(YieldOptions::parse(&plan()).unwrap()),
        true,
    );
    assert_eq!(receipt["status"], "installed");
    assert!(host.yields[opponent.index()].is_some());
    let request = args(&host, json!([]));
    assert_eq!(
        host.play(seat, "interact", &request).unwrap_err().status,
        409
    );
}

#[test]
fn command_metrics_accumulate_without_evicting_diagnostics_or_leaking_to_seats() {
    let mut host = ready_host();
    let diagnostics = host.diagnostics.clone();
    for _ in 0..200 {
        host.record_command_duration("interact", 10);
    }
    host.record_command_duration("interact", 30);
    host.record_command_duration("discover", 7);
    host.record_command_duration("private arbitrary method", 5);
    assert_eq!(host.diagnostics, diagnostics);
    let exported = host.export();
    assert_eq!(
        exported["commandMetrics"]["interact"],
        json!({"count":201,"totalMicros":2030,"maxMicros":30})
    );
    assert_eq!(exported["commandMetrics"]["unknown"]["count"], 1);
    assert_eq!(exported["commandMetrics"]["discover"]["count"], 1);
    assert!(
        !exported["commandMetrics"]
            .to_string()
            .contains("private arbitrary")
    );
    let status = host.organize("status", &json!({})).unwrap();
    assert_eq!(status["commandMetrics"], exported["commandMetrics"]);
    for seat in [PlayerId::One, PlayerId::Two] {
        assert!(host.view(seat).get("commandMetrics").is_none());
    }
}
