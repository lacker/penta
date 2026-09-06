//! Paying a chosen discard for an activated ability. This card's audit used
//! to say the runtime could not do it, because choosing which card goes
//! needs a window to ask in. It no longer needs one: the enumeration offers
//! a separate activation per discardable card, so the choice is made by
//! picking an action rather than by being asked afterwards.

use super::*;

/// Patchwork Gnomes with the given cards in hand.
fn staged(hand: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let mut gnomes = creature(37_000, cards::PATCHWORK_GNOMES, PlayerId::One);
    gnomes.entered_controller_turn = 0;
    let gnomes_id = gnomes.card.id;
    game.battlefield.push(gnomes);
    for (index, definition) in hand.iter().enumerate() {
        let held = card(
            37_100 + u32::try_from(index).expect("small hand"),
            *definition,
            PlayerId::One,
        );
        game.players[0].hand.push(held);
    }
    (game, gnomes_id)
}

fn activations(game: &Game) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::ActivateAbility { .. }))
        .collect()
}

fn shields(game: &Game, id: GameObjectId) -> Option<u8> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .map(|permanent| permanent.regeneration_shields)
}

#[test]
fn an_empty_hand_pays_for_nothing() {
    let (game, _) = staged(&[]);
    assert!(
        activations(&game).is_empty(),
        "with nothing to discard the ability cannot be activated"
    );
}

#[test]
fn each_card_in_hand_is_a_separate_way_to_pay() {
    let (game, _) = staged(&[cards::MOUNTAIN, cards::LIGHTNING_BOLT]);
    let offered: Vec<_> = activations(&game)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility { cost_objects, .. } => Some(cost_objects),
            _ => None,
        })
        .collect();
    assert_eq!(
        offered,
        vec![vec![GameObjectId(37_100)], vec![GameObjectId(37_101)]],
        "which card goes is chosen by picking an activation, not by a later decision"
    );
}

#[test]
fn discarding_arms_the_shield_and_saves_it() {
    let (mut game, gnomes) = staged(&[cards::MOUNTAIN]);
    game.apply(PlayerId::One, activations(&game)[0].clone())
        .expect("the card in hand pays for it");
    pass_priority_pair(&mut game);

    assert!(game.players[0].hand.is_empty(), "the card was discarded");
    assert_eq!(game.players[0].graveyard.len(), 1);
    assert_eq!(shields(&game, gnomes), Some(1));

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == gnomes)
        .expect("the Gnomes are on the battlefield")
        .damage = 99;
    game.check_state_based_actions();

    assert_eq!(
        shields(&game, gnomes),
        Some(0),
        "the shield replaced the destruction and was spent doing it"
    );
}
