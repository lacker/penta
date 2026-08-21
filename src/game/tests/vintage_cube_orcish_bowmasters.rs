//! Orcish Bowmasters: an arrow for every draw an opponent did not have
//! coming, and an Army that grows by one each time.

use super::*;

/// Answers every pending decision, pointing anything that must be pointed at
/// the opponent, then resolves whatever is left on the stack.
fn settle(game: &mut Game) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            // Two triggers waiting at once are ordered rather than picked
            // between, and that decision wants every option.
            let options = if decision.minimum > 1 {
                decision.options.iter().map(|option| option.id).collect()
            } else {
                decision
                    .options
                    .iter()
                    .find(|option| option.label == "your opponent")
                    .or_else(|| decision.options.first())
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
}

/// Player One's Bowmasters on an otherwise empty battlefield, with its own
/// entry trigger already resolved.
fn staged() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, cards::ORCISH_BOWMASTERS)
        .expect("cataloged");
    settle(&mut game);
    game
}

fn army(game: &Game) -> Option<&Permanent> {
    game.battlefield.iter().find(|permanent| {
        is_token_with(
            permanent,
            tokens::creature(&["Orc", "Army"], &[ManaColor::Black], 0, 0),
        )
    })
}

fn army_counters(game: &Game) -> u16 {
    army(game).map_or(0, |permanent| {
        permanent.counters(CounterKind::PlusOnePlusOne)
    })
}

/// The entry itself is one of the two ways the ability fires, and the amass
/// that follows it has no Army to find, so it makes one.
#[test]
fn entering_shoots_and_amasses() {
    let game = staged();

    assert_eq!(game.players[1].life, 19, "one arrow at the opponent");
    let army = army(&game).expect("amass made an Army");
    assert_eq!(army.counters(CounterKind::PlusOnePlusOne), 1);
    assert_eq!(game.power(army), Some(1), "a 0/0 with one counter");
}

/// Every later draw is another arrow, and the counters land on the Army that
/// is already there rather than making a second one.
#[test]
fn each_extra_draw_grows_the_same_army() {
    let mut game = staged();

    game.draw_cards(PlayerId::Two, 2);
    settle(&mut game);

    assert_eq!(game.players[1].life, 17, "two more arrows");
    assert_eq!(army_counters(&game), 3);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Orc", "Army"], &[ManaColor::Black], 0, 0)
            ))
            .count(),
        1,
        "amass grows an Army it already controls",
    );
}

/// Your own draws are not an opponent's, however many you take.
#[test]
fn your_own_draws_are_not_shot_at() {
    let mut game = staged();

    game.draw_cards(PlayerId::One, 3);
    settle(&mut game);

    assert_eq!(game.players[1].life, 19, "no further arrows");
    assert_eq!(army_counters(&game), 1);
}

/// The card an opponent is handed in their own draw step is spared, and the
/// next one in that same step is not.
#[test]
fn the_first_draw_of_their_draw_step_is_spared() {
    let mut game = staged();
    game.active_player = PlayerId::Two;
    game.step = Step::Draw;
    game.draw_step_draw_taken = [false; 2];

    game.draw_cards(PlayerId::Two, 1);
    settle(&mut game);
    assert_eq!(game.players[1].life, 19, "the turn-based draw is spared");
    assert_eq!(army_counters(&game), 1);

    game.draw_cards(PlayerId::Two, 1);
    settle(&mut game);
    assert_eq!(game.players[1].life, 18, "the second one is not");
    assert_eq!(army_counters(&game), 2);
}

/// The exemption belongs to the drawing player's own draw step. A draw taken
/// during your draw step is an ordinary draw for them.
#[test]
fn their_draw_during_your_draw_step_is_not_spared() {
    let mut game = staged();
    game.active_player = PlayerId::One;
    game.step = Step::Draw;
    game.draw_step_draw_taken = [false; 2];

    game.draw_cards(PlayerId::Two, 1);
    settle(&mut game);

    assert_eq!(game.players[1].life, 18);
    assert_eq!(army_counters(&game), 2);
}
