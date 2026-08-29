//! Taking a land off the top of your own library.
//!
//! The look already put what it took into a zone; the battlefield was the
//! one destination it refused. Only what was *taken* may go there -- a card
//! nobody chose has no reason to be put anywhere but back.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::Upkeep;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].library.clear();
    game
}

/// Into the Wilds out, with `top` as the next card of the library.
fn upkeep_with(top: CardDefinitionId) -> Game {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::INTO_THE_WILDS, PlayerId::One));
    game.players[PlayerId::One.index()].library.push(card(
        20_001,
        cards::GRIZZLY_BEARS,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()]
        .library
        .push(card(20_000, top, PlayerId::One));
    game.handle_upkeep_triggers();
    game
}

/// Answers the pending offer, taking everything it allows or nothing.
fn answer(game: &mut Game, take: bool) {
    for _ in 0..12 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = if take {
                decision
                    .options
                    .iter()
                    .map(|option| option.id)
                    .take(decision.maximum)
                    .collect()
            } else {
                Vec::new()
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
}

fn library(game: &Game) -> Vec<CardDefinitionId> {
    game.players[PlayerId::One.index()]
        .library
        .iter()
        .rev()
        .map(|card| card.definition)
        .collect()
}

/// A land on top can be taken, and it arrives without spending the land drop.
#[test]
fn a_land_on_top_can_be_put_onto_the_battlefield() {
    let mut game = upkeep_with(cards::FOREST);
    answer(&mut game, true);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::FOREST),
        "the Forest arrived",
    );
    assert_eq!(
        library(&game),
        vec![cards::GRIZZLY_BEARS],
        "and left the top"
    );
    assert!(
        game.players[PlayerId::One.index()].lands_played_this_turn == 0,
        "putting it there is not playing it",
    );
}

/// Declining leaves it on top, so it is still the next draw.
#[test]
fn declining_leaves_the_land_where_it_was() {
    let mut game = upkeep_with(cards::FOREST);
    answer(&mut game, false);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::FOREST),
        "nothing was taken",
    );
    assert_eq!(
        library(&game),
        vec![cards::FOREST, cards::GRIZZLY_BEARS],
        "and the order is undisturbed",
    );
}

/// A nonland card is not on offer at all, and goes back on top.
#[test]
fn a_nonland_card_is_never_offered() {
    let mut game = upkeep_with(cards::LIGHTNING_BOLT);
    let offered = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.options.len());
    answer(&mut game, true);

    assert_eq!(
        offered, None,
        "with nothing selectable there is no question to ask",
    );
    assert_eq!(
        library(&game),
        vec![cards::LIGHTNING_BOLT, cards::GRIZZLY_BEARS],
        "so it stays the next draw",
    );
}
