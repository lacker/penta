use super::*;

fn staged(dredger: CardDefinitionId, library_size: usize) -> Game {
    let mut game = ready_game();
    game.players[0]
        .graveyard
        .push(card(195_000, dredger, PlayerId::One));
    game.players[0].library = (0..library_size)
        .map(|n| {
            card(
                195_010 + u32::try_from(n).unwrap(),
                cards::PLAINS,
                PlayerId::One,
            )
        })
        .collect();
    game
}

fn choose_replacement(game: &mut Game, name: &str) {
    let choice = game
        .observe(PlayerId::One)
        .decision
        .expect("draw replacement choice");
    let option = choice
        .options
        .iter()
        .find(|option| option.label.contains(name))
        .unwrap()
        .id;
    game.choose_decision(PlayerId::One, choice.id, &[option]);
    game.finish_rules_procedure();
}

#[test]
fn dredge_requires_the_full_mill_and_may_be_declined() {
    for (dredger, count) in [
        (cards::SHAMBLING_SHELL, 3),
        (cards::GOLGARI_THUG, 4),
        (cards::STINKWEED_IMP, 5),
        (cards::GOLGARI_GRAVE_TROLL, 6),
    ] {
        let mut game = staged(dredger, count - 1);
        assert!(game.draw_card(PlayerId::One).is_some());
        assert_eq!(game.players[0].graveyard.len(), 1);
        assert!(game.observe(PlayerId::One).decision.is_none());

        let mut game = staged(dredger, count);
        game.draw_card(PlayerId::One);
        let decision = game.observe(PlayerId::One).decision.unwrap();
        game.choose_decision(PlayerId::One, decision.id, &[0]);
        assert_eq!(game.players[0].hand[0].definition, cards::PLAINS);
        assert_eq!(game.players[0].graveyard.len(), 1);

        let mut game = staged(dredger, count);
        game.draw_card(PlayerId::One);
        choose_replacement(&mut game, "Dredge");
        assert_eq!(game.players[0].hand[0].definition, dredger);
        assert_ne!(game.players[0].hand[0].id, CardInstanceId(195_000));
        assert_eq!(game.players[0].graveyard.len(), count);
        assert!(game.players[0].library.is_empty());
        assert_eq!(game.cards_drawn_this_turn[0], 0);
        assert!(!game.players[0].tried_to_draw_from_empty_library);
    }
}

#[test]
fn dredge_rechecks_newly_milled_cards_between_draws_and_defers_triggers() {
    let mut game = staged(cards::SHAMBLING_SHELL, 8);
    game.players[0].library[7].definition = cards::STINKWEED_IMP;
    game.players[0].library[6].definition = cards::NARCOMOEBA;
    game.draw_cards(PlayerId::One, 2);
    choose_replacement(&mut game, "Shambling Shell");
    assert!(
        game.stack.is_empty(),
        "mill triggers wait for the entire draw instruction"
    );
    choose_replacement(&mut game, "Stinkweed Imp");
    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .map(|c| c.definition)
            .collect::<Vec<_>>(),
        vec![cards::SHAMBLING_SHELL, cards::STINKWEED_IMP]
    );
    assert_eq!(game.cards_drawn_this_turn[0], 0);
    assert!(game.players[0].library.is_empty());
    assert!(
        game.pending_triggers.len() + game.stack.len() > 0,
        "Narcomoeba triggered"
    );
}

#[test]
fn dredge_competes_with_mandatory_draw_replacement_and_respects_draw_prohibition() {
    let mut game = staged(cards::GOLGARI_THUG, 4);
    game.put_onto_battlefield(PlayerId::Two, cards::HULLBREACHER)
        .unwrap();
    game.draw_card(PlayerId::One);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.options.len(), 2);
    assert!(decision.options.iter().all(|option| option.id != 0));
    choose_replacement(&mut game, "Golgari Thug");
    assert_eq!(game.players[0].hand[0].definition, cards::GOLGARI_THUG);
    assert_eq!(
        game.battlefield.len(),
        1,
        "no Treasure from the replaced draw"
    );

    let mut game = staged(cards::GOLGARI_THUG, 4);
    game.put_onto_battlefield(PlayerId::Two, cards::NARSET_PARTER_OF_VEILS)
        .unwrap();
    game.cards_drawn_this_turn[0] = 1;
    game.draw_card(PlayerId::One);
    assert!(game.observe(PlayerId::One).decision.is_none());
    assert_eq!(game.players[0].library.len(), 4);
}

#[test]
fn dredge_still_returns_its_source_when_milled_cards_are_exiled() {
    let mut game = staged(cards::GOLGARI_THUG, 4);
    game.put_onto_battlefield(PlayerId::Two, cards::LEYLINE_OF_THE_VOID)
        .unwrap();
    game.draw_card(PlayerId::One);
    choose_replacement(&mut game, "Golgari Thug");
    assert_eq!(game.players[0].exile.len(), 4);
    assert!(game.players[0].graveyard.is_empty());
    assert_eq!(game.players[0].hand[0].definition, cards::GOLGARI_THUG);
}

#[test]
fn dredge_choice_round_trips_through_checkpoint() {
    let mut game = staged(cards::GOLGARI_THUG, 4);
    game.draw_card(PlayerId::One);
    let (wire, hypothesis) = checkpoint_fixture(&game, PlayerId::One);
    let mut rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &hypothesis,
        195_099,
    )
    .expect("graveyard replacement reconstructs");
    choose_replacement(&mut rebuilt, "Golgari Thug");
    assert_eq!(rebuilt.players[0].hand[0].definition, cards::GOLGARI_THUG);
    assert_eq!(rebuilt.cards_drawn_this_turn[0], 0);
}

#[test]
fn dredge_grave_troll_counts_itself_before_reanimation_and_can_regenerate() {
    let mut game = staged(cards::GOLGARI_GRAVE_TROLL, 8);
    game.players[0]
        .graveyard
        .push(card(195_100, cards::STINKWEED_IMP, PlayerId::One));
    game.move_card_from_nonbattlefield_zone(
        CardInstanceId(195_000),
        ZoneKind::Graveyard,
        ZoneKind::Battlefield,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        None,
    )
    .unwrap();
    game.finish_rules_procedure();
    let troll = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::GOLGARI_GRAVE_TROLL)
        .unwrap();
    assert_eq!(troll.counters(CounterKind::PlusOnePlusOne), 2);
    let source = troll.card.id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source: id, .. } if *id == source),
        )
        .expect("regeneration pays a counter and one mana");
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);
    let troll = game
        .battlefield
        .iter()
        .find(|p| p.card.id == source)
        .unwrap();
    assert_eq!(troll.counters(CounterKind::PlusOnePlusOne), 1);
}
