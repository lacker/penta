//! An arrival trigger filtered by power, with an optional toll.
//!
//! Three separate filters have to hold at once: another creature, one you
//! control, and small. Each test breaks exactly one of them, so a predicate
//! that quietly dropped any of the three would fail here.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].library.clear();
    for index in 0..8 {
        game.players[PlayerId::One.index()].library.push(card(
            30_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    game
}

/// The Mentor out, then `arrival` entering under `controller`.
fn arrival(arrival: CardDefinitionId, controller: PlayerId) -> (Game, usize) {
    let mut game = ready();
    let mentor = creature(10_000, cards::MENTOR_OF_THE_MEEK, PlayerId::One);
    game.battlefield.push(mentor);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    let before = game.players[PlayerId::One.index()].hand.len();
    game.put_onto_battlefield(controller, arrival)
        .expect("cataloged");
    (game, before)
}

/// Answers a pending toll, paying it or declining.
fn answer(game: &mut Game, pay: bool) -> bool {
    let mut asked = false;
    for _ in 0..12 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            asked = true;
            let options = if pay {
                decision
                    .options
                    .last()
                    .map(|option| vec![option.id])
                    .unwrap_or_default()
            } else {
                // Declining is an option rather than an empty answer: the
                // offer names both branches.
                decision
                    .options
                    .first()
                    .map(|option| vec![option.id])
                    .unwrap_or_default()
            };
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
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    asked
}

fn drew(game: &Game, before: usize) -> bool {
    game.players[PlayerId::One.index()].hand.len() > before
}

/// A 2/2 is small enough, and paying the toll draws.
#[test]
fn a_small_creature_offers_the_toll() {
    let (mut game, before) = arrival(cards::GRIZZLY_BEARS, PlayerId::One);
    assert!(answer(&mut game, true), "the toll was offered");
    assert!(drew(&game, before), "and paying it drew a card");
}

/// Declining the toll draws nothing.
#[test]
fn declining_the_toll_draws_nothing() {
    let (mut game, before) = arrival(cards::GRIZZLY_BEARS, PlayerId::One);
    answer(&mut game, false);
    assert!(!drew(&game, before));
}

/// A 4/4 is too big, so nothing is even asked.
#[test]
fn a_big_creature_is_never_offered() {
    let (mut game, before) = arrival(cards::SERRA_ANGEL, PlayerId::One);
    assert!(!answer(&mut game, true), "power four is over the line");
    assert!(!drew(&game, before));
}

/// The opponent's small creature is not yours.
#[test]
fn an_opponents_creature_is_never_offered() {
    let (mut game, before) = arrival(cards::GRIZZLY_BEARS, PlayerId::Two);
    assert!(!answer(&mut game, true), "not a creature you control");
    assert!(!drew(&game, before));
}

/// "Another": a second Mentor triggers the first, but neither triggers
/// itself.
#[test]
fn the_mentor_does_not_trigger_on_its_own_arrival() {
    let mut game = ready();
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    let before = game.players[PlayerId::One.index()].hand.len();
    game.put_onto_battlefield(PlayerId::One, cards::MENTOR_OF_THE_MEEK)
        .expect("cataloged");

    assert!(!answer(&mut game, true), "it is not another creature");
    assert!(!drew(&game, before));
}
