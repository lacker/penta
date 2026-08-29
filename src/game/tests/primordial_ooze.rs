//! A creature that grows every upkeep and charges you for it.
//!
//! The counter goes on first and X is read afterwards, so the toll is the
//! size the Ooze has just grown to rather than the size it was. Declining is
//! two things at once -- it taps, and it bills -- which is what makes an
//! unpaid Ooze stop attacking as well as hurt.

use super::*;

/// Resolves everything waiting, taking the *last* option of each decision
/// rather than the first. For an optional payment that is the branch that
/// pays; `drain_pending` takes the first and so always declines.
fn drain_taking_the_last_option(game: &mut Game) {
    for _ in 0..16 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let take = decision.minimum.max(1).min(decision.maximum);
            let options = decision
                .options
                .iter()
                .rev()
                .map(|option| option.id)
                .take(take)
                .collect::<Vec<_>>();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

/// Player one's upkeep with an Ooze that already carries `counters`. The mana
/// goes into the pool once the turn has started, because the untap step would
/// have emptied anything placed before it.
fn upkeep_with_ooze(counters: u16, mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    let mut ooze = creature(10_000, cards::PRIMORDIAL_OOZE, PlayerId::One);
    ooze.set_counters(CounterKind::PlusOnePlusOne, counters);
    let ooze_id = ooze.card.id;
    game.battlefield.push(ooze);

    game.commit_next_turn(PlayerId::One, Vec::new());
    game.players[PlayerId::One.index()].mana_pool.colorless = mana;
    (game, ooze_id)
}

fn ooze_of(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
}

#[test]
fn it_grows_every_upkeep() {
    let (mut game, ooze) = upkeep_with_ooze(0, 0);
    drain_pending(&mut game);

    let ooze = ooze_of(&game, ooze);
    assert_eq!(ooze.counters(CounterKind::PlusOnePlusOne), 1);
    assert_eq!(game.power(ooze), Some(2), "a 1/1 with one counter");
}

/// Declining taps it and bills for the counters it now has, the new one
/// included.
#[test]
fn declining_taps_it_and_bills_for_every_counter() {
    let (mut game, ooze) = upkeep_with_ooze(2, 0);
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE) - 3,
        "two counters plus the one it just grew",
    );
    assert!(ooze_of(&game, ooze).tapped, "and it is tapped");
}

/// Paying leaves it untapped and costs no life, which is the whole reason to
/// keep feeding it.
#[test]
fn paying_leaves_it_alone() {
    let (mut game, ooze) = upkeep_with_ooze(2, 10);
    drain_taking_the_last_option(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE),
    );
    assert!(!ooze_of(&game, ooze).tapped);
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.colorless,
        7,
        "three of the ten paid the toll",
    );
}
