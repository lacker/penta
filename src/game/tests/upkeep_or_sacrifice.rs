//! "At the beginning of your upkeep, sacrifice this unless you pay" -- the
//! rent a cheap oversized body charges every turn. Both branches matter: the
//! creature stays only if the payment is actually made, and it goes if the
//! player declines or cannot pay at all.

use super::*;

/// School of Piranha under player one at the start of its controller's
/// upkeep, with `lands` Islands untapped to pay the {1}{U} with.
fn upkeep(lands: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut school = creature(43_000, cards::SCHOOL_OF_PIRANHA, PlayerId::One);
    school.entered_controller_turn = 0;
    let school_id = school.card.id;
    game.battlefield.push(school);
    for index in 0..lands {
        let mut island = creature(
            43_100 + u32::try_from(index).expect("a small fixture"),
            cards::ISLAND,
            PlayerId::One,
        );
        island.entered_controller_turn = 0;
        game.battlefield.push(island);
    }
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    game.resolve_stack_top();
    (game, school_id)
}

fn is_asked(game: &Game) -> bool {
    game.observe(PlayerId::One).decision.is_some()
}

fn survives(game: &Game, school: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == school)
}

#[test]
fn paying_the_upkeep_keeps_it() {
    let (mut game, school) = upkeep(2);
    assert!(is_asked(&game), "the controller is asked whether to pay");
    choose_decision_by_label(&mut game, PlayerId::One, "Pay the cost");
    drain_pending(&mut game);
    assert!(survives(&game, school), "the rent was paid");
}

#[test]
fn declining_sacrifices_it() {
    let (mut game, school) = upkeep(2);
    choose_decision_by_label(&mut game, PlayerId::One, "Decline");
    drain_pending(&mut game);
    assert!(
        !survives(&game, school),
        "declining is the branch that eats the creature"
    );
    assert_eq!(game.players[0].graveyard.len(), 1);
}

#[test]
fn with_no_mana_it_cannot_be_kept() {
    let (mut game, school) = upkeep(0);
    assert!(
        !is_asked(&game),
        "with no way to pay there is nothing to ask about"
    );
    drain_pending(&mut game);
    assert!(
        !survives(&game, school),
        "nothing to pay with is the same as not paying"
    );
}
