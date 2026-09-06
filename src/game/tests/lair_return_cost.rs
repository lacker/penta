//! A resolving payment whose cost is returning a permanent. Both halves need
//! covering: paying it puts the named land back in hand and keeps the Lair,
//! and the predicate excludes other Lairs -- a negation, so a board of
//! nothing but Lairs must offer no way to pay at all.

use super::*;

/// Crosis's Catacombs entering under player one, with `lands` beside it.
fn entering(lands: &[CardDefinitionId]) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    for (index, definition) in lands.iter().enumerate() {
        let mut land = creature(
            51_100 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::One,
        );
        land.entered_controller_turn = 0;
        game.battlefield.push(land);
    }
    // Played from hand, so the enters trigger runs the way it would in a game.
    let lair = CardInstanceId(51_000);
    game.players[0]
        .hand
        .push(card(51_000, cards::CROSIS_S_CATACOMBS, PlayerId::One));
    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: lair,
            option: PlayOptionId::DEFAULT,
        },
    )
    .expect("a land drop is available");
    for _ in 0..8 {
        if game.observe(PlayerId::One).decision.is_some() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    (game, lair)
}

/// Playing the land gives it a new object id, so the Lair is found by what
/// it is rather than by the id it had in hand.
fn survives(game: &Game) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == ObjectKind::Card(cards::CROSIS_S_CATACOMBS))
}

#[test]
fn paying_returns_a_land_and_keeps_the_lair() {
    let (mut game, _) = entering(&[cards::MOUNTAIN]);
    // Each payable permanent is its own option, so the label names the land.
    choose_decision_by_label(&mut game, PlayerId::One, "Return Mountain");
    drain_pending(&mut game);

    assert!(survives(&game), "the Lair stayed");
    assert_eq!(game.players[0].hand.len(), 1, "the Mountain went to hand");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(51_100)),
        "and left the battlefield"
    );
}

#[test]
fn declining_sacrifices_the_lair() {
    let (mut game, _) = entering(&[cards::MOUNTAIN]);
    choose_decision_by_label(&mut game, PlayerId::One, "Decline");
    drain_pending(&mut game);

    assert!(!survives(&game));
    assert!(
        game.players[0].hand.is_empty(),
        "declining costs nothing but the Lair"
    );
}

#[test]
fn another_lair_cannot_pay_for_this_one() {
    let (mut game, _) = entering(&[cards::TREVAS_RUINS]);
    assert!(
        !game
            .observe(PlayerId::One)
            .decision
            .is_some_and(|decision| decision
                .options
                .iter()
                .any(|option| option.label.starts_with("Return"))),
        "with only a Lair to give back there is nothing to return"
    );
    drain_pending(&mut game);
    assert!(!survives(&game), "so the unpaid branch sacrifices it");
}
