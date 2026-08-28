#[test]
fn a_supported_draw_action_window_rebuilds_and_resumes() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let deck = crate::Deck {
        main: vec![crate::card::cards::MOUNTAIN; 60],
        sideboard: vec![crate::card::cards::FOREST],
    };
    let mut game = Game::new(catalog.clone(), [deck.clone(), deck], 43).expect("game starts");
    let player = PlayerId::One;
    let card = game.players[player.index()].hand[0].id;
    game.players[player.index()].hand[0] =
        crate::game::tests::card(card.0, crate::card::cards::TERMINUS, player);
    game.cards_drawn_this_turn[player.index()] = 1;
    game.drawn_this_turn[player.index()] = vec![card];
    game.queue_draw_action_window(player, card);

    let observation = game.observe(player);
    let actions = crate::protocol::protocol_actions(&observation);
    let observation_json = crate::protocol::observation_json_for_format(
        &catalog,
        game.format,
        &observation,
        true,
        &actions,
    );
    let definitions = |cards: &[CardInstance]| {
        cards
            .iter()
            .map(|card| card.definition.get())
            .collect::<Vec<_>>()
    };
    let hidden = json!({
        "hands": {
            "p2": definitions(&game.players[PlayerId::Two.index()].hand),
        },
        "libraries": {
            "p1": definitions(&game.players[PlayerId::One.index()].library),
            "p2": definitions(&game.players[PlayerId::Two.index()].library),
        },
        "outsideGame": {
            "p1": definitions(&game.players[PlayerId::One.index()].outside_game),
            "p2": definitions(&game.players[PlayerId::Two.index()].outside_game),
        },
    });

    assert_eq!(observation_json["checkpoint"]["hasDeferredState"], false);
    let mut missing_outside_game = hidden.clone();
    missing_outside_game
        .as_object_mut()
        .expect("hidden hypothesis is an object")
        .remove("outsideGame");
    let error = Game::from_observation_checkpoint(
        catalog.clone(),
        game.format,
        &observation_json,
        &missing_outside_game,
        1_007,
    )
    .expect_err("outside-game contents cannot be discarded during reconstruction");
    assert!(error.contains("outsideGame"));
    let mut rebuilt =
        Game::from_observation_checkpoint(catalog, game.format, &observation_json, &hidden, 1_007)
            .expect("supported decision reconstructs");
    assert_eq!(
        std::array::from_fn::<_, 2, _>(|index| {
            rebuilt.players[index]
                .outside_game
                .iter()
                .map(|card| card.definition)
                .collect::<Vec<_>>()
        }),
        [
            vec![crate::card::cards::FOREST],
            vec![crate::card::cards::FOREST]
        ],
        "outside-game hypotheses survive as Ring choices"
    );
    assert_eq!(rebuilt.pending_decisions.len(), 1);
    let rebuilt_observation = rebuilt.observe(player);
    assert_eq!(
        crate::protocol::protocol_actions(&rebuilt_observation),
        actions,
        "the rebuilt decision offers the same indexed actions"
    );
    assert!(matches!(
        rebuilt.pending_decisions[0].continuation,
        DecisionContinuation::DrawActionWindow { card: rebuilt_card } if rebuilt_card == card
    ));
    let decision = rebuilt.pending_decisions[0].observation.id;
    rebuilt.choose_decision(player, decision, &[1]);
    let [trigger] = rebuilt.pending_triggers.as_slice() else {
        panic!("revealing creates one linked Miracle trigger");
    };
    assert_eq!(trigger.source.object, card);
    assert_eq!(
        trigger.resolver,
        StackAbilityResolver::CastOffer(crate::card::AlternativeCastKindDef::Miracle)
    );
}

fn game_with_draw_action_window(definition: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = crate::game::tests::ready_game();
    let player = PlayerId::One;
    let card = crate::game::tests::card(80_900, definition, player);
    let card_id = card.id;
    game.players[player.index()].hand.push(card);
    game.cards_drawn_this_turn[player.index()] = 1;
    game.drawn_this_turn[player.index()] = vec![card_id];
    game.add_unrestricted_mana(player, ManaColor::White, 1);
    game.queue_draw_action_window(player, card_id);
    (game, card_id)
}

fn wire_for_viewer(game: &Game, viewer: PlayerId) -> Value {
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    )
}

#[test]
fn an_ordinary_draw_action_window_settles_before_its_checkpoint_round_trip() {
    let (game, card) = game_with_draw_action_window(crate::card::cards::PLAINS);
    let player = PlayerId::One;
    assert!(
        game.pending_decisions.is_empty(),
        "an ordinary card's empty draw action resolves atomically"
    );

    let wire = wire_for_viewer(&game, player);
    assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
    let rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, player),
        80_898,
    )
    .expect("the owner can reconstruct the settled ordinary draw");
    assert!(rebuilt.pending_decisions.is_empty());
    assert!(rebuilt.pending_triggers.is_empty());
    assert_eq!(rebuilt.next_decision_id, game.next_decision_id);
    assert!(
        rebuilt.players[player.index()]
            .hand
            .iter()
            .any(|held| held.id == card),
        "the drawn ordinary card remains in hand"
    );
}

#[test]
fn a_draw_action_window_checkpoint_cannot_be_made_public() {
    let (game, _) = game_with_draw_action_window(crate::card::cards::TERMINUS);
    let mut wire = wire_for_viewer(&game, PlayerId::One);
    wire["decision"]["visibility"] = json!("Public");
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, PlayerId::One),
        80_899,
    )
    .expect_err("a draw-action window cannot be changed into a public decision");
    assert!(
        error.contains("draw-action window decision visibility disagrees"),
        "unexpected error: {error}"
    );
}

fn reveal_miracle_and_place_its_trigger(game: &mut Game) {
    let reveal = game
        .observe(PlayerId::One)
        .decision
        .expect("the Miracle reveal is pending");
    game.choose_decision(PlayerId::One, reveal.id, &[1]);
    assert_eq!(game.pending_triggers.len(), 1);
    game.finish_rules_procedure();
    assert!(game.pending_triggers.is_empty());
    assert_eq!(game.stack.len(), 1);
}

/// A Miracle's trigger is about a card that is still in its owner's hand.
///
/// While the trigger is waiting it fails closed for the opponent, who is not
/// told which object it is about. Once it is on the stack the observation
/// names that object itself, so the checkpoint says where the card sits
/// instead of dropping the payload -- the same disclosure the standing offer
/// one step later already makes.
#[test]
fn a_miracle_trigger_in_a_hidden_hand_is_carried_by_position_on_the_stack() {
    let (mut game, card) = game_with_draw_action_window(crate::card::cards::TERMINUS);
    let reveal = game
        .observe(PlayerId::One)
        .decision
        .expect("the Miracle reveal is pending");
    game.choose_decision(PlayerId::One, reveal.id, &[1]);

    let owner_pending = game.checkpoint_json(PlayerId::One);
    assert_eq!(owner_pending["hasDeferredState"], false);
    assert_eq!(
        owner_pending["pendingTriggers"][0]["source"]["object"],
        card.0
    );
    let opponent_pending = game.checkpoint_json(PlayerId::Two);
    assert_eq!(opponent_pending["hasDeferredState"], true);
    assert_eq!(opponent_pending["pendingTriggers"], json!([]));

    game.finish_rules_procedure();
    assert_eq!(game.stack.len(), 1);
    let owner_stacked = game.checkpoint_json(PlayerId::One);
    assert_eq!(owner_stacked["hasDeferredState"], false);
    assert!(owner_stacked["stack"][0]["abilityPayload"].is_object());
    // The owner can read their own hand, so nothing has to be positioned.
    assert_eq!(
        owner_stacked["stack"][0]["abilityPayload"]["sourceOrigin"],
        Value::Null
    );

    let opponent_stacked = game.checkpoint_json(PlayerId::Two);
    assert_eq!(opponent_stacked["hasDeferredState"], false);
    let origin = &opponent_stacked["stack"][0]["abilityPayload"]["sourceOrigin"];
    assert_eq!(origin["objectId"], card.0, "the object the stack names");
    assert_eq!(origin["seat"], 0);
    assert_eq!(origin["zone"], json!("hand"));
    assert!(origin["index"].is_u64(), "and where it sits in that hand");

    // And the opponent's seat rebuilds a game whose stack names the same
    // object, out of a hand it was handed rather than one it could read.
    let wire = wire_for_viewer(&game, PlayerId::Two);
    let rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, PlayerId::Two),
        80_903,
    )
    .expect("a stacked Miracle trigger reconstructs for the opponent");
    assert_eq!(rebuilt.stack[0].source, Some(card));
    assert_eq!(
        crate::protocol::protocol_actions(&rebuilt.observe(PlayerId::Two)),
        crate::protocol::protocol_actions(&game.observe(PlayerId::Two)),
    );
}

#[test]
fn a_standing_miracle_offer_round_trips_for_both_seats_and_resumes() {
    let (mut game, card) = game_with_draw_action_window(crate::card::cards::TERMINUS);
    reveal_miracle_and_place_its_trigger(&mut game);
    for _ in 0..2 {
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("priority passes to resolve the Miracle trigger");
    }
    assert!(matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::MayCastAlternative {
            player: PlayerId::One,
            card: offered,
            ability: AbilityOrigin::Printed {
                definition: crate::card::cards::TERMINUS,
                ..
            },
        } if offered == card
    ));

    let mut owner_rebuilt = None;
    for viewer in [PlayerId::One, PlayerId::Two] {
        let wire = wire_for_viewer(&game, viewer);
        assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
        let original_actions = crate::protocol::protocol_actions(&game.observe(viewer));
        let rebuilt = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &true_hidden_hypothesis(&game, viewer),
            80_901,
        )
        .expect("the public standing Miracle offer reconstructs for either seat");
        assert_eq!(
            crate::protocol::protocol_actions(&rebuilt.observe(viewer)),
            original_actions
        );
        if viewer == PlayerId::One {
            owner_rebuilt = Some(rebuilt);
        }
    }

    let mut cast = owner_rebuilt.expect("the deciding seat reconstructed");
    let mut declined = cast.clone();
    let decline = declined
        .observe(PlayerId::One)
        .decision
        .expect("the rebuilt offer can be declined");
    declined.choose_decision(PlayerId::One, decline.id, &[0]);
    assert!(declined.pending_decisions.is_empty());
    assert!(declined.legal_actions(PlayerId::One).iter().all(
        |action| !matches!(action, Action::CastSpell { card: offered, .. } if *offered == card)
    ));

    let cast_action = cast
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::CastSpell { card: offered, .. } if *offered == card),
        )
        .expect("the rebuilt offer retains its exact cast action");
    cast.apply(PlayerId::One, cast_action)
        .expect("the reconstructed Miracle offer can be accepted");
    assert!(cast.pending_decisions.is_empty());

    let wire = wire_for_viewer(&game, PlayerId::One);
    let hidden = true_hidden_hypothesis(&game, PlayerId::One);
    let mut wrong_alternative = wire.clone();
    wrong_alternative["checkpoint"]["decisionState"]["continuation"]["ability"]["abilityId"] =
        json!(0);
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wrong_alternative,
        &hidden,
        80_902,
    )
    .expect_err("a checkpoint cannot manufacture another alternative-cast offer");
    assert!(
        error.contains("is not the card's linked Miracle clause"),
        "unexpected error: {error}"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn a_revealing_group_choice_round_trips_and_resumes() {
    let mut game = crate::game::tests::ready_game();
    game.players[0].library = vec![
        crate::game::tests::card(81_001, crate::card::cards::LIGHTNING_BOLT, PlayerId::One),
        crate::game::tests::card(81_002, crate::card::cards::SAVANNAH_LIONS, PlayerId::One),
    ];
    let domri = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::DOMRI_RADE)
        .expect("Domri enters");
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let plus_one = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, .. }
                if *source == domri
                    && matches!(ability, crate::AbilityOrigin::Printed { ability, .. }
                        if *ability == crate::AbilityId::PRIMARY))
        })
        .expect("Domri's +1 is offered");
    game.apply(PlayerId::One, plus_one).unwrap();
    crate::game::tests::pass_until_decision(&mut game);
    assert_eq!(game.pending_decisions.len(), 1);
    assert_eq!(game.decision_player(), Some(PlayerId::One));

    let (_wire, mut rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 81_003);
    assert!(matches!(
        rebuilt.pending_decisions[0].continuation,
        DecisionContinuation::ChooseForEffect { .. }
    ));
    let decision = rebuilt.observe(PlayerId::One).decision.unwrap();
    let choice = decision.options[0].id;
    rebuilt.choose_decision(PlayerId::One, decision.id, &[choice]);

    assert!(
        rebuilt.players[0]
            .hand
            .iter()
            .any(|card| card.definition == crate::card::cards::SAVANNAH_LIONS)
    );
    assert!(rebuilt.events.iter().any(|event| matches!(
        event,
        GameEvent::CardRevealed {
            player: PlayerId::One,
            definition: crate::card::cards::SAVANNAH_LIONS,
            ..
        }
    )));
}

#[test]
fn an_arranged_group_round_trips_and_resumes() {
    let mut game = crate::game::tests::ready_game();
    game.players[0].hand.clear();
    game.players[0].library = vec![
        crate::game::tests::card(81_104, crate::card::cards::MOUNTAIN, PlayerId::One),
        crate::game::tests::card(81_103, crate::card::cards::SERRA_ANGEL, PlayerId::One),
        crate::game::tests::card(81_102, crate::card::cards::LIGHTNING_BOLT, PlayerId::One),
        crate::game::tests::card(81_101, crate::card::cards::SAVANNAH_LIONS, PlayerId::One),
    ];
    let augur = crate::game::tests::card(
        81_100,
        crate::card::cards::AUGUR_OF_BOLAS,
        PlayerId::One,
    );
    game.players[0].hand.push(augur.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        crate::game::tests::cast_action(augur.id, Vec::new(), Vec::new(), 0),
    )
    .expect("Augur can be cast");
    crate::game::tests::pass_priority_pair(&mut game);
    crate::game::tests::pass_priority_pair(&mut game);
    let (_wire, mut rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 81_105);
    assert!(matches!(
        rebuilt.pending_decisions[0].continuation,
        DecisionContinuation::ChooseForEffect { .. }
    ));
    let choose = rebuilt.observe(PlayerId::One).decision.unwrap();
    let bolt = choose
        .options
        .iter()
        .find(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(crate::card::cards::LIGHTNING_BOLT)
            })
        })
        .expect("Bolt is eligible")
        .id;
    rebuilt.choose_decision(PlayerId::One, choose.id, &[bolt]);

    let (_wire, mut rebuilt) = rebuild_current_checkpoint(&rebuilt, PlayerId::One, 81_106);
    assert!(matches!(
        rebuilt.pending_decisions[0].continuation,
        DecisionContinuation::ChooseObjectOrderForEffect { .. }
    ));
    let order = rebuilt.observe(PlayerId::One).decision.unwrap();
    assert_eq!(
        order.order_semantics,
        Some(DecisionOrderSemantics::Resolution)
    );
    let answer = order.options.iter().map(|option| option.id).collect::<Vec<_>>();
    rebuilt.choose_decision(PlayerId::One, order.id, &answer);

    assert!(rebuilt.pending_decisions.is_empty());
    assert_eq!(rebuilt.players[0].library.len(), 3);
    assert!(rebuilt.players[0]
        .hand
        .iter()
        .any(|card| card.definition == crate::card::cards::LIGHTNING_BOLT));
}

/// Each decision in a multi-stage distribution carries the remaining effect
/// program, so restoring it resumes at the same group choice.
#[test]
fn a_multistage_group_distribution_round_trips_and_resumes() {
    let mut game = crate::game::tests::ready_game();
    game.players[0].hand.clear();
    game.players[0].library = vec![
        crate::game::tests::card(83_003, crate::card::cards::SAVANNAH_LIONS, PlayerId::One),
        crate::game::tests::card(83_002, crate::card::cards::MOUNTAIN, PlayerId::One),
        crate::game::tests::card(83_001, crate::card::cards::LIGHTNING_BOLT, PlayerId::One),
    ];
    let iteration = crate::game::tests::card(
        83_010,
        crate::card::cards::EXPRESSIVE_ITERATION,
        PlayerId::One,
    );
    let iteration_id = iteration.id;
    game.players[0].hand.push(iteration);
    game.add_unrestricted_mana(PlayerId::One, crate::card::ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, crate::card::ManaColor::Red, 1);
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == iteration_id))
        .expect("two mana casts it");
    game.apply(PlayerId::One, cast).unwrap();
    for _ in 0..4 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority).unwrap();
    }
    assert_eq!(game.pending_decisions.len(), 1);

    let (_wire, mut rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 83_020);
    assert!(matches!(
        rebuilt.pending_decisions[0].continuation,
        DecisionContinuation::ChooseForEffect { .. }
    ));

    // Resuming finishes the distribution: one card to hand, one underneath,
    // and the last exiled.
    for _ in 0..8 {
        let Some(decision) = rebuilt.observe(PlayerId::One).decision else {
            break;
        };
        let choice = decision.options[0].id;
        rebuilt.choose_decision(PlayerId::One, decision.id, &[choice]);
    }

    assert_eq!(rebuilt.players[0].hand.len(), 1, "one card came home");
    assert_eq!(rebuilt.players[0].exile.len(), 1, "and one was exiled");
    assert_eq!(rebuilt.players[0].library.len(), 1);
}

#[test]
fn a_choose_one_of_each_group_round_trips_and_resumes() {
    let mut game = crate::game::tests::ready_game();
    game.players[0].hand.clear();
    game.players[0].library = vec![
        crate::game::tests::card(82_001, crate::card::cards::MOUNTAIN, PlayerId::One),
        crate::game::tests::card(82_002, crate::card::cards::LIGHTNING_BOLT, PlayerId::One),
        crate::game::tests::card(82_003, crate::card::cards::SOL_RING, PlayerId::One),
    ];
    game.put_onto_battlefield(PlayerId::One, crate::card::cards::ATRAXA_GRAND_UNIFIER)
        .expect("Atraxa enters");
    for _ in 0..6 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority).unwrap();
    }
    assert_eq!(game.pending_decisions.len(), 1);

    let (wire, mut rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 82_004);
    let state = &wire["checkpoint"]["decisionState"]["continuation"];
    assert_eq!(state["next"], json!(0), "the artifact pick is first");
    assert!(matches!(
        rebuilt.pending_decisions[0].continuation,
        DecisionContinuation::ChooseOneOfEachForEffect { next: 0, .. }
    ));
    let mut wrong_type = wire.clone();
    wrong_type["checkpoint"]["decisionState"]["continuation"]["next"] = json!(4);
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wrong_type,
        &true_hidden_hypothesis(&game, PlayerId::One),
        82_004,
    )
    .expect_err("a one-of-each choice cannot claim to be asking about another type");
    assert!(
        error.contains("one-of-each") || error.contains("decision"),
        "unexpected error: {error}"
    );

    // Resuming takes the Sol Ring for the artifact pick, then walks the rest.
    let decision = rebuilt.observe(PlayerId::One).decision.unwrap();
    let choice = decision.options[0].id;
    rebuilt.choose_decision(PlayerId::One, decision.id, &[choice]);
    for _ in 0..8 {
        let Some(decision) = rebuilt.observe(PlayerId::One).decision else {
            break;
        };
        rebuilt.choose_decision(PlayerId::One, decision.id, &[]);
    }

    assert!(
        rebuilt.players[0]
            .hand
            .iter()
            .any(|card| card.definition == crate::card::cards::SOL_RING),
        "the artifact pick came home",
    );
    assert_eq!(
        rebuilt.players[0].library.len(),
        2,
        "and the two cards nobody took went back",
    );
}
