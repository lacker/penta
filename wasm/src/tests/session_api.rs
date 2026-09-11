use super::*;

#[test]
fn session_api_face_down_cast_withholds_identity_from_events_and_browser_beats() {
    let mut web = WebGame::new("Sligh", "The Deck", "external", false, 42, None).unwrap();
    web.enable_session_api().unwrap();
    for role in ["bot", "human"] {
        let seat = web.session.decision_seat().unwrap();
        let observation = web.session.observe(seat);
        let keep = penta::protocol::protocol_actions(&observation)
            .iter()
            .position(|action| matches!(action, Action::KeepHand))
            .unwrap();
        web.session_act(role, u32::try_from(keep).unwrap()).unwrap();
    }
    let opponent = web.human.opponent();
    web.session
        .engine_mut()
        .set_hand(opponent, &[penta::card::cards::EXALTED_ANGEL])
        .unwrap();
    let human = web.human;
    web.session.engine_mut().set_hand(human, &[]).unwrap();
    for _ in 0..3 {
        web.session
            .engine_mut()
            .put_onto_battlefield(opponent, penta::card::cards::MOUNTAIN)
            .unwrap();
    }
    let observation = web.session.observe(opponent);
    let cast = penta::protocol::protocol_actions(&observation)
        .iter()
        .position(|action| matches!(action, Action::CastSpell { .. }))
        .unwrap();
    web.session_act("bot", u32::try_from(cast).unwrap())
        .unwrap();
    let view: Value = serde_json::from_str(&web.session_observe_json("human").unwrap()).unwrap();
    assert!(
        view["updates"]
            .as_array()
            .unwrap()
            .iter()
            .any(|event| event["type"] == "SpellCast" && event["card"]["faceDown"] == true)
    );
    assert!(!view.to_string().contains("Exalted Angel"));
    assert!(!web.snapshot().to_string().contains("Exalted Angel"));
}

fn domri_session(top: CardDefinitionId) -> WebGame {
    let mut web = WebGame::new(
        "Greer G/R Aggro",
        "Briksza Naya Midrange",
        "external",
        true,
        42,
        Some("isd-m14-standard".into()),
    )
    .unwrap();
    web.enable_session_api().unwrap();
    for role in ["human", "bot"] {
        let view: Value = serde_json::from_str(&web.session_observe_json(role).unwrap()).unwrap();
        let keep = view["legalActions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|action| action["type"] == "KeepHand")
            .unwrap();
        web.session_act(
            role,
            u32::try_from(keep["index"].as_u64().unwrap()).unwrap(),
        )
        .unwrap();
    }
    let human = web.human;
    web.session
        .engine_mut()
        .set_hand(human, &[penta::card::cards::MOUNTAIN])
        .unwrap();
    web.session
        .engine_mut()
        .set_hand(human.opponent(), &[])
        .unwrap();
    web.session.engine_mut().set_library(human, &[top]).unwrap();
    let domri = web
        .session
        .engine_mut()
        .put_onto_battlefield(human, penta::card::cards::DOMRI_RADE)
        .unwrap();
    let observation = web.session.observe(human);
    let index = penta::protocol::protocol_actions(&observation)
        .iter()
        .position(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == domri),
        )
        .unwrap();
    web.session_act("human", u32::try_from(index).unwrap())
        .unwrap();
    web
}

#[test]
fn session_api_forced_private_inspection_retains_information_for_only_its_viewer() {
    let web = domri_session(penta::card::cards::MOUNTAIN);
    let human: Value = serde_json::from_str(&web.session_observe_json("human").unwrap()).unwrap();
    let bot: Value = serde_json::from_str(&web.session_observe_json("bot").unwrap()).unwrap();
    assert!(human["decision"].is_null());
    assert!(human["forcedAction"].is_null());
    let inspections = human["updates"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|update| update["type"] == "AutomaticDecision")
        .collect::<Vec<_>>();
    assert_eq!(inspections.len(), 1);
    assert!(inspections[0].to_string().contains("Mountain"));
    assert!(
        web.snapshot()["events"]
            .as_array()
            .unwrap()
            .iter()
            .any(|line| line
                .as_str()
                .is_some_and(|line| line.starts_with("Automatic:") && line.contains("Mountain")))
    );
    assert!(!bot["updates"].to_string().contains("AutomaticDecision"));
    assert!(!bot["updates"].to_string().contains("Mountain"));
    assert_eq!(
        human.to_string(),
        serde_json::from_str::<Value>(&web.session_observe_json("human").unwrap())
            .unwrap()
            .to_string(),
        "reads do not consume history"
    );
}

#[test]
fn session_api_optional_reveal_stops_and_public_identity_survives_forced_resolution() {
    let mut web = domri_session(penta::card::cards::GHOR_CLAN_RAMPAGER);
    let human: Value = serde_json::from_str(&web.session_observe_json("human").unwrap()).unwrap();
    assert_eq!(human["decision"]["minimum"], 0);
    assert_eq!(human["decision"]["maximum"], 1);
    assert!(
        human["forcedAction"].is_null(),
        "revealing the creature is optional"
    );
    let option = human["decision"]["options"][0]["id"].clone();
    web.choose_decision(
        u32::try_from(human["decision"]["id"].as_u64().unwrap()).unwrap(),
        &json!([option]).to_string(),
    )
    .unwrap();
    for role in ["human", "bot"] {
        let view: Value = serde_json::from_str(&web.session_observe_json(role).unwrap()).unwrap();
        assert!(
            view["updates"]
                .as_array()
                .unwrap()
                .iter()
                .any(|event| event["type"] == "CardRevealed"
                    && event["card"]["name"] == "Ghor-Clan Rampager")
        );
        assert!(!view["updates"].to_string().contains("GameStarted"));
    }
}

#[test]
fn session_api_skips_forced_actions_preserving_native_choices_and_replay() {
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
    let mut forced = 0;
    let mut mana_choices = 0;
    for _ in 0..160 {
        for (role, seat) in [("human", PlayerId::One), ("bot", PlayerId::Two)] {
            let mut observed: Value =
                serde_json::from_str(&web.session_observe_json(role).unwrap()).unwrap();
            observed.as_object_mut().unwrap().remove("updates");
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
        assert!(observation["forcedAction"].is_null());
        mana_choices += usize::from(
            actions
                .iter()
                .any(|action| action["type"] == "ActivateManaAbility"),
        );
        let index = u32::try_from(action["index"].as_u64().unwrap()).unwrap();
        web.session_act(role, index).unwrap();
        native.act(index as usize).unwrap();
        loop {
            let Some(seat) = native.decision_seat() else {
                break;
            };
            let current: Value = serde_json::from_str(&native.observe_json(seat)).unwrap();
            if current["forcedAction"].is_null() {
                break;
            }
            let index = current["legalActions"]
                .as_array()
                .unwrap()
                .iter()
                .position(|action| {
                    let mut action = action.clone();
                    action.as_object_mut().unwrap().remove("index");
                    action == current["forcedAction"]
                })
                .unwrap();
            native.act(index).unwrap();
            forced += 1;
        }
    }
    assert!(
        forced > 10,
        "unique continuations should require no client commands"
    );
    assert!(mana_choices > 10, "optional mana actions remain choices");
    let replay = WebGame::from_replay_json(&web.replay_json()).unwrap();
    for role in ["human", "bot"] {
        assert_eq!(
            web.session_observe_json(role).unwrap(),
            replay.session_observe_json(role).unwrap()
        );
    }
}
