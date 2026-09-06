//! "Whenever this creature attacks, it doesn't untap during its controller's
//! next untap step." The rule has to be live *during* that untap step and
//! gone afterwards, which is what until-your-next-upkeep means here: the
//! untap step precedes upkeep.

use super::*;

/// Apes of Rath having attacked, with the turn handed back and forth so the
/// controller's next untap step has happened.
fn attacked_then_untap_step() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut apes = creature(30_000, cards::APES_OF_RATH, PlayerId::One);
    apes.entered_controller_turn = 0;
    let apes_id = apes.card.id;
    game.battlefield.push(apes);
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: apes_id,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("it attacks");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    for _ in 0..8 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    (game, apes_id)
}

fn tapped(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the Apes are on the battlefield")
        .tapped
}

#[test]
fn attacking_taps_it_and_the_next_untap_step_leaves_it_tapped() {
    let (mut game, apes) = attacked_then_untap_step();
    assert!(tapped(&game, apes), "attacking tapped it");

    // The opponent's whole turn, then the controller's untap step.
    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    assert!(tapped(&game, apes), "it is not the controller's untap step");
    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_pending(&mut game);

    assert!(
        tapped(&game, apes),
        "the rule covers the controller's next untap step, so it stays tapped"
    );
}

#[test]
fn the_untap_step_after_that_one_does_untap_it() {
    let (mut game, apes) = attacked_then_untap_step();
    for player in [PlayerId::Two, PlayerId::One] {
        game.commit_next_turn(player, Vec::new());
        drain_pending(&mut game);
    }
    assert!(tapped(&game, apes), "the skipped untap step");

    for player in [PlayerId::Two, PlayerId::One] {
        game.commit_next_turn(player, Vec::new());
        drain_pending(&mut game);
    }

    assert!(
        !tapped(&game, apes),
        "the rule lasted one untap step, not forever"
    );
}
