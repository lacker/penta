//! A mana ability that pays life and produces a colour of the payer's
//! choosing. The planner is the only thing that activates a mana ability, and
//! it declines any whose cost is itself mana -- so what needs covering is
//! that a life cost is not treated the same way: the Lens must actually be
//! reached, and reaching it must cost the life.

use super::*;

/// Ancestral Recall, which costs {U}, in hand with `lenses` Phyrexian Lenses
/// as the only mana on the battlefield.
fn staged(lenses: usize) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].mana_pool = ManaPool::default();
    let spell = card(47_000, cards::ANCESTRAL_RECALL, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    for index in 0..lenses {
        let mut lens = creature(
            47_100 + u32::try_from(index).expect("a small fixture"),
            cards::PHYREXIAN_LENS,
            PlayerId::One,
        );
        lens.entered_controller_turn = 0;
        game.battlefield.push(lens);
    }
    (game, spell_id)
}

fn cast_of(game: &Game, spell: CardInstanceId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
}

#[test]
fn without_the_lens_there_is_no_blue() {
    let (game, spell) = staged(0);
    assert!(cast_of(&game, spell).is_none());
}

#[test]
fn the_lens_makes_the_colour_and_charges_the_life() {
    let (mut game, spell) = staged(1);
    let cast = cast_of(&game, spell).expect("the Lens can make the {U}");
    game.apply(PlayerId::One, cast)
        .expect("a tap and a life pay for it");

    assert_eq!(game.players[0].life, 19, "one life for one mana");
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == GameObjectId(47_100))
            .expect("the Lens is still on the battlefield")
            .tapped,
        "and the tap went with it"
    );
    assert_eq!(game.stack.len(), 1);
}
