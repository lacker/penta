//! Meekstone.
//!
//! The prohibition is aimed by a live power reading rather than by a list
//! frozen when the artifact entered, so a creature pumped past two stays
//! tapped and one shrunk below three untaps as usual. That is the whole
//! card, and it is also the half a snapshot implementation would get wrong.

use super::*;

/// Both players' creatures tapped, with a Meekstone under player two.
fn meekstone_board() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::MEEKSTONE, PlayerId::Two));

    // Serra Angel is a 4/4; Sedge Troll is a 2/2.
    let mut big = creature(10_001, cards::SERRA_ANGEL, PlayerId::One);
    big.tapped = true;
    let big_id = big.card.id;
    game.battlefield.push(big);
    let mut small = creature(10_002, cards::SEDGE_TROLL, PlayerId::One);
    small.tapped = true;
    let small_id = small.card.id;
    game.battlefield.push(small);
    (game, big_id, small_id)
}

fn is_tapped(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .tapped
}

fn take_turn(game: &mut Game, player: PlayerId) {
    game.commit_next_turn(player, Vec::new());
    drain_pending(game);
}

#[test]
fn only_the_big_creature_stays_tapped() {
    let (mut game, big, small) = meekstone_board();

    take_turn(&mut game, PlayerId::One);

    assert!(is_tapped(&game, big), "a 4/4 is held down");
    assert!(!is_tapped(&game, small), "and a 2/2 is not");
}

/// The reading is live: shrinking the big creature frees it.
#[test]
fn shrinking_past_three_frees_a_creature() {
    let (mut game, big, _) = meekstone_board();
    attach_constant_resolved_characteristics(
        &mut game,
        big,
        &[AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(-2),
            ValueDef::Constant(0),
        )],
        ContinuousEffectExpiration::Never,
    );

    take_turn(&mut game, PlayerId::One);

    assert!(!is_tapped(&game, big), "a 2/4 untaps as usual");
}

/// And the other way: pumping a small creature holds it down.
#[test]
fn pumping_past_two_holds_a_creature_down() {
    let (mut game, _, small) = meekstone_board();
    attach_constant_resolved_characteristics(
        &mut game,
        small,
        &[AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(1),
            ValueDef::Constant(0),
        )],
        ContinuousEffectExpiration::Never,
    );

    take_turn(&mut game, PlayerId::One);

    assert!(is_tapped(&game, small), "a 3/2 is held down");
}

/// It covers both players, which is what "creatures" without a controller
/// clause means.
#[test]
fn it_holds_its_own_controllers_creatures_too() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::MEEKSTONE, PlayerId::Two));
    let mut theirs = creature(10_001, cards::SERRA_ANGEL, PlayerId::Two);
    theirs.tapped = true;
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);

    take_turn(&mut game, PlayerId::Two);

    assert!(is_tapped(&game, theirs_id));
}
