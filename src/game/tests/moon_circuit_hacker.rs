//! Moon-Circuit Hacker: the loot is a discard the turn it arrives is exempt
//! from, so the same trigger costs a card or does not depending on when the
//! Ninja entered.

use super::*;

/// Player One attacking with Moon-Circuit Hacker, `entered` recorded as the
/// turn it arrived on, and two cards in hand to discard from.
fn staged(entered: u32) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.turn = 5;
    let mut hacker = creature(10_000, cards::MOON_CIRCUIT_HACKER, PlayerId::One);
    hacker.attacking = true;
    hacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    hacker.entered_turn = entered;
    hacker.entered_controller_turn = entered;
    let id = hacker.card.id;
    game.battlefield.push(hacker);
    for index in 0..2 {
        game.players[0]
            .hand
            .push(card(10_100 + index, cards::SAVANNAH_LIONS, PlayerId::One));
    }
    game.active_player = PlayerId::One;
    game.step = Step::DeclareBlockers;
    game.priority = PlayerId::One;
    (game, id)
}

/// Deals the combat damage and resolves the trigger, accepting every optional
/// branch. Declining is the first option offered, so the affirmative one has
/// to be picked deliberately or the trigger measures nothing.
fn connect(game: &mut Game) -> usize {
    game.advance_step();
    assert_eq!(game.step, Step::CombatDamage);
    let mut answered = 0;
    for _ in 0..24 {
        if let Some(decision) = game.pending_decisions.first().cloned() {
            let options = &decision.observation.options;
            let option = options
                .iter()
                .find(|option| option.label == "Do it")
                .unwrap_or_else(|| {
                    options
                        .last()
                        .expect("an offered decision has at least one option")
                })
                .id;
            game.apply(
                decision.observation.player,
                Action::ChooseDecision {
                    decision: decision.observation.id,
                    options: vec![option],
                },
            )
            .expect("the offered option is legal");
            answered += 1;
            continue;
        }
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.step != Step::CombatDamage
        {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    answered
}

#[test]
fn a_ninja_that_arrived_this_turn_draws_without_discarding() {
    let (mut game, _) = staged(5);
    connect(&mut game);
    assert_eq!(
        game.players[0].hand.len(),
        3,
        "the turn it entered is exempt from the discard, so the draw is clean"
    );
}

#[test]
fn a_ninja_that_was_already_out_discards_after_drawing() {
    let (mut game, _) = staged(4);
    connect(&mut game);
    assert_eq!(
        game.players[0].hand.len(),
        2,
        "a Ninja that survived a turn loots instead of drawing"
    );
}
