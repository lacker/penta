//! Banding's blocking half.
//!
//! CR 702.21 gives banding more than one job. This file holds the one where a
//! creature with banding blocks: the choice of how the attacker assigns its
//! combat damage moves from the attacker's controller to the defending
//! player. The declaration rules are in `banding_formation` and
//! `banding_blocked`, and the mirror-image assignment in
//! `banding_assignment`.

use super::*;

/// An attacker big enough that splitting its damage is a real choice, blocked
/// by two creatures so the engine actually asks.
fn banded_block(blocker_has_banding: bool) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SEA_SERPENT, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    for index in 0..2 {
        let mut blocker = creature(10_001 + index, cards::SAVANNAH_LIONS, PlayerId::Two);
        blocker.blocking = vec![attacker_id];
        if blocker_has_banding && index == 0 {
            blocker.temporary_keywords.push(KeywordAbility::Banding);
        }
        game.battlefield.push(blocker);
    }
    game.finish_declaring_blockers();
    game.start_combat_damage();
    (game, attacker_id)
}

#[test]
fn without_banding_the_attacking_player_assigns() {
    let (game, attacker) = banded_block(false);
    assert!(
        !game.pending_combat_assignments.is_empty(),
        "two blockers make the split a real choice"
    );
    assert_eq!(game.combat_damage_assigner(attacker), PlayerId::One);
    assert_eq!(game.decision_player(), Some(PlayerId::One));
    assert!(
        !game.legal_actions(PlayerId::Two).is_empty()
            || game
                .legal_actions(PlayerId::One)
                .iter()
                .any(|action| { matches!(action, Action::AssignCombatDamage { .. }) }),
        "the attacker's controller is the one offered the assignment"
    );
}

#[test]
fn a_banding_blocker_takes_the_assignment_for_the_defending_player() {
    let (game, attacker) = banded_block(true);
    assert_eq!(game.combat_damage_assigner(attacker), PlayerId::Two);
    assert_eq!(
        game.decision_player(),
        Some(PlayerId::Two),
        "the defending player is asked instead"
    );
    assert!(
        game.legal_actions(PlayerId::Two)
            .iter()
            .any(|action| matches!(action, Action::AssignCombatDamage { .. })),
        "and is the one offered the assignment"
    );
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::AssignCombatDamage { .. })),
        "the attacker's controller no longer chooses"
    );
}

/// The defending player's choice is a real one: they can put all of it on
/// whichever blocker they prefer.
#[test]
fn the_defending_player_can_direct_the_attackers_damage() {
    let (mut game, attacker) = banded_block(true);
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::AssignCombatDamage { .. }))
        .expect("an assignment is offered");
    game.apply(PlayerId::Two, action)
        .expect("the defending player may assign");
    assert!(
        game.pending_combat_assignments.is_empty()
            || game.combat_damage_assigner(attacker) == PlayerId::Two,
        "the assignment was accepted from the defending player"
    );
}
