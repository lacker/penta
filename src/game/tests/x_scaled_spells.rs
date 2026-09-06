//! Spells whose reach is the X they were cast for. The chosen value has to
//! flow into a predicate rather than an amount, which is a different path
//! from "deal X damage": Meltdown compares X against each permanent's mana
//! value, and Flowstone Slide uses X in one direction and its negation in
//! the other.

use super::*;

fn cast_for_x(game: &mut Game, spell: CardInstanceId, x: u16) {
    let cast = cast_action(spell, Vec::new(), Vec::new(), x);
    assert!(
        game.legal_actions(PlayerId::One).contains(&cast),
        "the spell is castable for X = {x}"
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(game);
    game.check_state_based_actions();
}

#[test]
fn meltdown_reaches_exactly_as_far_as_x() {
    for (x, survivors) in [(0u16, 2usize), (1, 1), (3, 0)] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[0].hand.clear();
        // Mana values zero, one, and three.
        game.battlefield
            .push(creature(67_000, cards::ORNITHOPTER, PlayerId::Two));
        game.battlefield
            .push(creature(67_001, cards::SOL_RING, PlayerId::Two));
        game.battlefield
            .push(creature(67_002, cards::WORN_POWERSTONE, PlayerId::Two));
        let meltdown = card(67_010, cards::MELTDOWN, PlayerId::One);
        game.players[0].hand.push(meltdown.clone());
        game.players[0].mana_pool.red = 1;
        game.players[0].mana_pool.colorless = 6;

        cast_for_x(&mut game, meltdown.id, x);
        assert_eq!(
            game.battlefield.len(),
            survivors,
            "X = {x} should leave {survivors} artifact(s)"
        );
    }
}

/// The two halves of +X/-X move in opposite directions, so a Slide that
/// used X for both would leave a 2/2 alone instead of killing it.
#[test]
fn flowstone_slide_kills_by_the_toughness_it_takes() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.battlefield
        .push(creature(67_100, cards::GRIZZLY_BEARS, PlayerId::Two));
    game.battlefield
        .push(creature(67_101, cards::SERRA_ANGEL, PlayerId::Two));
    let slide = card(67_110, cards::FLOWSTONE_SLIDE, PlayerId::One);
    game.players[0].hand.push(slide.clone());
    game.players[0].mana_pool.red = 2;
    game.players[0].mana_pool.colorless = 4;

    cast_for_x(&mut game, slide.id, 2);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(67_101))
        .expect("a 4/4 survives losing two toughness");
    assert_eq!(
        (
            game.power(angel).expect("power"),
            game.toughness(angel).expect("toughness")
        ),
        (6, 2),
        "power went up by X and toughness down by it"
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(67_100)),
        "and the 2/2 lost its last point of toughness"
    );
}
