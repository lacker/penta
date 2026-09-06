//! Upkeep triggers that fire on every player's turn rather than only their
//! controller's. Two things go wrong quietly here: "that player" has to
//! resolve to whoever's upkeep it is rather than to the permanent's
//! controller, and a trigger written for every upkeep has to fire on the
//! opponent's turn at all.

use super::*;

/// `permanent` under player one, with `lands` Forests each side, at the
/// start of `active`'s upkeep.
fn upkeep(permanent: CardDefinitionId, active: PlayerId, lands: usize) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    let mut mine = creature(69_000, permanent, PlayerId::One);
    mine.entered_controller_turn = 0;
    game.battlefield.push(mine);
    for (index, owner) in [PlayerId::One, PlayerId::Two].into_iter().enumerate() {
        for offset in 0..lands {
            let mut land = creature(
                69_100 + u32::try_from(index * 8 + offset).expect("a small fixture"),
                cards::FOREST,
                owner,
            );
            land.entered_controller_turn = 0;
            game.battlefield.push(land);
        }
    }
    game.active_player = active;
    game.priority = active;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    game.resolve_stack_top();
    game
}

/// How many Forests each player still controls.
fn forests(game: &Game) -> (usize, usize) {
    let count = |player: PlayerId| {
        game.battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && permanent.card.definition == ObjectKind::Card(cards::FOREST)
            })
            .count()
    };
    (count(PlayerId::One), count(PlayerId::Two))
}

#[test]
fn braids_asks_whoever_s_upkeep_it_is() {
    let game = upkeep(cards::BRAIDS_CABAL_MINION, PlayerId::One, 2);
    assert_eq!(
        game.decision_player(),
        Some(PlayerId::One),
        "on my upkeep I am the one who sacrifices"
    );

    let game = upkeep(cards::BRAIDS_CABAL_MINION, PlayerId::Two, 2);
    assert_eq!(
        game.decision_player(),
        Some(PlayerId::Two),
        "and on theirs it is the opponent, though the Braids are mine"
    );
}

#[test]
fn braids_only_offers_that_player_s_own_permanents() {
    let mut game = upkeep(cards::BRAIDS_CABAL_MINION, PlayerId::Two, 2);
    choose_decision_by_label(&mut game, PlayerId::Two, "Forest");
    drain_pending(&mut game);
    assert_eq!(
        forests(&game),
        (2, 1),
        "the Forest that went was the one the opponent controlled"
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(69_000)),
        "and my Braids are untouched"
    );
}

fn saprolings(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            permanent.controller == PlayerId::One && permanent.card.id != GameObjectId(69_000)
        })
        .count()
}

#[test]
fn the_force_makes_a_token_on_both_upkeeps() {
    let mut game = upkeep(cards::VERDANT_FORCE, PlayerId::One, 0);
    drain_pending(&mut game);
    assert_eq!(saprolings(&game), 1, "one on my own upkeep");

    let mut game = upkeep(cards::VERDANT_FORCE, PlayerId::Two, 0);
    drain_pending(&mut game);
    assert_eq!(
        saprolings(&game),
        1,
        "and one on the opponent's, still under my control"
    );
}

#[test]
fn the_elemental_eats_a_land_or_itself() {
    let mut game = upkeep(cards::BOG_ELEMENTAL, PlayerId::One, 2);
    choose_decision_by_label(&mut game, PlayerId::One, "Sacrifice Forest");
    drain_pending(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(69_000)),
        "a land was there to feed it"
    );
    assert_eq!(forests(&game).0, 1, "and one of mine is gone");

    let mut game = upkeep(cards::BOG_ELEMENTAL, PlayerId::One, 0);
    drain_pending(&mut game);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(69_000)),
        "with no land to sacrifice it eats itself"
    );
}
