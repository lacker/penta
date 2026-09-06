//! Two abilities that move a creature out of the game entirely, by opposite
//! routes. Visara destroys with a rider that stops the regeneration shield
//! from saving it; Wayward Soul puts itself on top of its owner's library,
//! which is not a destruction at all and so answers the same removal a
//! regeneration shield would.

use super::*;

#[test]
fn visara_kills_through_a_regeneration_shield() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut visara = creature(61_000, cards::VISARA_THE_DREADFUL, PlayerId::One);
    visara.entered_controller_turn = 0;
    let visara_id = visara.card.id;
    game.battlefield.push(visara);
    let mut troll = creature(61_001, cards::SEDGE_TROLL, PlayerId::Two);
    troll.entered_controller_turn = 0;
    troll.regeneration_shields = 1;
    game.battlefield.push(troll);

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == visara_id
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(GameObjectId(61_001)))
            }
            _ => false,
        })
        .expect("the Troll is a legal target");
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);
    game.check_state_based_actions();

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(61_001)),
        "the shield could not replace a destruction that says it cannot"
    );
    assert_eq!(game.players[1].graveyard.len(), 1);
}

#[test]
fn wayward_soul_goes_back_to_the_top_of_the_library() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut soul = creature(61_010, cards::WAYWARD_SOUL, PlayerId::One);
    soul.entered_controller_turn = 0;
    let soul_id = soul.card.id;
    game.battlefield.push(soul);
    game.players[0].mana_pool.blue = 1;
    let before = game.players[0].library.len();

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == soul_id),
        )
        .expect("one blue pays for it");
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty(), "it left the battlefield");
    assert!(
        game.players[0].graveyard.is_empty(),
        "and did not die on the way"
    );
    assert_eq!(
        game.players[0].library.len(),
        before + 1,
        "the library is one deeper, which is the next draw"
    );
}
