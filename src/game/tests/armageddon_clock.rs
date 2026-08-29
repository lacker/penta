//! A clock everybody watches and anybody can wind back.
//!
//! It ticks up in your upkeep and goes off in your draw step, so the window
//! to take a counter off sits between the two -- and it is open to whoever
//! wants it, which is what makes the Clock a shared problem rather than a
//! threat one player builds alone.

use super::*;

/// A Clock under player one carrying `counters`, with `mana` colorless in
/// each player's pool.
fn clock_out(counters: u16, mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    let mut clock = creature(10_000, cards::ARMAGEDDON_CLOCK, PlayerId::One);
    clock.set_counters(CounterKind::named("doom"), counters);
    let clock_id = clock.card.id;
    game.battlefield.push(clock);
    for player in [PlayerId::One, PlayerId::Two] {
        game.players[player.index()].mana_pool.colorless = mana;
    }
    (game, clock_id)
}

fn doom_on(game: &Game, clock: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == clock)
        .expect("still there")
        .counters(CounterKind::named("doom"))
}

fn winding(game: &Game, player: PlayerId, clock: GameObjectId) -> Option<Action> {
    game.legal_actions(player)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == clock))
}

#[test]
fn it_ticks_up_in_upkeep_and_goes_off_in_the_draw_step() {
    let (mut game, clock) = clock_out(2, 0);
    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_pending(&mut game);
    assert_eq!(doom_on(&game, clock), 3, "one more than it had");

    // The turn stops at upkeep, so the draw step is stepped into by hand.
    game.advance_step();
    game.finish_rules_procedure();
    drain_pending(&mut game);

    for player in [PlayerId::One, PlayerId::Two] {
        assert_eq!(
            game.players[player.index()].life,
            i16::from(rules::STARTING_LIFE) - 3,
            "and it hit each player for all three",
        );
    }
}

/// The window is an upkeep, either player's, and the Clock's own controller
/// is not the only one who may use it.
#[test]
fn either_player_may_wind_it_back_during_an_upkeep() {
    let (mut game, clock) = clock_out(3, 4);
    game.step = Step::Upkeep;

    assert!(winding(&game, PlayerId::One, clock).is_some());
    game.priority = PlayerId::Two;
    let action = winding(&game, PlayerId::Two, clock).expect("the opponent may wind it too");
    game.apply(PlayerId::Two, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    assert_eq!(doom_on(&game, clock), 2, "one counter came off");
    assert_eq!(
        game.players[PlayerId::Two.index()].mana_pool.colorless,
        0,
        "and the winder paid for it",
    );
}

/// Outside an upkeep nobody may wind it, which is what stops it being
/// answered at the last moment.
#[test]
fn it_cannot_be_wound_back_outside_an_upkeep() {
    let (mut game, clock) = clock_out(3, 4);
    game.step = Step::PrecombatMain;

    assert!(winding(&game, PlayerId::One, clock).is_none());
    game.priority = PlayerId::Two;
    assert!(winding(&game, PlayerId::Two, clock).is_none());
}

/// A Clock with no counters deals nothing, which is the state it is wound
/// back to.
#[test]
fn an_unwound_clock_deals_nothing() {
    let (mut game, clock) = clock_out(0, 0);
    game.step = Step::Upkeep;
    game.advance_step();
    game.finish_rules_procedure();
    drain_pending(&mut game);

    assert_eq!(doom_on(&game, clock), 0, "no upkeep ran, so no tick");
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE),
    );
}
