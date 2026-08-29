//! One count read twice, rounded opposite ways.
//!
//! An odd number of Forests is why the card spells out both halves: five
//! Forests is +2/+3, not +2/+2 or +3/+3. The count is live, so the bonus
//! follows the board rather than being fixed when the Aura landed.

use super::*;

/// Aspect of Wolf on a Sedge Troll, with `forests` Forests under player one
/// and `theirs` under player two.
fn enchanted(forests: u32, theirs: u32) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;

    let host = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.push(host);

    let mut aura = creature(10_001, cards::ASPECT_OF_WOLF, PlayerId::One);
    aura.attached_to = Some(host_id);
    game.battlefield.push(aura);

    for index in 0..forests {
        game.battlefield
            .push(creature(10_100 + index, cards::FOREST, PlayerId::One));
    }
    for index in 0..theirs {
        game.battlefield
            .push(creature(10_200 + index, cards::FOREST, PlayerId::Two));
    }
    (game, host_id)
}

/// The Troll's printed size is 2/2, so the bonus is what these read.
fn bonus(game: &Game, host: GameObjectId) -> (i16, i16) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == host)
        .expect("still there");
    (
        game.power(permanent).expect("a creature") - 2,
        game.toughness(permanent).expect("a creature") - 2,
    )
}

/// An even count halves evenly, so both directions land on the same number.
#[test]
fn an_even_count_gives_the_same_bonus_both_ways() {
    let (game, host) = enchanted(4, 0);
    assert_eq!(bonus(&game, host), (2, 2));
}

/// The reason both halves are printed: an odd count splits.
#[test]
fn an_odd_count_rounds_power_down_and_toughness_up() {
    let (game, host) = enchanted(5, 0);
    assert_eq!(bonus(&game, host), (2, 3));
}

/// One Forest is the sharpest case: nothing to power, one to toughness.
#[test]
fn a_single_forest_gives_nothing_and_one() {
    let (game, host) = enchanted(1, 0);
    assert_eq!(bonus(&game, host), (0, 1));
}

/// The control: no Forests, no bonus, and the halving of zero is zero both
/// ways rather than rounding up to one.
#[test]
fn no_forests_gives_nothing() {
    let (game, host) = enchanted(0, 0);
    assert_eq!(bonus(&game, host), (0, 0));
}

/// "Forests you control", so the other side's do not count.
#[test]
fn forests_across_the_table_do_not_count() {
    let (game, host) = enchanted(2, 6);
    assert_eq!(bonus(&game, host), (1, 1));
}

/// The count is read live, so losing a Forest shrinks the creature.
#[test]
fn the_bonus_follows_the_board() {
    let (mut game, host) = enchanted(5, 0);
    assert_eq!(bonus(&game, host), (2, 3));

    let a_forest = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::FOREST)
        .expect("a Forest is out")
        .card
        .id;
    game.battlefield
        .retain(|permanent| permanent.card.id != a_forest);

    assert_eq!(bonus(&game, host), (2, 2), "four Forests halve evenly");
}
