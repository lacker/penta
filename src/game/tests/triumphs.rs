//! "If you control the creature with the greatest power."
//!
//! A tie counts, which is the whole difficulty: the question is not whether
//! one creature stands alone but whether anything is strictly bigger. What
//! these check are all three answers -- bigger, tied, and smaller -- and the
//! empty board, where nobody controls the greatest anything.

use super::*;

/// Player one's upkeep with Triumph of Ferocity out, and creatures of the
/// given powers under each player.
fn upkeep_with(mine: &[i16], theirs: &[i16]) -> Game {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let triumph = creature(10_000, cards::TRIUMPH_OF_FEROCITY, PlayerId::One);
    game.battlefield.push(triumph);

    let mut next = 11_000;
    for (owner, powers) in [(PlayerId::One, mine), (PlayerId::Two, theirs)] {
        for power in powers {
            let permanent = creature(next, cards::SEDGE_TROLL, owner);
            let permanent_id = permanent.card.id;
            // Sedge Troll is a 2/2, so the bonus carries it to `power`.
            game.battlefield.push(permanent);
            attach_constant_resolved_characteristics(
                &mut game,
                permanent_id,
                &[AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(i32::from(*power) - 2),
                    ValueDef::Constant(0),
                )],
                ContinuousEffectExpiration::Never,
            );
            next += 1;
        }
    }
    game
}

/// Runs player one's upkeep and reports how many cards they drew. The turn's
/// own draw step does not run in this harness, so the count is the Triumph's
/// alone.
fn cards_drawn_over_upkeep(game: &mut Game) -> usize {
    let before = game.players[PlayerId::One.index()].hand.len();
    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_pending(game);
    game.players[PlayerId::One.index()]
        .hand
        .len()
        .saturating_sub(before)
}

#[test]
fn controlling_the_biggest_creature_draws() {
    let mut game = upkeep_with(&[4], &[3]);
    assert_eq!(cards_drawn_over_upkeep(&mut game), 1);
}

/// A tie counts, which is the case a naive "strictly greater" check misses.
#[test]
fn a_tie_still_counts() {
    let mut game = upkeep_with(&[3], &[3]);
    assert_eq!(cards_drawn_over_upkeep(&mut game), 1);
}

#[test]
fn being_outclassed_draws_nothing_extra() {
    let mut game = upkeep_with(&[2], &[5]);
    assert_eq!(cards_drawn_over_upkeep(&mut game), 0);
}

/// With no creatures at all nobody controls the greatest, so the condition
/// is false rather than vacuously true.
#[test]
fn an_empty_board_is_not_the_greatest() {
    let mut game = upkeep_with(&[], &[]);
    assert_eq!(cards_drawn_over_upkeep(&mut game), 0);
}
