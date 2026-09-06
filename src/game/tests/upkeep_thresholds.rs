//! Two upkeep triggers gated on a life total, in opposite directions. The
//! intervening-if is checked as the step begins and again on resolution, so
//! what needs covering is that the threshold actually gates: neither should
//! fire from the wrong side of it, and the boundary itself is inclusive.

use super::*;

/// `definition` under player one at the start of its controller's upkeep,
/// with that player on `life`.
fn upkeep(definition: CardDefinitionId, life: i16) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut enchantment = creature(63_000, definition, PlayerId::One);
    enchantment.entered_controller_turn = 0;
    game.battlefield.push(enchantment);
    game.players[0].life = life;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    drain_pending(&mut game);
    game
}

#[test]
fn convalescence_gains_only_while_the_life_total_is_low() {
    assert_eq!(
        upkeep(cards::CONVALESCENCE, 10).players[0].life,
        11,
        "ten or less is inclusive, so the gain happens"
    );
    assert_eq!(
        upkeep(cards::CONVALESCENCE, 11).players[0].life,
        11,
        "and one more than the threshold is outside it"
    );
}

#[test]
fn the_test_of_endurance_wins_only_at_fifty() {
    let short = upkeep(cards::TEST_OF_ENDURANCE, 49);
    assert!(short.result().is_none(), "forty-nine is not fifty");

    let enough = upkeep(cards::TEST_OF_ENDURANCE, 50);
    assert!(
        matches!(
            enough.result(),
            Some(GameResult::Winner {
                winner: PlayerId::One,
                ..
            })
        ),
        "fifty or more is inclusive, and the game is over"
    );
}
