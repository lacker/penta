fn game_paused_during_wheel_resolution() -> (Game, GameObjectId) {
    let mut game = crate::game::tests::ready_game();
    let wheel = crate::game::tests::card(
        80_950,
        crate::card::cards::WHEEL_OF_FORTUNE,
        PlayerId::One,
    );
    game.players[PlayerId::One.index()].hand.push(wheel.clone());
    game.players[PlayerId::One.index()].mana_pool.red = 3;
    game.players[PlayerId::One.index()].library = (80_951..80_957)
        .map(|id| crate::game::tests::card(id, crate::card::cards::PLAINS, PlayerId::One))
        .chain(std::iter::once(crate::game::tests::card(
            80_957,
            crate::card::cards::TERMINUS,
            PlayerId::One,
        )))
        .collect();

    game.apply(
        PlayerId::One,
        crate::game::tests::cast_action(wheel.id, Vec::new(), Vec::new(), 0),
    )
    .expect("Wheel can be cast");
    let resolving_id = game.stack.last().expect("Wheel is on the stack").id;
    game.apply(PlayerId::One, Action::PassPriority)
        .expect("the caster passes");
    game.apply(PlayerId::Two, Action::PassPriority)
        .expect("the opponent passes to resolve Wheel");

    assert!(matches!(
        game.pending_decisions.first(),
        Some(pending)
            if pending.observation.visibility == crate::DecisionVisibility::Private
                && matches!(pending.continuation, DecisionContinuation::DrawActionWindow { .. })
    ));
    let resolving = game
        .pending_procedures
        .iter()
        .find_map(|procedure| match procedure {
            crate::game::PendingProcedure::FinishStackResolution { object, resolved }
                if *resolved && object.id == resolving_id =>
            {
                Some(object.as_ref())
            }
            _ => None,
        })
        .expect("Wheel remains a resolving stack object");
    assert_eq!(
        game.retired_objects.get(&resolving.id),
        Some(&RetiredObject::Stack(Box::new(resolving.clone())))
    );
    (game, resolving_id)
}

fn finish_stack_snapshot(wire: &mut Value) -> &mut Value {
    wire["checkpoint"]["pendingProcedures"]
        .as_array_mut()
        .expect("pending procedures are an array")
        .iter_mut()
        .find(|procedure| procedure["kind"] == "finishStackResolution")
        .expect("the deferred stack completion is serialized")
}

fn rejected_finish_stack_tamper(
    game: &Game,
    wire: &Value,
    mutate: impl FnOnce(&mut Value),
) -> String {
    let mut tampered = wire.clone();
    mutate(finish_stack_snapshot(&mut tampered));
    Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &tampered,
        &true_hidden_hypothesis(game, PlayerId::One),
        80_958,
    )
    .expect_err("the altered deferred completion must be rejected")
}

#[test]
fn a_deferred_stack_completion_round_trips_and_rejects_tampering() {
    let (game, resolving_id) = game_paused_during_wheel_resolution();
    let (wire, mut rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 80_959);
    assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
    assert!(wire["checkpoint"]["retiredObjects"]
        .as_array()
        .is_some_and(|retired| retired.iter().any(|entry| {
            entry["kind"] == "stack"
                && entry["object"]["objectId"] == u64::from(resolving_id.0)
        })));

    let decision = rebuilt.pending_decisions[0].observation.id;
    rebuilt
        .apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision,
                options: Vec::new(),
            },
        )
        .expect("the rebuilt game resumes the declined draw action");
    assert!(rebuilt.pending_procedures.is_empty());
    assert!(rebuilt.players[PlayerId::One.index()]
        .graveyard
        .iter()
        .any(|card| card.definition == crate::card::cards::WHEEL_OF_FORTUNE));

    let definition_error = rejected_finish_stack_tamper(&game, &wire, |procedure| {
        procedure["object"]["objectKind"]["definition"] =
            json!(crate::card::cards::ISLAND.get());
    });
    assert!(
        definition_error.contains("does not match its retired stack object"),
        "unexpected error: {definition_error}"
    );

    let object_error = rejected_finish_stack_tamper(&game, &wire, |procedure| {
        procedure["object"]["objectId"] = json!(4_000_000_000_u32);
    });
    assert!(
        object_error.contains("does not match its retired stack object"),
        "unexpected error: {object_error}"
    );

    let resolved_error = rejected_finish_stack_tamper(&game, &wire, |procedure| {
        procedure["resolved"] = json!(false);
    });
    assert!(
        resolved_error.contains("cannot represent a failed resolution"),
        "unexpected error: {resolved_error}"
    );
}
