//! A death trigger that shrinks every creature. The source is already in the
//! graveyard when it resolves, so "all creatures" means the ones still on
//! the battlefield -- and it reaches its own controller's board as readily
//! as the other, which is the half that makes the card a liability as well
//! as a threat.

use super::*;

/// The Buzzard under player one, with `mine` and `theirs` other creatures.
fn staged(mine: &[CardDefinitionId], theirs: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut buzzard = creature(72_000, cards::DEATH_S_HEAD_BUZZARD, PlayerId::One);
    buzzard.entered_controller_turn = 0;
    game.battlefield.push(buzzard);
    for (index, (definition, controller)) in mine
        .iter()
        .map(|d| (d, PlayerId::One))
        .chain(theirs.iter().map(|d| (d, PlayerId::Two)))
        .enumerate()
    {
        let mut other = creature(
            72_100 + u32::try_from(index).expect("a small fixture"),
            *definition,
            controller,
        );
        other.entered_controller_turn = 0;
        game.battlefield.push(other);
    }
    game
}

fn kill_the_buzzard(game: &mut Game) {
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == GameObjectId(72_000))
        .expect("the Buzzard is on the battlefield")
        .damage = 99;
    game.check_state_based_actions();
    drain_pending(game);
    game.check_state_based_actions();
}

#[test]
fn the_shrink_reaches_both_boards() {
    // Savannah Lions is a 2/1 on each side; Serra Angel is a 4/4.
    let mut game = staged(
        &[cards::SAVANNAH_LIONS],
        &[cards::SAVANNAH_LIONS, cards::SERRA_ANGEL],
    );
    kill_the_buzzard(&mut game);

    let alive: Vec<_> = game
        .battlefield
        .iter()
        .map(|permanent| permanent.card.id)
        .collect();
    assert_eq!(
        alive,
        vec![GameObjectId(72_102)],
        "both 2/1s died, the Buzzard's own controller's included, and the 4/4 lived"
    );
}

#[test]
fn a_creature_that_survives_keeps_the_penalty_only_for_the_turn() {
    let mut game = staged(&[], &[cards::SERRA_ANGEL]);
    kill_the_buzzard(&mut game);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(72_100))
        .expect("a 4/4 survives -1/-1");
    assert_eq!(
        (
            game.power(angel).expect("power"),
            game.toughness(angel).expect("toughness")
        ),
        (3, 3),
        "it is a 3/3 while the effect lasts"
    );
}
