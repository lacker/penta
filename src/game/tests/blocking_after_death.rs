//! A block read from a creature that is already dead.
//!
//! "Creatures blocking or blocked by it" has two directions, and only one of
//! them survives its subject. A blocker's own record names what it blocked,
//! so when Abu Ja'far is the attacker the surviving blocker answers for
//! itself. When Abu Ja'far is the blocker, the relationship is recorded on
//! Abu Ja'far -- and the trigger asking is a death trigger, so that record is
//! always last-known by the time it is read.

use super::*;

/// Abu Ja'far in a committed block, on the side `attacking` says.
fn combat(attacking: bool, other: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;

    let mut abu = creature(10_000, cards::ABU_JAFAR, PlayerId::One);
    let mut opponent = creature(10_001, other, PlayerId::Two);
    if attacking {
        abu.attacking = true;
        abu.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        opponent.blocking = vec![abu.card.id];
    } else {
        opponent.attacking = true;
        opponent.attack_defender = Some(AttackDefender::Player(PlayerId::One));
        abu.blocking = vec![opponent.card.id];
    }
    let (abu_id, other_id) = (abu.card.id, opponent.card.id);
    game.battlefield.push(abu);
    game.battlefield.push(opponent);
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    (game, abu_id, other_id)
}

fn survives(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

/// Kills Abu Ja'far outright and lets the trigger resolve.
fn destroy_abu(game: &mut Game, abu: GameObjectId) {
    game.damage_target_from(None, Some(Target::Permanent(abu)), 5);
    game.check_state_based_actions();
    drain_pending(game);
}

/// Abu Ja'far blocking: the relationship lives on Abu Ja'far, and Abu Ja'far
/// is gone, so this is the direction that needs last-known information.
#[test]
fn what_it_blocked_dies_with_it() {
    let (mut game, abu, attacker) = combat(false, cards::SEDGE_TROLL);
    destroy_abu(&mut game, abu);

    assert!(!survives(&game, abu));
    assert!(!survives(&game, attacker), "the creature it blocked dies");
}

/// Abu Ja'far attacking: the blocker records what it blocked and is still
/// there to say so, so this direction reads it directly.
#[test]
fn what_blocked_it_dies_with_it() {
    let (mut game, abu, blocker) = combat(true, cards::SEDGE_TROLL);
    destroy_abu(&mut game, abu);

    assert!(!survives(&game, abu));
    assert!(
        !survives(&game, blocker),
        "the creature that blocked it dies"
    );
}

/// Only the creature it met. A bystander in the same combat is untouched.
#[test]
fn a_creature_from_another_block_survives() {
    let (mut game, abu, blocker) = combat(true, cards::SEDGE_TROLL);
    let bystander = creature(10_002, cards::SEDGE_TROLL, PlayerId::Two);
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);

    destroy_abu(&mut game, abu);

    assert!(!survives(&game, blocker));
    assert!(survives(&game, bystander_id));
}

/// Dying outside combat kills nothing, because there is no block to read.
#[test]
fn dying_out_of_combat_takes_nobody_with_it() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let abu = creature(10_000, cards::ABU_JAFAR, PlayerId::One);
    let abu_id = abu.card.id;
    game.battlefield.push(abu);
    let bystander = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);

    destroy_abu(&mut game, abu_id);

    assert!(!survives(&game, abu_id));
    assert!(survives(&game, bystander_id));
}
