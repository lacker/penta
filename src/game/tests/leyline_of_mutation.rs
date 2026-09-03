//! Leyline of Mutation's five-colored battlefield-granted alternative cost.

use super::*;

fn casts_of(game: &Game, spell: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .collect()
}

#[test]
fn leyline_of_mutation_offers_and_charges_its_five_color_alternative() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(121_100, cards::LEYLINE_OF_MUTATION, PlayerId::One));
    let spell = card(121_101, cards::OMNISCIENCE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);

    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    assert!(
        casts_of(&game, spell_id).is_empty(),
        "four colors cannot pay Leyline's alternative"
    );

    game.players[PlayerId::One.index()].mana_pool.green = 1;
    let casts = casts_of(&game, spell_id);
    assert_eq!(casts.len(), 1, "the printed ten-mana cost is not payable");
    assert!(matches!(
        &casts[0],
        Action::CastSpell { choices, .. } if choices.costs().alternative().is_some()
    ));

    game.apply(PlayerId::One, casts.into_iter().next().unwrap())
        .expect("one mana of each color pays Leyline's alternative");
    let pool = game.players[PlayerId::One.index()].mana_pool;
    assert_eq!(pool.white, 0);
    assert_eq!(pool.blue, 0);
    assert_eq!(pool.black, 0);
    assert_eq!(pool.red, 0);
    assert_eq!(pool.green, 0);
    assert_eq!(
        game.stack.last().unwrap().card.definition,
        ObjectKind::Card(cards::OMNISCIENCE)
    );
}
