//! An Aura that bills its host's controller when the host dies.
//!
//! Both halves read a creature that is already in the graveyard by the time
//! the trigger resolves, so both come from last-known information: how tough
//! it was, and who controlled it.

use super::*;

/// Player two's `host` under player one's Creature Bond, killed outright.
fn bonded_and_killed(host: CardDefinitionId) -> Game {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let creature_host = creature(10_000, host, PlayerId::Two);
    let host_id = creature_host.card.id;
    game.battlefield.push(creature_host);
    let mut aura = creature(10_001, cards::CREATURE_BOND, PlayerId::One);
    aura.attached_to = Some(host_id);
    game.battlefield.push(aura);

    game.destroy_permanent(host_id);
    drain_pending(&mut game);
    game.check_state_based_actions();
    drain_pending(&mut game);
    game
}

fn life(game: &Game, player: PlayerId) -> i16 {
    game.players[player.index()].life
}

#[test]
fn it_bills_the_hosts_controller_for_its_toughness() {
    let game = bonded_and_killed(cards::SERRA_ANGEL);

    assert_eq!(
        life(&game, PlayerId::Two),
        i16::from(rules::STARTING_LIFE) - 4,
        "a 4/4 died, so four damage to the player who controlled it",
    );
    assert_eq!(
        life(&game, PlayerId::One),
        i16::from(rules::STARTING_LIFE),
        "and none to the Aura's controller",
    );
}

/// Toughness, not power. A creature whose two numbers differ is what tells
/// the two apart.
#[test]
fn it_reads_toughness_rather_than_power() {
    let game = bonded_and_killed(cards::SAVANNAH_LIONS);

    assert_eq!(
        life(&game, PlayerId::Two),
        i16::from(rules::STARTING_LIFE) - 1,
        "a 2/1 bills one, not two",
    );
}
