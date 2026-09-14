use super::*;

fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.players[0].mana_pool = ManaPool::default();
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    let hollow = card(195_000, cards::HOLLOW_ONE, PlayerId::One);
    let source = hollow.id;
    game.players[0].hand.push(hollow);
    (game, source)
}

fn can_cast(game: &Game, source: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One)
        .iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == source))
}

#[test]
fn hollow_one_discard_discount_survives_checkpoint_and_expires_next_turn() {
    let (mut game, hollow) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let mut own = Vec::new();
    for index in 0..2 {
        let held = card(195_010 + index, cards::SWAMP, PlayerId::One);
        own.push(held.id);
        game.players[0].hand.push(held);
    }
    let opposing = card(195_020, cards::SWAMP, PlayerId::Two);
    let opposing_id = opposing.id;
    game.players[1].hand.push(opposing);
    game.discard_cards(PlayerId::Two, &[opposing_id]);
    assert!(
        !can_cast(&game, hollow),
        "opponent's discards do not reduce it"
    );
    game.discard_cards(PlayerId::One, &own);
    assert!(can_cast(&game, hollow), "two cards reduce five mana to one");
    assert_eq!(game.cards_discarded_this_turn, [2, 1]);

    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &hidden,
        195_030,
    )
    .unwrap();
    assert_eq!(rebuilt.cards_discarded_this_turn, [2, 1]);
    assert!(can_cast(&rebuilt, hollow));
    let mut legacy_wire = wire.clone();
    legacy_wire
        .get_mut("checkpoint")
        .unwrap()
        .as_object_mut()
        .unwrap()
        .remove("cardsDiscardedThisTurn");
    let legacy = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &legacy_wire,
        &hidden,
        195_031,
    )
    .unwrap();
    assert_eq!(legacy.cards_discarded_this_turn, [0, 0]);

    let turn = game.turn;
    for _ in 0..60 {
        if game.turn > turn {
            break;
        }
        game.advance_step();
        drain_pending(&mut game);
    }
    assert_eq!(game.cards_discarded_this_turn, [0, 0]);
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    assert!(!can_cast(&game, hollow));
}

#[test]
fn hollow_one_cycling_counts_once_as_a_discard_before_the_draw() {
    let (mut game, hollow) = staged();
    let cycling = card(195_100, cards::HOLLOW_ONE, PlayerId::One);
    let cycling_id = cycling.id;
    game.players[0].hand.push(cycling);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    let activation = game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::ActivateAbility { source, .. } if *source == cycling_id)
    }).expect("cycling costs two mana");
    let library_before = game.players[0].library.len();
    game.apply(PlayerId::One, activation).unwrap();
    assert_eq!(game.cards_discarded_this_turn, [1, 0]);
    assert_eq!(game.players[0].library.len(), library_before);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].library.len(), library_before - 1);
    assert!(
        !can_cast(&game, hollow),
        "two mana remains; one cycle only discounts by two"
    );
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    assert!(can_cast(&game, hollow));
}
