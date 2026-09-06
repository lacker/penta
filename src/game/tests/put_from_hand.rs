//! Putting a creature onto the battlefield from hand rather than casting it.
//! The clause is optional, so activating it with nothing to bring is legal
//! and does nothing; and what arrives is a permanent rather than a spell, so
//! it never goes on the stack and nothing gets a chance to counter it.

use super::*;

/// Elvish Piper on the battlefield, with `hand` cards held.
fn staged(hand: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let mut piper = creature(73_000, cards::ELVISH_PIPER, PlayerId::One);
    piper.entered_controller_turn = 0;
    let piper_id = piper.card.id;
    game.battlefield.push(piper);
    for (index, definition) in hand.iter().enumerate() {
        game.players[0].hand.push(card(
            73_100 + u32::try_from(index).expect("small hand"),
            *definition,
            PlayerId::One,
        ));
    }
    game.players[0].mana_pool.green = 1;
    (game, piper_id)
}

fn pipe(game: &mut Game, piper: GameObjectId) {
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == piper))
        .expect("one green and a tap pay for it");
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(game);
}

#[test]
fn the_creature_arrives_without_being_cast() {
    let (mut game, piper) = staged(&[cards::SERRA_ANGEL]);
    pipe(&mut game, piper);
    choose_decision_by_label(&mut game, PlayerId::One, "Serra Angel");
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == ObjectKind::Card(cards::SERRA_ANGEL)),
        "the Angel is on the battlefield"
    );
    assert!(game.players[0].hand.is_empty(), "and no longer in hand");
    assert!(game.stack.is_empty(), "it never used the stack");
}

/// "You may" means the ability is still usable with nothing to bring, and
/// resolving it that way changes nothing.
#[test]
fn an_empty_hand_still_lets_it_be_activated() {
    let (mut game, piper) = staged(&[]);
    pipe(&mut game, piper);
    drain_pending(&mut game);

    assert_eq!(game.battlefield.len(), 1, "only the Piper is there");
    assert!(game.players[0].hand.is_empty());
}

#[test]
fn a_noncreature_card_is_not_on_offer() {
    let (mut game, piper) = staged(&[cards::LIGHTNING_BOLT]);
    pipe(&mut game, piper);

    assert!(
        !game
            .observe(PlayerId::One)
            .decision
            .is_some_and(|decision| decision
                .options
                .iter()
                .any(|option| option.label.contains("Lightning Bolt"))),
        "the Bolt is not a creature card"
    );
}
