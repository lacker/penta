//! Creatures and Auras whose size is a count of the board. The two forms are
//! different in a way the printed text makes explicit: a "*/*" creature has
//! its power and toughness *defined* by the count, so an empty board makes it
//! a 0/0 that dies; a printed body is *modified* by it and survives at its
//! printed size. Whose permanents count is the other half, and it is easy to
//! write "yours" for a clause that says "on the battlefield".

use super::*;

/// One subject under player one, with `mine` and `theirs` extra permanents.
fn board(
    subject: CardDefinitionId,
    mine: &[CardDefinitionId],
    theirs: &[CardDefinitionId],
) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut it = creature(56_000, subject, PlayerId::One);
    it.entered_controller_turn = 0;
    let id = it.card.id;
    game.battlefield.push(it);
    for (index, (definition, controller)) in mine
        .iter()
        .map(|d| (d, PlayerId::One))
        .chain(theirs.iter().map(|d| (d, PlayerId::Two)))
        .enumerate()
    {
        let mut extra = creature(
            56_100 + u32::try_from(index).expect("a small fixture"),
            *definition,
            controller,
        );
        extra.entered_controller_turn = 0;
        game.battlefield.push(extra);
    }
    (game, id)
}

fn stats(game: &Game, id: GameObjectId) -> Option<(i16, i16)> {
    let permanent = game.battlefield.iter().find(|p| p.card.id == id)?;
    Some((game.power(permanent)?, game.toughness(permanent)?))
}

#[test]
fn a_defined_size_is_the_count_and_nothing_else() {
    let (game, elder) = board(cards::DUNGROVE_ELDER, &[cards::FOREST, cards::FOREST], &[]);
    assert_eq!(stats(&game, elder), Some((2, 2)), "two Forests, a 2/2");

    let (game, elder) = board(
        cards::DUNGROVE_ELDER,
        &[cards::FOREST, cards::FOREST, cards::FOREST],
        &[],
    );
    assert_eq!(stats(&game, elder), Some((3, 3)));
}

/// With nothing to count a defined body is a 0/0, which state-based actions
/// bury. A modified one is not.
#[test]
fn an_empty_count_kills_only_the_defined_body() {
    let (mut game, elder) = board(cards::DUNGROVE_ELDER, &[], &[]);
    game.check_state_based_actions();
    assert_eq!(stats(&game, elder), None, "a 0/0 Elder dies");

    let (game, enchantress) = board(cards::YAVIMAYA_ENCHANTRESS, &[], &[]);
    assert_eq!(
        stats(&game, enchantress),
        Some((2, 2)),
        "the Enchantress keeps her printed size with nothing to count"
    );
}

/// "Each Goblin on the battlefield" is not "each Goblin you control": the
/// opponent's count too.
#[test]
fn a_board_wide_count_reaches_both_sides() {
    let (game, one) = board(cards::RECKLESS_ONE, &[cards::RAGING_GOBLIN], &[]);
    assert_eq!(
        stats(&game, one),
        Some((2, 2)),
        "itself and the friendly Goblin"
    );

    let (game, one) = board(
        cards::RECKLESS_ONE,
        &[cards::RAGING_GOBLIN],
        &[cards::RAGING_GOBLIN],
    );
    assert_eq!(
        stats(&game, one),
        Some((3, 3)),
        "the opponent's Goblin counts as well"
    );
}

/// Marauding Knight counts what the *opponent* controls, so its own
/// controller's Plains are worth nothing to it.
#[test]
fn an_opponent_scoped_count_ignores_your_own() {
    let (game, knight) = board(
        cards::MARAUDING_KNIGHT,
        &[cards::PLAINS, cards::PLAINS],
        &[],
    );
    assert_eq!(
        stats(&game, knight),
        Some((2, 2)),
        "your own Plains do nothing"
    );

    let (game, knight) = board(
        cards::MARAUDING_KNIGHT,
        &[],
        &[cards::PLAINS, cards::PLAINS],
    );
    assert_eq!(stats(&game, knight), Some((4, 4)), "theirs are what count");
}
