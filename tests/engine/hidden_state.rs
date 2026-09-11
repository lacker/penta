use penta::card::cards;
use penta::{Game, PlayerId, ZoneError, card, decks};

fn game() -> Game {
    Game::new(
        card::catalog().unwrap(),
        [decks::the_deck(), decks::goblins()],
        11,
    )
    .unwrap()
}

#[test]
fn hands_and_libraries_read_back_unredacted() {
    let game = game();
    // observe() gives the opponent's hand only as a count; the simulation
    // view gives the cards themselves.
    assert_eq!(game.observe(PlayerId::One).opponent_hand_size, 7);
    assert_eq!(game.hand(PlayerId::Two).len(), 7);
    assert_eq!(game.library(PlayerId::Two).len(), 53);
}

#[test]
fn the_same_position_can_be_played_out_as_two_different_worlds() {
    // The point of the API: you do not know their last card, so build both
    // worlds and roll each out. Neither is a permutation of the true state.
    let mut bolt_world = game();
    let mut counter_world = game();
    for (world, guess) in [
        (&mut bolt_world, cards::LIGHTNING_BOLT),
        (&mut counter_world, cards::COUNTERSPELL),
    ] {
        world
            .set_hand(PlayerId::Two, &[cards::MOUNTAIN, guess])
            .unwrap();
    }

    assert_eq!(bolt_world.hand(PlayerId::Two).len(), 2);
    assert_eq!(
        bolt_world.hand(PlayerId::Two)[1].definition,
        cards::LIGHTNING_BOLT
    );
    assert_eq!(
        counter_world.hand(PlayerId::Two)[1].definition,
        cards::COUNTERSPELL
    );
    // Fresh cards get fresh identities rather than reusing anything.
    assert_ne!(
        bolt_world.hand(PlayerId::Two)[0].object,
        game().hand(PlayerId::Two)[0].object
    );
    // Both worlds are playable.
    for world in [&mut bolt_world, &mut counter_world] {
        world
            .apply(PlayerId::One, penta::Action::KeepHand)
            .expect("a rewritten world plays on");
    }
}

#[test]
fn a_library_can_be_stacked_or_emptied() {
    let mut game = game();
    let top_first = [cards::BLACK_LOTUS, cards::MOUNTAIN, cards::LIGHTNING_BOLT];
    game.set_library(PlayerId::Two, &top_first).unwrap();
    assert_eq!(
        game.library(PlayerId::Two)
            .into_iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        top_first,
        "the simulation surface reads back the documented top-first order",
    );

    game.set_library(PlayerId::Two, &[]).unwrap();
    assert!(
        game.library(PlayerId::Two).is_empty(),
        "a simulation may explore an empty library"
    );
}

#[test]
fn a_card_outside_the_catalog_is_rejected() {
    let mut game = game();
    let unknown = penta::CardDefinitionId::from_uuid("00000000-0000-0000-0000-00000000ea60");
    assert_eq!(
        game.set_hand(PlayerId::Two, &[unknown]),
        Err(ZoneError::UnknownCard(unknown)),
    );
}
