//! "Whenever this creature blocks or becomes blocked by ..."
//!
//! One printed clause covering both sides of a block, so the same card has to
//! fire whether it attacked or blocked, and "that creature" is whichever one
//! is on the other side. What these check is both directions, the predicate
//! that narrows which blocks count, and that end of combat is earlier than
//! the end step.

use super::*;

/// Puts `attacker` and `blocker` into a committed block and runs the triggers.
fn block(game: &mut Game, attacker: GameObjectId, blocker: GameObjectId) {
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    for permanent in &mut game.battlefield {
        if permanent.card.id == attacker {
            permanent.attacking = true;
            permanent.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        }
        if permanent.card.id == blocker {
            permanent.blocking = vec![attacker];
        }
    }
    game.finish_declaring_blockers();
    drain_pending(game);
}

fn on_battlefield(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

fn damage_on(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("permanent should be on the battlefield")
        .damage
}

/// Begin the end-of-combat step, put its delayed triggers on the stack, and
/// let them resolve while combat is still in progress.
fn end_combat(game: &mut Game) {
    game.step = Step::EndOfCombat;
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::EndOfCombat,
        player: game.active_player,
    });
    game.finish_rules_procedure();
    drain_pending(game);
}

#[test]
fn a_basilisk_kills_what_blocks_it_and_what_it_blocks() {
    // It attacked and was blocked.
    let mut attacking = ready_game();
    let basilisk = creature(10_000, cards::THICKET_BASILISK, PlayerId::One);
    let basilisk_id = basilisk.card.id;
    attacking.battlefield.push(basilisk);
    let blocker = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    let blocker_id = blocker.card.id;
    attacking.battlefield.push(blocker);
    block(&mut attacking, basilisk_id, blocker_id);
    assert!(
        on_battlefield(&attacking, blocker_id),
        "still alive during combat"
    );
    end_combat(&mut attacking);
    assert!(
        !on_battlefield(&attacking, blocker_id),
        "the creature that blocked it dies at end of combat"
    );

    // It blocked. Same clause, other direction.
    let mut blocking = ready_game();
    let attacker = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let attacker_id = attacker.card.id;
    blocking.battlefield.push(attacker);
    let basilisk = creature(10_001, cards::THICKET_BASILISK, PlayerId::Two);
    let basilisk_id = basilisk.card.id;
    blocking.battlefield.push(basilisk);
    block(&mut blocking, attacker_id, basilisk_id);
    end_combat(&mut blocking);
    assert!(
        !on_battlefield(&blocking, attacker_id),
        "the creature it blocked dies just the same"
    );
}

#[test]
fn ashmouth_hound_damages_each_creature_blocking_it() {
    let mut game = ready_game();
    let hound = creature(10_000, cards::ASHMOUTH_HOUND, PlayerId::One);
    let hound_id = hound.card.id;
    game.battlefield.push(hound);
    let first = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    let first_id = first.card.id;
    game.battlefield.push(first);
    let second = creature(10_002, cards::CRAW_WURM, PlayerId::Two);
    let second_id = second.card.id;
    game.battlefield.push(second);

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    for permanent in &mut game.battlefield {
        if permanent.card.id == hound_id {
            permanent.attacking = true;
            permanent.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        }
        if [first_id, second_id].contains(&permanent.card.id) {
            permanent.blocking = vec![hound_id];
        }
    }
    game.finish_declaring_blockers();
    drain_pending(&mut game);

    assert_eq!(damage_on(&game, first_id), 1);
    assert_eq!(damage_on(&game, second_id), 1);
}

/// Abomination names green or white, so the predicate has to both admit and
/// refuse. Checking only the refusal would pass even if nothing fired at all.
#[test]
fn the_predicate_narrows_which_blocks_count() {
    for (blocker_definition, survives) in [
        // Sedge Troll is red, which is neither colour named.
        (cards::SEDGE_TROLL, true),
        (cards::THICKET_BASILISK, false),
    ] {
        let mut game = ready_game();
        let abomination = creature(10_000, cards::ABOMINATION, PlayerId::One);
        let abomination_id = abomination.card.id;
        game.battlefield.push(abomination);
        let blocker = creature(10_001, blocker_definition, PlayerId::Two);
        let blocker_id = blocker.card.id;
        game.battlefield.push(blocker);

        block(&mut game, abomination_id, blocker_id);
        end_combat(&mut game);
        assert_eq!(
            on_battlefield(&game, blocker_id),
            survives,
            "colour decides whether Abomination's clause applies"
        );
    }
}

/// Aisling Leprechaun repaints rather than destroys, which is the other
/// consumer of the same trigger and reads "that creature" as its recipient.
#[test]
fn the_leprechaun_repaints_what_it_meets() {
    let mut game = ready_game();
    let leprechaun = creature(10_000, cards::AISLING_LEPRECHAUN, PlayerId::One);
    let leprechaun_id = leprechaun.card.id;
    game.battlefield.push(leprechaun);
    let blocker = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);

    block(&mut game, leprechaun_id, blocker_id);

    assert_eq!(
        game.object_colors(blocker_id),
        [false, false, false, false, true],
        "the creature it met is green and nothing else"
    );
}
