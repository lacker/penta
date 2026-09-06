//! Spells that add mana in proportion to something countable, and a spell
//! that draws as many cards as it discarded. Each has a count that could be
//! taken from the wrong place -- the wrong graveyard, the wrong half of the
//! battlefield, or a fixed number instead of the hand it just emptied.

use super::*;

fn cast(game: &mut Game, spell: CardInstanceId) {
    let action = cast_action(spell, Vec::new(), Vec::new(), 0);
    assert!(
        game.legal_actions(PlayerId::One).contains(&action),
        "the spell is castable"
    );
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(game);
}

#[test]
fn songs_of_the_damned_counts_only_your_own_creature_cards() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[1].graveyard.clear();
    // Two creature cards yours, one non-creature yours, one creature theirs.
    game.players[0]
        .graveyard
        .push(card(68_000, cards::GRIZZLY_BEARS, PlayerId::One));
    game.players[0]
        .graveyard
        .push(card(68_001, cards::SERRA_ANGEL, PlayerId::One));
    game.players[0]
        .graveyard
        .push(card(68_002, cards::LIGHTNING_BOLT, PlayerId::One));
    game.players[1]
        .graveyard
        .push(card(68_003, cards::GRIZZLY_BEARS, PlayerId::Two));

    let songs = card(68_010, cards::SONGS_OF_THE_DAMNED, PlayerId::One);
    game.players[0].hand.push(songs.clone());
    game.players[0].mana_pool.black = 1;
    cast(&mut game, songs.id);

    assert_eq!(
        game.players[0].mana_pool.black, 2,
        "two creature cards of your own, and the spell's own black is spent"
    );
}

#[test]
fn brightstone_ritual_counts_every_goblin_on_the_battlefield() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.battlefield
        .push(creature(68_100, cards::RAGING_GOBLIN, PlayerId::One));
    game.battlefield
        .push(creature(68_101, cards::RAGING_GOBLIN, PlayerId::Two));
    game.battlefield
        .push(creature(68_102, cards::GRIZZLY_BEARS, PlayerId::One));

    let ritual = card(68_110, cards::BRIGHTSTONE_RITUAL, PlayerId::One);
    game.players[0].hand.push(ritual.clone());
    game.players[0].mana_pool.red = 1;
    cast(&mut game, ritual.id);

    assert_eq!(
        game.players[0].mana_pool.red, 2,
        "both Goblins count and the Bears do not"
    );
}

#[test]
fn tolarian_winds_draws_as_many_as_it_discarded() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    let winds = card(68_200, cards::TOLARIAN_WINDS, PlayerId::One);
    game.players[0].hand.push(winds.clone());
    for index in 0..3 {
        game.players[0]
            .hand
            .push(card(68_210 + index, cards::MOUNTAIN, PlayerId::One));
    }
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    let library = game.players[0].library.len();
    cast(&mut game, winds.id);

    assert_eq!(
        game.players[0].hand.len(),
        3,
        "three cards went and three came back"
    );
    assert_eq!(
        game.players[0].library.len(),
        library - 3,
        "and they came off the library"
    );
    assert_eq!(
        game.players[0].graveyard.len(),
        4,
        "three discarded plus the spell itself"
    );
}
