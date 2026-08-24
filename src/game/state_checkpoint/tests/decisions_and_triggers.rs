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
fn a_revealing_top_card_selection_round_trips_and_resumes() {
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
    for _ in 0..4 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority).unwrap();
    }
    assert_eq!(game.pending_decisions.len(), 1);
    assert_eq!(game.decision_player(), Some(PlayerId::One));

    let (wire, mut rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 81_003);
    let selection_state = &wire["checkpoint"]["decisionState"]["continuation"];
    assert!(selection_state.get("revealSelected").is_none());
    assert!(selection_state.get("selectedZone").is_none());
    assert!(selection_state.get("restZone").is_none());
    let mut duplicate_detached = wire.clone();
    let revealed = duplicate_detached["checkpoint"]["decisionState"]["continuation"]["revealed"]
        .as_array_mut()
        .expect("top-card continuation carries detached inspected cards");
    revealed.push(revealed[0].clone());
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &duplicate_detached,
        &true_hidden_hypothesis(&game, PlayerId::One),
        81_003,
    )
    .expect_err("a top-card continuation cannot repeat one detached object id");
    assert!(
        error.contains("detached-card list repeats object id"),
        "unexpected error: {error}"
    );
    let mut spliced = wire.clone();
    spliced["checkpoint"]["decisionState"]["continuation"]["continuation"]["effect"]["path"] =
        json!([999]);
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &spliced,
        &true_hidden_hypothesis(&game, PlayerId::One),
        81_003,
    )
    .expect_err("top-card semantics cannot be replaced by edited placement fields or paths");
    assert!(
        error.contains("locator is absent") || error.contains("top-card selection locator"),
        "unexpected error: {error}"
    );
    let mut wrong_kind = wire.clone();
    wrong_kind["decision"]["kind"] = json!("TriggerOrder");
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wrong_kind,
        &true_hidden_hypothesis(&game, PlayerId::One),
        81_003,
    )
    .expect_err("top-card selection rejects another decision procedure's kind");
    assert!(
        error.contains("decision kind disagrees"),
        "unexpected error: {error}"
    );
    assert!(matches!(
        rebuilt.pending_decisions[0].continuation,
        DecisionContinuation::TopCardSelection {
            selection: crate::card::TopCardSelectionDef {
                reveal_selected: true,
                counted: None,
                ..
            },
            ..
        }
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
fn a_hand_search_checkpoint_preserves_duplicate_card_object_ids() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let deck = crate::Deck {
        main: vec![crate::card::cards::MOUNTAIN; 60],
        sideboard: Vec::new(),
    };
    let mut game = Game::new(catalog.clone(), [deck.clone(), deck], 44).expect("game starts");
    let player = PlayerId::One;
    let original_ids = game.players[player.index()]
        .hand
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    assert!(
        original_ids.len() > 1,
        "the hand contains duplicate Mountains"
    );
    game.queue_zone_search(
        player,
        ZoneKind::Hand,
        crate::card::ObjectPredicateDef::Any,
        0,
        1,
        false,
        ZoneKind::Graveyard,
        crate::card::ZonePlacement::Top,
        false,
        None,
        None,
        false,
        original_ids[0],
        player,
    );

    let (viewer, wire) = checkpoint_wire(&game);
    let original_options = game
        .observe(viewer)
        .decision
        .expect("the hand search is offered")
        .options
        .iter()
        .filter_map(|option| option.card.map(|(id, _)| id))
        .collect::<Vec<_>>();
    let rebuilt = Game::from_observation_checkpoint(
        catalog,
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        1_008,
    )
    .expect("the duplicate-card hand search reconstructs");

    assert_eq!(
        rebuilt.players[player.index()]
            .hand
            .iter()
            .map(|card| card.id)
            .collect::<Vec<_>>(),
        original_ids,
        "a public hand already has exact object identities and must not be rebound by definition"
    );
    assert_eq!(
        rebuilt
            .observe(viewer)
            .decision
            .expect("the rebuilt hand search is offered")
            .options
            .iter()
            .filter_map(|option| option.card.map(|(id, _)| id))
            .collect::<Vec<_>>(),
        original_options
    );
}

#[test]
fn uncataloged_executable_state_fails_closed() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let deck = crate::Deck {
        main: vec![crate::card::cards::MOUNTAIN; 60],
        sideboard: Vec::new(),
    };
    let mut game = Game::new(catalog, [deck.clone(), deck], 47).expect("game starts");
    let player = PlayerId::One;
    let card = game.players[player.index()].hand[0].id;
    game.temporary_ability_grants.push(TemporaryAbilityGrant {
        object: card,
        ability: crate::card::AbilityDef::static_ability(
            "An intentionally uncataloged test ability.",
            crate::card::EffectDef::None,
        ),
    });

    let checkpoint = game.checkpoint_json(player);
    assert_eq!(checkpoint["hasDeferredState"], true);
}

#[test]
fn an_emblem_rebuilds_with_identity_and_source_provenance() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let deck = crate::Deck {
        main: vec![crate::card::cards::MOUNTAIN; 60],
        sideboard: Vec::new(),
    };
    let mut game = Game::new(catalog.clone(), [deck.clone(), deck], 53).expect("game starts");
    let controller = PlayerId::One;
    let creator = catalog
        .get(crate::card::cards::DOMRI_RADE)
        .and_then(|definition| definition.part(CardPartId::PRIMARY))
        .and_then(|part| part.rules.ability(AbilityId(2)))
        .expect("Domri has an emblem-creating ultimate");
    let EffectDef::CreateEmblem { emblem: authored } = creator
        .declarative_effect()
        .expect("Domri's ultimate is declarative")
    else {
        panic!("Domri's ultimate creates an emblem")
    };
    let card = game.unbacked_emblem_object(authored, controller);
    let emblem_id = card.id;
    let mut emblem = Permanent::entering(
        card,
        CardPartId::PRIMARY,
        controller,
        game.turns_started[controller.index()],
        game.turn,
    );
    emblem.timestamp = game.allocate_continuous_effect_timestamp();
    emblem.emblem_source = Some(AbilityOrigin::Printed {
        definition: crate::card::cards::DOMRI_RADE,
        part: CardPartId::PRIMARY,
        ability: AbilityId(2),
    });
    game.emblems.push(emblem);

    let observation = game.observe(controller);
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

    let rebuilt =
        Game::from_observation_checkpoint(catalog, game.format, &observation_json, &hidden, 1_009)
            .expect("emblem reconstructs");
    assert_eq!(rebuilt.emblems.len(), 1);
    assert_eq!(rebuilt.emblems[0].card.id, emblem_id);
    assert_eq!(rebuilt.emblems[0].card.definition, ObjectKind::Emblem);
    assert!(matches!(
        rebuilt.emblems[0].card.characteristics,
        CharacteristicSource::Emblem(_)
    ));
    assert_eq!(rebuilt.observed_emblems(), observation.emblems);
}

#[test]
#[allow(clippy::too_many_lines)]
fn installed_trigger_round_trip_preserves_targets_bindings_and_x() {
    let mut game = crate::game::tests::ready_game();
    let first_target = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SAVANNAH_LIONS)
        .expect("the first captured target enters");
    let second_target = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SERRA_ANGEL)
        .expect("the second captured target enters");
    let installed_locator = ability_locator(&game.catalog, |ability| {
        ability.text
            == "At the beginning of the next end step, destroy that creature if it attacked this turn."
    })
    .expect("Berserk's installed ability has a semantic locator");
    let ability = catalog_ability(&game.catalog, &installed_locator)
        .expect("the installed ability rehydrates");
    let DeclarativeAbilityDef::Triggered(triggered) = ability.definition else {
        panic!("Berserk's nested ability is triggered");
    };
    let source = source_for_locator(first_target, &installed_locator);
    let mut context = EffectResolutionContext::empty();
    context.bind_single_object(
        ObjectBindingIndex::PRIMARY,
        Some(Target::Permanent(first_target)),
    );
    let selections = vec![
        TargetSelection::single(TargetSlotId(0), Target::Permanent(first_target)),
        TargetSelection::single(TargetSlotId(1), Target::Permanent(second_target)),
    ];
    let effect = ability
        .declarative_effect()
        .expect("the installed trigger is declarative");
    let capture = |target_base, x| TriggerCapture {
        source,
        presentation: ObjectCharacteristics::card(
            crate::card::cards::BERSERK,
            CardPartId::PRIMARY,
        ),
        owner: PlayerId::One,
        controller: PlayerId::One,
        text: ability.text,
        target_defs: Vec::new(),
        // Repeated/modal occurrences share a semantic ability locator but
        // address distinct ranges in the flattened lexical selections.
        targets: selections.clone(),
        effect,
        resolver: StackAbilityResolver::Declarative(ScopedEffect {
            effect,
            target_base,
        }),
        context: context.clone(),
        condition: triggered.condition,
        modes: None,
        x,
    };
    game.installed_triggers.push(InstalledTrigger {
        id: 40,
        event: triggered.event,
        capture: capture(0, 6),
        lifetime: InstalledTriggerLifetime::Once,
    });
    game.installed_triggers.push(InstalledTrigger {
        id: 41,
        event: triggered.event,
        capture: capture(1, 7),
        lifetime: InstalledTriggerLifetime::Once,
    });
    game.next_installed_trigger_id = 42;

    let viewer = game.decision_player().expect("the game awaits an action");
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
    let hidden = true_hidden_hypothesis(&game, viewer);
    let rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 1_010)
            .expect("the installed trigger reconstructs");
    assert_eq!(rebuilt.installed_triggers, game.installed_triggers);
    assert_eq!(rebuilt.next_installed_trigger_id, 42);

    let mut malformed = wire.clone();
    malformed["checkpoint"]["nextInstalledTriggerId"] = json!(41);
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &malformed,
        &hidden,
        1_010,
    )
    .expect_err("an installed trigger id must precede the next installed trigger id");
    assert!(
        error.contains("does not follow"),
        "unexpected error: {error}"
    );

    let mut missing_definition = wire;
    missing_definition["checkpoint"]["installedTriggers"][0]["presentation"]["definition"] =
        json!(u16::MAX);
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &missing_definition,
        &hidden,
        1_010,
    )
    .expect_err("an installed trigger must name a cataloged presentation definition");
    assert!(
        error.contains("presentation locator is absent"),
        "unexpected error: {error}",
    );
}

#[test]
fn installed_trigger_retains_a_retired_lexical_target_and_resumes_from_lki() {
    fn pass_priority_pair(game: &mut Game) {
        let first = game.priority;
        game.apply(first, Action::PassPriority)
            .expect("the first seat passes");
        game.apply(first.opponent(), Action::PassPriority)
            .expect("the second seat passes");
    }

    fn reach_mana_drain_payout(game: &mut Game) {
        game.finish_cleanup();
        game.start_next_turn();
        assert_eq!(game.active_player, PlayerId::Two);
        game.step = Step::Draw;
        game.advance_step();
        game.finish_rules_procedure();
        for _ in 0..4 {
            if game.stack.is_empty() {
                break;
            }
            let player = game.priority;
            game.apply(player, Action::PassPriority)
                .expect("priority passes to resolve the installed trigger");
        }
    }

    let mut game = crate::game::tests::ready_game();
    let angel = crate::game::tests::card(95_000, crate::card::cards::SERRA_ANGEL, PlayerId::One);
    let drain = crate::game::tests::card(95_001, crate::card::cards::MANA_DRAIN, PlayerId::Two);
    game.players[PlayerId::One.index()].hand.push(angel.clone());
    game.players[PlayerId::Two.index()].hand.push(drain.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 5);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: angel.id,
            choices: crate::CastChoices::default(),
            sacrifices: Vec::new(),
        },
    )
    .expect("Serra Angel is castable");
    let retired_target = game.stack.last().expect("the Angel is on the stack").id;
    game.apply(PlayerId::One, Action::PassPriority)
        .expect("the caster passes");
    game.apply(
        PlayerId::Two,
        Action::CastSpell {
            card: drain.id,
            choices: crate::CastChoices::default().with_targets(vec![TargetSelection::single(
                TargetSlotId(0),
                Target::Spell(retired_target),
            )]),
            sacrifices: Vec::new(),
        },
    )
    .expect("Mana Drain is castable");
    pass_priority_pair(&mut game);

    assert!(game.stack.is_empty(), "the Angel was countered");
    assert_eq!(game.installed_triggers.len(), 1);
    assert!(game.retired_objects.contains_key(&retired_target));
    let viewer = game.decision_player().expect("the game awaits priority");
    let (wire, mut rebuilt) = rebuild_current_checkpoint(&game, viewer, 1_010_001);
    assert!(
        wire["checkpoint"]["retiredObjects"]
            .as_array()
            .is_some_and(|objects| !objects.is_empty()),
        "the lexical target's last-known stack object must be checkpointed",
    );
    assert!(rebuilt.retired_objects.contains_key(&retired_target));

    reach_mana_drain_payout(&mut game);
    reach_mana_drain_payout(&mut rebuilt);
    assert_eq!(game.players[PlayerId::Two.index()].mana_pool.colorless, 5);
    assert_eq!(
        rebuilt.players[PlayerId::Two.index()].mana_pool.colorless,
        5
    );
}

#[test]
fn a_retired_card_direct_target_reconstructs_as_dangling_and_fizzles() {
    fn pass_priority_pair(game: &mut Game) {
        for _ in 0..2 {
            let player = game.priority;
            game.apply(player, Action::PassPriority)
                .expect("priority passes");
        }
    }

    let mut game = crate::game::tests::ready_game();
    let ooze = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SCAVENGING_OOZE)
        .expect("Scavenging Ooze enters");
    let food = crate::game::tests::card(95_100, crate::card::cards::SAVANNAH_LIONS, PlayerId::Two);
    let food_id = food.id;
    game.players[PlayerId::Two.index()].graveyard.push(food);
    game.players[PlayerId::One.index()].mana_pool.green = 2;

    for _ in 0..2 {
        let activation = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, targets, .. }
                    if *source == ooze
                        && targets
                            .iter()
                            .flat_map(TargetSelection::targets)
                            .any(|target| *target == Target::Card(food_id)))
            })
            .expect("the Ooze can target the same graveyard card twice");
        game.apply(PlayerId::One, activation)
            .expect("the Ooze activates");
    }
    assert_eq!(game.stack.len(), 2);

    pass_priority_pair(&mut game);
    assert_eq!(game.stack.len(), 1, "only the lower activation remains");
    assert!(game.retired_objects.contains_key(&food_id));
    assert!(game.players[PlayerId::Two.index()].graveyard.is_empty());
    let life_after_first = game.players[PlayerId::One.index()].life;
    let counters_after_first = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == ooze)
        .expect("the Ooze remains")
        .counters.count(crate::CounterKind::PlusOnePlusOne);

    let viewer = game.decision_player().expect("the game awaits priority");
    let (wire, mut rebuilt) = rebuild_current_checkpoint(&game, viewer, 95_101);
    assert_eq!(
        wire["checkpoint"]["stack"][0]["requiresRetiredObject"],
        json!(false),
        "a direct target that left its zone needs no last-known information",
    );
    assert!(
        wire["checkpoint"]["retiredObjects"]
            .as_array()
            .is_some_and(|objects| objects.iter().all(|object| {
                object["card"]["objectId"].as_u64() != Some(u64::from(food_id.0))
            })),
        "the stale direct target must not retain the exiled card's identity",
    );

    pass_priority_pair(&mut game);
    pass_priority_pair(&mut rebuilt);
    for resolved in [&game, &rebuilt] {
        assert!(resolved.stack.is_empty());
        assert_eq!(
            resolved.players[PlayerId::One.index()].life,
            life_after_first
        );
        assert_eq!(
            resolved
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == ooze)
                .expect("the Ooze remains")
                .counters.count(crate::CounterKind::PlusOnePlusOne),
            counters_after_first,
            "the stale lower activation fizzles instead of using retired-card LKI",
        );
    }
}

#[test]
fn a_spell_created_installed_trigger_reconstructs_at_an_action_boundary() {
    let mut game = crate::game::tests::ready_game();
    let player = PlayerId::One;
    let whelp_id = GameObjectId(10_000);
    game.battlefield.push(crate::game::tests::creature(
        whelp_id.0,
        crate::card::cards::DRAGON_WHELP,
        player,
    ));

    for _ in 0..4 {
        game.players[player.index()].mana_pool.red = 1;
        let ability = {
            let whelp = game
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == whelp_id)
                .expect("Dragon Whelp remains on the battlefield");
            game.activated_ability_origin(whelp, 0)
        };
        game.apply(
            player,
            crate::Action::ActivateAbility {
                source: whelp_id,
                ability,
                targets: Vec::new(),
                cost_objects: Vec::new(),
                x: 0,
                modes: Vec::new(),
            },
        )
        .expect("Dragon Whelp activates");
        for _ in 0..4 {
            if game.stack.is_empty() {
                break;
            }
            let priority = game.priority;
            game.apply(priority, crate::Action::PassPriority)
                .expect("priority passes while resolving the activation");
        }
    }
    assert_eq!(game.installed_triggers.len(), 1);
    game.sacrifice_permanent(whelp_id);
    assert!(game.retired_objects.contains_key(&whelp_id));

    let viewer = game.decision_player().expect("the game awaits an action");
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
    let rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        1_011,
    )
    .expect("the installed trigger reconstructs");
    assert_eq!(rebuilt.installed_triggers.len(), 1);
    assert!(rebuilt.retired_objects.contains_key(&whelp_id));
    let rebuilt_observation = rebuilt.observe(viewer);
    let rebuilt_actions = crate::protocol::protocol_actions(&rebuilt_observation);
    assert_eq!(rebuilt_actions, actions);
    assert_eq!(
        crate::protocol::observation_json_for_format(
            &rebuilt.catalog,
            rebuilt.format,
            &rebuilt_observation,
            rebuilt.in_pregame(),
            &rebuilt_actions,
        ),
        wire
    );
}
