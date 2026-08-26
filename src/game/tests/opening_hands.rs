use super::*;

fn definition(game: &Game, name: &str) -> CardDefinitionId {
    game.catalog
        .find_by_name(name)
        .unwrap_or_else(|| panic!("{name} is cataloged"))
}

fn opening_game(p1: &[&str], p2: &[&str]) -> Game {
    let mut game = ready_game();
    game.pregame = Some(Pregame::Mulligan(PlayerId::One));
    game.priority = PlayerId::One;
    game.pending_decisions.clear();
    for (player, names) in [(PlayerId::One, p1), (PlayerId::Two, p2)] {
        game.players[player.index()].hand = names
            .iter()
            .enumerate()
            .map(|(index, name)| {
                card(
                    20_000 + u32::try_from(player.index() * 100 + index).unwrap(),
                    definition(&game, name),
                    player,
                )
            })
            .collect();
    }
    game
}

fn keep_both(game: &mut Game) {
    game.apply(PlayerId::One, Action::KeepHand).unwrap();
    game.apply(PlayerId::Two, Action::KeepHand).unwrap();
}

fn choose_nothing(game: &mut Game, player: PlayerId) {
    let decision = game
        .observe(player)
        .decision
        .expect("an opening-hand finish decision is waiting");
    game.apply(
        player,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .unwrap();
}

fn first_pregame_action(game: &Game, player: PlayerId) -> Action {
    game.legal_actions(player)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { .. }))
        .expect("a declarative pregame action is offered")
}

fn reveal_providence_and_resolve_first_upkeep(game: &mut Game) {
    keep_both(game);
    game.apply(PlayerId::One, first_pregame_action(game, PlayerId::One))
        .unwrap();
    choose_nothing(game, PlayerId::One);
    game.active_player = PlayerId::One;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    pass_priority_pair(game);
}

#[test]
fn every_in_scope_paper_card_declares_a_pregame_ability() {
    let game = ready_game();
    let cards = [
        "Serum Powder",
        "Leyline of the Meek",
        "Leyline of Singularity",
        "Leyline of the Void",
        "Leyline of Lightning",
        "Leyline of Lifeforce",
        "Gemstone Caverns",
        "Leyline of Sanctity",
        "Leyline of Anticipation",
        "Leyline of Punishment",
        "Leyline of Vitality",
        "Chancellor of the Annex",
        "Chancellor of the Spires",
        "Chancellor of the Dross",
        "Chancellor of the Forge",
        "Chancellor of the Tangle",
        "Providence",
        "Sphinx of Foresight",
        "Leyline of Combustion",
        "Leyline of Abundance",
        "Leyline of the Guildpact",
        "Devourer of Destiny",
        "Leyline of Hope",
        "Leyline of Resonance",
        "Leyline of Mutation",
        "Leyline of Transformation",
        "Leyline Axe",
        "Quicksilver, Brash Blur",
    ];
    assert_eq!(cards.len(), 28);
    for name in cards {
        let definition = game.catalog.get(definition(&game, name)).unwrap();
        assert!(
            definition
                .rules
                .ability_clauses()
                .iter()
                .any(|ability| { matches!(ability.definition, DeclarativeAbilityDef::Pregame(_)) }),
            "{name} declares its opening-hand behavior structurally"
        );
    }
}

#[test]
fn leylines_are_optional_and_resolve_in_starting_player_order() {
    let mut game = opening_game(
        &["Leyline of Sanctity", "Leyline of Anticipation"],
        &["Leyline of the Void"],
    );
    let sanctity = definition(&game, "Leyline of Sanctity");
    let anticipation = definition(&game, "Leyline of Anticipation");
    let void = definition(&game, "Leyline of the Void");
    keep_both(&mut game);

    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
    game = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &hidden,
        20_500,
    )
    .expect("opening-hand actions reconstruct");

    assert_eq!(game.decision_player(), Some(PlayerId::One));
    let first = first_pregame_action(&game, PlayerId::One);
    game.apply(PlayerId::One, first).unwrap();
    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(game.battlefield[0].card.definition, sanctity);

    // Declining the remaining action leaves its source in hand and advances
    // to the non-starting player's opening-hand actions.
    choose_nothing(&mut game, PlayerId::One);
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == anticipation)
    );
    assert_eq!(game.decision_player(), Some(PlayerId::Two));
    game.apply(PlayerId::Two, first_pregame_action(&game, PlayerId::Two))
        .unwrap();
    choose_nothing(&mut game, PlayerId::Two);

    assert!(!game.in_pregame());
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.controller == PlayerId::Two && permanent.card.definition == void
    }));
}

#[test]
fn gemstone_caverns_is_only_offered_to_the_nonstarting_player_with_a_card_to_exile() {
    let mut game = opening_game(
        &["Gemstone Caverns", "Mountain"],
        &["Gemstone Caverns", "Gemstone Caverns", "Forest"],
    );
    let caverns = definition(&game, "Gemstone Caverns");
    keep_both(&mut game);

    assert_eq!(game.decision_player(), Some(PlayerId::Two));
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            let Action::ActivateAbility { cost_objects, .. } = action else {
                return false;
            };
            cost_objects.first().is_some_and(|cost| {
                game.players[1]
                    .hand
                    .iter()
                    .any(|card| card.id == *cost && card.definition == caverns)
            })
        })
        .expect("one Caverns may exile the other");
    let Action::ActivateAbility { cost_objects, .. } = &action else {
        unreachable!()
    };
    assert_eq!(cost_objects.len(), 1);
    game.apply(PlayerId::Two, action).unwrap();
    choose_nothing(&mut game, PlayerId::Two);

    let entered = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == caverns)
        .expect("Gemstone Caverns begins on the battlefield");
    assert_eq!(entered.controller, PlayerId::Two);
    assert_eq!(entered.counters(CounterKind::Luck), 1);
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == caverns)
    );
    assert!(
        game.legal_actions(PlayerId::Two)
            .iter()
            .all(|action| !matches!(action, Action::ActivateAbility { .. })),
        "the Caverns exiled as a cost leaves no stale action"
    );
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == caverns)
    );
}

#[test]
fn serum_powder_replaces_the_hand_without_counting_as_a_mulligan() {
    let mut game = opening_game(&["Serum Powder", "Mountain", "Mountain"], &["Forest"]);
    let original = game.players[0]
        .hand
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    let library_before = game.players[0].library.len();
    let action = first_pregame_action(&game, PlayerId::One);
    game.apply(PlayerId::One, action).unwrap();

    assert_eq!(game.mulligans[0], 0);
    assert_eq!(game.players[0].hand.len(), original.len());
    assert_eq!(
        game.players[0].library.len(),
        library_before - original.len()
    );
    assert_eq!(game.players[0].exile.len(), original.len());
    assert_eq!(game.pregame, Some(Pregame::Mulligan(PlayerId::One)));
}

#[test]
fn chancellor_reveal_installs_a_once_only_first_upkeep_trigger() {
    let mut game = opening_game(&["Chancellor of the Forge"], &[]);
    let chancellor = definition(&game, "Chancellor of the Forge");
    keep_both(&mut game);
    game.apply(PlayerId::One, first_pregame_action(&game, PlayerId::One))
        .unwrap();
    choose_nothing(&mut game, PlayerId::One);

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == chancellor)
    );
    assert_eq!(game.installed_triggers.len(), 1);
    game.active_player = PlayerId::One;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    assert!(game.installed_triggers.is_empty());
    pass_priority_pair(&mut game);
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.controller == PlayerId::One
            && matches!(permanent.card.definition, ObjectKind::Token)
    }));
}

#[test]
fn providence_sets_life_through_the_normal_gain_or_loss_event() {
    for (starting_life, gained, lost) in [(20, 6, None), (26, 0, None), (40, 0, Some(14))] {
        let mut game = opening_game(&["Providence"], &[]);
        game.players[0].life = starting_life;
        reveal_providence_and_resolve_first_upkeep(&mut game);

        assert_eq!(game.players[0].life, 26);
        assert_eq!(game.life_gained_this_turn[0], gained);
        assert_eq!(
            game.events
                .iter()
                .filter_map(|event| match event {
                    GameEvent::LifeLost {
                        player: PlayerId::One,
                        amount,
                    } => Some(*amount),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            lost.into_iter().collect::<Vec<_>>(),
        );
    }
}

#[test]
fn providence_life_gain_is_modified_once_without_compensating_back_to_26() {
    let mut game = opening_game(&["Providence"], &[]);
    game.battlefield
        .push(creature(20_500, cards::RHOX_FAITHMENDER, PlayerId::One));
    reveal_providence_and_resolve_first_upkeep(&mut game);

    assert_eq!(game.players[0].life, 32);
    assert_eq!(game.life_gained_this_turn[0], 12);
    assert!(game.events.iter().all(|event| !matches!(
        event,
        GameEvent::LifeLost {
            player: PlayerId::One,
            ..
        }
    )));
}

#[test]
fn scry_preserves_explicit_bottom_and_top_order_and_reconstructs() {
    let mut game = ready_game();
    let mountain = definition(&game, "Mountain");
    let forest = definition(&game, "Forest");
    let island = definition(&game, "Island");
    game.players[0].library = vec![
        card(30_000, mountain, PlayerId::One),
        card(30_001, forest, PlayerId::One),
        card(30_002, island, PlayerId::One),
    ];
    game.queue_scry(PlayerId::One, 3);

    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
    game = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &hidden,
        30_100,
    )
    .expect("a pending scry reconstructs");

    let first = game.observe(PlayerId::One).decision.unwrap();
    let forest_option = first
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(_, card)| card.card_definition() == Some(forest))
        })
        .unwrap()
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: first.id,
            options: vec![forest_option],
        },
    )
    .unwrap();
    let second = game.observe(PlayerId::One).decision.unwrap();
    let option_for = |wanted| {
        second
            .options
            .iter()
            .find(|option| {
                option
                    .card
                    .is_some_and(|(_, card)| card.card_definition() == Some(wanted))
            })
            .unwrap()
            .id
    };
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: second.id,
            options: vec![option_for(mountain), option_for(island)],
        },
    )
    .unwrap();

    assert_eq!(
        game.players[0]
            .library
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![forest, island, mountain],
        "the vector is bottom-to-top"
    );
}
