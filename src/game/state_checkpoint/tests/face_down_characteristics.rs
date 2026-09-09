use super::*;

#[test]
fn checkpoint_round_trips_the_controllers_face_down_characteristics() {
    for (seed, viewer, characteristics, expected_tag) in [
        (
            81_020,
            PlayerId::One,
            crate::card::face_down::manifest(),
            "ordinaryTwoTwo",
        ),
        (
            81_021,
            PlayerId::One,
            crate::card::face_down::cloak(),
            "wardTwoTwo",
        ),
    ] {
        let mut game = crate::game::tests::ready_game();
        game.battlefield.clear();
        let mut angel =
            crate::game::tests::creature(10_000, crate::card::cards::SERRA_ANGEL, PlayerId::One);
        angel.face_down = Some(characteristics);
        angel.turn_up_for_mana_cost = true;
        let id = angel.card.id;
        game.battlefield.push(angel);

        let (wire, rebuilt) = rebuild_current_checkpoint(&game, viewer, seed);
        let observed = wire["battlefield"]
            .as_array()
            .expect("battlefield array")
            .iter()
            .find(|permanent| permanent["objectId"] == json!(id.0))
            .expect("the permanent is observed");
        assert_eq!(
            observed["definition"],
            json!(crate::card::cards::SERRA_ANGEL.get())
        );
        let stored = wire["checkpoint"]["battlefield"]
            .as_array()
            .expect("checkpoint battlefield array")
            .iter()
            .find(|permanent| permanent["objectId"] == json!(id.0))
            .expect("the permanent is checkpointed");
        assert_eq!(stored["faceDown"], expected_tag);

        let permanent = rebuilt
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("the face-down permanent reconstructs");
        assert_eq!(permanent.face_down, Some(characteristics));
        assert!(permanent.turn_up_for_mana_cost);
        assert_eq!(permanent.card.definition, crate::card::cards::SERRA_ANGEL,);
    }
}

#[test]
fn an_unlocated_card_specific_face_down_value_fails_closed() {
    let mut game = crate::game::tests::ready_game();
    game.battlefield.clear();
    let mut angel =
        crate::game::tests::creature(10_000, crate::card::cards::SERRA_ANGEL, PlayerId::One);
    angel.face_down = Some(crate::FaceDownCharacteristics::land(
        "Face-down Forest",
        &["Forest"],
    ));
    game.battlefield.push(angel);

    let observation = game.observe(PlayerId::One);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        false,
        &game.legal_actions(PlayerId::One),
    );
    assert_eq!(wire["checkpoint"]["hasDeferredState"], true);
    assert!(
        wire["checkpoint"]["battlefield"][0]
            .get("faceDown")
            .is_none(),
        "an arbitrary runtime value is not flattened into a misleading preset"
    );
}

#[test]
fn face_down_checkpoint_refuses_to_reconstruct_a_hidden_identity() {
    let mut game = crate::game::tests::ready_game();
    let mut permanent =
        crate::game::tests::creature(10_000, crate::card::cards::SERRA_ANGEL, PlayerId::One);
    permanent.face_down = Some(crate::card::face_down::cloak());
    game.phased_out.push(permanent);
    let observation = game.observe(PlayerId::Two);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        false,
        &game.legal_actions(PlayerId::Two),
    );
    assert_eq!(wire["checkpoint"]["hasDeferredState"], true);
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, PlayerId::Two),
        81_022,
    )
    .err()
    .expect("the missing hidden identity cannot reconstruct");
    assert!(
        error.contains("hidden face-down object hypotheses"),
        "{error}"
    );
}
