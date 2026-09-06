//! Handing a keyword to another creature for the turn. Haste is the one
//! whose effect is visible without combat maths: a creature that arrived
//! this turn cannot be declared as an attacker, and the same creature can
//! once the grant resolves. The grant is also for this turn only, which is
//! what separates it from a printed keyword.

use super::*;

/// Battle Rampart and a Grizzly Bears that arrived this turn, in the
/// attacking player's declare-attackers step.
fn staged() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let mut rampart = creature(60_000, cards::BATTLE_RAMPART, PlayerId::One);
    rampart.entered_controller_turn = 0;
    let rampart_id = rampart.card.id;
    game.battlefield.push(rampart);
    // Summoning sick: it entered on the turn now being played.
    let mut bear = creature(60_001, cards::GRIZZLY_BEARS, PlayerId::One);
    bear.entered_controller_turn = 2;
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    game.players[0].mana_pool.red = 1;
    game.turns_started = [2, 1];
    game.active_player = PlayerId::One;
    // Abilities are used before attackers are declared: the declare-attackers
    // step offers only declarations until the attack is locked in.
    game.step = Step::BeginningOfCombat;
    game.attackers_declared = false;
    game.priority = PlayerId::One;
    (game, rampart_id, bear_id)
}

fn can_attack(game: &mut Game, attacker: GameObjectId) -> bool {
    game.step = Step::DeclareAttackers;
    game.priority = PlayerId::One;
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::DeclareAttacker { attacker: a, .. } if *a == attacker),
    )
}

#[test]
fn a_creature_that_just_arrived_cannot_attack() {
    let (mut game, _, bear) = staged();
    assert!(
        !can_attack(&mut game, bear),
        "summoning sickness holds it back"
    );
}

#[test]
fn the_rampart_hands_it_haste_for_the_turn() {
    let (mut game, rampart, bear) = staged();
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == rampart
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(bear))
            }
            _ => false,
        })
        .expect("the Bears are a legal target for the grant");
    game.apply(PlayerId::One, activation)
        .expect("one red pays for it");
    pass_priority_pair(&mut game);

    assert!(
        can_attack(&mut game, bear),
        "with haste the same creature may be declared"
    );
    assert!(
        !can_attack(&mut game, rampart),
        "and the Rampart still has defender, so it cannot come along"
    );
}
