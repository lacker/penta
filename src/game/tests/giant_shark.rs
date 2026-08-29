//! Giant Shark, and the turn-long record of damage its trigger reads.
//!
//! "Has been dealt damage this turn" is not "has damage marked on it".
//! Regeneration wipes the marks and cleanup wipes them at end of turn, so
//! the Shark reads a separate flag that only the turn boundary clears.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

/// Blocks a bloodied or unbloodied attacker with the Shark and reports the
/// Shark's power afterwards.
fn shark_blocks(wound_the_attacker: bool) -> Option<i16> {
    let mut game = ready();
    let mut attacker = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let shark = creature(10_100, cards::GIANT_SHARK, PlayerId::One);
    let shark_id = shark.card.id;
    game.battlefield.push(shark);
    game.put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .expect("cataloged");

    if wound_the_attacker {
        game.damage_target_from(None, Some(Target::Permanent(attacker_id)), 1);
    }

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.apply(
        PlayerId::One,
        Action::DeclareBlocker {
            blocker: shark_id,
            attacker: attacker_id,
        },
    )
    .expect("the block is legal");
    game.finish_declaring_blockers();
    drain_pending(&mut game);

    stats(&game, shark_id).0
}

#[test]
fn the_shark_smells_blood_on_the_creature_it_blocks() {
    assert_eq!(shark_blocks(false), Some(4), "an unhurt attacker is safe");
    assert_eq!(shark_blocks(true), Some(6), "a bloodied one is not");
}

/// The record outlives the marks: a creature that regenerated has no damage
/// marked on it and still counts.
#[test]
fn the_record_outlives_the_damage_marks() {
    let mut game = ready();
    let mut attacker = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let shark = creature(10_100, cards::GIANT_SHARK, PlayerId::One);
    let shark_id = shark.card.id;
    game.battlefield.push(shark);
    game.put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .expect("cataloged");

    game.damage_target_from(None, Some(Target::Permanent(attacker_id)), 1);
    // Wipe the marks the way a regeneration shield would, leaving only the
    // turn-long record behind.
    let index = game
        .battlefield
        .iter()
        .position(|permanent| permanent.card.id == attacker_id)
        .expect("still there");
    game.battlefield[index].damage = 0;

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.apply(
        PlayerId::One,
        Action::DeclareBlocker {
            blocker: shark_id,
            attacker: attacker_id,
        },
    )
    .expect("the block is legal");
    game.finish_declaring_blockers();
    drain_pending(&mut game);

    assert_eq!(
        stats(&game, shark_id),
        (Some(6), Some(4)),
        "the damage happened, whatever the creature looks like now",
    );
}

/// It cannot attack into a player with no Island, and dies when its own
/// controller has none.
#[test]
fn the_shark_needs_islands_on_both_sides() {
    let mut game = ready();
    game.active_player = PlayerId::One;
    let shark = creature(10_000, cards::GIANT_SHARK, PlayerId::One);
    let shark_id = shark.card.id;
    game.battlefield.push(shark);
    game.put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .expect("cataloged");
    game.step = Step::DeclareAttackers;

    let can_attack = |game: &Game| {
        game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::DeclareAttacker { attacker, .. } if *attacker == shark_id),
        )
    };
    assert!(!can_attack(&game), "the defender controls no Island");

    game.put_onto_battlefield(PlayerId::Two, cards::ISLAND)
        .expect("cataloged");
    assert!(can_attack(&game), "now they do");

    // Take away its own controller's Island and the state trigger fires.
    let island = game
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.controller == PlayerId::One && permanent.card.definition == cards::ISLAND
        })
        .expect("it is there")
        .card
        .id;
    game.battlefield
        .retain(|permanent| permanent.card.id != island);
    game.step = Step::PostcombatMain;
    game.priority = PlayerId::One;
    game.check_state_based_actions();
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == shark_id),
        "no Islands, no Shark",
    );
}
