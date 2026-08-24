#[test]
fn gilded_drakes_target_fizzle_exception_survives_checkpoint_reconstruction() {
    let mut game = crate::game::tests::ready_game();
    game.battlefield.push(crate::game::tests::creature(
        91_010,
        crate::card::cards::SERRA_ANGEL,
        PlayerId::Two,
    ));
    let angel = game.battlefield[0].card.id;
    let drake = crate::game::tests::card(
        91_000,
        crate::card::cards::GILDED_DRAKE,
        PlayerId::One,
    );
    game.players[PlayerId::One.index()].hand.push(drake.clone());
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        crate::game::tests::cast_action(drake.id, Vec::new(), Vec::new(), 0),
    )
    .expect("Gilded Drake can be cast");
    crate::game::tests::pass_until_decision(&mut game);
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the enters trigger asks for a target");
    let target = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(card, _)| card == angel))
        .expect("the opponent's Angel is a legal target")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![target],
        },
    )
    .expect("the target is selected");
    let drake = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == crate::card::cards::GILDED_DRAKE)
        .expect("the Drake entered before its trigger")
        .card
        .id;
    assert!(matches!(
        game.stack.last().and_then(|object| object.ability.as_ref()),
        Some(StackAbilityPayload {
            resolver: StackAbilityResolver::DeclarativeIgnoringTargetFizzle(_),
            ..
        })
    ));

    let (_, mut rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 91_020);
    assert!(matches!(
        rebuilt
            .stack
            .last()
            .and_then(|object| object.ability.as_ref()),
        Some(StackAbilityPayload {
            resolver: StackAbilityResolver::DeclarativeIgnoringTargetFizzle(_),
            ..
        })
    ));

    rebuilt.destroy_permanent(angel);
    for _ in 0..8 {
        if rebuilt.stack.is_empty() && rebuilt.pending_triggers.is_empty() {
            break;
        }
        let player = rebuilt.priority;
        rebuilt
            .apply(player, Action::PassPriority)
            .expect("priority passes while the reconstructed trigger resolves");
    }
    assert!(
        rebuilt
            .battlefield
            .iter()
            .all(|permanent| permanent.card.id != drake),
        "the reconstructed trigger resolves and sacrifices the Drake",
    );
}
