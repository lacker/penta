//! Fear as a pairing restriction: an artifact creature or a black creature
//! may block, and nothing else may. Written declaratively rather than as a
//! keyword flag, so what needs covering is that the "except by" predicate
//! admits exactly those two kinds.

use super::*;

/// Razortooth Rats attacking, with `blocker` on the other side.
fn combat(blocker: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut rats = creature(27_000, cards::RAZORTOOTH_RATS, PlayerId::One);
    rats.attacking = true;
    rats.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let rats_id = rats.card.id;
    let mut wall = creature(27_001, blocker, PlayerId::Two);
    wall.entered_controller_turn = 0;
    let wall_id = wall.card.id;
    game.battlefield.push(rats);
    game.battlefield.push(wall);
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    (game, rats_id, wall_id)
}

fn can_block(game: &Game, blocker: GameObjectId, attacker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::Two).into_iter().any(|action| {
        matches!(
            action,
            Action::DeclareBlocker { blocker: b, attacker: a } if b == blocker && a == attacker
        )
    })
}

#[test]
fn an_ordinary_white_creature_cannot_block_it() {
    let (game, rats, blocker) = combat(cards::SAVANNAH_LIONS);
    assert!(
        !can_block(&game, blocker, rats),
        "a white nonartifact creature is outside both halves of the predicate"
    );
}

#[test]
fn a_black_creature_can_block_it() {
    let (game, rats, blocker) = combat(cards::RAZORTOOTH_RATS);
    assert!(
        can_block(&game, blocker, rats),
        "a black creature blocks through fear"
    );
}

#[test]
fn a_colorless_artifact_creature_can_block_it() {
    let (game, rats, blocker) = combat(cards::ORNITHOPTER);
    assert!(
        can_block(&game, blocker, rats),
        "an artifact creature blocks through fear whatever its colour"
    );
}
