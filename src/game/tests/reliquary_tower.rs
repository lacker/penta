//! Reliquary Tower, and the maximum hand size it removes.
//!
//! The limit is read at cleanup rather than captured, and the clause says
//! "you" -- so it covers its controller's cleanup and nobody else's, and
//! losing the Tower puts the limit straight back.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game
}

fn fill_hand(game: &mut Game, player: PlayerId, count: u32) {
    game.players[player.index()].hand.clear();
    for index in 0..count {
        game.players[player.index()]
            .hand
            .push(card(30_000 + index, cards::GRIZZLY_BEARS, player));
    }
}

/// Answers whether the active player's cleanup stopped to ask for a discard.
fn cleanup_asks(game: &mut Game) -> bool {
    game.cleanup();
    game.cleanup_pending
}

#[test]
fn the_tower_lifts_the_limit_for_its_controller() {
    let mut game = ready();
    fill_hand(&mut game, PlayerId::One, 9);
    assert!(cleanup_asks(&mut game), "nine cards and no Tower");

    let mut game = ready();
    fill_hand(&mut game, PlayerId::One, 9);
    game.put_onto_battlefield(PlayerId::One, cards::RELIQUARY_TOWER)
        .expect("cataloged");
    assert!(!cleanup_asks(&mut game), "the Tower lifts it");
}

/// "You", so the opponent's Tower does nothing for you.
#[test]
fn an_opposing_tower_does_not_help() {
    let mut game = ready();
    fill_hand(&mut game, PlayerId::One, 9);
    game.put_onto_battlefield(PlayerId::Two, cards::RELIQUARY_TOWER)
        .expect("cataloged");
    assert!(cleanup_asks(&mut game), "theirs, not yours");
}

/// Read at cleanup rather than captured, so losing the Tower first restores
/// the limit for that very cleanup.
#[test]
fn the_limit_returns_when_the_tower_leaves() {
    let mut game = ready();
    fill_hand(&mut game, PlayerId::One, 9);
    game.put_onto_battlefield(PlayerId::One, cards::RELIQUARY_TOWER)
        .expect("cataloged");
    let tower = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::RELIQUARY_TOWER)
        .expect("it is there")
        .card
        .id;
    game.battlefield
        .retain(|permanent| permanent.card.id != tower);

    assert!(cleanup_asks(&mut game), "the limit is back");
}

/// A hand under the limit never asks, Tower or not.
#[test]
fn a_small_hand_is_never_asked() {
    let mut game = ready();
    fill_hand(&mut game, PlayerId::One, 7);
    assert!(!cleanup_asks(&mut game), "seven is the limit, not over it");
}
