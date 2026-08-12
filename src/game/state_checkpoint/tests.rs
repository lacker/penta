use super::*;
use crate::ManaColor;
use crate::card::KeywordAbility;
use crate::game::DecisionContinuation;
use serde_json::json;

mod adversarial;
mod broad_audit;
mod rare_states;
mod semantics_coverage;
mod trajectory;

#[test]
fn catalog_semantics_rehydrate_an_animation_without_a_card_name_switch() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let key = animation_snapshot(&crate::card::abilities::MISHRAS_FACTORY_ANIMATION);
    let rebuilt = catalog_animation(&catalog, &key).expect("animation is cataloged");
    assert_eq!(*rebuilt, crate::card::abilities::MISHRAS_FACTORY_ANIMATION);
}

#[test]
fn catalog_semantics_rehydrate_top_level_and_nested_abilities() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let top_level = catalog
        .definitions()
        .into_iter()
        .flat_map(|definition| &definition.parts)
        .flat_map(|part| part.rules.indexed_abilities())
        .next()
        .expect("catalog has an ability")
        .definition;
    let locator = ability_locator(&catalog, |candidate| *candidate == top_level)
        .expect("top-level ability has a locator");
    assert_eq!(catalog_ability(&catalog, &locator), Some(top_level));

    let granted_text =
        "At the beginning of your upkeep, sacrifice this artifact unless you pay {2}.";
    let locator = ability_locator(&catalog, |candidate| candidate.text == granted_text)
        .expect("nested granted ability has a locator");
    let rebuilt = catalog_ability(&catalog, &locator).expect("nested locator resolves");
    assert_eq!(rebuilt.text, granted_text);
    assert!(
        !locator.nested.is_empty(),
        "the granted clause is addressed beneath its printed source"
    );
}

#[test]
fn every_runtime_keyword_has_a_stable_checkpoint_round_trip() {
    let mut keywords = vec![
        KeywordAbility::Flying,
        KeywordAbility::Trample,
        KeywordAbility::Haste,
        KeywordAbility::FirstStrike,
        KeywordAbility::DoubleStrike,
        KeywordAbility::Banding,
        KeywordAbility::Vigilance,
        KeywordAbility::Defender,
        KeywordAbility::Deathtouch,
        KeywordAbility::Lifelink,
        KeywordAbility::Reach,
        KeywordAbility::Flash,
        KeywordAbility::Hexproof,
        KeywordAbility::Shroud,
        KeywordAbility::Intimidate,
        KeywordAbility::Undying,
        KeywordAbility::Indestructible,
        KeywordAbility::AttacksEachCombatIfAble,
        KeywordAbility::Mountainwalk,
        KeywordAbility::Forestwalk,
    ];
    keywords.extend(
        [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
            ManaColor::Colorless,
        ]
        .map(KeywordAbility::ProtectionFrom),
    );
    for keyword in keywords {
        assert_eq!(parse_keyword(keyword_snapshot(keyword)), keyword);
    }
}

#[test]
fn checkpoint_redacts_opposing_hidden_object_ids() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let deck = crate::Deck {
        main: vec![crate::card::cards::MOUNTAIN; 60],
        sideboard: Vec::new(),
    };
    let mut game = Game::new(catalog, [deck.clone(), deck], 41).expect("game starts");
    let hidden = game.players[PlayerId::Two.index()].hand[0].id;
    game.drawn_this_turn[PlayerId::Two.index()] = vec![hidden];
    game.miracle_window = Some(hidden);

    let checkpoint = game.checkpoint_json(PlayerId::One);
    assert_eq!(checkpoint["drawnThisTurn"][1], json!([]));
    assert!(checkpoint["miracleWindow"].is_null());

    let own = game.players[PlayerId::One.index()].hand[0].id;
    game.drawn_this_turn[PlayerId::One.index()] = vec![own];
    game.miracle_window = Some(own);
    let checkpoint = game.checkpoint_json(PlayerId::One);
    assert_eq!(checkpoint["drawnThisTurn"][0], json!([own.0]));
    assert_eq!(checkpoint["miracleWindow"], own.0);
}

#[test]
fn checkpoint_json_is_a_projection_of_one_typed_snapshot_schema() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let deck = crate::Deck {
        main: vec![crate::card::cards::MOUNTAIN; 60],
        sideboard: Vec::new(),
    };
    let game = Game::new(catalog, [deck.clone(), deck], 42).expect("game starts");
    let json = game.checkpoint_json(PlayerId::One);
    let snapshot: GameSnapshot =
        serde_json::from_value(json.clone()).expect("checkpoint matches GameSnapshot");
    assert_eq!(
        serde_json::to_value(snapshot).expect("snapshot serializes"),
        json
    );

    let mut additive = json.clone();
    additive["futureBookkeeping"] = json!({ "ignored": true });
    serde_json::from_value::<GameSnapshot>(additive)
        .expect("typed snapshots ignore unknown additive object members");

    let mut malformed = json;
    malformed
        .as_object_mut()
        .expect("snapshot object")
        .remove("turnsStarted");
    assert!(
        serde_json::from_value::<GameSnapshot>(malformed)
            .expect_err("missing required state must fail")
            .to_string()
            .contains("turnsStarted")
    );
}

#[test]
fn checkpoint_encodes_draw_replacement_and_procedure_state() {
    let mut game = crate::game::tests::ready_game();
    game.pending_procedures
        .push_back(crate::game::PendingProcedure::FinishStepAdvance);
    game.defer_empty_library_loss = true;
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
    assert_eq!(wire["checkpoint"]["deferEmptyLibraryLoss"], true);
    assert_eq!(
        wire["checkpoint"]["pendingProcedures"][0]["kind"],
        "finishStepAdvance"
    );
    let rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        4_243,
    )
    .expect("procedure state reconstructs");
    assert!(rebuilt.defer_empty_library_loss);
    assert!(matches!(
        rebuilt.pending_procedures.front(),
        Some(crate::game::PendingProcedure::FinishStepAdvance)
    ));
}

#[test]
fn ring_replacement_and_outside_game_choice_reconstruct_and_resume() {
    let mut game = crate::game::tests::ready_game();
    game.players[PlayerId::One.index()].outside_game = game
        .build_zone(PlayerId::One, &[crate::card::cards::SERRA_ANGEL])
        .expect("outside-game card builds");
    let ring = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::RING_OF_MARUF)
        .expect("Ring enters");
    game.players[PlayerId::One.index()].mana_pool.colorless = 5;
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                crate::Action::ActivateAbility { source, .. } if *source == ring
            )
        })
        .expect("Ring activation is legal");
    game.apply(PlayerId::One, activation)
        .expect("Ring activates");
    for _ in 0..2 {
        let priority = game.priority;
        game.apply(priority, crate::Action::PassPriority)
            .expect("priority passes");
    }
    assert_eq!(game.draw_replacements[0].len(), 1);

    let replacement_boundary = checkpoint_wire(&game);
    assert_eq!(
        replacement_boundary.1["checkpoint"]["hasDeferredState"],
        false
    );
    let rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &replacement_boundary.1,
        &true_hidden_hypothesis(&game, replacement_boundary.0),
        4_244,
    )
    .expect("active Ring replacement reconstructs");
    assert_eq!(rebuilt.draw_replacements[0].len(), 1);

    game.draw_cards(PlayerId::One, 2);
    assert!(matches!(
        game.pending_procedures.front(),
        Some(crate::game::PendingProcedure::DrawCards {
            player: PlayerId::One,
            remaining: 1
        })
    ));
    let (viewer, wire) = checkpoint_wire(&game);
    let actions = crate::protocol::protocol_actions(&game.observe(viewer));
    let mut rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        4_245,
    )
    .expect("Ring choice and interrupted draw reconstruct");
    assert_eq!(
        crate::protocol::protocol_actions(&rebuilt.observe(viewer)),
        actions
    );
    let decision = rebuilt
        .observe(viewer)
        .decision
        .expect("Ring choice remains");
    let outside = decision
        .options
        .iter()
        .find(|option| option.zone == crate::game::DecisionZone::OutsideGame)
        .expect("outside-game option is rebound");
    rebuilt
        .apply(
            viewer,
            crate::Action::ChooseDecision {
                decision: decision.id,
                options: vec![outside.id],
            },
        )
        .expect("rebuilt Ring choice resumes");
    assert!(rebuilt.players[0].outside_game.is_empty());
    assert!(
        rebuilt.players[0]
            .hand
            .iter()
            .any(|card| card.definition == crate::card::cards::SERRA_ANGEL)
    );
    assert!(rebuilt.pending_procedures.is_empty());
}

fn checkpoint_wire(game: &Game) -> (PlayerId, Value) {
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
    (viewer, wire)
}

#[test]
fn a_supported_pending_decision_rebuilds_and_resumes() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let deck = crate::Deck {
        main: vec![crate::card::cards::MOUNTAIN; 60],
        sideboard: vec![crate::card::cards::FOREST],
    };
    let mut game = Game::new(catalog.clone(), [deck.clone(), deck], 43).expect("game starts");
    let player = PlayerId::One;
    let card = game.players[player.index()].hand[0].id;
    game.queue_miracle_reveal(player, card);

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
            .map(|card| card.definition.0)
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
        DecisionContinuation::MiracleReveal { card: rebuilt_card } if rebuilt_card == card
    ));
    let decision = rebuilt.pending_decisions[0].observation.id;
    rebuilt.choose_decision(player, decision, &[1]);
    assert_eq!(rebuilt.miracle_window, Some(card));
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
    let definition = crate::card::cards::DOMRI_RADE_EMBLEM;
    let card = game.unbacked_object(
        definition,
        controller,
        CharacteristicSource::Ability(definition),
    );
    let emblem_id = card.id;
    let mut emblem = Permanent::entering(
        card,
        CardPartId::PRIMARY,
        controller,
        game.turns_started[controller.index()],
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
            .map(|card| card.definition.0)
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
    assert_eq!(rebuilt.observed_emblems(), observation.emblems);
}

#[test]
fn a_spell_created_delayed_trigger_reconstructs_at_an_action_boundary() {
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
                cost_object: None,
                x: 0,
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
    assert_eq!(game.delayed_triggers.len(), 1);
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
    .expect("the delayed trigger reconstructs");
    assert_eq!(rebuilt.delayed_triggers.len(), 1);
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

/// A hypothesis the seat could have chosen instead of the truth: both hidden
/// libraries reversed and the opposing hand rotated. Card counts and every
/// card the seat can actually see survive, so this stays consistent with what
/// the observation says while naming different cards than the host holds.
/// Reconstruction that only works from [`true_hidden_hypothesis`] would be no
/// use to a search bot, which never knows the truth.
fn determinized(hidden: &Value, viewer: PlayerId) -> Value {
    let mut hidden = hidden.clone();
    for seat in ["p1", "p2"] {
        if let Some(library) = hidden
            .get_mut("libraries")
            .and_then(|libraries| libraries.get_mut(seat))
            .and_then(Value::as_array_mut)
        {
            library.reverse();
        }
    }
    let opponent = seat_label(viewer.opponent());
    if let Some(hand) = hidden
        .get_mut("hands")
        .and_then(|hands| hands.get_mut(opponent))
        .and_then(Value::as_array_mut)
        && !hand.is_empty()
    {
        hand.rotate_left(1);
    }
    hidden
}

fn true_hidden_hypothesis(game: &Game, viewer: PlayerId) -> Value {
    let definitions = |cards: &[CardInstance]| {
        cards
            .iter()
            .map(|card| card.definition.0)
            .collect::<Vec<_>>()
    };
    let opponent = viewer.opponent();
    let opponent_hand = &game.players[opponent.index()].hand;
    let drawn_indices = game.drawn_this_turn[opponent.index()]
        .iter()
        .filter_map(|id| opponent_hand.iter().position(|card| card.id == *id))
        .collect::<Vec<_>>();
    let miracle_window = game.miracle_window.and_then(|id| {
        opponent_hand
            .iter()
            .position(|card| card.id == id)
            .map(|hand_index| {
                json!({
                    "seat": seat_label(opponent),
                    "handIndex": hand_index,
                })
            })
    });
    let mut discard_choices = serde_json::Map::new();
    if let Some(DecisionContinuation::DiscardForEffect { chosen, .. }) = game
        .pending_decisions
        .first()
        .map(|pending| &pending.continuation)
    {
        for (player, cards) in chosen {
            if *player == viewer {
                continue;
            }
            let hand = &game.players[player.index()].hand;
            let indices = cards
                .iter()
                .filter_map(|id| hand.iter().position(|card| card.id == *id))
                .collect::<Vec<_>>();
            discard_choices.insert(seat_label(*player).into(), json!(indices));
        }
    }
    json!({
        "hands": {
            (seat_label(opponent)): definitions(opponent_hand),
        },
        "libraries": {
            "p1": definitions(&game.players[PlayerId::One.index()].library),
            "p2": definitions(&game.players[PlayerId::Two.index()].library),
        },
        "outsideGame": {
            "p1": definitions(&game.players[PlayerId::One.index()].outside_game),
            "p2": definitions(&game.players[PlayerId::Two.index()].outside_game),
        },
        "drawnThisTurn": {
            (seat_label(opponent)): drawn_indices,
        },
        "miracleWindow": miracle_window,
        "decision": {
            "discardChoices": discard_choices,
        },
    })
}

#[test]
fn checkpoint_round_trips_extra_turns_and_the_regular_turn_anchor() {
    let mut game = crate::game::tests::ready_game();
    game.extra_turns = vec![PlayerId::One];
    game.next_regular_player = PlayerId::Two;
    let viewer = PlayerId::One;
    let observation = game.observe(viewer);
    let observation = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        false,
        &game.legal_actions(viewer),
    );
    let hidden = json!({
        "hands": {"p2": []},
        "libraries": {
            "p1": game.players[0].library.iter().map(|card| card.definition.0).collect::<Vec<_>>(),
            "p2": game.players[1].library.iter().map(|card| card.definition.0).collect::<Vec<_>>(),
        },
        "outsideGame": {"p1": [], "p2": []},
    });

    let mut rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &observation,
        &hidden,
        91,
    )
    .expect("scheduler checkpoint reconstructs");

    assert_eq!(rebuilt.extra_turns, vec![PlayerId::One]);
    assert_eq!(rebuilt.next_regular_player, PlayerId::Two);
    rebuilt.start_next_turn();
    assert_eq!(rebuilt.active_player, PlayerId::One);
    assert_eq!(rebuilt.next_regular_player, PlayerId::Two);
    rebuilt.start_next_turn();
    assert_eq!(rebuilt.active_player, PlayerId::Two);
    assert_eq!(rebuilt.next_regular_player, PlayerId::One);
}
