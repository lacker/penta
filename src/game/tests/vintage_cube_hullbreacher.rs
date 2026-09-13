//! Hullbreacher: their draw spell becomes your mana, and their turn-based
//! draw is the one card they still get.

use super::*;

/// Hullbreacher on the battlefield under Player One, with both libraries
/// stocked.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.players[0].library.clear();
    game.players[1].library.clear();
    for index in 0..8 {
        game.players[0]
            .library
            .push(card(118_000 + index, cards::ISLAND, PlayerId::One));
        game.players[1]
            .library
            .push(card(118_100 + index, cards::ISLAND, PlayerId::Two));
    }
    let hullbreacher = game
        .put_onto_battlefield(PlayerId::One, cards::HULLBREACHER)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, hullbreacher)
}

fn treasures(game: &Game, player: PlayerId) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| permanent.controller == player)
        .filter(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Treasure"))
        })
        .count()
}

/// A draw of theirs outside their draw step becomes a Treasure for you.
#[test]
fn their_draw_becomes_your_treasure() {
    let (mut game, _) = staged();
    let library = game.players[1].library.len();

    game.draw_cards(PlayerId::Two, 3);

    assert!(game.players[1].hand.is_empty(), "they drew nothing");
    assert_eq!(
        game.players[1].library.len(),
        library,
        "and the cards stayed in their library",
    );
    assert_eq!(
        treasures(&game, PlayerId::One),
        3,
        "three Treasures for you"
    );
    assert_eq!(treasures(&game, PlayerId::Two), 0, "and none for them");
}

/// Your own draws are untouched.
#[test]
fn your_own_draws_are_not_replaced() {
    let (mut game, _) = staged();

    game.draw_cards(PlayerId::One, 2);

    assert_eq!(game.players[0].hand.len(), 2, "you drew both");
    assert_eq!(treasures(&game, PlayerId::One), 0, "and made no Treasure");
}

/// "Except the first one they draw in each of their draw steps": that one
/// still happens, and the next one in the same step does not.
#[test]
fn their_turn_based_draw_still_happens() {
    let (mut game, _) = staged();
    game.active_player = PlayerId::Two;
    game.step = Step::Draw;
    game.draw_step_draw_taken = [false; 2];

    game.draw_cards(PlayerId::Two, 1);
    assert_eq!(game.players[1].hand.len(), 1, "their draw step card");
    assert_eq!(treasures(&game, PlayerId::One), 0, "no Treasure for it");

    game.draw_cards(PlayerId::Two, 1);

    assert_eq!(game.players[1].hand.len(), 1, "and the second one is not");
    assert_eq!(treasures(&game, PlayerId::One), 1, "that one is a Treasure");
}

/// Your own draw step is not theirs: a draw of theirs during your draw step
/// is replaced.
#[test]
fn their_draw_during_your_draw_step_is_replaced() {
    let (mut game, _) = staged();
    game.active_player = PlayerId::One;
    game.step = Step::Draw;
    game.draw_step_draw_taken = [false; 2];

    game.draw_cards(PlayerId::Two, 1);

    assert!(game.players[1].hand.is_empty(), "it was replaced");
    assert_eq!(treasures(&game, PlayerId::One), 1);
}

/// The Treasure taps for mana of any colour, which is what it is for.
#[test]
fn the_treasure_makes_mana() {
    let (mut game, _) = staged();
    game.draw_cards(PlayerId::Two, 1);
    let treasure = game
        .battlefield
        .iter()
        .find(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Treasure"))
        })
        .expect("the Treasure is there")
        .card
        .id;

    let add = Action::ActivateManaAbility {
        source: treasure,
        ability: mana_ability_for(&game, treasure, ManaColor::Red),
        color: ManaColor::Red,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    game.apply(PlayerId::One, add).expect("it taps for red");

    assert_eq!(game.players[0].mana_pool.red, 1);
}

/// Flash: he can be cast on their turn, which is the point of holding him.
#[test]
fn he_has_flash() {
    let (game, hullbreacher) = staged();
    let body = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == hullbreacher)
        .expect("he is there");

    assert!(game.permanent_has_executable_keyword(body, KeywordAbility::Flash));
}

/// "If you control multiple Hullbreachers while an opponent would draw a
/// card, you'll create only one Treasure." The first replacement eats the
/// draw, and what the second one would replace is no longer happening: three
/// of him make three Treasures over three draws, not nine.
#[test]
fn a_second_hullbreacher_adds_no_treasure() {
    let (mut game, _) = staged();
    for _ in 0..2 {
        game.put_onto_battlefield(PlayerId::One, cards::HULLBREACHER)
            .expect("cataloged");
    }
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::HULLBREACHER)
            .count(),
        3,
        "three of him, and nothing about him is legendary",
    );

    game.draw_cards(PlayerId::Two, 1);
    // Three applicable replacements is a question for the player drawing
    // (they pick the order), and every answer to it ends the same way.
    drain_pending(&mut game);
    assert_eq!(
        treasures(&game, PlayerId::One),
        1,
        "one draw replaced is one Treasure, however many of him watched it",
    );

    game.draw_cards(PlayerId::Two, 2);
    drain_pending(&mut game);
    assert_eq!(
        treasures(&game, PlayerId::One),
        3,
        "and one per draw after that",
    );
    assert!(game.players[1].hand.is_empty(), "they drew nothing at all");
}

/// "This card is put directly into your hand. It is not drawn." The
/// Hullbreacher replaces draws and nothing else, so a Demonic Tutor across
/// the table finds its card and keeps it: no Treasure, and the card is in
/// their hand rather than back in their library.
#[test]
fn a_tutor_is_not_a_draw_and_makes_no_treasure() {
    let (mut game, _) = staged();
    let tutor = card(118_400, cards::DEMONIC_TUTOR, PlayerId::Two);
    let tutor_id = tutor.id;
    game.players[1].hand.push(tutor);
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::Two, color, 2);
    }
    game.turn += 1;
    game.active_player = PlayerId::Two;
    game.turns_started[PlayerId::Two.index()] += 1;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;
    let library = game.players[1].library.len();

    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == tutor_id))
        .expect("two mana casts it");
    game.apply(PlayerId::Two, cast).expect("it is cast");
    drain_pending(&mut game);

    assert_eq!(
        game.players[1].hand.len(),
        1,
        "the card they searched for is in their hand",
    );
    assert_eq!(
        game.players[1].library.len(),
        library - 1,
        "and it came out of their library",
    );
    assert_eq!(
        treasures(&game, PlayerId::One),
        0,
        "a search is not a draw, so the Hullbreacher never saw it",
    );
}
