//! A bird that fattens on whatever died.
//!
//! It feeds at every end step, not only its controller's, and only when
//! something actually went to a graveyard. The counters it stores are spent
//! two at a time, which is what makes a quiet turn worth nothing to it.

use super::*;

/// Vultures under player one, with `carrion` already on them.
fn vultures_out(carrion: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    let mut vultures = creature(10_000, cards::OSAI_VULTURES, PlayerId::One);
    vultures.set_counters(CounterKind::named("carrion"), carrion);
    let vultures_id = vultures.card.id;
    game.battlefield.push(vultures);
    (game, vultures_id)
}

/// Steps into the end step, where the feeding trigger lives.
fn reach_the_end_step(game: &mut Game) {
    game.step = Step::PostcombatMain;
    game.advance_step();
    game.finish_rules_procedure();
    drain_pending(game);
}

fn carrion_on(game: &Game, vultures: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == vultures)
        .expect("still there")
        .counters(CounterKind::named("carrion"))
}

#[test]
fn something_dying_feeds_it() {
    let (mut game, vultures) = vultures_out(0);
    let food = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let food_id = food.card.id;
    game.battlefield.push(food);
    game.destroy_permanent(food_id);
    drain_pending(&mut game);

    reach_the_end_step(&mut game);

    assert_eq!(carrion_on(&game, vultures), 1);
}

/// The control, and the intervening-if: a turn where nothing died leaves it
/// hungry.
#[test]
fn a_quiet_turn_feeds_it_nothing() {
    let (mut game, vultures) = vultures_out(0);

    reach_the_end_step(&mut game);

    assert_eq!(carrion_on(&game, vultures), 0);
}

#[test]
fn two_counters_buy_a_point_of_each() {
    let (mut game, vultures) = vultures_out(2);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == vultures)
        })
        .expect("two counters is enough to spend");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    assert_eq!(carrion_on(&game, vultures), 0, "both counters went");
    let bird = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == vultures)
        .expect("still there");
    assert_eq!(game.power(bird), Some(2), "a 1/1 grown to 2/2");
    assert_eq!(game.toughness(bird), Some(2));
}

/// One counter is not two, so the ability is not offered at all.
#[test]
fn one_counter_cannot_be_spent() {
    let (game, vultures) = vultures_out(1);

    assert!(!game.legal_actions(PlayerId::One).iter().any(|action| {
        matches!(action, Action::ActivateAbility { source, .. } if *source == vultures)
    }),);
}
