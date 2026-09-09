//! Temporary self-effect helpers must preserve resolution-time values and
//! expire a combined stat change and ability grant together.

use super::*;

fn staged(definition: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let source = creature(10_000, definition, PlayerId::One);
    let id = source.card.id;
    game.battlefield.push(source);
    (game, id)
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the source remains on the battlefield")
}

fn stats(game: &Game, id: GameObjectId) -> (i16, i16) {
    let source = permanent(game, id);
    (game.power(source).unwrap(), game.toughness(source).unwrap())
}

fn activate(game: &mut Game, id: GameObjectId) {
    let action = plain_activation(id, activated_ability_for(game, id, 0));
    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action)
        .expect("the cost is payable");
}

#[test]
fn dynamic_pumps_read_power_at_each_resolution_and_expire_at_cleanup() {
    let (mut game, source) = staged(cards::YEW_SPIRIT);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 4);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);

    activate(&mut game, source);
    activate(&mut game, source);
    assert_eq!(game.stack.len(), 2);
    assert_eq!(
        stats(&game, source),
        (3, 3),
        "payment does not apply the pump"
    );

    pass_priority_pair(&mut game);
    assert_eq!(game.stack.len(), 1);
    assert_eq!(stats(&game, source), (6, 6));
    pass_priority_pair(&mut game);
    assert_eq!(
        stats(&game, source),
        (12, 12),
        "the next resolution sees six power"
    );

    game.finish_cleanup();
    assert_eq!(stats(&game, source), (3, 3));
}

#[test]
fn a_combined_stat_change_and_ability_grant_resolve_and_expire_together() {
    let (mut game, source) = staged(cards::LEAPING_LIZARD);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    activate(&mut game, source);
    assert_eq!(stats(&game, source), (2, 3));
    assert!(!game.has_flying(permanent(&game, source)));

    pass_priority_pair(&mut game);
    assert_eq!(stats(&game, source), (2, 2));
    assert!(game.has_flying(permanent(&game, source)));

    game.finish_cleanup();
    assert_eq!(stats(&game, source), (2, 3));
    assert!(!game.has_flying(permanent(&game, source)));
}
