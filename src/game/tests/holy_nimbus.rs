//! The shared mechanics behind the two Holy Nimbus creatures.

use super::*;

fn activation(game: &Game, player: PlayerId, source: GameObjectId) -> Option<Action> {
    game.legal_actions(player).into_iter().find(|action| {
        matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source)
    })
}

fn survives(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

#[test]
fn persistent_regeneration_replaces_each_destruction_until_prohibited() {
    let mut game = ready_game();
    game.turns_started = [5, 5];
    let clergy = creature(10_000, cards::CLERGY_OF_THE_HOLY_NIMBUS, PlayerId::One);
    let clergy_id = clergy.card.id;
    game.battlefield.push(clergy);

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == clergy_id)
        .expect("the Clergy is present")
        .damage = 1;
    game.check_state_based_actions();
    let regenerated = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == clergy_id)
        .expect("the persistent replacement regenerated the Clergy");
    assert!(regenerated.tapped);
    assert_eq!(regenerated.damage, 0, "regeneration heals lethal damage");
    assert_eq!(regenerated.regeneration_shields, 0, "no shield was spent");

    game.destroy_permanent(clergy_id);
    assert!(
        survives(&game, clergy_id),
        "the persistent replacement also applies to an explicit destroy effect",
    );

    game.priority = PlayerId::Two;
    game.players[PlayerId::Two.index()].mana_pool.colorless = 1;
    let action = activation(&game, PlayerId::Two, clergy_id).expect("the opponent may activate");
    game.apply(PlayerId::Two, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    game.destroy_permanent(clergy_id);
    assert!(
        !survives(&game, clergy_id),
        "the turn-scoped prohibition stops the regeneration replacement",
    );
}

#[test]
fn only_opponents_are_offered_the_regeneration_prohibition() {
    let mut game = ready_game();
    game.turns_started = [5, 5];
    let clergy = creature(10_000, cards::CLERGY_OF_THE_HOLY_NIMBUS, PlayerId::One);
    let clergy_id = clergy.card.id;
    game.battlefield.push(clergy);
    for player in [PlayerId::One, PlayerId::Two] {
        game.players[player.index()].mana_pool.colorless = 2;
    }

    assert!(
        activation(&game, PlayerId::One, clergy_id).is_none(),
        "the permanent's controller is excluded",
    );
    game.priority = PlayerId::Two;
    assert!(
        activation(&game, PlayerId::Two, clergy_id).is_some(),
        "an opponent is allowed",
    );
}

#[test]
fn destruction_that_cannot_regenerate_ignores_the_persistent_replacement() {
    let mut game = ready_game();
    let clergy = creature(10_000, cards::CLERGY_OF_THE_HOLY_NIMBUS, PlayerId::One);
    let clergy_id = clergy.card.id;
    game.battlefield.push(clergy);

    game.destroy_permanent_without_regeneration(clergy_id);

    assert!(!survives(&game, clergy_id));
}
