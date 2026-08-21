//! Ocelot Pride: a Cat at the end of every turn you gained life, and once the
//! city's blessing lands, every token you made that turn doubled.

use super::*;

fn staged() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, cards::OCELOT_PRIDE)
        .expect("cataloged");
    drain_pending(&mut game);
    game.life_gained_this_turn = [0; 2];
    game
}

/// Runs the end step and resolves whatever it raises.
fn end_step(game: &mut Game) {
    game.active_player = PlayerId::One;
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(game);
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .take(decision.minimum.max(1))
                .map(|option| option.id)
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn cats(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Cat"], &[ManaColor::White], 1, 1),
            )
        })
        .count()
}

/// Fills Player One's battlefield out to `count` permanents, Ocelot included,
/// and settles the board so ascend has had its chance to look.
fn widen(game: &mut Game, count: usize) {
    while game.battlefield.len() < count {
        game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
            .expect("cataloged");
    }
    drain_pending(game);
    game.check_state_based_actions();
}

/// Gaining life is what turns the trigger on.
#[test]
fn gaining_life_makes_a_cat() {
    let mut game = staged();
    game.gain_life(PlayerId::One, 1);
    end_step(&mut game);

    assert_eq!(cats(&game), 1);
}

/// Without a life gain nothing happens at all.
#[test]
fn a_turn_with_no_life_gained_makes_nothing() {
    let mut game = staged();
    end_step(&mut game);

    assert_eq!(cats(&game), 0);
}

/// Under ten permanents the blessing stays out of reach, so the Cat arrives
/// alone. Eight before the trigger, nine after it.
#[test]
fn under_ten_permanents_does_not_ascend() {
    let mut game = staged();
    widen(&mut game, 8);
    game.gain_life(PlayerId::One, 1);
    end_step(&mut game);

    assert!(!game.citys_blessing[0], "nine is not ten");
    assert_eq!(cats(&game), 1, "one Cat and no copy of it");
}

/// Ten permanents ascend, and the Cat the trigger just made is one of the
/// tokens that entered this turn -- so it doubles itself.
#[test]
fn ten_permanents_ascend_and_double_the_cat() {
    let mut game = staged();
    widen(&mut game, 10);
    assert!(game.citys_blessing[0], "ten permanents is the blessing");

    game.gain_life(PlayerId::One, 1);
    end_step(&mut game);

    assert_eq!(cats(&game), 2, "the new Cat and a copy of it");
}

/// "For the rest of the game": dropping back under ten does not take it away.
#[test]
fn the_blessing_is_never_lost() {
    let mut game = staged();
    widen(&mut game, 10);
    assert!(game.citys_blessing[0]);

    game.battlefield
        .retain(|permanent| permanent.card.definition == cards::OCELOT_PRIDE);
    game.check_state_based_actions();

    assert!(game.citys_blessing[0], "once given, it stays");
}

/// The blessing belongs to the ascending player alone.
#[test]
fn an_opponents_board_does_not_ascend_you() {
    let mut game = staged();
    for _ in 0..12 {
        game.put_onto_battlefield(PlayerId::Two, cards::MOUNTAIN)
            .expect("cataloged");
    }
    drain_pending(&mut game);
    game.check_state_based_actions();

    assert!(!game.citys_blessing[0], "their permanents are not yours");
    assert!(!game.citys_blessing[1], "and they control no ascend");
}
