use super::*;

#[test]
fn session_api_preserves_both_seat_observations_and_every_priority_choice() {
    let mut web = WebGame::new("Sligh", "The Deck", "external", true, 42, None).unwrap();
    web.enable_session_api().unwrap();
    let mut native = penta::protocol::BotGame::new_with_format(
        penta::Format::OldSchool9394,
        "Sligh",
        "The Deck",
        penta::protocol::Opponent::External,
        PlayerId::Two,
        42,
    )
    .unwrap();
    let mut passes = 0;
    for _ in 0..160 {
        for (role, seat) in [("human", PlayerId::One), ("bot", PlayerId::Two)] {
            let observed: Value =
                serde_json::from_str(&web.session_observe_json(role).unwrap()).unwrap();
            let expected: Value = serde_json::from_str(&native.observe_json(seat)).unwrap();
            assert_eq!(observed, expected);
        }
        let Some(seat) = native.decision_seat() else {
            break;
        };
        let role = if seat == PlayerId::One {
            "human"
        } else {
            "bot"
        };
        assert_eq!(web.session_decision_role().as_deref(), Some(role));
        let observation: Value = serde_json::from_str(&native.observe_json(seat)).unwrap();
        let actions = observation["legalActions"].as_array().unwrap();
        let action = actions
            .iter()
            .find(|action| {
                !matches!(
                    action["type"].as_str(),
                    Some("PassPriority" | "ActivateManaAbility" | "TakeMulligan")
                )
            })
            .unwrap_or(&actions[0]);
        passes += usize::from(action["type"] == "PassPriority");
        let index = u32::try_from(action["index"].as_u64().unwrap()).unwrap();
        web.session_act(role, index).unwrap();
        native.act(index as usize).unwrap();
    }
    assert!(
        passes > 10,
        "the API must return even ordinary passing decisions"
    );
    let replay = WebGame::from_replay_json(&web.replay_json()).unwrap();
    for role in ["human", "bot"] {
        assert_eq!(
            web.session_observe_json(role).unwrap(),
            replay.session_observe_json(role).unwrap()
        );
    }
}
