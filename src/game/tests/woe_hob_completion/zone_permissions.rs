use super::*;

#[test]
fn woe_hob_hearth_counts_adventure_cards_once_and_stoke_genius_discards_before_drawing() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.players[0].graveyard = game
            .build_zone(
                PlayerId::One,
                &[
                    cards::LIGHTNING_BOLT,
                    cards::DIVINATION,
                    cards::BESOTTED_KNIGHT,
                    cards::GRIZZLY_BEARS,
                ],
            )
            .unwrap();
        let hearth = hand(&mut game, cards::HEARTH_ELEMENTAL);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
        let body = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::CastSpell { card, choices, .. }
                if *card == hearth && choices.play_option() == PlayOptionId::DEFAULT)
            })
            .unwrap();
        game.apply(PlayerId::One, body).unwrap();
        assert_eq!(game.players[0].mana_pool, ManaPool::default());
        drain_pending(&mut game);
        assert!(
            game.battlefield
                .iter()
                .any(|p| p.card.definition == cards::HEARTH_ELEMENTAL)
        );

        let adventure = hand(&mut game, cards::HEARTH_ELEMENTAL);
        hand(&mut game, cards::PLAINS);
        hand(&mut game, cards::FOREST);
        game.players[0].library = game
            .build_zone(PlayerId::One, &[cards::ISLAND, cards::MOUNTAIN])
            .unwrap();
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::CastSpell { card, choices, .. }
                if *card == adventure && choices.play_option() == PlayOptionId(1))
            })
            .unwrap();
        game.apply(PlayerId::One, action).unwrap();
        drain_pending(&mut game);
        assert_eq!(game.players[0].hand.len(), 2);
        assert!(
            game.players[0]
                .hand
                .iter()
                .all(|card| [cards::ISLAND, cards::MOUNTAIN].contains(&card.definition))
        );
        assert!(
            game.players[0]
                .graveyard
                .iter()
                .any(|card| card.definition == cards::PLAINS)
        );
        let exiled = game.players[0]
            .exile
            .iter()
            .find(|card| card.definition == cards::HEARTH_ELEMENTAL)
            .unwrap();
        assert!(
            game.exile_play_permission(exiled.id, PlayerId::One)
                .unwrap()
                .adventure_return_only
        );
    }
}

#[test]
fn woe_hob_tablet_grants_a_land_play_and_the_permission_follows_only_the_milled_incarnation() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.players[0].library = game.build_zone(PlayerId::One, &[cards::FOREST]).unwrap();
        game.put_onto_battlefield(PlayerId::One, cards::TABLET_OF_DISCOVERY)
            .unwrap();
        drain_pending(&mut game);
        let forest = game.players[0].graveyard[0].id;
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == forest))
            .expect("Tablet permits playing the milled land");
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for resolved in [&mut game, &mut rebuilt] {
            resolved.apply(PlayerId::One, action.clone()).unwrap();
            let forest = resolved
                .battlefield
                .iter()
                .find(|p| p.card.definition == cards::FOREST)
                .unwrap()
                .card
                .id;
            resolved.sacrifice_permanent(forest);
            let returned = resolved.players[0].graveyard.last().unwrap().id;
            assert_ne!(returned, forest);
            assert!(
                resolved
                    .exile_play_permission(returned, PlayerId::One)
                    .is_none()
            );
        }
    }
}

#[test]
fn woe_hob_airbend_grants_the_owner_a_fixed_cost_after_reconstruction() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let original = game
            .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
        game.put_onto_battlefield(PlayerId::One, cards::AIRBENDER_ASCENSION)
            .unwrap();
        drain_pending(&mut game);
        assert!(!game.battlefield.iter().any(|p| p.card.id == original));
        let exiled = game.players[1].exile[0].id;
        assert!(game.exile_play_permission(exiled, PlayerId::One).is_none());
        assert_eq!(
            game.exile_play_permission(exiled, PlayerId::Two)
                .unwrap()
                .cost,
            ExilePlayCost::AlternativeMana(mana_cost!("{2}"))
        );
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::Two);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            state.active_player = PlayerId::Two;
            state.priority = PlayerId::Two;
            state.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, 2);
            let action = cast_for(state, PlayerId::Two, exiled)
                .expect("airbend does not require green mana");
            state.apply(PlayerId::Two, action).unwrap();
            assert_eq!(state.players[1].mana_pool, ManaPool::default());
            drain_pending(state);
            assert!(state.battlefield.iter().any(
                |p| p.card.definition == cards::GRIZZLY_BEARS && p.controller == PlayerId::Two
            ));
        }
    }
}

#[test]
fn woe_hob_thor_permission_includes_the_next_end_step_but_expires_at_cleanup() {
    let mut game = setup(true);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT])
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::THOR_GOD_OF_THUNDER)
        .unwrap();
    drain_pending(&mut game);
    let exiled = game.players[0].exile[0].id;
    game.turns_started[0] += 1;
    game.step = Step::End;
    assert!(game.exile_play_permission(exiled, PlayerId::One).is_some());
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
            .unwrap();
    assert!(
        rebuilt
            .exile_play_permission(exiled, PlayerId::One)
            .is_some()
    );
    rebuilt.finish_cleanup();
    assert!(
        rebuilt
            .exile_play_permission(exiled, PlayerId::One)
            .is_none()
    );
}

#[test]
fn woe_hob_seam_rip_returns_before_priority_and_survives_reconstruction() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
        let seam = game
            .put_onto_battlefield(PlayerId::One, cards::SEAM_RIP)
            .unwrap();
        drain_pending(&mut game);
        assert_eq!(game.players[1].exile.len(), 1);
        assert_eq!(game.exile_returns.len(), 1);
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            state.sacrifice_permanent(seam);
            assert!(state.players[1].exile.is_empty());
            assert!(state.exile_returns.is_empty());
            assert!(state.battlefield.iter().any(
                |p| p.card.definition == cards::GRIZZLY_BEARS && p.controller == PlayerId::Two
            ));
            assert!(
                state.stack.is_empty(),
                "returning the card does not use the stack"
            );
        }
        let mut gone = setup(prepared);
        gone.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
        let seam = gone
            .put_onto_battlefield(PlayerId::One, cards::SEAM_RIP)
            .unwrap();
        gone.sacrifice_permanent(seam);
        drain_pending(&mut gone);
        assert!(gone.players[1].exile.is_empty());
        assert!(
            gone.battlefield
                .iter()
                .any(|p| p.card.definition == cards::GRIZZLY_BEARS)
        );
    }
}

#[test]
fn woe_hob_cloak_returns_a_chosen_hand_card_to_its_original_zone() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.players[1].hand = game
            .build_zone(PlayerId::Two, &[cards::DIVINATION])
            .unwrap();
        let cloak = game
            .put_onto_battlefield(PlayerId::One, cards::CLOAK_AND_DAGGER_ENTWINED)
            .unwrap();
        drain_pending(&mut game);
        assert!(game.players[1].hand.is_empty());
        assert_eq!(game.players[1].exile.len(), 1);
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            state.sacrifice_permanent(cloak);
            assert_eq!(state.players[1].hand.len(), 1);
            assert_eq!(state.players[1].hand[0].definition, cards::DIVINATION);
            assert!(state.players[1].exile.is_empty());
            assert!(state.stack.is_empty());
        }
    }
}
