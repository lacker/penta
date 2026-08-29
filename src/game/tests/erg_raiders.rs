//! A creature that bills its controller for staying home.
//!
//! Two facts about the permanent itself, both read at the end step: whether
//! it went to war, and whether it has been around long enough to be asked.
//! The turn it arrives is free, which is what stops it punishing a player who
//! could not have attacked with it anyway.

use super::*;

/// Erg Raiders under player one at their end step. `turns_out` is how many of
/// their turns have passed since it arrived, and `attacked` whether it went.
fn end_step_with_raiders(turns_out: u32, attacked: bool) -> Game {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = turns_out;
    game.active_player = PlayerId::One;
    let mut raiders = creature(10_000, cards::ERG_RAIDERS, PlayerId::One);
    raiders.attacked_this_turn = attacked;
    game.battlefield.push(raiders);

    game.step = Step::PostcombatMain;
    game.advance_step();
    game.finish_rules_procedure();
    drain_pending(&mut game);
    game
}

fn life(game: &Game) -> i16 {
    game.players[PlayerId::One.index()].life
}

#[test]
fn staying_home_costs_its_controller_two() {
    let game = end_step_with_raiders(5, false);

    assert_eq!(life(&game), i16::from(rules::STARTING_LIFE) - 2);
}

/// The first half of the condition: a creature that attacked owes nothing.
#[test]
fn attacking_costs_nothing() {
    let game = end_step_with_raiders(5, true);

    assert_eq!(life(&game), i16::from(rules::STARTING_LIFE));
}

/// The second half, and the reason the card is playable: the turn it arrives
/// it could not have attacked, so it is not billed for not attacking.
#[test]
fn the_turn_it_arrives_is_free() {
    let game = end_step_with_raiders(0, false);

    assert_eq!(life(&game), i16::from(rules::STARTING_LIFE));
}
