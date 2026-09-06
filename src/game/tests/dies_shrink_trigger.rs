//! A death trigger that targets. The source is in the graveyard before the
//! trigger resolves, so what needs covering is that the -1/-1 still lands and
//! is lethal in its own right: a 1/1 it names dies to toughness, not damage.

use super::*;

/// Festering Goblin under player one, with `others` 1/1s opposite it.
fn staged(others: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut goblin = creature(40_000, cards::FESTERING_GOBLIN, PlayerId::One);
    goblin.entered_controller_turn = 0;
    let goblin_id = goblin.card.id;
    game.battlefield.push(goblin);
    for index in 0..others {
        let mut lion = creature(
            40_100 + u32::try_from(index).expect("a small fixture"),
            cards::SAVANNAH_LIONS,
            PlayerId::Two,
        );
        lion.entered_controller_turn = 0;
        game.battlefield.push(lion);
    }
    (game, goblin_id)
}

fn kill_the_goblin(game: &mut Game, goblin: GameObjectId) {
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == goblin)
        .expect("the Goblin is on the battlefield")
        .damage = 99;
    game.check_state_based_actions();
    drain_pending(game);
    game.check_state_based_actions();
}

fn survivors(game: &Game) -> usize {
    game.battlefield.len()
}

#[test]
fn the_shrink_kills_the_only_legal_target() {
    // Savannah Lions is a 2/1, so one point of toughness is all it has.
    let (mut game, goblin) = staged(1);
    kill_the_goblin(&mut game, goblin);

    assert_eq!(
        survivors(&game),
        0,
        "the Goblin died and took the 2/1 with it from the graveyard"
    );
    assert_eq!(
        game.players[1].graveyard.len(),
        1,
        "the Lions died to toughness rather than to damage"
    );
}

#[test]
fn with_nothing_to_target_the_goblin_just_dies() {
    let (mut game, goblin) = staged(0);
    kill_the_goblin(&mut game, goblin);

    assert_eq!(survivors(&game), 0);
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "a trigger with no legal target is simply removed"
    );
}
