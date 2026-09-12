use super::*;

#[test]
fn lost_caverns_named_counters_round_trip_with_their_printed_names() {
    let mut game = crate::game::tests::ready_game();
    game.battlefield.clear();
    let examples = [
        (crate::poc::cards::GRASPING_SHADOWS, "dread"),
        (crate::poc::cards::CONTESTED_GAME_BALL, "point"),
        (crate::poc::cards::TREASURE_MAP, "landmark"),
    ];
    for (definition, name) in examples {
        let id = game
            .put_onto_battlefield(PlayerId::One, definition)
            .unwrap();
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == id)
            .unwrap()
            .add_counters(CounterKind::named(name), 2);
    }
    let (wire, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 94_501);
    let serialized = serde_json::to_string(&wire).unwrap();
    for (definition, name) in examples {
        assert!(
            serialized.contains(name),
            "checkpoint must name {name} counters"
        );
        let permanent = rebuilt
            .battlefield
            .iter()
            .find(|p| p.card.definition == definition)
            .unwrap();
        assert_eq!(permanent.counters(CounterKind::named(name)), 2);
    }
}
