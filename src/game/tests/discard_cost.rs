//! Discarding a chosen card as an activation cost.
//!
//! The card travels with the activation rather than being a mid-payment
//! decision, so the enumerator offers one action per discardable card and an
//! empty hand offers none at all.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game
}

fn activations(game: &Game, source: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| {
            matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source)
        })
        .collect()
}

/// One activation per card in hand, because each is a different cost.
#[test]
fn the_prophet_offers_one_activation_per_discardable_card() {
    let mut game = ready();
    let prophet = creature(10_000, cards::MAD_PROPHET, PlayerId::One);
    let prophet_id = prophet.card.id;
    game.battlefield.push(prophet);

    assert!(
        activations(&game, prophet_id).is_empty(),
        "an empty hand cannot pay",
    );

    for index in 0..3 {
        game.players[PlayerId::One.index()].hand.push(card(
            30_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    assert_eq!(
        activations(&game, prophet_id).len(),
        3,
        "one per card in hand",
    );
}

/// Paying it discards the chosen card and draws, leaving the hand the same
/// size but one card different.
#[test]
fn paying_the_cost_loots() {
    let mut game = ready();
    let prophet = creature(10_000, cards::MAD_PROPHET, PlayerId::One);
    let prophet_id = prophet.card.id;
    game.battlefield.push(prophet);
    game.players[PlayerId::One.index()].hand.push(card(
        30_000,
        cards::GRIZZLY_BEARS,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].library.push(card(
        31_000,
        cards::AIR_ELEMENTAL,
        PlayerId::One,
    ));

    let action = activations(&game, prophet_id)
        .into_iter()
        .next()
        .expect("one card, one activation");
    game.apply(PlayerId::One, action).expect("legal");
    drain_pending(&mut game);

    let hand = &game.players[PlayerId::One.index()].hand;
    assert_eq!(hand.len(), 1, "one out, one in");
    assert_eq!(
        hand[0].definition,
        cards::AIR_ELEMENTAL,
        "the drawn card, not the discarded one",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].graveyard.len(),
        1,
        "and the cost went to the graveyard",
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == prophet_id)
            .expect("still there")
            .tapped,
        "the tap was part of the cost too",
    );
}

/// The Market grants the same ability to a land, which taps for it.
#[test]
fn the_market_lets_a_land_loot() {
    let mut game = ready();
    game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("cataloged");
    let land = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MOUNTAIN)
        .expect("it is there")
        .card
        .id;
    let mut aura = creature(10_000, cards::TIN_STREET_MARKET, PlayerId::One);
    aura.attached_to = Some(land);
    game.battlefield.push(aura);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    assert!(
        activations(&game, land).is_empty(),
        "nothing in hand to pay with",
    );

    game.players[PlayerId::One.index()].hand.push(card(
        30_000,
        cards::GRIZZLY_BEARS,
        PlayerId::One,
    ));
    assert_eq!(
        activations(&game, land).len(),
        1,
        "a card in hand makes the land a looter",
    );
}
