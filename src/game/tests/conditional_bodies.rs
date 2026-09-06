//! Statics whose condition is read live off the board. Mogg Squad shrinks as
//! creatures arrive and grows back as they leave; Fledgling Osprey and
//! Metathran Elite switch on only while something is attached to them. In
//! every case the condition is checked when the question is asked rather
//! than when the permanent arrived, so the same board reached two ways gives
//! the same answer.

use super::*;

fn board(subject: CardDefinitionId, others: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let mut it = creature(77_000, subject, PlayerId::One);
    it.entered_controller_turn = 0;
    let id = it.card.id;
    game.battlefield.push(it);
    for index in 0..others {
        let mut other = creature(
            77_100 + u32::try_from(index).expect("a small fixture"),
            cards::GRIZZLY_BEARS,
            if index % 2 == 0 {
                PlayerId::Two
            } else {
                PlayerId::One
            },
        );
        other.entered_controller_turn = 0;
        game.battlefield.push(other);
    }
    (game, id)
}

fn stats(game: &Game, id: GameObjectId) -> Option<(i16, i16)> {
    let permanent = game.battlefield.iter().find(|p| p.card.id == id)?;
    Some((game.power(permanent)?, game.toughness(permanent)?))
}

#[test]
fn the_squad_shrinks_by_one_for_each_other_creature() {
    let (game, squad) = board(cards::MOGG_SQUAD, 0);
    assert_eq!(stats(&game, squad), Some((3, 3)), "alone it is a 3/3");

    let (game, squad) = board(cards::MOGG_SQUAD, 1);
    assert_eq!(stats(&game, squad), Some((2, 2)));

    let (mut game, squad) = board(cards::MOGG_SQUAD, 2);
    assert_eq!(stats(&game, squad), Some((1, 1)), "both sides count");

    // Removing one gives the size back, which is what "live" means.
    game.battlefield
        .retain(|permanent| permanent.card.id != GameObjectId(77_101));
    assert_eq!(stats(&game, squad), Some((2, 2)));
}

#[test]
fn the_osprey_flies_only_while_something_is_attached() {
    let (mut game, osprey) = board(cards::FLEDGLING_OSPREY, 0);
    let unenchanted = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == osprey)
        .expect("the Osprey is there");
    assert!(
        !game.has_flying(unenchanted),
        "bare, it stays on the ground"
    );

    let aura = creature(77_200, cards::HERO_S_RESOLVE, PlayerId::One);
    let aura_id = aura.card.id;
    game.battlefield.push(aura);
    assert!(game.try_attach(aura_id, osprey), "the Aura goes on it");
    game.check_state_based_actions();

    let enchanted = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == osprey)
        .expect("the Osprey is still there");
    assert!(
        game.has_flying(enchanted),
        "and with an Aura on it, it flies"
    );
}
