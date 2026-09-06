//! Who a damage spell reaches. "Each player" includes the caster and "each
//! opponent" does not, which is the whole difference between Flame Rift and
//! Sizzle at nearly the same cost; and Steam Blast reaches the board and
//! both players at once. A spell that hit the wrong set would still deal
//! the right number to somebody.

use super::*;

fn cast_no_target(game: &mut Game, spell: CardInstanceId) {
    let action = cast_action(spell, Vec::new(), Vec::new(), 0);
    assert!(
        game.legal_actions(PlayerId::One).contains(&action),
        "the spell is castable"
    );
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(game);
    game.check_state_based_actions();
}

fn staged(spell: CardDefinitionId) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let card = card(69_000, spell, PlayerId::One);
    let id = card.id;
    game.players[0].hand.push(card);
    game.players[0].mana_pool.red = 2;
    game.players[0].mana_pool.colorless = 3;
    (game, id)
}

#[test]
fn flame_rift_hits_its_own_caster_too() {
    let (mut game, spell) = staged(cards::FLAME_RIFT);
    cast_no_target(&mut game, spell);
    assert_eq!(
        (game.players[0].life, game.players[1].life),
        (16, 16),
        "four to each player, the caster included"
    );
}

#[test]
fn sizzle_spares_its_own_caster() {
    let (mut game, spell) = staged(cards::SIZZLE);
    cast_no_target(&mut game, spell);
    assert_eq!(
        (game.players[0].life, game.players[1].life),
        (20, 17),
        "three to the opponent only"
    );
}

#[test]
fn steam_blast_reaches_the_board_and_both_players() {
    let (mut game, spell) = staged(cards::STEAM_BLAST);
    game.battlefield
        .push(creature(69_100, cards::GRIZZLY_BEARS, PlayerId::One));
    game.battlefield
        .push(creature(69_101, cards::SAVANNAH_LIONS, PlayerId::Two));
    game.battlefield
        .push(creature(69_102, cards::SERRA_ANGEL, PlayerId::Two));
    cast_no_target(&mut game, spell);

    assert_eq!((game.players[0].life, game.players[1].life), (18, 18));
    let alive: Vec<_> = game
        .battlefield
        .iter()
        .map(|permanent| permanent.card.id)
        .collect();
    assert_eq!(
        alive,
        vec![GameObjectId(69_102)],
        "the 4/4 survives two damage and both smaller creatures do not"
    );
}
