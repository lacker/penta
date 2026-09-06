//! Sliver clauses that speak about every Sliver on the table, not only the
//! ones you control. Both halves are easy to write from one seat and be
//! wrong: a trigger that pays "its controller" must pay whoever owns the
//! Sliver that connected, and a grant written for all Slivers has to reach
//! the opponent's as well.

use super::*;

/// Player two attacking with a Sliver, with `mine` under player one, at the
/// point where combat damage has been dealt.
fn attacked_by_a_sliver(mine: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::Two.index()] = 5;
    for (index, definition) in mine.iter().enumerate() {
        let mut permanent = creature(
            76_000 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::One,
        );
        permanent.entered_controller_turn = 0;
        game.battlefield.push(permanent);
    }
    let mut attacker = creature(76_100, cards::MNEMONIC_SLIVER, PlayerId::Two);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    game.battlefield.push(attacker);
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    game.step = Step::CombatDamage;
    game.begin_combat_damage_assignment();
    take_default_combat_assignment(&mut game);
    for _ in 0..12 {
        drain_pending(&mut game);
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
    game
}

#[test]
fn without_the_essence_sliver_nobody_gains() {
    let game = attacked_by_a_sliver(&[]);
    assert_eq!(
        (game.players[0].life, game.players[1].life),
        (18, 20),
        "two damage and nothing else"
    );
}

#[test]
fn the_essence_sliver_pays_the_slivers_own_controller() {
    let game = attacked_by_a_sliver(&[cards::ESSENCE_SLIVER]);
    assert_eq!(
        (game.players[0].life, game.players[1].life),
        (18, 22),
        "the Sliver that connected was theirs, so the life is theirs -- \
         even though the Essence Sliver is mine"
    );
}

/// Whether player two's `blocker` may block player one's attacking Sliver,
/// with `mine` also under player one.
fn can_block(mine: &[CardDefinitionId], blocker: CardDefinitionId) -> bool {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    for (index, definition) in mine.iter().enumerate() {
        let mut permanent = creature(
            76_200 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::One,
        );
        permanent.entered_controller_turn = 0;
        game.battlefield.push(permanent);
    }
    let mut attacker = creature(76_300, cards::MNEMONIC_SLIVER, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let mut theirs = creature(76_301, blocker, PlayerId::Two);
    theirs.entered_controller_turn = 0;
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    game.active_player = PlayerId::One;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = false;
    game.priority = PlayerId::Two;
    game.legal_actions(PlayerId::Two).into_iter().any(|action| {
        matches!(
            action,
            Action::DeclareBlocker { blocker: b, attacker: a } if b == theirs_id && a == attacker_id
        )
    })
}

#[test]
fn a_bear_blocks_a_sliver_ordinarily() {
    assert!(
        can_block(&[], cards::GRIZZLY_BEARS),
        "nothing stops it without the Shifting Sliver"
    );
}

#[test]
fn the_shifting_sliver_leaves_only_slivers_able_to_block() {
    assert!(
        !can_block(&[cards::SHIFTING_SLIVER], cards::GRIZZLY_BEARS),
        "a Bears is not a Sliver"
    );
    assert!(
        can_block(&[cards::SHIFTING_SLIVER], cards::CRYPT_SLIVER),
        "but the opponent's own Sliver still may"
    );
}

/// The mirror: player two attacks with a Sliver while player one controls
/// `mine`, and player one tries to block with a Grizzly Bears.
fn can_block_their_sliver(mine: &[CardDefinitionId]) -> bool {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::Two.index()] = 5;
    for (index, definition) in mine.iter().enumerate() {
        let mut permanent = creature(
            76_400 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::One,
        );
        permanent.entered_controller_turn = 0;
        game.battlefield.push(permanent);
    }
    let mut bear = creature(76_500, cards::GRIZZLY_BEARS, PlayerId::One);
    bear.entered_controller_turn = 0;
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let mut attacker = creature(76_501, cards::MNEMONIC_SLIVER, PlayerId::Two);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = false;
    game.priority = PlayerId::One;
    game.legal_actions(PlayerId::One).into_iter().any(|action| {
        matches!(
            action,
            Action::DeclareBlocker { blocker: b, attacker: a } if b == bear_id && a == attacker_id
        )
    })
}

#[test]
fn the_shifting_sliver_protects_the_opponents_slivers_too() {
    assert!(
        can_block_their_sliver(&[]),
        "the Bears blocks an ordinary attacking Sliver"
    );
    assert!(
        !can_block_their_sliver(&[cards::SHIFTING_SLIVER]),
        "and stops once my own Shifting Sliver is out, because the clause          says Slivers rather than Slivers you control"
    );
}
