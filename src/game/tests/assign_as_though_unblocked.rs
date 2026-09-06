//! "You may have this creature assign its combat damage as though it weren't
//! blocked." It is an extra way to divide, not a change to the block: the
//! creature is still blocked, so the blocker still hits back and everything
//! watching a block still saw one. Unlike trample it asks nothing of the
//! blockers, because none of them are assigned any damage at all.

use super::*;

/// Thorn Elemental attacking, blocked by `blockers` Grizzly Bears.
fn combat(blockers: usize) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut elemental = creature(52_000, cards::THORN_ELEMENTAL, PlayerId::One);
    elemental.attacking = true;
    elemental.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let elemental_id = elemental.card.id;
    game.battlefield.push(elemental);
    let mut blocker_ids = Vec::new();
    for index in 0..blockers {
        let mut bear = creature(
            52_100 + u32::try_from(index).expect("a small fixture"),
            cards::GRIZZLY_BEARS,
            PlayerId::Two,
        );
        bear.entered_controller_turn = 0;
        bear.blocking = vec![elemental_id];
        blocker_ids.push(bear.card.id);
        game.battlefield.push(bear);
    }
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    game.step = Step::CombatDamage;
    game.begin_combat_damage_assignment();
    (game, elemental_id, blocker_ids)
}

/// The divisions the attacker's controller may choose between.
fn assignments(game: &Game, attacker: GameObjectId) -> Vec<Vec<(Target, u16)>> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::AssignCombatDamage {
                attacker: actual,
                assignments,
            } if actual == attacker => Some(
                assignments
                    .into_iter()
                    .map(|one| (one.recipient, one.amount))
                    .collect(),
            ),
            _ => None,
        })
        .collect()
}

#[test]
fn everything_may_go_to_the_defending_player() {
    let (game, elemental, blockers) = combat(1);
    let straight_through = vec![
        (Target::Permanent(blockers[0]), 0),
        (Target::Player(PlayerId::Two), 7),
    ];
    assert!(
        assignments(&game, elemental).contains(&straight_through),
        "seven to the player and none to the blocker is on the list"
    );
}

/// Unlike trample, nothing has to be killed first: two blockers get zero
/// each rather than lethal each.
#[test]
fn the_blockers_need_no_lethal_damage_first() {
    let (game, elemental, blockers) = combat(2);
    let straight_through = vec![
        (Target::Permanent(blockers[0]), 0),
        (Target::Permanent(blockers[1]), 0),
        (Target::Player(PlayerId::Two), 7),
    ];
    assert!(assignments(&game, elemental).contains(&straight_through));
}

#[test]
fn an_ordinary_creature_is_offered_no_such_division() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut wurm = creature(52_010, cards::CRAW_WURM, PlayerId::One);
    wurm.attacking = true;
    wurm.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let wurm_id = wurm.card.id;
    game.battlefield.push(wurm);
    let mut bear = creature(52_011, cards::GRIZZLY_BEARS, PlayerId::Two);
    bear.entered_controller_turn = 0;
    bear.blocking = vec![wurm_id];
    game.battlefield.push(bear);
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    game.step = Step::CombatDamage;
    game.begin_combat_damage_assignment();

    assert!(
        assignments(&game, wurm_id).iter().all(|division| division
            .iter()
            .all(|(recipient, _)| !matches!(recipient, Target::Player(_)))),
        "a blocked creature without the clause reaches no player"
    );
}
