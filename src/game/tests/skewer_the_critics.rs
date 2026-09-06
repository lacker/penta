//! Spectacle is an alternative cost gated on a board condition, so what
//! matters is when the cheap cast is offered at all -- and that it turns on
//! for any life loss, not only for combat damage.

use super::*;

/// Player One holding Skewer the Critics with one red mana and nothing else.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    let skewer = card(26_000, cards::SKEWER_THE_CRITICS, PlayerId::One);
    let id = skewer.id;
    game.players[0].hand.push(skewer);
    (game, id)
}

fn castable(game: &Game, card: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).into_iter().any(
        |action| matches!(action, Action::CastSpell { card: candidate, .. } if candidate == card),
    )
}

#[test]
fn one_red_mana_does_not_cast_it_before_any_life_is_lost() {
    let (game, skewer) = staged();
    assert!(
        !castable(&game, skewer),
        "without spectacle it costs three mana, which one red does not pay"
    );
}

#[test]
fn losing_life_any_way_at_all_turns_the_spectacle_cost_on() {
    let (mut game, skewer) = staged();
    assert!(!castable(&game, skewer));

    // Not combat damage: spectacle asks only that an opponent lost life.
    game.lose_life(PlayerId::Two, 1);

    assert!(
        castable(&game, skewer),
        "an opponent having lost life offers the one-mana cast"
    );
}

#[test]
fn the_controllers_own_life_loss_does_not_count() {
    let (mut game, skewer) = staged();
    game.lose_life(PlayerId::One, 3);

    assert!(
        !castable(&game, skewer),
        "spectacle reads opponents' life loss, not the caster's"
    );
}
