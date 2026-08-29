//! The other side of the blocking relationship.
//!
//! A Wall printing "creatures it's blocking" reads the relationship from
//! itself outwards; The Wretched reads it inwards, from the creatures that
//! blocked it. Both directions are needed because only the blocker records
//! what it blocked -- an attacker's own record does not name its blockers.

use super::*;

/// The Wretched attacking, blocked by `blockers` for player two.
fn wretched_blocked_by(count: usize) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut wretched = creature(10_000, cards::THE_WRETCHED, PlayerId::One);
    wretched.attacking = true;
    wretched.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let wretched_id = wretched.card.id;
    game.battlefield.push(wretched);

    let mut blockers = Vec::new();
    for index in 0..count {
        let mut blocker = creature(
            10_001 + u32::try_from(index).expect("a small index"),
            cards::SEDGE_TROLL,
            PlayerId::Two,
        );
        blocker.blocking = vec![wretched_id];
        blockers.push(blocker.card.id);
        game.battlefield.push(blocker);
    }
    game.active_player = PlayerId::One;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    (game, wretched_id, blockers)
}

fn controller(game: &Game, permanent: GameObjectId) -> Option<PlayerId> {
    game.battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .map(|candidate| candidate.controller)
}

/// Runs combat up to the end-of-combat step and lets the trigger resolve
/// there. Advancing past the step first would clear combat, and the trigger
/// reads the blocking relationship as it resolves.
fn finish_combat(game: &mut Game) {
    for _ in 0..8 {
        if game.step == Step::EndOfCombat {
            break;
        }
        game.advance_step();
    }
    assert_eq!(game.step, Step::EndOfCombat, "combat reached its end");
    // Driving steps directly skips the rules procedure that places captured
    // triggers on the stack, so run it before passing priority.
    game.finish_rules_procedure();
    // Resolve the trigger where it sits: combat is still on, which is the
    // whole point of an end-of-combat trigger.
    while !game.stack.is_empty() {
        game.resolve_stack_top();
        drain_pending(game);
    }
}

#[test]
fn it_takes_the_creatures_that_blocked_it() {
    let (mut game, _, blockers) = wretched_blocked_by(2);
    for blocker in &blockers {
        assert_eq!(controller(&game, *blocker), Some(PlayerId::Two));
    }

    finish_combat(&mut game);

    for blocker in &blockers {
        assert_eq!(
            controller(&game, *blocker),
            Some(PlayerId::One),
            "every creature that blocked it changed hands"
        );
    }
}

/// A creature that blocked something else is not blocking The Wretched, so
/// the relationship is what selects them rather than being in combat.
#[test]
fn a_creature_blocking_something_else_stays_put() {
    let (mut game, _, _) = wretched_blocked_by(1);
    // A 0/2, so the creature blocking it survives the exchange and can be
    // checked afterwards.
    let mut other_attacker = creature(10_010, cards::ICATIAN_MONEYCHANGER, PlayerId::One);
    other_attacker.attacking = true;
    other_attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let other_attacker_id = other_attacker.card.id;
    game.battlefield.push(other_attacker);
    let mut elsewhere = creature(10_011, cards::SEDGE_TROLL, PlayerId::Two);
    elsewhere.blocking = vec![other_attacker_id];
    let elsewhere_id = elsewhere.card.id;
    game.battlefield.push(elsewhere);

    finish_combat(&mut game);

    assert_eq!(
        controller(&game, elsewhere_id),
        Some(PlayerId::Two),
        "it blocked the other attacker, so The Wretched has no claim on it"
    );
}

/// Nothing blocked it, so nothing changes hands.
#[test]
fn an_unblocked_wretched_takes_nothing() {
    let (mut game, _, _) = wretched_blocked_by(0);
    let bystander = creature(10_010, cards::SEDGE_TROLL, PlayerId::Two);
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);

    finish_combat(&mut game);

    assert_eq!(controller(&game, bystander_id), Some(PlayerId::Two));
}
