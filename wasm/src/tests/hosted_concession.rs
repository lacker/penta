use super::*;

#[test]
fn hosted_concession_history_replays_without_an_indexed_action() {
    let config = json!({
        "format": "old-school-93-94", "p1Deck": "Sligh", "p2Deck": "Goblins",
        "opponent": "external", "seed": 31,
    })
    .to_string();
    let mut game = HostedGame::from_config_json(&config).unwrap();
    game.concede("p2").unwrap();
    let history = game.history_json();
    assert_eq!(
        serde_json::from_str::<Value>(&history).unwrap(),
        json!([{ "concede": "p2" }])
    );
    let replay = HostedGame::replay_config_json(&config, &history).unwrap();
    for seat in ["p1", "p2"] {
        assert_eq!(
            game.observe_json(seat).unwrap(),
            replay.observe_json(seat).unwrap()
        );
    }
}
