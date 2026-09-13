//! Multiversal Passage: a shock land that is whichever basic type the hand
//! actually wants.

use super::*;

/// Player One holding a Passage, with the life to pay for it.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let card = card(93_000, cards::MULTIVERSAL_PASSAGE, PlayerId::One);
    let passage = card.id;
    game.players[0].hand.push(card);
    game.players[0].lands_played_this_turn = 0;
    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    (game, passage)
}

/// Plays it, choosing `land_type`, and pays or declines the two life.
fn play(game: &mut Game, passage: GameObjectId, land_type: &str, pay: bool) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == passage))
        .expect("the land is playable");
    game.apply(PlayerId::One, action).expect("it is played");

    let choice = game
        .observe(PlayerId::One)
        .decision
        .expect("it asks for a basic land type");
    let chosen = choice
        .options
        .iter()
        .find(|option| option.label == land_type)
        .unwrap_or_else(|| panic!("{land_type} is offered: {:?}", choice.options))
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: choice.id,
            options: vec![chosen],
        },
    )
    .expect("naming a type is legal");

    let payment = game
        .observe(PlayerId::One)
        .decision
        .expect("it then offers the two life");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: payment.id,
            options: vec![u32::from(pay)],
        },
    )
    .expect("answering the payment is legal");
    drain_pending(game);
}

fn on_battlefield(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MULTIVERSAL_PASSAGE)
        .expect("the Passage is on the battlefield")
}

/// It offers all five basic land types and nothing else.
#[test]
fn it_offers_the_five_basic_land_types() {
    let (mut game, passage) = staged();
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == passage))
        .expect("the land is playable");
    game.apply(PlayerId::One, action).expect("it is played");

    let choice = game
        .observe(PlayerId::One)
        .decision
        .expect("it asks for a basic land type");
    let mut labels = choice
        .options
        .iter()
        .map(|option| option.label.clone())
        .collect::<Vec<_>>();
    labels.sort();

    assert_eq!(
        labels,
        vec!["Forest", "Island", "Mountain", "Plains", "Swamp"],
    );
}

/// It is the chosen type, and taps for that type's mana.
#[test]
fn it_becomes_the_chosen_type() {
    let (mut game, passage) = staged();

    play(&mut game, passage, "Island", true);

    let land = on_battlefield(&game);
    assert!(
        game.effective_subtypes(land)
            .contains(crate::card::Subtype::named("Island"))
    );
    assert!(
        !game
            .effective_subtypes(land)
            .contains(crate::card::Subtype::named("Forest")),
        "one type, the one chosen",
    );
    let colors = game
        .mana_ability_activations(land)
        .into_iter()
        .map(|activation| activation.color)
        .collect::<Vec<_>>();
    assert_eq!(colors, vec![ManaColor::Blue]);
}

/// A different choice is a different land.
#[test]
fn a_different_choice_is_a_different_land() {
    let (mut game, passage) = staged();

    play(&mut game, passage, "Mountain", true);

    let land = on_battlefield(&game);
    assert!(
        game.effective_subtypes(land)
            .contains(crate::card::Subtype::named("Mountain"))
    );
    assert_eq!(
        game.mana_ability_activations(land)
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Red],
    );
}

/// Paying costs two life and the land arrives untapped.
#[test]
fn paying_two_life_brings_it_in_untapped() {
    let (mut game, passage) = staged();
    game.players[0].life = 20;

    play(&mut game, passage, "Swamp", true);

    assert_eq!(game.players[0].life, 18);
    assert!(!on_battlefield(&game).tapped);
}

/// Declining keeps the life and the land arrives tapped.
#[test]
fn declining_brings_it_in_tapped() {
    let (mut game, passage) = staged();
    game.players[0].life = 20;

    play(&mut game, passage, "Plains", false);

    assert_eq!(game.players[0].life, 20);
    assert!(on_battlefield(&game).tapped);
}
