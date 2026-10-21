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
        host.play(
            seat,
            "play",
            &json!({"requestId":format!("keep-{}",seat.index()),
            "revision":host.revision,"choices":[{"action":{"type":"KeepHand"}}]}),
        )
        .unwrap();
    }
    assert!(ordinary_priority(
        &host.observations[host.session.decision_seat().unwrap().index()]
    ));
    host
}

// Proposal sessions preserve ordinary priority windows. Test opponents pass
// explicitly; the yielding seat's policy still owns only its own passes.
fn pass_until(host: &mut Host, step: &str, active: Option<PlayerId>) {
    for index in 0..100 {
        let seat = host.session.decision_seat().unwrap();
        let observation = &host.observations[seat.index()];
        if observation["step"] == step
            && active.is_none_or(|player| observation["activeSeat"] == crate::seat_name(player))
        {
            return;
        }
        assert!(
            ordinary_priority(observation),
            "setup encountered a player choice"
        );
        let args = json!({"revision":host.revision,
            "requestId":format!("setup-pass-{}-{index}",host.journal.len()),
            "choices":[{"proposal":{"action":{"type":"PassPriority"}}}]});
        assert_eq!(
            host.play(seat, "play", &args).unwrap()["receipt"]["accepted"],
            1
        );
    }
    panic!("explicit passes did not reach {step}");
}

fn start(host: &mut Host, seat: PlayerId, id: &str, until: Value, max: usize) -> Value {
    let mut args = json!({"requestId":id,"revision":host.revision,"maxPasses":max});
    args["until"] = until;
    host.yield_request(seat, "yield", &args).unwrap()
}

#[test]
fn bounded_yield_retains_updates_and_replays_every_pass() {
    let mut host = ready_host();
    let seat = host.session.decision_seat().unwrap();
    let before = host.observations[seat.index()]["updates"]
        .as_array()
        .unwrap()
        .clone();
    let journal = host.journal.len();
    let view = start(
        &mut host,
        seat,
        "yield",
        json!([{"type":"nextStep","step":"End"}]),
        1,
    );
    assert_eq!(host.journal.len(), journal + 1);
    assert_eq!(
        host.journal.last().unwrap()["choice"],
        json!({"autoPass":true})
    );
    assert_eq!(view["yield"]["passes"], 1);
    assert_eq!(view["yield"]["stopReason"], "passLimit");
    assert!(
        host.observations[seat.index()]["updates"]
            .as_array()
            .unwrap()
            .starts_with(&before)
    );
    assert_eq!(
        replay_bundle(&host.export()).unwrap()["commands"],
        host.journal.len()
    );
}

#[test]
fn policies_compose_latch_across_handoffs_and_expire() {
    let mut host = ready_host();
    let first = host.session.decision_seat().unwrap();
    start(
        &mut host,
        first,
        "first",
        json!([{"type":"nextStep","step":"End"},{"type":"opponentActs"}]),
        64,
    );
    pass_until(&mut host, "End", None);
    let policy = host.yields[first.index()].as_ref().unwrap();
    assert_eq!(policy.reason, Some("condition"));
    assert_eq!(policy.condition, Some(0));
    assert!(policy.passes < 64);
    assert_eq!(host.observations[first.index()]["step"], "End");
    replay_bundle(&host.export()).unwrap();
    let current = host.session.decision_seat().unwrap();
    start(
        &mut host,
        current,
        "future-end",
        json!([{"type":"nextStep","step":"End"}]),
        64,
    );
    // The current End occurrence does not satisfy a newly installed policy.
    assert!(host.yields[current.index()].as_ref().unwrap().passes > 0);
    host.play(
        current.opponent(),
        "concede",
        &json!({"requestId":"concede"}),
    )
    .unwrap();
    assert_eq!(
        host.yields[current.index()].as_ref().unwrap().reason,
        Some("gameEnded")
    );
    replay_bundle(&host.export()).unwrap();
}

#[test]
fn yield_receipts_are_idempotent_and_share_the_move_namespace() {
    let mut host = ready_host();
    let seat = host.session.decision_seat().unwrap();
    let args = json!({"requestId":"one","revision":host.revision,
        "until":[{"type":"opponentCastsSpell"}]});
    let first = host.yield_request(seat, "yield", &args).unwrap();
    let commands = host.journal.len();
    assert_eq!(
        host.yield_request(seat, "yield", &args).unwrap()["receipt"],
        first["receipt"]
    );
    assert_eq!(commands, host.journal.len());
    assert_eq!(
        host.play(seat, "concede", &json!({"requestId":"one"}))
            .unwrap_err()
            .status,
        409
    );
    let cancelled = host
        .yield_request(seat, "cancelYield", &json!({"requestId":"cancel"}))
        .unwrap();
    assert_eq!(cancelled["yield"]["stopReason"], "cancelled");
    if cancelled["status"] == "waiting" {
        assert!(cancelled.get("observation").is_none());
        assert!(cancelled.get("revision").is_none());
    }
}

#[test]
fn invalid_or_stale_policies_cannot_change_the_match() {
    let mut host = ready_host();
    let seat = host.session.decision_seat().unwrap();
    for until in [
        json!([]),
        json!([{"type":"nextStep","step":"Untap"}]),
        json!([{"type":"opponentActs","extra":true}]),
        json!([{"type":"unknown"}]),
    ] {
        assert!(
            host.yield_request(
                seat,
                "yield",
                &json!({"requestId":"bad","revision":host.revision,"until":until})
            )
            .is_err()
        );
    }
    assert!(
        host.yield_request(
            seat,
            "yield",
            &json!({"requestId":"bad","revision":"stale",
        "until":[{"type":"opponentActs"}]})
        )
        .is_err()
    );
    assert!(host.yields.iter().all(Option::is_none));
}

#[test]
fn public_events_and_non_mana_menu_conditions_are_conservative() {
    let condition: Condition = serde_json::from_value(json!({"type":"opponentActs"})).unwrap();
    for kind in [
        "SpellCast",
        "AbilityActivated",
        "LandPlayed",
        "ManaAdded",
        "AttackDeclared",
        "BlockDeclared",
    ] {
        assert!(condition.matches_event(PlayerId::One, &json!({"type":kind,"seat":"p2"})));
        assert!(!condition.matches_event(PlayerId::One, &json!({"type":kind,"seat":"p1"})));
    }
    assert!(!condition.matches_event(PlayerId::One, &json!({"type":"CardDrawn","seat":"p2"})));
    let condition: Condition = serde_json::from_value(json!({"type":"nonManaAction"})).unwrap();
    assert!(!condition.matches_menu(
        &json!({"legalActions":[{"type":"PassPriority"},{"type":"ActivateManaAbility"}]})
    ));
    assert!(
        condition.matches_menu(
            &json!({"legalActions":[{"type":"PassPriority"},{"type":"BeginPayment"}]})
        )
    );
}

#[test]
fn latched_stop_survives_opponent_priority_without_automatically_passing() {
    let mut host = ready_host();
    let seat = host.session.decision_seat().unwrap();
    start(
        &mut host,
        seat,
        "end",
        json!([{"type":"nextStep","step":"End"}]),
        64,
    );
    pass_until(&mut host, "End", None);
    assert_eq!(host.observations[seat.index()]["step"], "End");
    assert!(host.observations[seat.index()]["forcedAction"].is_null());
    let mut policies = host.yields.clone();
    let policy = policies[seat.index()].as_mut().unwrap();
    policy.reason = None;
    policy.condition = None;
    policy.awaiting_stop = true;
    policy.until = serde_json::from_value(json!([{"type":"opponentCastsSpell"}])).unwrap();
    let mut observations = host.observations.clone();
    observations[seat.index()]["updates"]
        .as_array_mut()
        .unwrap()
        .push(json!({"type":"SpellCast","seat":crate::seat_name(seat.opponent())}));
    evaluate(&mut policies, &observations, Some(seat.opponent()), false);
    assert_eq!(
        policies[seat.index()].as_ref().unwrap().reason,
        Some("condition")
    );
    assert!(policies[seat.index()].as_ref().unwrap().awaiting_stop);
    let before = host.session.observe(seat);
    assert_eq!(advance(&mut host.session, &mut policies).unwrap(), 0);
    assert_eq!(host.session.observe(seat), before);
    assert!(!policies[seat.index()].as_ref().unwrap().awaiting_stop);
}

#[test]
fn own_required_decision_always_stops_even_without_a_matching_condition() {
    let mut host = ready_host();
    let seat = host.session.decision_seat().unwrap();
    start(
        &mut host,
        seat,
        "end",
        json!([{"type":"nextStep","step":"End"}]),
        64,
    );
    pass_until(&mut host, "End", None);
    let policy = host.yields[seat.index()].as_mut().unwrap();
    policy.reason = None;
    policy.awaiting_stop = true;
    policy.until = serde_json::from_value(json!([{"type":"opponentActs"}])).unwrap();
    host.observations[seat.index()]["decision"] = json!({"id":1});
    host.observations[seat.index()]["forcedAction"] = Value::Null;
    host.observations[seat.index()]["actionContext"]["ordinaryPriority"] = json!(false);
    host.evaluate_yields();
    assert_eq!(
        host.yields[seat.index()].as_ref().unwrap().reason,
        Some("decision")
    );
}

#[test]
fn next_step_uses_future_events_and_active_seat_filter() {
    let condition: Condition =
        serde_json::from_value(json!({"type":"nextStep","step":"End","activeSeat":"opponent"}))
            .unwrap();
    assert!(!condition.matches_event(
        PlayerId::One,
        &json!({"type":"StepChanged","step":"End","activePlayer":"p1"})
    ));
    assert!(condition.matches_event(
        PlayerId::One,
        &json!({"type":"StepChanged","step":"End","activePlayer":"p2"})
    ));
    assert!(!condition.matches_menu(&json!({"step":"End","activeSeat":"p2","legalActions":[]})));
}

#[test]
fn two_active_policies_consume_independent_bounded_pass_budgets() {
    let mut host = ready_host();
    let seat = host.session.decision_seat().unwrap();
    start(
        &mut host,
        seat,
        "end",
        json!([{"type":"nextStep","step":"End"}]),
        64,
    );
    pass_until(&mut host, "End", None);
    let template = host.yields[seat.index()].as_ref().unwrap().clone();
    for player in [PlayerId::One, PlayerId::Two] {
        let mut policy = template.clone();
        policy.reason = None;
        policy.condition = None;
        policy.awaiting_stop = true;
        policy.passes = 0;
        policy.max_passes = 3;
        policy.cursor = host.observations[player.index()]["updates"]
            .as_array()
            .unwrap()
            .len();
        policy.until = serde_json::from_value(json!([{"type":"opponentCastsSpell"}])).unwrap();
        host.yields[player.index()] = Some(policy);
    }
    let commands = host.journal.len();
    host.drive_yields();
    assert!(host.journal.len() - commands <= 6);
    for policy in host.yields.iter().flatten() {
        assert!(policy.passes > 0 && policy.passes <= 3);
    }
    let current = host.session.decision_seat().unwrap();
    assert!(
        host.yields[current.index()]
            .as_ref()
            .unwrap()
            .reason
            .is_some()
    );
    replay_bundle(&host.export()).unwrap();
}

#[test]
fn opponent_land_play_latches_until_the_yielded_seat_can_respond() {
    let mut host = ready_host();
    let seat = host.session.decision_seat().unwrap();
    start(
        &mut host,
        seat,
        "until-opponent",
        json!([{"type":"opponentActs"}]),
        64,
    );
    pass_until(&mut host, "PrecombatMain", Some(seat.opponent()));
    let opponent = host.session.decision_seat().unwrap();
    assert_eq!(opponent, seat.opponent());
    let land = host.session.discover(opponent)["legalActions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|action| action["type"] == "PlayLand")
        .unwrap()["index"]
        .clone();
    host.play(
        opponent,
        "play",
        &json!({"requestId":"land","revision":host.revision,
        "choices":[{"index":land}]}),
    )
    .unwrap();
    assert_eq!(host.session.decision_seat(), Some(opponent));
    let policy = host.yields[seat.index()].as_ref().unwrap();
    assert_eq!(policy.reason, Some("condition"));
    assert!(policy.awaiting_stop);
    host.play(
        opponent,
        "play",
        &json!({"requestId":"pass","revision":host.revision,
        "choices":[{"action":{"type":"PassPriority"}}]}),
    )
    .unwrap();
    assert_eq!(host.session.decision_seat(), Some(seat));
    assert_eq!(host.view(seat)["status"], "ready");
    assert_eq!(host.view(seat)["yield"]["stopReason"], "condition");
    replay_bundle(&host.export()).unwrap();
}

#[test]
fn legacy_replay_without_yields_remains_readable() {
    let host = ready_host();
    let mut legacy = host.export();
    legacy["nativeReplayVersion"] = json!(1);
    let mut session = host.config.session_version(1, false).unwrap();
    for command in legacy["commands"].as_array_mut().unwrap() {
        command.as_object_mut().unwrap().remove("forced");
        let seat = match command["seat"].as_str().unwrap() {
            "p1" => PlayerId::One,
            "p2" => PlayerId::Two,
            _ => unreachable!(),
        };
        session.apply_choice(seat, &command["choice"]).unwrap();
        session.advance_forced().unwrap();
    }
    legacy["observations"] =
        json!({"p1":session.observe(PlayerId::One),"p2":session.observe(PlayerId::Two)});
    replay_bundle(&legacy).unwrap();
}
