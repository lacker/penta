//! A mana ability whose cost is a sacrifice. Mana abilities are never
//! offered as standalone actions -- the planner reaches them while paying
//! for something -- so what needs covering is that the planner will actually
//! spend the source: a cast nothing else can pay for becomes legal, and
//! taking it puts the creature in the graveyard.

use super::*;

/// Goblin Electromancer, which costs {U}{R}, in hand with `toads` Morgue
/// Toads as the only mana on the battlefield.
fn staged(toads: usize) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].mana_pool = ManaPool::default();
    let spell = card(42_000, cards::GOBLIN_ELECTROMANCER, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    for index in 0..toads {
        let mut toad = creature(
            42_100 + u32::try_from(index).expect("a small fixture"),
            cards::MORGUE_TOAD,
            PlayerId::One,
        );
        toad.entered_controller_turn = 0;
        game.battlefield.push(toad);
    }
    (game, spell_id)
}

fn can_cast(game: &Game, spell: CardInstanceId) -> bool {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if card == spell))
}

#[test]
fn without_the_toad_there_is_no_mana_at_all() {
    let (game, spell) = staged(0);
    assert!(
        !can_cast(&game, spell),
        "an empty battlefield pays for nothing"
    );
}

#[test]
fn the_toad_pays_for_a_spell_of_both_its_colours() {
    let (mut game, spell) = staged(1);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the Toad's two mana cover {U}{R}");
    game.apply(PlayerId::One, cast)
        .expect("the sacrifice pays for it");

    assert!(
        game.battlefield.is_empty(),
        "the Toad was sacrificed to make the mana"
    );
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "and it is in the graveyard rather than merely gone"
    );
    assert_eq!(game.stack.len(), 1, "the spell is on the stack");
}
