//! Underworld Dreams damages whoever drew, which is read off the event
//! rather than assumed from the trigger's own "an opponent" filter.

use super::*;

fn staged() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.battlefield
        .push(creature(31_000, cards::UNDERWORLD_DREAMS, PlayerId::One));
    for player in [PlayerId::One, PlayerId::Two] {
        game.players[player.index()].library = (0..4)
            .map(|i| card(31_100 + i, cards::SAVANNAH_LIONS, player))
            .collect();
        game.players[player.index()].life = 20;
    }
    game
}

fn settle(game: &mut Game) {
    for _ in 0..12 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
}

#[test]
fn an_opponents_draw_damages_that_opponent() {
    let mut game = staged();
    game.draw_card(PlayerId::Two);
    settle(&mut game);

    assert_eq!(
        game.players[1].life, 19,
        "the player who drew took the damage"
    );
    assert_eq!(game.players[0].life, 20, "and its controller took none");
}

#[test]
fn the_controllers_own_draw_does_nothing() {
    let mut game = staged();
    game.draw_card(PlayerId::One);
    settle(&mut game);

    assert_eq!(
        (game.players[0].life, game.players[1].life),
        (20, 20),
        "the trigger reads opponents only"
    );
}
