//! Reckless Impulse: "until the end of your next turn" is a turn longer than
//! "until your next end step", and the difference only shows when the cards
//! were exiled on the holder's own turn.

use super::*;

/// Player One having resolved Reckless Impulse on their own turn, with the
/// two exiled cards' ids.
fn cast_on_your_own_turn() -> (Game, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (10_100, cards::SAVANNAH_LIONS),
            (10_101, cards::LIGHTNING_BOLT),
        ],
    );
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;

    let impulse = card(10_000, cards::RECKLESS_IMPULSE, PlayerId::One);
    game.players[0].hand.push(impulse.clone());
    game.players[0].mana_pool = ManaPool {
        red: 1,
        colorless: 1,
        ..ManaPool::default()
    };
    game.apply(
        PlayerId::One,
        cast_action(impulse.id, Vec::new(), Vec::new(), 0),
    )
    .expect("Reckless Impulse is castable");
    pass_priority_pair(&mut game);

    let exiled = game.players[0]
        .exile
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    assert_eq!(exiled.len(), 2, "both cards are exiled");
    (game, exiled)
}

fn playable(game: &Game, cards: &[GameObjectId]) -> bool {
    cards
        .iter()
        .all(|card| game.exile_play_permission(*card, PlayerId::One).is_some())
}

#[test]
fn the_cards_stay_playable_through_the_holders_following_turn() {
    let (mut game, exiled) = cast_on_your_own_turn();
    assert!(
        playable(&game, &exiled),
        "playable the turn they are exiled"
    );

    game.turns_started[PlayerId::Two.index()] += 1;
    game.active_player = PlayerId::Two;
    assert!(
        playable(&game, &exiled),
        "the opponent's turn does not end the grant"
    );

    game.turns_started[PlayerId::One.index()] += 1;
    game.active_player = PlayerId::One;
    assert!(
        playable(&game, &exiled),
        "\"your next turn\" is the whole of it, which is what a next end step would have cut short"
    );
}

#[test]
fn the_grant_lapses_once_the_turn_after_that_begins() {
    let (mut game, exiled) = cast_on_your_own_turn();

    game.turns_started[PlayerId::One.index()] += 2;
    assert!(
        !playable(&game, &exiled),
        "the permission does not outlive the turn it named"
    );
}
