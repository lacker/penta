//! Protection whose predicate is a creature type rather than a colour. The
//! keyword is one shape with a predicate inside it, so what needs covering is
//! that a subtype predicate actually filters: the named type cannot block,
//! and everything else still can.

use super::*;

/// Shoreline Raider attacking, with `blocker` on the other side.
fn combat(blocker: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut raider = creature(44_000, cards::SHORELINE_RAIDER, PlayerId::One);
    raider.attacking = true;
    raider.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let raider_id = raider.card.id;
    game.battlefield.push(raider);
    let mut defender = creature(44_001, blocker, PlayerId::Two);
    defender.entered_controller_turn = 0;
    let defender_id = defender.card.id;
    game.battlefield.push(defender);
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    (game, raider_id, defender_id)
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
fn a_kavu_cannot_block_it() {
    let (game, raider, blocker) = combat(cards::ROGUE_KAVU);
    assert!(
        !can_block(&game, blocker, raider),
        "protection from Kavu stops the block"
    );
}

#[test]
fn anything_else_can() {
    let (game, raider, blocker) = combat(cards::GRIZZLY_BEARS);
    assert!(
        can_block(&game, blocker, raider),
        "the predicate names one creature type and nothing wider"
    );
}

#[test]
fn protection_from_artifacts_reads_the_card_type() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut scion = creature(44_010, cards::YAVIMAYA_SCION, PlayerId::One);
    scion.attacking = true;
    scion.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let scion_id = scion.card.id;
    game.battlefield.push(scion);
    let mut thopter = creature(44_011, cards::ORNITHOPTER, PlayerId::Two);
    thopter.entered_controller_turn = 0;
    let thopter_id = thopter.card.id;
    game.battlefield.push(thopter);
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;

    assert!(
        !can_block(&game, thopter_id, scion_id),
        "an artifact creature is inside the predicate whatever its colour"
    );
}
