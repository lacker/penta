//! Death triggers that read what the dying permanent became or was. A
//! trigger that returns "it" has to name the card now sitting in the
//! graveyard rather than the permanent that is gone, and a trigger that
//! watches for a flying creature has to read the keyword off a creature
//! that has already left the battlefield.

use super::*;

/// `mine` under player one, with the first of them killable by id `66_000`.
fn staged(mine: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    for (index, definition) in mine.iter().enumerate() {
        let mut permanent = creature(
            66_000 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::One,
        );
        permanent.entered_controller_turn = 0;
        game.battlefield.push(permanent);
    }
    game
}

fn kill(game: &mut Game, id: u32) {
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == GameObjectId(id))
        .expect("the creature is on the battlefield")
        .damage = 99;
    game.check_state_based_actions();
    drain_pending(game);
    game.check_state_based_actions();
}

fn in_zone(game: &Game, zone: &[CardInstance], definition: CardDefinitionId) -> bool {
    let _ = game;
    zone.iter()
        .any(|card| card.definition == ObjectKind::Card(definition))
}

#[test]
fn a_plain_creature_stays_in_the_graveyard() {
    let mut game = staged(&[cards::GRIZZLY_BEARS]);
    kill(&mut game, 66_000);
    assert!(
        in_zone(&game, &game.players[0].graveyard, cards::GRIZZLY_BEARS),
        "the Bears are where a dead creature goes"
    );
}

#[test]
fn the_phoenix_comes_back_to_hand_instead() {
    let mut game = staged(&[cards::SHIVAN_PHOENIX]);
    kill(&mut game, 66_000);

    assert!(
        in_zone(&game, &game.players[0].hand, cards::SHIVAN_PHOENIX),
        "the card it became was returned to its owner's hand"
    );
    assert!(
        !in_zone(&game, &game.players[0].graveyard, cards::SHIVAN_PHOENIX),
        "and it did not stay in the graveyard as well"
    );
}

fn soulcatcher_power(dying: CardDefinitionId) -> i16 {
    let mut game = staged(&[dying, cards::SOULCATCHER]);
    kill(&mut game, 66_000);
    let catcher = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Card(cards::SOULCATCHER))
        .expect("the Soulcatcher survived");
    game.power(catcher).expect("power")
}

#[test]
fn the_soulcatcher_grows_only_for_a_flier() {
    assert_eq!(
        soulcatcher_power(cards::SERRA_ANGEL),
        2,
        "a flier dying leaves a counter behind"
    );
    assert_eq!(
        soulcatcher_power(cards::GRIZZLY_BEARS),
        1,
        "a ground creature dying does not"
    );
}
