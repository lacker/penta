//! Raid Bombardment: the defender is read off each attacker, so a
//! planeswalker takes the ping the player would otherwise have taken.

use super::*;

/// Player One in declare-attackers with Raid Bombardment out and `attackers`
/// beside it, all of them able to attack.
fn staged(attackers: &[CardDefinitionId]) -> (Game, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.battlefield
        .push(creature(10_400, cards::RAID_BOMBARDMENT, PlayerId::One));
    let ids = attackers
        .iter()
        .enumerate()
        .map(|(index, definition)| {
            let permanent = creature(
                10_000 + u32::try_from(index).expect("a small fixture"),
                *definition,
                PlayerId::One,
            );
            let id = permanent.card.id;
            game.battlefield.push(permanent);
            id
        })
        .collect();
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;
    game.players[1].life = 20;
    (game, ids)
}

fn add_planeswalker(game: &mut Game) -> GameObjectId {
    let mut planeswalker = creature(10_500, cards::VRASKA_THE_UNSEEN, PlayerId::Two);
    planeswalker.set_counters(CounterKind::Loyalty, 5);
    let id = planeswalker.card.id;
    game.battlefield.push(planeswalker);
    id
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if !game.pending_decisions.is_empty() {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

fn attack(game: &mut Game, assignments: &[(GameObjectId, AttackDefender)]) {
    for (attacker, defender) in assignments {
        game.apply(
            PlayerId::One,
            Action::DeclareAttacker {
                attacker: *attacker,
                defender: *defender,
            },
        )
        .expect("it attacks");
    }
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    settle(game);
}

fn loyalty(game: &Game, walker: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == walker)
        .expect("the planeswalker is still there")
        .counters(CounterKind::Loyalty)
}

/// A 2/1 is under the cap, so the attack pings the defending player. This is
/// the ordinary case, before combat damage is anywhere near.
#[test]
fn a_small_attacker_pings_the_defending_player() {
    let (mut game, attackers) = staged(&[cards::SAVANNAH_LIONS]);
    attack(
        &mut game,
        &[(attackers[0], AttackDefender::Player(PlayerId::Two))],
    );
    assert_eq!(
        game.players[1].life, 19,
        "one damage for the one attacker under the cap"
    );
}

/// The cap is on power, and a 4/4 is over it. Nothing triggers at all, so
/// the two attackers together deal exactly the one ping.
#[test]
fn only_attackers_within_the_power_cap_trigger() {
    let (mut game, attackers) = staged(&[cards::SAVANNAH_LIONS, cards::SERRA_ANGEL]);
    attack(
        &mut game,
        &[
            (attackers[0], AttackDefender::Player(PlayerId::Two)),
            (attackers[1], AttackDefender::Player(PlayerId::Two)),
        ],
    );
    assert_eq!(
        game.players[1].life, 19,
        "the 4/4 is over the cap and contributes nothing"
    );
}

/// The reason this needs the attacker's own defender rather than "your
/// opponent": an attacker aimed at a planeswalker takes loyalty off it and
/// leaves the player alone.
#[test]
fn an_attacker_aimed_at_a_planeswalker_pings_the_planeswalker() {
    let (mut game, attackers) = staged(&[cards::SAVANNAH_LIONS]);
    let walker = add_planeswalker(&mut game);
    attack(
        &mut game,
        &[(attackers[0], AttackDefender::Planeswalker(walker))],
    );
    assert_eq!(
        game.players[1].life, 20,
        "the player is not what this attacker is attacking"
    );
    assert_eq!(loyalty(&game, walker), 4, "the planeswalker took the ping");
}
