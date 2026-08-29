//! Two Zombies that bank the turn's deaths at each end step.
//!
//! The number is a count of deaths this turn, not of bodies in a graveyard:
//! a creature already in the yard when the turn began feeds neither. Khabál
//! Ghoul keeps what it banks as size; Scavenging Ghoul spends it one
//! regeneration at a time.

use super::*;

/// `ghoul` on the battlefield with `fodder` creatures alongside it under
/// player two.
fn ghoul_board(ghoul: CardDefinitionId, fodder: u32) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    let permanent = creature(10_000, ghoul, PlayerId::One);
    let ghoul_id = permanent.card.id;
    game.battlefield.push(permanent);

    let mut ids = Vec::new();
    for index in 0..fodder {
        let victim = creature(10_100 + index, cards::SEDGE_TROLL, PlayerId::Two);
        ids.push(victim.card.id);
        game.battlefield.push(victim);
    }
    game.priority = PlayerId::One;
    (game, ghoul_id, ids)
}

/// Kills `victims` outright, which is what makes them creatures that died
/// this turn.
fn destroy(game: &mut Game, victims: &[GameObjectId]) {
    for victim in victims {
        game.destroy_permanent(*victim);
    }
    drain_pending(game);
}

fn run_end_step(game: &mut Game) {
    game.step = Step::End;
    game.begin_step_triggers();
    drain_pending(game);
}

fn counters(game: &Game, id: GameObjectId, kind: CounterKind) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .counters(kind)
}

#[test]
fn khabal_ghoul_banks_one_counter_for_each_death() {
    let (mut game, ghoul, victims) = ghoul_board(cards::KHABAL_GHOUL, 3);
    destroy(&mut game, &victims[..2]);
    run_end_step(&mut game);

    assert_eq!(counters(&game, ghoul, CounterKind::PlusOnePlusOne), 2);
}

/// The control: a quiet turn banks nothing, and the count is of deaths rather
/// than of creatures sitting in a graveyard.
#[test]
fn a_turn_with_no_deaths_banks_nothing() {
    let (mut game, ghoul, victims) = ghoul_board(cards::KHABAL_GHOUL, 3);
    destroy(&mut game, &victims[..2]);
    run_end_step(&mut game);
    assert_eq!(counters(&game, ghoul, CounterKind::PlusOnePlusOne), 2);

    // A new turn: the graveyard still holds two bodies, but none of them
    // died this turn.
    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    run_end_step(&mut game);

    assert_eq!(
        counters(&game, ghoul, CounterKind::PlusOnePlusOne),
        2,
        "the bank did not grow on a turn nothing died",
    );
}

#[test]
fn the_counters_make_khabal_ghoul_bigger() {
    let (mut game, ghoul, victims) = ghoul_board(cards::KHABAL_GHOUL, 3);
    destroy(&mut game, &victims);
    run_end_step(&mut game);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == ghoul)
        .expect("still there");
    assert_eq!(
        (game.power(permanent), game.toughness(permanent)),
        (Some(4), Some(4)),
        "a 1/1 with three +1/+1 counters",
    );
}

#[test]
fn scavenging_ghoul_banks_corpse_counters_instead() {
    let (mut game, ghoul, victims) = ghoul_board(cards::SCAVENGING_GHOUL, 3);
    destroy(&mut game, &victims[..2]);
    run_end_step(&mut game);

    assert_eq!(counters(&game, ghoul, CounterKind::named("corpse")), 2);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == ghoul)
        .expect("still there");
    assert_eq!(
        (game.power(permanent), game.toughness(permanent)),
        (Some(2), Some(2)),
        "corpse counters are regenerations, not size",
    );
}

#[test]
fn a_corpse_counter_buys_one_regeneration() {
    let (mut game, ghoul, victims) = ghoul_board(cards::SCAVENGING_GHOUL, 3);
    destroy(&mut game, &victims[..2]);
    run_end_step(&mut game);
    game.step = Step::PrecombatMain;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == ghoul))
        .expect("two banked counters is two activations on offer");
    game.apply(PlayerId::One, action)
        .expect("removing a counter is the whole cost");
    drain_pending(&mut game);

    assert_eq!(
        counters(&game, ghoul, CounterKind::named("corpse")),
        1,
        "one counter spent, one left",
    );
}

/// An empty bank buys nothing: the cost is the counter, so there is no
/// activation to offer.
#[test]
fn an_empty_bank_offers_no_regeneration() {
    let (game, ghoul, _victims) = ghoul_board(cards::SCAVENGING_GHOUL, 3);
    assert!(!game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source, .. } if *source == ghoul)
    ),);
}
