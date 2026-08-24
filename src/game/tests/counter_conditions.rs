//! Clauses that read a counter count.
//!
//! Homarid is the reason this exists: it has two static clauses that apply at
//! different exact counts and a state trigger that empties the pile. What
//! these check is that the count is read live, that "exactly" means exactly,
//! and that the trigger clears the pile rather than the clause simply
//! ceasing to apply.

use super::*;
use crate::ImplementationStatus;

fn homarid_at(tide: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    let homarid = creature(10_000, cards::HOMARID, PlayerId::One);
    let homarid_id = homarid.card.id;
    game.battlefield.push(homarid);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == homarid_id)
        .expect("just pushed")
        .add_counters(CounterKind::named("tide"), tide);
    (game, homarid_id)
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still on the battlefield");
    (game.power(permanent), game.toughness(permanent))
}

/// Homarid is a printed 2/2 that reads 1/1 at one tide counter, 2/2 at two,
/// and 3/3 at three. Two is the case that proves "exactly" is doing work.
#[test]
fn the_count_selects_which_clause_applies() {
    for (tide, expected) in [(1, (1, 1)), (2, (2, 2)), (3, (3, 3))] {
        let (game, homarid) = homarid_at(tide);
        assert_eq!(
            stats(&game, homarid),
            (Some(expected.0), Some(expected.1)),
            "with {tide} tide counters"
        );
    }
}

/// Four or more empties the pile, which is a trigger rather than the clauses
/// simply not applying.
#[test]
fn four_counters_clears_them() {
    let (mut game, homarid) = homarid_at(4);
    // A state trigger is captured while state-based actions are checked, so
    // the board has to be looked at before anything can fire.
    game.check_state_based_actions();
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == homarid)
            .expect("still there")
            .counters(CounterKind::named("tide")),
        0,
        "the state trigger removed them all"
    );
    assert_eq!(
        stats(&game, homarid),
        (Some(2), Some(2)),
        "and with none left it is a printed 2/2 again"
    );
}

/// Icatian Moneychanger's payout scales with what it has accumulated, which
/// is the value side of the same counter reading.
#[test]
fn the_moneychanger_pays_out_per_counter() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.step = Step::Upkeep;
    let changer = creature(10_000, cards::ICATIAN_MONEYCHANGER, PlayerId::One);
    let changer_id = changer.card.id;
    game.battlefield.push(changer);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == changer_id)
        .expect("just pushed")
        .add_counters(CounterKind::named("credit"), 4);
    let before = game.players[PlayerId::One.index()].life;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == changer_id)
        })
        .expect("it can be cashed in during upkeep");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        before + 4,
        "one life for each of the four counters"
    );
}

#[test]
fn every_counter_condition_identity_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::MERCHANT_SHIP,
        cards::ICATIAN_MONEYCHANGER,
        cards::HOMARID,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
