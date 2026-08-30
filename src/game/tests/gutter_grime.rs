//! Gutter Grime: every nontoken creature you lose leaves another Ooze, and
//! every Ooze stays tied to the particular enchantment that created it.

use super::*;

const SLIME: CounterKind = CounterKind::named("slime");

fn staged(grimes: usize) -> (Game, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut ids = Vec::new();
    for _ in 0..grimes {
        ids.push(
            game.put_onto_battlefield(PlayerId::One, cards::GUTTER_GRIME)
                .expect("Gutter Grime is cataloged"),
        );
    }
    drain_pending(&mut game);
    (game, ids)
}

fn tokens(game: &Game) -> Vec<&Permanent> {
    game.battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == ObjectKind::Token)
        .collect()
}

fn grime(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("Gutter Grime remains on the battlefield")
}

fn kill_bear(game: &mut Game, controller: PlayerId) {
    let bear = game
        .put_onto_battlefield(controller, cards::GRIZZLY_BEARS)
        .expect("Grizzly Bears is cataloged");
    drain_pending(game);
    game.move_permanents_to_graveyard(&[bear]);
    drain_pending(game);
}

#[test]
fn each_death_grows_every_ooze_from_the_same_grime() {
    let (mut game, grimes) = staged(1);
    let grime_id = grimes[0];

    kill_bear(&mut game, PlayerId::One);
    assert_eq!(grime(&game, grime_id).counters(SLIME), 1);
    assert_eq!(tokens(&game).len(), 1);
    assert_eq!(
        (
            game.power(tokens(&game)[0]),
            game.toughness(tokens(&game)[0])
        ),
        (Some(1), Some(1)),
    );

    kill_bear(&mut game, PlayerId::One);
    assert_eq!(grime(&game, grime_id).counters(SLIME), 2);
    assert_eq!(tokens(&game).len(), 2);
    for token in tokens(&game) {
        assert_eq!(
            (game.power(token), game.toughness(token)),
            (Some(2), Some(2)),
            "both Oozes read the live counter count",
        );
    }
}

#[test]
fn two_grimes_keep_their_tokens_separately_linked() {
    let (mut game, grimes) = staged(2);
    kill_bear(&mut game, PlayerId::One);

    assert_eq!(tokens(&game).len(), 2, "one trigger from each Grime");
    assert!(
        tokens(&game)
            .iter()
            .any(|token| token.created_by == Some(grimes[0]))
    );
    assert!(
        tokens(&game)
            .iter()
            .any(|token| token.created_by == Some(grimes[1]))
    );

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == grimes[0])
        .expect("the first Grime remains")
        .add_counters(SLIME, 1);

    for token in tokens(&game) {
        let expected = if token.created_by == Some(grimes[0]) {
            2
        } else {
            1
        };
        assert_eq!(
            (game.power(token), game.toughness(token)),
            (Some(expected), Some(expected)),
            "each Ooze reads only its own creator",
        );
    }
}

#[test]
fn an_ooze_becomes_zero_zero_when_its_grime_leaves() {
    let (mut game, grimes) = staged(1);
    kill_bear(&mut game, PlayerId::One);
    assert_eq!(tokens(&game).len(), 1);

    game.move_permanents_to_graveyard(&grimes);
    game.check_state_based_actions();
    drain_pending(&mut game);

    assert!(
        tokens(&game).is_empty(),
        "the orphaned 0/0 Ooze dies to state-based actions",
    );
}

#[test]
fn tokens_and_opposing_creatures_do_not_feed_the_grime() {
    let (mut game, grimes) = staged(1);
    let grime_id = grimes[0];
    kill_bear(&mut game, PlayerId::One);
    let ooze = tokens(&game)[0].card.id;

    game.move_permanents_to_graveyard(&[ooze]);
    drain_pending(&mut game);
    assert_eq!(
        grime(&game, grime_id).counters(SLIME),
        1,
        "tokens do not count"
    );
    assert!(tokens(&game).is_empty(), "no replacement Ooze was made");

    kill_bear(&mut game, PlayerId::Two);
    assert_eq!(
        grime(&game, grime_id).counters(SLIME),
        1,
        "an opponent's creature does not count",
    );
    assert!(tokens(&game).is_empty());
}
