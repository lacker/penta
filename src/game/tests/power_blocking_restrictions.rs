//! "Can't block creatures with power 2 or greater."
//!
//! Authored as the permission it leaves behind rather than as a prohibition,
//! which is what the blocking vocabulary already had. The power it reads is
//! the attacker's current power, so a pump changes the answer -- the question
//! is asked while blockers are declared, long after static effects settle.

use super::*;

/// One attacker for player one and `blocker` for player two, in the blocker
/// step.
fn combat(attacker: CardDefinitionId, blocker: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacking = creature(10_000, attacker, PlayerId::One);
    attacking.attacking = true;
    attacking.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield.push(attacking);
    let defending = creature(10_001, blocker, PlayerId::Two);
    let blocker_id = defending.card.id;
    game.battlefield.push(defending);
    (game, blocker_id)
}

fn can_block(game: &Game, blocker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::Two).iter().any(
        |action| matches!(action, Action::DeclareBlocker { blocker: actual, .. } if *actual == blocker),
    )
}

#[test]
fn the_orcs_block_something_small() {
    // Mons's Goblin Raiders is a 1/1.
    let (game, orcs) = combat(cards::MONSS_GOBLIN_RAIDERS, cards::IRONCLAW_ORCS);
    assert!(can_block(&game, orcs));
}

#[test]
fn the_orcs_will_not_block_something_big() {
    let (game, orcs) = combat(cards::SEDGE_TROLL, cards::IRONCLAW_ORCS);
    assert!(!can_block(&game, orcs), "a 2/2 is already too much");
}

/// Brassclaw Orcs prints the same restriction, and reads it the same way.
#[test]
fn the_brassclaws_read_the_same_line() {
    let (game, orcs) = combat(cards::MONSS_GOBLIN_RAIDERS, cards::BRASSCLAW_ORCS);
    assert!(can_block(&game, orcs));

    let (game, orcs) = combat(cards::SEDGE_TROLL, cards::BRASSCLAW_ORCS);
    assert!(!can_block(&game, orcs));
}

/// The power is read live, including the continuous statics. A white 1/1 the
/// Orcs would happily block becomes one they cannot the moment a Crusade
/// lands -- and nothing about either creature was touched.
#[test]
fn a_statically_pumped_attacker_becomes_too_big() {
    let (mut game, orcs) = combat(cards::ICATIAN_JAVELINEERS, cards::IRONCLAW_ORCS);
    assert!(can_block(&game, orcs), "a 1/1 is small enough");

    game.battlefield
        .push(creature(10_002, cards::CRUSADE, PlayerId::One));

    assert!(
        !can_block(&game, orcs),
        "the same creature, one point bigger"
    );
}

/// The Veteran's restriction names a colour as well, so a big red creature
/// is still blockable.
#[test]
fn the_veteran_only_fears_big_white_creatures() {
    let (game, veteran) = combat(cards::SEDGE_TROLL, cards::ORCISH_VETERAN);
    assert!(can_block(&game, veteran), "a 2/2, but not white");

    let (game, veteran) = combat(cards::SERRA_ANGEL, cards::ORCISH_VETERAN);
    assert!(!can_block(&game, veteran), "white and far too big");

    let (game, veteran) = combat(cards::SAVANNAH_LIONS, cards::ORCISH_VETERAN);
    assert!(
        !can_block(&game, veteran),
        "a white 2/1 is exactly what it refuses"
    );
}
