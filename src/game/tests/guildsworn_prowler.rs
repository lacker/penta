//! Guildsworn Prowler: "if it wasn't blocking" is read off the creature as it
//! leaves, so the intervening-if has to see combat state the battlefield no
//! longer holds.

use super::*;

/// Player One with a Prowler out and a 2/2 across the table, in a main phase.
/// The blocking case moves itself into combat.
fn staged() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for id in 0..4 {
        game.players[0]
            .library
            .push(card(11_000 + id, cards::MOUNTAIN, PlayerId::One));
    }
    let prowler = creature(10_000, cards::GUILDSWORN_PROWLER, PlayerId::One);
    let prowler_id = prowler.card.id;
    game.battlefield.push(prowler);
    let attacker = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [3, 3];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, prowler_id, attacker_id)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if !game.pending_decisions.is_empty() {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

/// Killed outside combat, so the condition holds and the card is drawn.
#[test]
fn a_prowler_that_was_not_blocking_draws() {
    let (mut game, prowler, _) = staged();
    let before = game.players[0].hand.len();
    game.destroy_permanent(prowler);
    game.check_state_based_actions();
    settle(&mut game);

    assert_eq!(
        game.players[0].hand.len(),
        before + 1,
        "it was not blocking, so the trigger draws"
    );
}

/// Killed while blocking. The creature is gone by the time the intervening-if
/// is asked, so this is what proves the condition reads combat state from
/// last-known information rather than from the battlefield.
#[test]
fn a_prowler_that_was_blocking_draws_nothing() {
    let (mut game, prowler, attacker) = staged();
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.priority = PlayerId::Two;
    {
        let permanent = game
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == attacker)
            .expect("the attacker is there");
        permanent.attacking = true;
        permanent.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    }
    game.apply(
        PlayerId::One,
        Action::DeclareBlocker {
            blocker: prowler,
            attacker,
        },
    )
    .expect("it blocks");
    game.apply(PlayerId::One, Action::FinishDeclaringBlockers)
        .expect("the declaration finishes");
    let prowler_is_blocking = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == prowler)
        .expect("the Prowler is on the battlefield")
        .blocking
        .is_empty();
    let prowler_is_blocking = !prowler_is_blocking;
    assert!(prowler_is_blocking, "the fixture has it actually blocking");
    let before = game.players[0].hand.len();
    game.destroy_permanent(prowler);
    game.check_state_based_actions();
    settle(&mut game);

    assert_eq!(
        game.players[0].hand.len(),
        before,
        "it was blocking, so the trigger draws nothing"
    );
}
