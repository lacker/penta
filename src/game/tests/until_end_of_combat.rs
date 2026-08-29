//! Effects that last only for one combat.
//!
//! This is the shortest duration the engine has: it expires as the
//! end-of-combat step finishes rather than waiting for cleanup, so a creature
//! pumped for one combat is back to its printed size in the postcombat main
//! phase -- and an extra combat later in the turn starts it over.

use super::*;

/// Murk Dwellers attacking unopposed, with its trigger already resolved.
/// Blocking is what settles "isn't blocked", so this drives blockers rather
/// than stopping once attackers are declared.
fn unblocked_murk_dwellers() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let dwellers = creature(10_000, cards::MURK_DWELLERS, PlayerId::One);
    let dwellers_id = dwellers.card.id;
    game.battlefield.push(dwellers);

    game.active_player = PlayerId::One;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    let dwellers_permanent = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == dwellers_id)
        .expect("just pushed");
    dwellers_permanent.attacking = true;
    dwellers_permanent.attack_defender = Some(AttackDefender::Player(PlayerId::Two));

    game.finish_declaring_blockers();
    drain_pending(&mut game);
    (game, dwellers_id)
}

fn power(game: &Game, permanent: GameObjectId) -> Option<i16> {
    game.battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .and_then(|candidate| game.power(candidate))
}

#[test]
fn the_pump_lands_while_combat_is_still_on() {
    let (game, dwellers) = unblocked_murk_dwellers();
    assert_eq!(power(&game, dwellers), Some(4), "a 2/2 became a 4/2");
}

/// The half an until-end-of-turn implementation would get wrong: the bonus
/// is gone once combat ends, not at cleanup.
#[test]
fn the_pump_ends_with_combat_rather_than_the_turn() {
    let (mut game, dwellers) = unblocked_murk_dwellers();

    while game.step != Step::PostcombatMain {
        game.advance_step();
        drain_pending(&mut game);
    }

    assert_eq!(
        power(&game, dwellers),
        Some(2),
        "back to its printed size before the turn is over"
    );
}
