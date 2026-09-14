use super::*;

#[test]
fn dredge_does_not_replace_the_opponents_draw() {
    let mut game = ready_game();
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::LIFE_FROM_THE_LOAM])
        .unwrap();
    game.players[0].library = game.build_zone(PlayerId::One, &[cards::FOREST; 4]).unwrap();
    assert!(game.draw_card(PlayerId::Two).is_some());
    assert!(game.observe(PlayerId::One).decision.is_none());
    assert!(game.observe(PlayerId::Two).decision.is_none());
    assert_eq!(game.players[0].library.len(), 4);
    assert_eq!(
        game.players[0].graveyard[0].definition,
        cards::LIFE_FROM_THE_LOAM
    );
}

#[test]
fn blast_zone_charges_twice_x_and_uses_sacrificed_counter_count() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let zone = game
        .put_onto_battlefield(PlayerId::One, cards::BLAST_ZONE)
        .unwrap();
    let bear = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    let elf = game
        .put_onto_battlefield(PlayerId::Two, cards::LLANOWAR_ELVES)
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    let actions = game.legal_actions(PlayerId::One);
    assert!(!actions.iter().any(
        |action| matches!(action, Action::ActivateAbility { source, x: 2, .. } if *source == zone)
    ));
    let activation = actions.into_iter().find(|action| matches!(action, Action::ActivateAbility { source, x: 1, .. } if *source == zone)).unwrap();
    game.apply(PlayerId::One, activation).unwrap();
    drain_pending(&mut game);
    let permanent = game
        .battlefield
        .iter_mut()
        .find(|p| p.card.id == zone)
        .unwrap();
    assert_eq!(permanent.counters(CounterKind::named("charge")), 2);
    permanent.tapped = false;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let action = game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::ActivateAbility { source, ability, .. } if *source == zone && game.ability_for_origin(zone, *ability).is_some_and(|a| a.text.starts_with("{3}")))
    }).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert!(!game.battlefield.iter().any(|p| p.card.id == zone));
    drain_pending(&mut game);
    assert!(!game.battlefield.iter().any(|p| p.card.id == bear));
    assert!(game.battlefield.iter().any(|p| p.card.id == elf));
}

#[test]
fn drop_of_honey_chooses_only_destroyable_creatures_tied_for_least_power() {
    let mut game = ready_game();
    game.battlefield.clear();
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let lion = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .unwrap();
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == bear)
        .unwrap()
        .temporary_keywords
        .push(KeywordAbility::Indestructible);
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == lion)
        .unwrap()
        .regeneration_shields = 1;
    // Shroud cannot stop this choice: the ability does not target.
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == lion)
        .unwrap()
        .temporary_keywords
        .push(KeywordAbility::Shroud);
    game.put_onto_battlefield(PlayerId::One, cards::DROP_OF_HONEY)
        .unwrap();
    game.active_player = PlayerId::One;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);
    assert!(!game.battlefield.iter().any(|p| p.card.id == lion));
    assert!(game.battlefield.iter().any(|p| p.card.id == bear));
    assert!(game.battlefield.iter().any(|p| p.card.id == angel));
    // The only weakest creature is indestructible; the stronger one is not an alternative.
    game.handle_upkeep_triggers();
    drain_pending(&mut game);
    assert!(game.battlefield.iter().any(|p| p.card.id == angel));
}

#[test]
fn drop_of_honey_captures_an_empty_battlefield_before_a_creature_returns() {
    let mut game = ready_game();
    game.battlefield.clear();
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let honey = game
        .put_onto_battlefield(PlayerId::One, cards::DROP_OF_HONEY)
        .unwrap();
    game.move_permanents_to_zone(&[bear], ZoneKind::Exile, ZonePlacement::Top);
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut game =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 42)
            .unwrap();
    drain_pending(&mut game);
    assert!(!game.battlefield.iter().any(|p| p.card.id == honey));
}

#[test]
fn life_from_the_loam_returns_up_to_three_own_lands_and_can_target_none() {
    for count in [0, 3] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[0].hand = game
            .build_zone(PlayerId::One, &[cards::LIFE_FROM_THE_LOAM])
            .unwrap();
        let loam = game.players[0].hand[0].id;
        game.players[0].graveyard = game
            .build_zone(
                PlayerId::One,
                &[
                    cards::FOREST,
                    cards::WASTELAND,
                    cards::DARK_DEPTHS,
                    cards::GRIZZLY_BEARS,
                ],
            )
            .unwrap();
        game.players[1].graveyard = game.build_zone(PlayerId::Two, &[cards::FOREST]).unwrap();
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
        let actions = game.legal_actions(PlayerId::One);
        let forbidden = [
            game.players[0].graveyard[3].id,
            game.players[1].graveyard[0].id,
        ];
        for action in &actions {
            if let Action::CastSpell { card, choices, .. } = action
                && *card == loam
            {
                assert!(
                    !choices
                        .targets()
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|t| matches!(t, Target::Card(id) if forbidden.contains(id)))
                );
            }
        }
        let action = actions
            .into_iter()
            .find(|action| {
                matches!(action, Action::CastSpell { card, choices, .. } if *card == loam
                && choices.targets().iter().flat_map(TargetSelection::targets).count() == count)
            })
            .expect("the requested target count is legal");
        game.apply(PlayerId::One, action).unwrap();
        drain_pending(&mut game);
        assert_eq!(game.players[0].hand.len(), count);
        assert_eq!(game.players[0].graveyard.len(), 5 - count);
    }
}
