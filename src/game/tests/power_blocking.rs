//! Blocking restrictions that compare power against the attacker's.
//!
//! "Creatures with power less than this creature's power can't block it" is
//! read live against the source, so pumping the attacker widens the
//! restriction mid-combat rather than being fixed when it attacked.

use super::*;

/// Wandering Wolf attacking, with one prospective blocker of `power`.
fn blocking_board(blocker: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let wolf = creature(10_000, cards::WANDERING_WOLF, PlayerId::One);
    let wolf_id = wolf.card.id;
    game.battlefield.push(wolf);
    let defender = creature(10_001, blocker, PlayerId::Two);
    let defender_id = defender.card.id;
    game.battlefield.push(defender);

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::Two;
    let wolf = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == wolf_id)
        .expect("just pushed");
    wolf.attacking = true;
    wolf.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    (game, wolf_id, defender_id)
}

fn can_block(game: &Game, blocker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::Two).iter().any(
        |action| matches!(action, Action::DeclareBlocker { blocker: actual, .. } if *actual == blocker),
    )
}

#[test]
fn a_weaker_creature_cannot_block_it() {
    // Wandering Wolf is a 2/1. Savannah Lions is 2/1 as well, so it is not
    // weaker and blocks fine.
    let (game, _, lions) = blocking_board(cards::SAVANNAH_LIONS);
    assert!(
        can_block(&game, lions),
        "equal power is not less than, so it may block"
    );

    // Icatian Moneychanger is a 0/2, which is weaker.
    let (game, _, changer) = blocking_board(cards::ICATIAN_MONEYCHANGER);
    assert!(!can_block(&game, changer), "a 0/2 has power less than two");
}

/// The comparison is against current power, so a pump changes the answer.
#[test]
fn pumping_the_attacker_widens_the_restriction() {
    let (mut game, wolf_id, lions) = blocking_board(cards::SAVANNAH_LIONS);
    assert!(can_block(&game, lions));

    attach_constant_resolved_characteristics(
        &mut game,
        wolf_id,
        &[AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(1),
            ValueDef::Constant(0),
        )],
        ContinuousEffectExpiration::Never,
    );

    assert!(
        !can_block(&game, lions),
        "a 3/1 attacker now outclasses the 2/1 blocker"
    );
}

/// The other direction: a restriction on what the blocker may block, rather
/// than on who may block the attacker.
#[test]
fn a_block_only_creature_refuses_ground_attackers() {
    let mut game = ready_game();
    // Gloomwidow can block only creatures with flying.
    let widow = creature(10_000, cards::GLOOMWIDOW, PlayerId::Two);
    let widow_id = widow.card.id;
    game.battlefield.push(widow);
    let ground = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let ground_id = ground.card.id;
    game.battlefield.push(ground);
    let flyer = creature(10_002, cards::SCRAPSKIN_DRAKE, PlayerId::One);
    let flyer_id = flyer.card.id;
    game.battlefield.push(flyer);

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::Two;
    for permanent in &mut game.battlefield {
        if permanent.card.id == ground_id || permanent.card.id == flyer_id {
            permanent.attacking = true;
            permanent.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        }
    }

    let blocks: Vec<_> = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker {
                blocker, attacker, ..
            } if blocker == widow_id => Some(attacker),
            _ => None,
        })
        .collect();
    assert!(
        blocks.contains(&flyer_id),
        "a flyer is what it is allowed to block"
    );
    assert!(!blocks.contains(&ground_id), "and a ground creature is not");
}
