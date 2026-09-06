//! "Sacrifice a creature" names a kind of permanent, not another one, so the
//! outlet is allowed to eat itself. That is the corner worth covering: the
//! activation is legal, and the pump it pays for lands on a creature that is
//! already in the graveyard by the time the ability resolves.

use super::*;

/// Phyrexian Ghoul with `others` more creatures beside it, all under player one.
fn staged(others: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut ghoul = creature(35_000, cards::PHYREXIAN_GHOUL, PlayerId::One);
    ghoul.entered_controller_turn = 0;
    let ghoul_id = ghoul.card.id;
    game.battlefield.push(ghoul);
    for index in 0..others {
        let mut lion = creature(
            35_100 + u32::try_from(index).expect("small board"),
            cards::SAVANNAH_LIONS,
            PlayerId::One,
        );
        lion.entered_controller_turn = 0;
        game.battlefield.push(lion);
    }
    (game, ghoul_id)
}

fn sacrifices(game: &Game) -> Vec<GameObjectId> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility { cost_objects, .. } => Some(cost_objects),
            _ => None,
        })
        .flatten()
        .collect()
}

fn power(game: &Game, id: GameObjectId) -> Option<i16> {
    let permanent = game.battlefield.iter().find(|p| p.card.id == id)?;
    game.power(permanent)
}

#[test]
fn the_outlet_is_offered_itself_as_fodder() {
    let (game, ghoul) = staged(1);
    let offered = sacrifices(&game);
    assert!(
        offered.contains(&ghoul),
        "the Ghoul is a creature its controller controls, so it may pay its own cost"
    );
    assert!(
        offered.contains(&GameObjectId(35_100)),
        "so is the other creature"
    );
}

#[test]
fn eating_another_creature_grows_the_outlet() {
    let (mut game, ghoul) = staged(1);
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { cost_objects, .. }
                    if cost_objects == &vec![GameObjectId(35_100)]
            )
        })
        .expect("the other creature can be eaten");
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(power(&game, ghoul), Some(4), "a 2/2 grew by two");
    assert_eq!(
        power(&game, GameObjectId(35_100)),
        None,
        "the fodder is gone"
    );
}

#[test]
fn eating_itself_leaves_nothing_to_pump() {
    let (mut game, ghoul) = staged(0);
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { cost_objects, .. } if cost_objects == &vec![ghoul]
            )
        })
        .expect("with an empty board the Ghoul is its own only fodder");
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield.is_empty(),
        "the outlet paid itself and the pump resolved onto nothing"
    );
    assert_eq!(game.players[0].graveyard.len(), 1);
}
