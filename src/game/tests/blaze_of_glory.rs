//! One creature made to block the whole attack.
//!
//! Two rules at once, and the pair is the card: the ceiling on how many
//! attackers it may block comes off, and a requirement to use that ceiling
//! goes on. Either alone would be a different card -- permission without the
//! requirement is Two-Headed Giant of Foriys, and a requirement without the
//! permission would stop after one block.

use super::*;

/// Player one attacking with `attackers` Sedge Trolls, player two holding one
/// blocker, and Blaze of Glory in player one's hand.
fn attack_into_one_blocker(attackers: u32) -> (Game, GameObjectId, CardInstanceId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = true;
    for index in 0..attackers {
        let mut attacker = creature(10_000 + index, cards::SEDGE_TROLL, PlayerId::One);
        attacker.attacking = true;
        attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        game.battlefield.push(attacker);
    }
    let blocker = creature(20_000, cards::SERRA_ANGEL, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);

    let spell = card(30_000, cards::BLAZE_OF_GLORY, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.priority = PlayerId::One;
    (game, blocker_id, spell_id)
}

fn cast_it(game: &mut Game, spell: CardInstanceId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("a defending creature to point at");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(game);
}

fn offered_blocks(game: &Game, blocker: GameObjectId) -> Vec<GameObjectId> {
    game.legal_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker {
                blocker: actual,
                attacker,
            } if actual == blocker => Some(attacker),
            _ => None,
        })
        .collect()
}

fn may_finish(game: &Game) -> bool {
    game.legal_actions(PlayerId::Two)
        .iter()
        .any(|action| matches!(action, Action::FinishDeclaringBlockers))
}

fn block(game: &mut Game, blocker: GameObjectId, attacker: GameObjectId) {
    game.apply(PlayerId::Two, Action::DeclareBlocker { blocker, attacker })
        .expect("the block is legal");
}

#[test]
fn the_blocker_must_take_every_attacker_it_can() {
    let (mut game, blocker, spell) = attack_into_one_blocker(3);
    cast_it(&mut game, spell);
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;

    assert!(
        !may_finish(&game),
        "a block is required before the declaration can end"
    );
    let mut remaining = offered_blocks(&game, blocker);
    assert_eq!(remaining.len(), 3, "and every attacker is on offer");

    while let Some(attacker) = remaining.pop() {
        block(&mut game, blocker, attacker);
    }

    assert!(may_finish(&game), "all three blocked, so nothing is left");
    let blocking = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == blocker)
        .expect("still there")
        .blocking
        .len();
    assert_eq!(blocking, 3, "one creature in front of the whole attack");
}

/// Stopping partway is what the requirement forbids: the second and third
/// blocks are still on offer, so the declaration cannot end.
#[test]
fn it_cannot_stop_after_one_block() {
    let (mut game, blocker, spell) = attack_into_one_blocker(3);
    cast_it(&mut game, spell);
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;

    let first = offered_blocks(&game, blocker)[0];
    block(&mut game, blocker, first);

    assert!(!may_finish(&game));
}

/// The control: without the spell the same creature blocks once and stops,
/// and taking no block at all is fine.
#[test]
fn an_ordinary_blocker_blocks_once_and_may_decline() {
    let (mut game, blocker, _spell) = attack_into_one_blocker(3);
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;

    assert!(may_finish(&game), "declining is always allowed");
    let first = offered_blocks(&game, blocker)[0];
    block(&mut game, blocker, first);

    assert!(
        offered_blocks(&game, blocker).is_empty(),
        "one block is all it gets"
    );
    assert!(may_finish(&game));
}
