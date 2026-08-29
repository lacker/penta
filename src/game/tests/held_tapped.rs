//! Holding a permanent down for as long as the source stays tapped.
//!
//! Like the stat bonus that rides a tapped artifact, this rule has no
//! deadline: the Gremlins decide when it ends by untapping, which is why
//! they also print "you may choose not to untap". So the source is recorded
//! and the question is asked afresh at each untap step.

use super::*;

/// Gremlins with their ability already spent on a Sol Ring the other player
/// controls.
fn gremlins_holding_a_ring() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    game.turns_started[PlayerId::Two.index()] = 1;
    let gremlins = creature(10_000, cards::PHYREXIAN_GREMLINS, PlayerId::One);
    let gremlins_id = gremlins.card.id;
    game.battlefield.push(gremlins);
    let ring = creature(10_001, cards::SOL_RING, PlayerId::Two);
    let ring_id = ring.card.id;
    game.battlefield.push(ring);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == gremlins_id
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(ring_id))
            }
            _ => false,
        })
        .expect("the Gremlins can name the Ring");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);
    (game, gremlins_id, ring_id)
}

fn is_tapped(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .tapped
}

fn take_turn(game: &mut Game, player: PlayerId) {
    game.commit_next_turn(player, Vec::new());
    drain_pending(game);
}

#[test]
fn the_ring_taps_and_stays_down() {
    let (mut game, gremlins, ring) = gremlins_holding_a_ring();
    assert!(is_tapped(&game, ring), "the ability tapped it");
    assert!(is_tapped(&game, gremlins), "and the tap was its cost");

    take_turn(&mut game, PlayerId::Two);
    assert!(
        is_tapped(&game, ring),
        "its controller's untap step passes it by"
    );
}

/// The hold ends with the Gremlins untapping, and nothing has to be undone
/// on the Ring for that to happen.
#[test]
fn untapping_the_gremlins_lets_the_ring_go() {
    let (mut game, gremlins, ring) = gremlins_holding_a_ring();

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == gremlins)
        .expect("still there")
        .tapped = false;

    take_turn(&mut game, PlayerId::Two);
    assert!(!is_tapped(&game, ring));
}

/// An artifact the Gremlins never named is not held.
#[test]
fn another_artifact_untaps_as_usual() {
    let (mut game, _, _) = gremlins_holding_a_ring();
    let mut other = creature(10_002, cards::SOL_RING, PlayerId::Two);
    other.tapped = true;
    let other_id = other.card.id;
    game.battlefield.push(other);

    take_turn(&mut game, PlayerId::Two);
    assert!(!is_tapped(&game, other_id));
}
