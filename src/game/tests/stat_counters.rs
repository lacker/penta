//! Counters that change power and toughness.
//!
//! Only +1/+1 used to mean anything; the amounts now live on the counter kind
//! rather than in the stat calculation. What these check are a kind that
//! subtracts, a kind whose two halves differ, and the rule that keeps a
//! permanent from carrying opposing counters at once.

use super::*;
use crate::ImplementationStatus;

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still on the battlefield");
    (game.power(permanent), game.toughness(permanent))
}

fn put(game: &mut Game, id: GameObjectId, kind: CounterKind, amount: u16) {
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == id)
        .expect("on the battlefield")
        .add_counters(kind, amount);
}

#[test]
fn a_minus_one_counter_subtracts_from_both_halves() {
    let mut game = ready_game();
    // Sedge Troll is a 2/2.
    let troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);

    put(&mut game, troll_id, CounterKind::MinusOneMinusOne, 1);
    assert_eq!(stats(&game, troll_id), (Some(1), Some(1)));
}

/// A +1/+2 counter is the reason the amounts belong on the kind: its halves
/// are not the same number.
#[test]
fn a_plus_one_plus_two_counter_adds_different_amounts() {
    let mut game = ready_game();
    let troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);

    put(&mut game, troll_id, CounterKind::PlusOnePlusTwo, 2);
    assert_eq!(stats(&game, troll_id), (Some(4), Some(6)));
}

/// CR 122.3. The pair cancels, so the permanent is left with neither rather
/// than with both quietly summing to zero.
#[test]
fn opposing_counters_annihilate_in_pairs() {
    let mut game = ready_game();
    let troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);

    put(&mut game, troll_id, CounterKind::PlusOnePlusOne, 3);
    put(&mut game, troll_id, CounterKind::MinusOneMinusOne, 1);
    game.check_state_based_actions();

    let troll = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == troll_id)
        .expect("still there");
    assert_eq!(troll.counters(CounterKind::PlusOnePlusOne), 2);
    assert_eq!(
        troll.counters(CounterKind::MinusOneMinusOne),
        0,
        "one of each went away together"
    );
    assert_eq!(stats(&game, troll_id), (Some(4), Some(4)));
}

/// Unstable Mutation is worth having as a card test because its two clauses
/// pull opposite ways, and the counter is what eventually wins.
#[test]
fn unstable_mutation_shrinks_its_host_each_upkeep() {
    let mut game = ready_game();
    let host = creature(10_000, cards::SEDGE_TROLL, PlayerId::Two);
    let host_id = host.card.id;
    game.battlefield.push(host);
    let mut aura = creature(10_001, cards::UNSTABLE_MUTATION, PlayerId::One);
    aura.attached_to = Some(host_id);
    game.battlefield.push(aura);
    game.check_state_based_actions();

    assert_eq!(
        stats(&game, host_id),
        (Some(5), Some(5)),
        "a 2/2 with +3/+3"
    );

    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    assert_eq!(
        stats(&game, host_id),
        (Some(4), Some(4)),
        "one upkeep, one counter"
    );
}

#[test]
fn every_stat_counter_identity_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [cards::UNSTABLE_MUTATION, cards::ARMOR_THRULL] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
