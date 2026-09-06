//! Two clauses in one resolution. Zap deals damage and then draws, and both
//! halves have to happen off a single target choice; Cloudchaser Eagle's
//! enters trigger needs an enchantment to point at, and a board with none is
//! the case where a required target could go wrong rather than be absent.

use super::*;

#[test]
fn zap_deals_its_damage_and_draws_the_card() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let zap = card(53_000, cards::ZAP, PlayerId::One);
    game.players[0].hand.push(zap.clone());
    game.players[0].mana_pool.red = 3;
    let before = game.players[0].library.len();

    let cast = cast_action(zap.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 19, "one damage to the opponent");
    assert_eq!(game.players[0].hand.len(), 1, "and a card for the caster");
    assert_eq!(
        game.players[0].library.len(),
        before - 1,
        "which came off the caster's library, not the opponent's"
    );
}

/// The enters trigger names a required target, so what matters on a board
/// with no enchantment is that nothing goes wrong: the Eagle still arrives.
#[test]
fn the_eagle_arrives_even_with_nothing_to_destroy() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let eagle = card(53_010, cards::CLOUDCHASER_EAGLE, PlayerId::One);
    game.players[0].hand.push(eagle.clone());
    game.players[0].mana_pool.white = 4;

    let cast = cast_action(eagle.id, Vec::new(), Vec::new(), 0);
    assert!(
        game.legal_actions(PlayerId::One).contains(&cast),
        "an empty board does not stop the creature being cast"
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield.len(),
        1,
        "the Eagle is on the battlefield with its trigger simply gone"
    );
}

#[test]
fn the_eagle_destroys_the_enchantment_it_names() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.battlefield
        .push(creature(53_020, cards::IVORY_MASK, PlayerId::Two));
    let eagle = card(53_010, cards::CLOUDCHASER_EAGLE, PlayerId::One);
    game.players[0].hand.push(eagle.clone());
    game.players[0].mana_pool.white = 4;

    let cast = cast_action(eagle.id, Vec::new(), Vec::new(), 0);
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == CardInstanceId(53_020)),
        "the only enchantment was the only legal target"
    );
}
