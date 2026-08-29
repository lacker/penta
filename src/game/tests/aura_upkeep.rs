//! Auras that trigger on their host's controller's upkeep.
//!
//! "The upkeep of enchanted land's controller" is not the Aura's controller's
//! upkeep, and the two come apart the moment the Aura is on something an
//! opponent controls -- which is the only way any of these cards is ever
//! played. What these check is which upkeep fires it and who takes the damage.

use super::*;

/// Wanderlust on a creature `host_controller` controls, with the Aura itself
/// controlled by player one.
fn wanderlust_on(host_controller: PlayerId) -> Game {
    let mut game = ready_game();
    let host = creature(10_000, cards::SEDGE_TROLL, host_controller);
    let host_id = host.card.id;
    game.battlefield.push(host);

    let mut aura = creature(10_001, cards::WANDERLUST, PlayerId::One);
    aura.attached_to = Some(host_id);
    game.battlefield.push(aura);
    game.check_state_based_actions();
    game
}

fn life(game: &Game, player: PlayerId) -> i16 {
    game.players[player.index()].life
}

fn take_turn(game: &mut Game, player: PlayerId) {
    game.commit_next_turn(player, Vec::new());
    drain_pending(game);
}

#[test]
fn the_hosts_controller_takes_the_damage_on_their_own_upkeep() {
    let mut game = wanderlust_on(PlayerId::Two);
    let before = [life(&game, PlayerId::One), life(&game, PlayerId::Two)];

    // The Aura's controller takes a turn. Nothing should happen: it is not
    // their upkeep that this watches.
    take_turn(&mut game, PlayerId::One);
    assert_eq!(
        [life(&game, PlayerId::One), life(&game, PlayerId::Two)],
        before,
        "the Aura's own controller's upkeep is not the one named"
    );

    take_turn(&mut game, PlayerId::Two);
    assert_eq!(
        life(&game, PlayerId::Two),
        before[1] - 1,
        "the host's controller takes it on their upkeep"
    );
    assert_eq!(
        life(&game, PlayerId::One),
        before[0],
        "and the Aura's controller takes nothing"
    );
}

/// Enchanting your own permanent points the damage at yourself, which is what
/// makes "that player" a reading of the host rather than of the opponent.
#[test]
fn enchanting_your_own_permanent_points_it_at_you() {
    let mut game = wanderlust_on(PlayerId::One);
    let before = life(&game, PlayerId::One);

    take_turn(&mut game, PlayerId::One);
    assert_eq!(
        life(&game, PlayerId::One),
        before - 1,
        "the host's controller is the Aura's controller here"
    );
}

/// The trigger belongs to the attachment. An Aura that has come loose is
/// attached to nothing and names nobody.
#[test]
fn an_unattached_aura_triggers_for_nobody() {
    let mut game = ready_game();
    let mut aura = creature(10_000, cards::WANDERLUST, PlayerId::One);
    aura.attached_to = None;
    game.battlefield.push(aura);
    let before = [life(&game, PlayerId::One), life(&game, PlayerId::Two)];

    // State-based actions will bin it, but the point is that no upkeep in
    // between finds a player to damage.
    take_turn(&mut game, PlayerId::One);
    take_turn(&mut game, PlayerId::Two);
    assert_eq!(
        [life(&game, PlayerId::One), life(&game, PlayerId::Two)],
        before
    );
}
