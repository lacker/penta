use super::*;

fn staged(seed: u64, size: usize) -> (Game, GameObjectId) {
    let mut game = ready_game_with_seed(seed);
    game.set_library(PlayerId::One, &vec![cards::ISLAND; size])
        .unwrap();
    let katherine = game
        .put_onto_battlefield(PlayerId::One, cards::TRIUMPH_OF_SAINT_KATHERINE)
        .unwrap();
    (game, katherine)
}

#[test]
fn only_the_seven_card_pile_is_shuffled_and_no_card_is_revealed() {
    let mut positions = std::collections::BTreeSet::new();
    for seed in 0..16 {
        let (mut game, katherine) = staged(seed, 9);
        let bottom = game.players[0].library[..3].to_vec();
        game.sacrifice_permanent(katherine);
        assert_eq!(game.pending_triggers.len(), 1);
        drain_pending(&mut game);
        assert!(game.players[0].graveyard.is_empty());
        assert!(game.players[0].exile.is_empty());
        assert_eq!(game.players[0].library.len(), 10);
        assert_eq!(game.players[0].library[..3], bottom);
        let position = game.players[0]
            .library
            .iter()
            .position(|card| card.definition == cards::TRIUMPH_OF_SAINT_KATHERINE)
            .unwrap();
        assert!(position >= 3);
        positions.insert(position);
        for viewer in [PlayerId::One, PlayerId::Two] {
            assert!(
                !game
                    .events_for(viewer)
                    .iter()
                    .any(|event| matches!(event, GameEvent::CardRevealed { .. }))
            );
            assert!(game.observe(viewer).exiles[0].is_empty());
        }
    }
    assert!(positions.len() > 1, "the pile is randomized");
}

#[test]
fn fewer_than_six_library_cards_exiles_nothing() {
    for size in 0..6 {
        let (mut game, katherine) = staged(0, size);
        let library = game.players[0].library.clone();
        game.sacrifice_permanent(katherine);
        drain_pending(&mut game);
        assert_eq!(game.players[0].library, library);
        assert_eq!(game.players[0].graveyard.len(), 1);
        assert!(game.players[0].exile.is_empty());
    }
}

#[test]
fn leaving_and_returning_to_the_graveyard_does_not_restore_the_link() {
    let (mut game, katherine) = staged(0, 6);
    let library = game.players[0].library.clone();
    game.sacrifice_permanent(katherine);
    let dead = game.players[0].graveyard[0].id;
    let (exiled, _) = game
        .move_card_target_to_zone(
            dead,
            ZoneKind::Exile,
            ZoneMoveCause::Effect {
                controller: PlayerId::One,
            },
            None,
            ZonePlacement::Top,
        )
        .unwrap();
    game.move_card_target_to_zone(
        exiled,
        ZoneKind::Graveyard,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        None,
        ZonePlacement::Top,
    )
    .unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[0].library, library);
    assert_eq!(game.players[0].graveyard.len(), 1);
}

#[test]
fn a_stolen_katherine_does_not_trigger() {
    let (mut game, katherine) = staged(0, 6);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == katherine)
        .unwrap()
        .controller = PlayerId::Two;
    game.sacrifice_permanent(katherine);
    assert!(game.pending_triggers.is_empty());
    assert_eq!(game.players[0].graveyard.len(), 1);
}

#[test]
fn an_exact_six_card_library_returns_katherine_for_a_later_miracle() {
    let (mut game, katherine) = staged(0, 6);
    game.sacrifice_permanent(katherine);
    drain_pending(&mut game);
    assert_eq!(game.players[0].library.len(), 7);
    // Advance to the turn when Katherine is on top, preserving her new identity.
    while game.players[0].library.last().unwrap().definition != cards::TRIUMPH_OF_SAINT_KATHERINE {
        game.players[0].library.pop();
    }
    game.cards_drawn_this_turn = [0; 2];
    game.drawn_this_turn = [Vec::new(), Vec::new()];
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    let drawn = game.draw_card(PlayerId::One).unwrap();
    let reveal = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: reveal.id,
            options: vec![1],
        },
    )
    .unwrap();
    pass_until_decision(&mut game);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == drawn))
        .unwrap();
    game.apply(PlayerId::One, cast).unwrap();
    assert_eq!(game.players[0].mana_pool.total(), 0);
    drain_pending(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::TRIUMPH_OF_SAINT_KATHERINE)
        .unwrap();
    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Lifelink));
    assert_eq!(game.power(permanent), Some(5));
    assert_eq!(game.toughness(permanent), Some(5));
}

#[test]
fn exiling_the_pile_is_one_event_even_across_library_and_graveyard() {
    let (mut game, katherine) = staged(0, 6);
    let laelia = game
        .put_onto_battlefield(PlayerId::One, cards::LAELIA_THE_BLADE_REFORGED)
        .unwrap();
    game.sacrifice_permanent(katherine);
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == laelia)
            .unwrap()
            .counters(CounterKind::PlusOnePlusOne),
        1
    );
}

#[test]
fn the_seeded_pile_matches_reference_execution() {
    let (mut reference, katherine) = staged(42, 10);
    reference.set_prepared_engine_enabled(false);
    let mut prepared = reference.clone();
    prepared.set_prepared_engine_enabled(true);
    for game in [&mut reference, &mut prepared] {
        game.sacrifice_permanent(katherine);
        drain_pending(game);
    }
    assert_eq!(reference.players[0].library, prepared.players[0].library);
    for player in [PlayerId::One, PlayerId::Two] {
        assert_eq!(reference.observe(player), prepared.observe(player));
        assert_eq!(reference.events_for(player), prepared.events_for(player));
    }
}

#[test]
fn the_pending_trigger_and_returned_pile_round_trip_through_checkpoints() {
    let (mut game, katherine) = staged(0, 6);
    game.sacrifice_permanent(katherine);
    game.finish_rules_procedure();
    assert_eq!(game.stack.len(), 1);
    for viewer in [PlayerId::One, PlayerId::Two] {
        let (wire, hidden) = checkpoint_fixture(&game, viewer);
        let mut restored = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            42,
        )
        .unwrap();
        drain_pending(&mut restored);
        assert_eq!(restored.players[0].library.len(), 7);
        assert!(restored.players[0].graveyard.is_empty());
        let (wire, hidden) = checkpoint_fixture(&restored, viewer);
        Game::from_observation_checkpoint(
            restored.catalog.clone(),
            restored.format,
            &wire,
            &hidden,
            43,
        )
        .unwrap();
    }
}

#[test]
fn the_library_requirement_is_checked_on_resolution() {
    let (mut game, katherine) = staged(0, 5);
    game.sacrifice_permanent(katherine);
    assert_eq!(game.pending_triggers.len(), 1);
    game.set_library(PlayerId::One, &[cards::ISLAND; 6])
        .unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[0].library.len(), 7);
    assert!(game.players[0].graveyard.is_empty());
}
