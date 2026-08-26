//! World permanents share one global state-based action, regardless of name,
//! controller, or owner.

use super::*;

fn world_ids(game: &Game) -> Vec<GameObjectId> {
    game.battlefield
        .iter()
        .filter(|permanent| {
            game.effective_rules(permanent)
                .is_some_and(|rules| rules.has_supertype(CardSupertype::World))
        })
        .map(|permanent| permanent.card.id)
        .collect()
}

#[test]
fn newest_world_permanent_is_the_only_one_that_stays() {
    let mut game = ready_game();
    let older = game
        .put_onto_battlefield(PlayerId::One, cards::NETHER_VOID)
        .expect("Nether Void is cataloged");
    let newer = game
        .put_onto_battlefield(PlayerId::Two, cards::GRAVITY_SPHERE)
        .expect("Gravity Sphere is cataloged");
    drain_pending(&mut game);

    game.check_state_based_actions();

    assert_eq!(world_ids(&game), vec![newer]);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == older)
    );
    assert_eq!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::NETHER_VOID],
    );
}

#[test]
fn a_tie_for_newest_puts_every_world_permanent_into_its_owners_graveyard() {
    let mut game = ready_game();
    let first = game
        .put_onto_battlefield(PlayerId::One, cards::NETHER_VOID)
        .expect("Nether Void is cataloged");
    let second = game
        .put_onto_battlefield(PlayerId::Two, cards::GRAVITY_SPHERE)
        .expect("Gravity Sphere is cataloged");
    drain_pending(&mut game);

    let shared_timestamp = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == first)
        .expect("the first World permanent entered")
        .timestamp;
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == second)
        .expect("the second World permanent entered")
        .timestamp = shared_timestamp;

    game.check_state_based_actions();

    assert!(world_ids(&game).is_empty());
    assert_eq!(
        game.players[PlayerId::One.index()].graveyard[0].definition,
        cards::NETHER_VOID,
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].graveyard[0].definition,
        cards::GRAVITY_SPHERE,
    );
}
