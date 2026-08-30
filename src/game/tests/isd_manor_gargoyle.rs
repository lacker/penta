//! Manor Gargoyle's defender-dependent indestructibility and temporary flight.

use super::*;

fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    let gargoyle = game
        .put_onto_battlefield(PlayerId::One, cards::MANOR_GARGOYLE)
        .expect("Manor Gargoyle is cataloged");
    drain_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    (game, gargoyle)
}

fn gargoyle(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("Manor Gargoyle remains on the battlefield")
}

fn has_keyword(game: &Game, id: GameObjectId, keyword: KeywordAbility) -> bool {
    game.permanent_has_executable_keyword(gargoyle(game, id), keyword)
}

fn take_flight(game: &mut Game, id: GameObjectId) {
    let activation = plain_activation(id, activated_ability_for(game, id, 0));
    assert!(game.legal_actions(PlayerId::One).contains(&activation));
    game.apply(PlayerId::One, activation)
        .expect("one mana activates Manor Gargoyle");
    pass_priority_pair(game);
}

#[test]
fn activation_exchanges_defender_and_indestructible_for_flying_until_cleanup() {
    let (mut game, id) = staged();

    assert!(has_keyword(&game, id, KeywordAbility::Defender));
    assert!(has_keyword(&game, id, KeywordAbility::Indestructible));
    assert!(!game.has_flying(gargoyle(&game, id)));

    take_flight(&mut game, id);

    assert!(!has_keyword(&game, id, KeywordAbility::Defender));
    assert!(!has_keyword(&game, id, KeywordAbility::Indestructible));
    assert!(game.has_flying(gargoyle(&game, id)));

    game.finish_cleanup();

    assert!(has_keyword(&game, id, KeywordAbility::Defender));
    assert!(has_keyword(&game, id, KeywordAbility::Indestructible));
    assert!(!game.has_flying(gargoyle(&game, id)));
}

#[test]
fn lethal_damage_destroys_it_when_the_activation_removes_indestructible() {
    let (mut game, id) = staged();
    game.damage_target_from(None, Some(Target::Permanent(id)), 4);
    game.check_state_based_actions();
    assert_eq!(gargoyle(&game, id).damage, 4);

    take_flight(&mut game, id);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != id),
        "losing defender also removes indestructible before state-based actions",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MANOR_GARGOYLE)
    );
}
