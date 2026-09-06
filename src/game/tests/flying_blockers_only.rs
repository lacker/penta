//! "Can't be blocked except by creatures with flying" is the fear shape with
//! a keyword in the predicate rather than a colour, so what needs covering is
//! that the exception reads the blocker's current flying rather than its
//! printed text: a creature that gained flying gets through the restriction.

use super::*;

/// Treetop Rangers attacking, with `blocker` on the other side.
fn combat(blocker: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut rangers = creature(34_000, cards::TREETOP_RANGERS, PlayerId::One);
    rangers.attacking = true;
    rangers.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let rangers_id = rangers.card.id;
    let mut defender = creature(34_001, blocker, PlayerId::Two);
    defender.entered_controller_turn = 0;
    let defender_id = defender.card.id;
    game.battlefield.push(rangers);
    game.battlefield.push(defender);
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    (game, rangers_id, defender_id)
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
fn a_ground_creature_cannot_block_it() {
    let (game, rangers, blocker) = combat(cards::SAVANNAH_LIONS);
    assert!(!can_block(&game, blocker, rangers));
}

#[test]
fn a_printed_flier_can_block_it() {
    let (game, rangers, blocker) = combat(cards::SERRA_ANGEL);
    assert!(can_block(&game, blocker, rangers));
}

#[test]
fn a_creature_that_gained_flying_can_block_it() {
    let (mut game, rangers, blocker) = combat(cards::SAVANNAH_LIONS);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == blocker)
        .expect("the blocker is on the battlefield")
        .temporary_keywords
        .push(KeywordAbility::Flying);

    assert!(
        can_block(&game, blocker, rangers),
        "the exception asks what flies now, not what was printed"
    );
}
