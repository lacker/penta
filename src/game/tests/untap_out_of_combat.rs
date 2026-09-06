//! "You may untap it and remove it from combat." An optional trigger whose
//! whole point is the escape, so both branches need covering -- and the
//! decision offers declining first, which is what a test answering with the
//! first option would silently measure.

use super::*;

/// Gustcloak Runner attacking and blocked by a Grizzly Bears, with the
//  becomes-blocked trigger already waiting to be answered.
fn blocked() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut runner = creature(49_000, cards::GUSTCLOAK_RUNNER, PlayerId::One);
    runner.attacking = true;
    runner.tapped = true;
    runner.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let runner_id = runner.card.id;
    game.battlefield.push(runner);
    let mut bear = creature(49_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    bear.entered_controller_turn = 0;
    bear.blocking = vec![runner_id];
    game.battlefield.push(bear);
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    // Put the trigger on the stack and resolve it, stopping as soon as it
    // asks its question rather than answering it here.
    for _ in 0..8 {
        if game.observe(PlayerId::One).decision.is_some() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    (game, runner_id)
}

fn runner(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the Runner is on the battlefield")
}

#[test]
fn taking_the_escape_untaps_it_and_ends_its_combat() {
    let (mut game, runner_id) = blocked();
    choose_decision_by_label(&mut game, PlayerId::One, "Do it");
    drain_pending(&mut game);

    let escaped = runner(&game, runner_id);
    assert!(!escaped.tapped, "it untapped");
    assert!(!escaped.attacking, "and left combat");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.blocking.is_empty()),
        "the blocker has nothing left to block"
    );
}

#[test]
fn declining_leaves_it_blocked_and_tapped() {
    let (mut game, runner_id) = blocked();
    choose_decision_by_label(&mut game, PlayerId::One, "Decline");
    drain_pending(&mut game);

    let staying = runner(&game, runner_id);
    assert!(staying.tapped);
    assert!(
        staying.attacking,
        "it is still an attacker, and still blocked"
    );
}
