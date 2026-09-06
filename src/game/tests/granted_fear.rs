//! Granting fear. The catalog has no fear keyword flag -- fear is only ever
//! the "except by artifact or black creatures" pairing restriction -- so a
//! card that hands it out applies that restriction rather than granting the
//! static ability, which would have nothing to execute. What needs covering
//! is that the granted form actually restricts blocks the same way the
//! printed keyword does, and that it stops when the duration says.

use super::*;

/// Hooded Kavu attacking, with `blocker` opposite it and a black mana up.
fn combat(blocker: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut kavu = creature(64_000, cards::HOODED_KAVU, PlayerId::One);
    kavu.attacking = true;
    kavu.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    kavu.entered_controller_turn = 0;
    let kavu_id = kavu.card.id;
    game.battlefield.push(kavu);
    let mut defender = creature(64_001, blocker, PlayerId::Two);
    defender.entered_controller_turn = 0;
    let defender_id = defender.card.id;
    game.battlefield.push(defender);
    game.players[0].mana_pool.black = 1;
    game.step = Step::BeginningOfCombat;
    game.attackers_declared = true;
    game.priority = PlayerId::One;
    (game, kavu_id, defender_id)
}

fn can_block(game: &mut Game, blocker: GameObjectId, attacker: GameObjectId) -> bool {
    game.step = Step::DeclareBlockers;
    game.legal_actions(PlayerId::Two).into_iter().any(|action| {
        matches!(
            action,
            Action::DeclareBlocker { blocker: b, attacker: a } if b == blocker && a == attacker
        )
    })
}

fn grant_fear(game: &mut Game, kavu: GameObjectId) {
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == kavu))
        .expect("one black pays for it");
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(game);
}

#[test]
fn without_the_grant_anything_may_block() {
    let (mut game, kavu, bear) = combat(cards::GRIZZLY_BEARS);
    assert!(
        can_block(&mut game, bear, kavu),
        "a green creature blocks an ordinary Kavu"
    );
}

#[test]
fn the_granted_restriction_stops_a_green_blocker() {
    let (mut game, kavu, bear) = combat(cards::GRIZZLY_BEARS);
    grant_fear(&mut game, kavu);
    assert!(
        !can_block(&mut game, bear, kavu),
        "the same block is now illegal"
    );
}

#[test]
fn a_black_creature_still_gets_through_the_restriction() {
    let (mut game, kavu, rats) = combat(cards::RAZORTOOTH_RATS);
    grant_fear(&mut game, kavu);
    assert!(
        can_block(&mut game, rats, kavu),
        "fear lets black creatures block, granted or printed"
    );
}

#[test]
fn an_artifact_creature_still_gets_through_it() {
    let (mut game, kavu, thopter) = combat(cards::ORNITHOPTER);
    grant_fear(&mut game, kavu);
    assert!(can_block(&mut game, thopter, kavu));
}
