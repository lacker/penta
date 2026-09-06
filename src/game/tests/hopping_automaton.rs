//! A free activated ability that pumps in one direction and shrinks in the
//! other. Because it costs nothing, the interesting question is what stops a
//! player from activating it forever: the shrinking half does.

use super::*;

/// Hopping Automaton alone on the battlefield, able to activate.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut automaton = creature(31_000, cards::HOPPING_AUTOMATON, PlayerId::One);
    automaton.entered_controller_turn = 0;
    let id = automaton.card.id;
    game.battlefield.push(automaton);
    (game, id)
}

fn hop(game: &mut Game, id: GameObjectId) {
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == id))
        .expect("the free ability is available");
    game.apply(PlayerId::One, activation)
        .expect("the ability costs nothing");
    pass_priority_pair(game);
}

fn body(game: &Game, id: GameObjectId) -> Option<(i16, i16, bool)> {
    let permanent = game.battlefield.iter().find(|p| p.card.id == id)?;
    Some((
        game.power(permanent)?,
        game.toughness(permanent)?,
        game.has_flying(permanent),
    ))
}

#[test]
fn hopping_grants_flying_and_shrinks_the_body() {
    let (mut game, id) = staged();
    assert_eq!(body(&game, id), Some((2, 2, false)));

    hop(&mut game, id);
    assert_eq!(
        body(&game, id),
        Some((1, 1, true)),
        "one hop trades a point of each for flying"
    );
}

#[test]
fn hopping_twice_shrinks_it_to_nothing() {
    let (mut game, id) = staged();
    hop(&mut game, id);
    hop(&mut game, id);

    assert_eq!(
        body(&game, id),
        None,
        "a second hop leaves a 0/0, which state-based actions bury"
    );
}
