//! A Sliver that grants an activated ability to all Slivers. The point worth
//! covering is whose creature "this creature" means inside the granted
//! clause: the Sliver that has it, not the one that handed it out.

use super::*;

/// Barbed Sliver plus a second Sliver, both under player one.
fn staged() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut granter = creature(29_000, cards::BARBED_SLIVER, PlayerId::One);
    granter.entered_controller_turn = 0;
    let granter_id = granter.card.id;
    let mut other = creature(29_001, cards::MUSCLE_SLIVER, PlayerId::One);
    other.entered_controller_turn = 0;
    let other_id = other.card.id;
    game.battlefield.push(granter);
    game.battlefield.push(other);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    (game, granter_id, other_id)
}

fn power(game: &Game, id: GameObjectId) -> i16 {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the Sliver is on the battlefield");
    game.power(permanent).expect("power")
}

#[test]
fn the_granted_ability_pumps_whichever_sliver_activates_it() {
    let (mut game, granter, other) = staged();
    // Muscle Sliver's own anthem makes both 1 bigger, so the granter is a
    // 3/3 and the Muscle Sliver a 2/2 before anything is activated.
    assert_eq!((power(&game, granter), power(&game, other)), (3, 2));

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == other))
        .expect("the granted ability is offered on the other Sliver");
    game.apply(PlayerId::One, activation)
        .expect("two colorless pays for it");
    pass_priority_pair(&mut game);

    assert_eq!(
        (power(&game, granter), power(&game, other)),
        (3, 3),
        "the Sliver that activated it grew, and the one that granted it did not"
    );
}
