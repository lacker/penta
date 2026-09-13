use super::*;

fn duel() -> Game {
    let deck = Deck {
        commanders: vec![cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS],
        main: vec![cards::FOREST; 98],
        sideboard: Vec::new(),
    };
    let mut game = Game::new_with_format(
        Format::DuelCommander,
        card::catalog().unwrap(),
        [deck.clone(), deck],
        1,
    )
    .unwrap();
    game.pregame = None;
    game.step = Step::PrecombatMain;
    game
}

#[test]
fn duel_commander_uses_twenty_life_without_commander_damage_loss() {
    let mut game = duel();
    assert_eq!(game.players[0].life, 20);
    let bear = enter(&mut game, PlayerId::One, 0);
    game.players[1].life = 100;
    game.damage_target_from_kind(Some(bear), Some(Target::Player(PlayerId::Two)), 21, true);
    game.check_state_based_actions();
    assert!(game.result.is_none());
    assert_eq!(game.players[1].life, 79);
}

#[test]
fn duel_commander_first_cast_locks_other_commanders_and_survives_checkpoint() {
    for prepared in [false, true] {
        let mut game = duel();
        game.set_prepared_engine_enabled(prepared);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 10);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 10);
        let bear = game.players[0].command[0].id;
        let lion = game.players[0].command[1].id;
        let forbidden = castable(&game, lion).unwrap();
        game.apply(PlayerId::One, castable(&game, bear).unwrap())
            .unwrap();
        drain_pending(&mut game);
        assert!(castable(&game, lion).is_none());
        assert!(game.apply(PlayerId::One, forbidden).is_err());
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let restored = Game::from_observation_checkpoint(
            game.catalog.clone(),
            Format::DuelCommander,
            &wire,
            &hidden,
            4,
        )
        .unwrap();
        assert!(!restored.can_cast_commander_from_command_zone(lion));
        let bear = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::GRIZZLY_BEARS)
            .unwrap()
            .card
            .id;
        game.return_permanent_to_hand(bear);
        choose(&mut game, vec![0]);
        let bear = game.players[0]
            .command
            .iter()
            .find(|c| c.definition == cards::GRIZZLY_BEARS)
            .unwrap()
            .id;
        assert_eq!(game.commander_tax(bear), 2);
        assert!(castable(&game, bear).is_some());
    }
}

#[test]
fn duel_commander_casting_from_hand_does_not_lock_the_command_zone() {
    let mut game = duel();
    let lion = game.players[0].command[1].id;
    game.move_card_target_to_zone(
        lion,
        ZoneKind::Hand,
        ZoneMoveCause::Rules,
        None,
        ZonePlacement::Top,
    );
    choose(&mut game, vec![1]);
    let lion = game.players[0]
        .hand
        .iter()
        .find(|c| c.definition == cards::SAVANNAH_LIONS)
        .unwrap()
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.apply(PlayerId::One, castable(&game, lion).unwrap())
        .unwrap();
    drain_pending(&mut game);
    assert_eq!(game.commanders[1].casts, 0);
    let bear = game.players[0].command[0].id;
    assert!(game.can_cast_commander_from_command_zone(bear));
}

#[test]
fn duel_commander_outside_game_effects_offer_no_cards() {
    let mut game = duel();
    let card = game.players[0].library.pop().unwrap();
    game.players[0].outside_game.push(card);
    let source = game.players[0].command[0].id;
    let queued = game.queue_owned_card_choice(
        PlayerId::One,
        &[crate::card::CardChoiceSourceDef::OutsideGame],
        crate::card::ObjectPredicateDef::Any,
        0,
        1,
        false,
        ZoneKind::Hand,
        ZonePlacement::Top,
        None,
        source,
        PlayerId::One,
    );
    assert!(!queued);
    assert!(game.pending_decisions.is_empty());
    assert_eq!(game.players[0].outside_game.len(), 1);
}

#[test]
fn duel_commander_companion_action_is_exempt_from_outside_game_policy() {
    let deck = Deck {
        commanders: vec![cards::GRIZZLY_BEARS],
        main: vec![cards::FOREST; 99],
        sideboard: vec![cards::LURRUS_OF_THE_DREAM_DEN],
    };
    let mut game = Game::new_with_format(
        Format::DuelCommander,
        card::catalog().unwrap(),
        [deck.clone(), deck],
        1,
    )
    .unwrap();
    game.pregame = None;
    game.step = Step::PrecombatMain;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::TakeCompanion { .. }))
        .expect("companion exception");
    game.apply(PlayerId::One, action).unwrap();
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::LURRUS_OF_THE_DREAM_DEN)
    );
}

fn match_choice(game: &mut Game, options: Vec<u32>) {
    let decision = game.match_decision().expect("match choice");
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .unwrap();
}

#[test]
fn duel_commander_match_swaps_from_registered_pool_and_resets_history() {
    let mut main = vec![cards::FOREST; 98];
    main.push(cards::SAVANNAH_LIONS);
    let deck = Deck {
        commanders: vec![cards::GRIZZLY_BEARS],
        main,
        sideboard: Vec::new(),
    };
    let mut game = Game::new_with_format(
        Format::DuelCommander,
        card::catalog().unwrap(),
        [deck.clone(), deck],
        1,
    )
    .unwrap();
    game.set_match_mode(crate::match_play::MatchMode::FirstToTwoWins)
        .unwrap();
    match_choice(&mut game, vec![0]);
    game.pregame = None;
    game.commanders[0].casts = 2;
    game.apply(PlayerId::One, Action::Concede).unwrap();
    let decision = game.match_decision().unwrap();
    assert_eq!(decision.prompt, "Choose commanders for the next game");
    assert_eq!((decision.minimum, decision.maximum), (1, 2));
    let lion = decision
        .options
        .iter()
        .find(|o| o.label == "Savannah Lions")
        .unwrap()
        .id;
    match_choice(&mut game, vec![lion]);
    assert_eq!(
        game.commanders[0].definition,
        cards::GRIZZLY_BEARS,
        "current game is not rewritten before both submit"
    );
    assert_eq!(
        game.match_context.as_ref().unwrap().decks[0].commanders,
        vec![cards::SAVANNAH_LIONS]
    );
    let (wire, mut hidden) = checkpoint_fixture(&game, PlayerId::One);
    let context = game.match_context.as_ref().unwrap();
    hidden["matchDecks"] = serde_json::json!([null, {"registered": context.registered[1], "current": context.decks[1]}]);
    assert!(wire["checkpoint"]["matchState"]["decks"][1].is_null());
    let restored = Game::from_observation_checkpoint(
        game.catalog.clone(),
        Format::DuelCommander,
        &wire,
        &hidden,
        4,
    )
    .unwrap();
    assert_eq!(
        restored.match_context.as_ref().unwrap().decks[0].commanders,
        vec![cards::SAVANNAH_LIONS]
    );
    match_choice(&mut game, vec![0]);
    match_choice(&mut game, vec![0]);
    assert_eq!(game.players[0].command[0].definition, cards::SAVANNAH_LIONS);
    assert!(game.commanders.iter().all(|c| c.casts == 0));
    assert_eq!(game.players[0].life, 20);
    assert!(
        game.players[0]
            .library
            .iter()
            .chain(&game.players[0].hand)
            .any(|c| c.definition == cards::GRIZZLY_BEARS)
    );
    let context = game.match_context.as_ref().unwrap();
    let mut forged = context.decks[0].clone();
    forged.commanders = vec![cards::SERRA_ANGEL];
    assert!(!context.valid_deck(PlayerId::One, &forged, &game.catalog, Format::DuelCommander));
}
