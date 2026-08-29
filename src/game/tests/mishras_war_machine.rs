//! An upkeep "unless" whose cost is a card out of hand.
//!
//! Unlike a mill, a discard can be impossible: an empty hand has nothing to
//! choose, so the payment is not on offer at all and the damage is the only
//! branch left. Which card goes is settled after the branch is, because the
//! branch does not depend on it.

use super::*;

/// The War Machine under player one, with `hand` cards held.
fn machined(hand: u32) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    let machine = creature(10_000, cards::MISHRA_S_WAR_MACHINE, PlayerId::One);
    let machine_id = machine.card.id;
    game.battlefield.push(machine);
    for index in 0..hand {
        game.players[PlayerId::One.index()].hand.push(card(
            30_000 + index,
            cards::SEDGE_TROLL,
            PlayerId::One,
        ));
    }
    game.priority = PlayerId::One;
    (game, machine_id)
}

/// Answers each waiting decision by taking the option at `index`, clamped to
/// what is on offer.
fn drain_choosing(game: &mut Game, index: usize) {
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
            let pick = index.min(decision.options.len().saturating_sub(1));
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![decision.options[pick].id],
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

fn run_upkeep(game: &mut Game, choice: usize) {
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_choosing(game, choice);
}

fn tapped(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .tapped
}

fn life(game: &Game) -> i16 {
    game.players[PlayerId::One.index()].life
}

#[test]
fn discarding_avoids_the_damage_and_leaves_it_untapped() {
    let (mut game, machine) = machined(3);
    run_upkeep(&mut game, usize::MAX);

    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 2);
    assert_eq!(life(&game), i16::from(rules::STARTING_LIFE));
    assert!(!tapped(&game, machine), "paid, so it stays up");
}

/// The control: declining takes three and puts the Machine down, which is
/// the same clause rather than a second trigger watching for the damage.
#[test]
fn declining_takes_three_and_taps_it() {
    let (mut game, machine) = machined(3);
    run_upkeep(&mut game, 0);

    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 3);
    assert_eq!(life(&game), i16::from(rules::STARTING_LIFE) - 3);
    assert!(tapped(&game, machine));
}

/// An empty hand cannot pay at all, so the damage is not a choice. This is
/// where a discard differs from a mill, which pays with a short library.
#[test]
fn an_empty_hand_cannot_pay_and_takes_the_damage() {
    let (mut game, machine) = machined(0);
    run_upkeep(&mut game, usize::MAX);

    assert_eq!(life(&game), i16::from(rules::STARTING_LIFE) - 3);
    assert!(tapped(&game, machine));
}

#[test]
fn it_has_banding() {
    let (game, machine) = machined(1);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == machine)
        .expect("still there");
    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Banding));
}
