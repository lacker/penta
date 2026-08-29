//! A mana ability whose amount is read off the permanent offering it.
//!
//! "Add {G} for each +1/+1 counter" is a fixed number at the moment the
//! ability is offered, which is what lets it live in the mana runtime at all:
//! the planner has to know what a tap is worth before spending it. Evolve is
//! what puts the counters there.

use super::*;

/// A Gyre Sage with `counters` +1/+1 counters already on it.
fn sage(counters: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    let mut sage = creature(10_000, cards::GYRE_SAGE, PlayerId::One);
    sage.set_counters(CounterKind::PlusOnePlusOne, counters);
    let sage_id = sage.card.id;
    game.battlefield.push(sage);
    game.priority = PlayerId::One;
    (game, sage_id)
}

/// Taps the Sage and reports how much green arrived.
fn tap_for_green(game: &mut Game, sage: GameObjectId) -> u16 {
    let before = game.players[PlayerId::One.index()].mana_pool.green;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateManaAbility { source, .. } if *source == sage))
        .expect("an untapped Sage offers its mana ability");
    game.apply(PlayerId::One, action).expect("tapping is free");
    drain_pending(game);
    game.players[PlayerId::One.index()].mana_pool.green - before
}

#[test]
fn the_sage_taps_for_one_green_per_counter() {
    let (mut game, sage) = sage(3);
    assert_eq!(tap_for_green(&mut game, sage), 3);
}

/// The control: no counters, no mana. The ability is still offered, because
/// tapping for nothing is a legal thing to do.
#[test]
fn an_unevolved_sage_taps_for_nothing() {
    let (mut game, sage) = sage(0);
    assert_eq!(tap_for_green(&mut game, sage), 0);
}

/// Evolve is what banks the counters, so a bigger creature arriving makes the
/// next tap worth more.
#[test]
fn evolve_grows_what_the_next_tap_is_worth() {
    let (mut game, sage) = sage(0);

    // Air Elemental is a 4/4, comfortably bigger than a 1/2. It has to
    // arrive rather than be placed: evolve watches the entry.
    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: creature(10_100, cards::AIR_ELEMENTAL, PlayerId::One),
        from: ZoneKind::Hand,
        completion: EntryCompletion::None,
        redirected_to: None,
    });
    drain_pending(&mut game);

    assert_eq!(tap_for_green(&mut game, sage), 1, "one counter, one green");
}
