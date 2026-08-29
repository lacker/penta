//! "Whenever this creature attacks and isn't blocked."
//!
//! Not knowable when attackers are declared: it is answered once blocking is
//! done, and a creature nobody blocked is a different thing from one that was
//! blocked by something that has since left. What these check are both
//! answers, and that declining the optional half leaves the creature alive.

use super::*;

/// Attacks with `attacker`, optionally blocked by a creature player two
/// controls, and runs blocking through to its triggers.
fn attack(game: &mut Game, attacker: GameObjectId, blocker: Option<GameObjectId>) {
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    for permanent in &mut game.battlefield {
        if permanent.card.id == attacker {
            permanent.attacking = true;
            permanent.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        }
        if Some(permanent.card.id) == blocker {
            permanent.blocking = vec![attacker];
        }
    }
    game.finish_declaring_blockers();
    // The trigger has to reach the stack and resolve before the "you may"
    // is asked, but stop there rather than answering it.
    for _ in 0..8 {
        if !game.pending_decisions.is_empty()
            || (game.stack.is_empty() && game.pending_triggers.is_empty())
        {
            return;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            return;
        }
    }
}

/// Answers the pending "you may" with `accept`.
fn answer(game: &mut Game, accept: bool) {
    let decision = game
        .pending_decisions
        .first()
        .expect("the optional half asks first")
        .clone();
    let option = decision
        .observation
        .options
        .iter()
        .find(|option| option.label == if accept { "Do it" } else { "Decline" })
        .expect("both answers are offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.observation.id,
            options: vec![option],
        },
    )
    .expect("the choice is submitted");
    drain_pending(game);
}

fn on_battlefield(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

#[test]
fn an_unblocked_thrull_can_trade_itself_for_three_cards() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let thrull = creature(10_000, cards::MINDSTAB_THRULL, PlayerId::One);
    let thrull_id = thrull.card.id;
    game.battlefield.push(thrull);
    for id in 0..4 {
        game.players[PlayerId::Two.index()].hand.push(card(
            20_000 + id,
            cards::SEDGE_TROLL,
            PlayerId::Two,
        ));
    }

    attack(&mut game, thrull_id, None);
    answer(&mut game, true);

    assert_eq!(
        game.players[PlayerId::Two.index()].hand.len(),
        1,
        "three of the four cards are gone"
    );
    assert!(
        !on_battlefield(&game, thrull_id),
        "and it sacrificed itself to do it"
    );
}

/// Declining costs nothing, which is the point of the "may".
#[test]
fn declining_keeps_the_creature_and_the_cards() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let thrull = creature(10_000, cards::MINDSTAB_THRULL, PlayerId::One);
    let thrull_id = thrull.card.id;
    game.battlefield.push(thrull);
    game.players[PlayerId::Two.index()]
        .hand
        .push(card(20_000, cards::SEDGE_TROLL, PlayerId::Two));

    attack(&mut game, thrull_id, None);
    answer(&mut game, false);

    assert_eq!(game.players[PlayerId::Two.index()].hand.len(), 1);
    assert!(on_battlefield(&game, thrull_id));
}

/// Being blocked is what stops it, so the same board with a blocker asks
/// nothing at all.
#[test]
fn a_blocked_thrull_never_triggers() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let thrull = creature(10_000, cards::MINDSTAB_THRULL, PlayerId::One);
    let thrull_id = thrull.card.id;
    game.battlefield.push(thrull);
    let blocker = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);
    game.players[PlayerId::Two.index()]
        .hand
        .push(card(20_000, cards::SEDGE_TROLL, PlayerId::Two));

    attack(&mut game, thrull_id, Some(blocker_id));
    drain_pending(&mut game);

    assert!(
        game.pending_decisions.is_empty(),
        "nothing was asked, because it was blocked"
    );
    assert_eq!(game.players[PlayerId::Two.index()].hand.len(), 1);
    assert!(on_battlefield(&game, thrull_id));
}
