#[test]
fn an_attached_aura_search_checkpoint_preserves_its_player() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let deck = crate::Deck {
        main: vec![crate::card::cards::MOUNTAIN; 60],
        sideboard: Vec::new(),
    };
    let mut game = Game::new(catalog.clone(), [deck.clone(), deck], 45).expect("game starts");
    let player = PlayerId::One;
    game.players[player.index()].library.clear();
    game.players[player.index()].library.push(crate::game::tests::card(
        90_001,
        crate::card::cards::CURSE_OF_DEATH_S_HOLD,
        player,
    ));
    game.queue_zone_search(
        player,
        ZoneKind::Library,
        crate::card::ObjectPredicateDef::Subtype("Curse"),
        0,
        1,
        true,
        ZoneKind::Battlefield,
        crate::card::ZonePlacement::Top,
        true,
        None,
        None,
        false,
        Some(PlayerId::Two),
        GameObjectId(90_000),
        player,
    );

    let (viewer, wire) = checkpoint_wire(&game);
    let mut rebuilt = Game::from_observation_checkpoint(
        catalog,
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        1_009,
    )
    .expect("the attached-Aura search reconstructs");
    assert!(matches!(
        rebuilt.pending_decisions[0].continuation,
        DecisionContinuation::SearchZone {
            attached_player: Some(PlayerId::Two),
            ..
        }
    ));

    let decision = rebuilt
        .observe(player)
        .decision
        .expect("the rebuilt search is still waiting");
    rebuilt.choose_decision(player, decision.id, &[decision.options[0].id]);
    let curse = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == crate::card::cards::CURSE_OF_DEATH_S_HOLD)
        .expect("the selected Curse enters");
    assert_eq!(curse.attached_player, Some(PlayerId::Two));
}
