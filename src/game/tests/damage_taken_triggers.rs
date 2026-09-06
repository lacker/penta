//! "Whenever this creature is dealt combat damage, you gain that much
//! life", and the creatures that simply refuse to block. The first reads an
//! amount off an event whose source is somebody else's creature, which is
//! the opposite direction from every other damage trigger in the catalog;
//! the second is a prohibition a permanent puts on itself.

use super::*;

/// A Grizzly Bears of player two's attacking, blocked by player one's
/// `blocker`, with combat damage already dealt. The Bears is small enough
/// that a four-toughness blocker survives to be looked at afterwards.
fn blocked_by(blocker: CardDefinitionId) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::Two.index()] = 5;
    let mut attacker = creature(72_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let mut wall = creature(72_001, blocker, PlayerId::One);
    wall.entered_controller_turn = 0;
    wall.blocking = vec![attacker_id];
    game.battlefield.push(wall);
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    game.step = Step::CombatDamage;
    game.begin_combat_damage_assignment();
    take_default_combat_assignment(&mut game);
    // The damage event is captured as it is dealt; a round of priority is
    // what turns the capture into a trigger on the stack, and another
    // resolves it.
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
fn a_plain_wall_gains_nothing() {
    let game = blocked_by(cards::WALL_OF_WOOD);
    assert_eq!(
        game.players[0].life, 20,
        "blocking is not by itself lifegain"
    );
}

#[test]
fn the_wall_gains_what_it_was_dealt() {
    let game = blocked_by(cards::WALL_OF_ESSENCE);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(72_001)),
        "the Wall survived the damage it read"
    );
    assert_eq!(
        game.players[0].life, 22,
        "the Bears' two power came back as two life"
    );
}

/// Whether `blocker` may be declared against player two's attacker.
fn can_block(blocker: CardDefinitionId) -> bool {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::Two.index()] = 5;
    let mut attacker = creature(72_100, cards::GRIZZLY_BEARS, PlayerId::Two);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let mut mine = creature(72_101, blocker, PlayerId::One);
    mine.entered_controller_turn = 0;
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = false;
    game.priority = PlayerId::One;
    game.legal_actions(PlayerId::One).into_iter().any(|action| {
        matches!(
            action,
            Action::DeclareBlocker { blocker: b, attacker: a } if b == mine_id && a == attacker_id
        )
    })
}

#[test]
fn the_askari_refuses_to_block() {
    assert!(
        can_block(cards::GRIZZLY_BEARS),
        "an ordinary creature blocks"
    );
    assert!(
        !can_block(cards::FALLEN_ASKARI),
        "and the Askari never does"
    );
}
