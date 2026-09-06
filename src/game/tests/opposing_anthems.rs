//! Anthems that point two ways at once, and one that switches on a
//! graveyard. The Evincar's second clause is a negation -- "nonblack" --
//! which is the shape that quietly hits everything when the inner test
//! cannot be evaluated, so it is checked against a black creature, a green
//! one, and the Evincar itself.

use super::*;

/// The Evincar under player one, a black creature of mine, and a green one
/// of the opponent's.
fn evincar_board() -> (Game, GameObjectId, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut evincar = creature(75_000, cards::ASCENDANT_EVINCAR, PlayerId::One);
    evincar.entered_controller_turn = 0;
    let evincar_id = evincar.card.id;
    game.battlefield.push(evincar);
    let mut zombie = creature(75_001, cards::ZOMBIE_CANNIBAL, PlayerId::One);
    zombie.entered_controller_turn = 0;
    let zombie_id = zombie.card.id;
    game.battlefield.push(zombie);
    let mut bear = creature(75_002, cards::GRIZZLY_BEARS, PlayerId::Two);
    bear.entered_controller_turn = 0;
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    (game, evincar_id, zombie_id, bear_id)
}

fn size(game: &Game, id: GameObjectId) -> (i16, i16) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the creature is on the battlefield");
    (
        game.power(permanent).expect("power"),
        game.toughness(permanent).expect("toughness"),
    )
}

#[test]
fn the_evincar_grows_the_black_creatures() {
    let (game, _, zombie, _) = evincar_board();
    assert_eq!(size(&game, zombie), (2, 2), "a 1/1 Zombie one bigger");
}

#[test]
fn the_evincar_shrinks_only_the_nonblack_ones() {
    let (game, _, _, bear) = evincar_board();
    assert_eq!(
        size(&game, bear),
        (1, 1),
        "a green Bears is nonblack, so it shrinks"
    );
}

#[test]
fn the_evincar_is_untouched_by_either_clause() {
    let (game, evincar, _, _) = evincar_board();
    assert_eq!(
        size(&game, evincar),
        (3, 3),
        "black, so the shrink misses it, and \"other\", so the anthem does too"
    );
}

/// The Seraph and one other creature of mine, with `graveyard` cards in my
/// graveyard.
fn seraph_board(graveyard: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut seraph = creature(75_100, cards::SILVER_SERAPH, PlayerId::One);
    seraph.entered_controller_turn = 0;
    game.battlefield.push(seraph);
    let mut bear = creature(75_101, cards::GRIZZLY_BEARS, PlayerId::One);
    bear.entered_controller_turn = 0;
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    for index in 0..graveyard {
        let filler = card(
            75_200 + u32::try_from(index).expect("a small fixture"),
            cards::MOUNTAIN,
            PlayerId::One,
        );
        game.players[0].graveyard.push(filler);
    }
    (game, bear_id)
}

#[test]
fn the_seraph_waits_for_threshold() {
    let (game, bear) = seraph_board(6);
    assert_eq!(size(&game, bear), (2, 2), "six cards is one short");

    let (game, bear) = seraph_board(7);
    assert_eq!(
        size(&game, bear),
        (4, 4),
        "the seventh card switches the anthem on"
    );
}
