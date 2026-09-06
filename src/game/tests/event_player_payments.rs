//! A payment aimed at whoever the event names rather than at the
//! permanent's controller. Getting it backwards is invisible from one seat,
//! so the same trigger is driven from both.

use super::*;

/// Phyrexian Tyranny under player one, with `drawer` drawing a card.
fn tyranny(drawer: PlayerId) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    let tyranny = creature(89_200, cards::PHYREXIAN_TYRANNY, PlayerId::One);
    game.battlefield.push(tyranny);
    game.priority = drawer;
    game.draw_card(drawer);
    for _ in 0..12 {
        drain_pending(&mut game);
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
    game
}

#[test]
fn the_tax_falls_on_whoever_drew() {
    let game = tyranny(PlayerId::Two);
    assert_eq!(
        (game.players[0].life, game.players[1].life),
        (20, 18),
        "the opponent drew, so the opponent paid -- the Tyranny is mine"
    );

    let game = tyranny(PlayerId::One);
    assert_eq!(
        (game.players[0].life, game.players[1].life),
        (18, 20),
        "and it bills me just as readily"
    );
}
