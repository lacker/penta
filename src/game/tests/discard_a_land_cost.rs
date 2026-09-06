//! An activated ability whose cost is "discard a land card". The point worth
//! covering is that the cost names a kind of card rather than a count: a hand
//! full of spells cannot pay it, and paying it takes a land specifically.

use super::*;

/// Seismic Assault under player one, with the given cards in hand.
fn staged(hand: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let mut assault = creature(33_000, cards::SEISMIC_ASSAULT, PlayerId::One);
    assault.entered_controller_turn = 0;
    game.battlefield.push(assault);
    for (index, definition) in hand.iter().enumerate() {
        let card = card(
            33_100 + u32::try_from(index).expect("small hand"),
            *definition,
            PlayerId::One,
        );
        game.players[0].hand.push(card);
    }
    game
}

fn activations(game: &Game) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::ActivateAbility { .. }))
        .collect()
}

#[test]
fn a_hand_without_a_land_cannot_pay_the_cost() {
    let game = staged(&[cards::LIGHTNING_BOLT, cards::LIGHTNING_BOLT]);
    assert!(
        activations(&game).is_empty(),
        "two spells in hand pay for nothing"
    );
}

#[test]
fn paying_discards_the_land_and_deals_the_damage() {
    let mut game = staged(&[cards::MOUNTAIN, cards::LIGHTNING_BOLT]);
    let offered = activations(&game);
    // Every offer differs only in its target; the land is the one way to pay.
    assert!(
        offered.iter().all(|action| matches!(
            action,
            Action::ActivateAbility { cost_objects, .. }
                if cost_objects == &vec![GameObjectId(33_100)]
        )),
        "the land travels with every activation, and the spell never does"
    );
    let at_opponent = offered
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { targets, .. }
                    if targets == &vec![TargetSelection::single(
                        TargetSlotId(0),
                        Target::Player(PlayerId::Two),
                    )]
            )
        })
        .expect("the opponent is a legal target");

    game.apply(PlayerId::One, at_opponent)
        .expect("the land pays for it");
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 18, "two damage to the opponent");
    assert_eq!(
        game.players[0].hand.len(),
        1,
        "the land left the hand and the spell stayed"
    );
    assert_eq!(game.players[0].graveyard.len(), 1);
    assert!(
        activations(&game).is_empty(),
        "with only a spell left there is nothing to discard"
    );
}
