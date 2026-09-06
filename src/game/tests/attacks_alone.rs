//! "Whenever this creature attacks alone" is a bound on the whole
//! declaration, not on the attacker, so what needs covering is that adding a
//! second attacker silences it -- including when the second one is attacking
//! a different defender.

use super::*;

/// Player One in declare-attackers with Rogue Kavu and a Savannah Lions.
fn staged() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let kavu = creature(36_000, cards::ROGUE_KAVU, PlayerId::One);
    let kavu_id = kavu.card.id;
    let lions = creature(36_001, cards::SAVANNAH_LIONS, PlayerId::One);
    let lions_id = lions.card.id;
    game.battlefield.push(kavu);
    game.battlefield.push(lions);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;
    (game, kavu_id, lions_id)
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

fn attack(game: &mut Game, attackers: &[GameObjectId]) {
    for attacker in attackers {
        game.apply(
            PlayerId::One,
            Action::DeclareAttacker {
                attacker: *attacker,
                defender: AttackDefender::Player(PlayerId::Two),
            },
        )
        .expect("it attacks");
    }
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    settle(game);
}

fn power(game: &Game, id: GameObjectId) -> i16 {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the Kavu is on the battlefield");
    game.power(permanent).expect("power")
}

#[test]
fn attacking_by_itself_grows_it() {
    let (mut game, kavu, _) = staged();
    attack(&mut game, &[kavu]);
    assert_eq!(
        power(&game, kavu),
        3,
        "a 1/1 attacking alone becomes a 3/1, and the untapped creature that \
         stayed home is not part of the declaration"
    );
}

#[test]
fn a_second_attacker_silences_the_trigger() {
    let (mut game, kavu, lions) = staged();
    attack(&mut game, &[kavu, lions]);
    assert_eq!(
        power(&game, kavu),
        1,
        "the bound counts the declaration, so company costs it the bonus"
    );
}
