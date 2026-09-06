//! A discard tax attached to somebody else's combat damage. The trigger lives
//! on a permanent that is not in combat at all, so what has to be right is
//! the `sources` predicate: it selects out of the batch by tribe or by
//! controller rather than by being the ability's own source, and a card whose
//! predicate quietly required identity would sit there doing nothing.

use super::*;

/// `watcher` on player one's battlefield while `attacker` connects with
/// player two, who is holding two cards.
fn connect(watcher: CardDefinitionId, attacker: CardDefinitionId) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    for holder in [PlayerId::One, PlayerId::Two] {
        game.players[holder.index()].hand.clear();
        for offset in 0..2 {
            let filler = card(
                64_200 + u32::from(holder == PlayerId::Two) * 8 + offset,
                cards::MOUNTAIN,
                holder,
            );
            game.players[holder.index()].hand.push(filler);
        }
    }
    game.battlefield
        .push(creature(64_000, watcher, PlayerId::One));
    let mut threat = creature(64_001, attacker, PlayerId::One);
    threat.attacking = true;
    threat.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield.push(threat);

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

fn hands(game: &Game) -> (usize, usize) {
    (game.players[0].hand.len(), game.players[1].hand.len())
}

/// The Slaver names a tribe, so it reads a creature it does not control the
/// trigger for and ignores everything else that connects.
#[test]
fn cabal_slaver_taxes_goblin_damage_only() {
    let goblin = connect(cards::CABAL_SLAVER, cards::RAGING_GOBLIN);
    assert_eq!(
        hands(&goblin),
        (2, 1),
        "the Goblin connected, so its victim pitched a card"
    );

    let bear = connect(cards::CABAL_SLAVER, cards::GRIZZLY_BEARS);
    assert_eq!(
        hands(&bear),
        (2, 2),
        "the Bears is not a Goblin, so nothing was owed"
    );
}

/// Larceny drops the tribe and keeps the controller, so the same Bears that
/// the Slaver ignored is now a Hymn.
#[test]
fn larceny_taxes_any_creature_its_controller_sent_in() {
    let game = connect(cards::LARCENY, cards::GRIZZLY_BEARS);
    assert_eq!(
        hands(&game),
        (2, 1),
        "any creature I control that gets there costs them a card"
    );
}
