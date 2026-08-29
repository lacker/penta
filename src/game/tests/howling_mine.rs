//! An extra card for whoever is drawing, while the Mine stays untapped.
//!
//! The "if untapped" is an intervening-if, so it is read when the step begins
//! and again as the trigger resolves. That second reading is the whole reason
//! tapping the Mine in response denies the card, and it is what separates this
//! from a condition checked once.

use super::*;

/// A Howling Mine under player one, with both libraries stocked and the turn
/// sitting in `player`'s upkeep, one `advance_step` short of the draw.
fn mined(player: PlayerId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    let mine = creature(10_000, cards::HOWLING_MINE, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    for owner in [PlayerId::One, PlayerId::Two] {
        for index in 0..10 {
            let filler = card(
                30_000 + index + 100 * u32::from(owner as u8),
                cards::SEDGE_TROLL,
                owner,
            );
            game.players[owner.index()].library.push(filler);
        }
    }
    // Past the first turn, whose draw player one skips.
    game.turn = 5;
    game.active_player = player;
    game.priority = player;
    game.step = Step::Upkeep;
    (game, mine_id)
}

fn hand_size(game: &Game, player: PlayerId) -> usize {
    game.players[player.index()].hand.len()
}

fn set_tapped(game: &mut Game, id: GameObjectId, tapped: bool) {
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .tapped = tapped;
}

/// Walks from upkeep into the draw step and settles it, reporting how many
/// cards `player` gained on the way.
fn cards_drawn(game: &mut Game, player: PlayerId) -> usize {
    let before = hand_size(game, player);
    game.advance_step();
    assert_eq!(game.step, Step::Draw);
    pass_priority_pair(game);
    drain_pending(game);
    hand_size(game, player) - before
}

#[test]
fn its_controller_draws_an_extra_card() {
    let (mut game, _mine) = mined(PlayerId::One);
    assert_eq!(cards_drawn(&mut game, PlayerId::One), 2);
}

/// It is symmetric, which is the whole character of the card.
#[test]
fn the_opponent_draws_an_extra_card_too() {
    let (mut game, _mine) = mined(PlayerId::Two);
    assert_eq!(cards_drawn(&mut game, PlayerId::Two), 2);
}

/// The control: tapped when the step begins, so nothing is even put on the
/// stack.
#[test]
fn a_tapped_mine_gives_nothing() {
    let (mut game, mine) = mined(PlayerId::One);
    set_tapped(&mut game, mine, true);
    let before = hand_size(&game, PlayerId::One);

    game.advance_step();
    assert_eq!(game.step, Step::Draw);
    assert!(
        game.pending_triggers.is_empty() && game.stack.is_empty(),
        "a tapped Mine fails the condition as the step begins",
    );
    pass_priority_pair(&mut game);
    drain_pending(&mut game);

    assert_eq!(hand_size(&game, PlayerId::One) - before, 1);
}

/// Untapped when the step begins, tapped before the trigger resolves. The
/// intervening-if reads the state a second time and finds it false, so the
/// trigger is removed without effect.
#[test]
fn tapping_it_in_response_denies_the_extra_card() {
    let (mut game, mine) = mined(PlayerId::One);
    let before = hand_size(&game, PlayerId::One);

    game.advance_step();
    assert_eq!(game.step, Step::Draw);
    assert!(
        !game.pending_triggers.is_empty() || !game.stack.is_empty(),
        "the trigger is waiting, which is what leaves room to respond",
    );

    set_tapped(&mut game, mine, true);
    pass_priority_pair(&mut game);
    drain_pending(&mut game);

    assert_eq!(
        hand_size(&game, PlayerId::One) - before,
        1,
        "only the ordinary draw survived",
    );
}
