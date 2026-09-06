//! "Attacks and isn't blocked" is decided by the finished block declaration
//! rather than by damage, which is the whole reason a 0/1 Mosquito is a
//! threat. So the counter arrives with no combat damage dealt at all, and
//! does not arrive when anything blocks.

use super::*;

/// Swamp Mosquito attacking player two, blocked by `blockers` creatures.
fn attack(blockers: usize) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut mosquito = creature(62_000, cards::SWAMP_MOSQUITO, PlayerId::One);
    mosquito.attacking = true;
    mosquito.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let mosquito_id = mosquito.card.id;
    game.battlefield.push(mosquito);
    for index in 0..blockers {
        let mut blocker = creature(
            62_100 + u32::try_from(index).expect("a small fixture"),
            cards::SERRA_ANGEL,
            PlayerId::Two,
        );
        blocker.entered_controller_turn = 0;
        blocker.blocking = vec![mosquito_id];
        game.battlefield.push(blocker);
    }
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    game
}

fn poison(game: &Game) -> u16 {
    game.players[PlayerId::Two.index()]
        .counters
        .count(CounterKind::Poison)
}

#[test]
fn going_unblocked_hands_over_a_counter_without_any_damage() {
    let game = attack(0);
    assert_eq!(poison(&game), 1, "one counter for the unblocked attack");
    assert_eq!(
        game.players[1].life, 20,
        "and no damage, since the Mosquito has no power"
    );
}

#[test]
fn a_blocked_attacker_hands_over_nothing() {
    let game = attack(1);
    assert_eq!(poison(&game), 0, "blocking it stops the trigger entirely");
}
