//! "This creature assigns no combat damage this turn."
//!
//! A constraint on the assignment rather than a shield over the result: the
//! creature is not asked how to divide its damage at all. Both printed
//! carriers pay for their effect with the swing they were about to land, so
//! what these check is that the trade actually costs something -- and that
//! declining leaves the combat damage alone.

use super::*;

/// Attacks unblocked with `attacker` and runs the declaration through to the
/// point where its trigger is asking.
fn attack_unblocked(game: &mut Game, attacker: GameObjectId) {
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    for permanent in &mut game.battlefield {
        if permanent.card.id == attacker {
            permanent.attacking = true;
            permanent.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        }
    }
    game.finish_declaring_blockers();
}

/// Answers every pending decision: takes or declines the optional half, and
/// picks `aim` for anything else. "Target creature" includes the attacker
/// itself, so the choice has to be named rather than taken first.
fn settle(game: &mut Game, accept: bool, aim: &str) {
    for _ in 0..12 {
        if let Some(decision) = game.pending_decisions.first().cloned() {
            let wanted = if accept { "Do it" } else { "Decline" };
            let option = decision
                .observation
                .options
                .iter()
                .find(|option| option.label == wanted || option.label == aim)
                .expect("the decision offers what the test is after")
                .id;
            game.apply(
                decision.observation.player,
                Action::ChooseDecision {
                    decision: decision.observation.id,
                    options: vec![option],
                },
            )
            .expect("the choice is submitted");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn on_battlefield(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

/// The Zealot, its prey, and the life player two starts the swing on.
fn zealot_board() -> (Game, GameObjectId, GameObjectId, i16) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let zealot = creature(10_000, cards::FARRELS_ZEALOT, PlayerId::One);
    let zealot_id = zealot.card.id;
    game.battlefield.push(zealot);
    let prey = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    let prey_id = prey.card.id;
    game.battlefield.push(prey);
    let life = game.players[PlayerId::Two.index()].life;
    (game, zealot_id, prey_id, life)
}

#[test]
fn the_zealot_pays_for_its_three_with_the_swing() {
    let (mut game, zealot, prey, life) = zealot_board();

    attack_unblocked(&mut game, zealot);
    settle(&mut game, true, "Sedge Troll");
    game.deal_combat_damage();

    assert!(!on_battlefield(&game, prey), "three killed the 2/2");
    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        life,
        "and the Zealot assigned nothing"
    );
    assert!(on_battlefield(&game, zealot));
}

/// Declining is what tells the two apart: the same board, the same swing,
/// and the combat damage lands.
#[test]
fn declining_leaves_the_combat_damage_alone() {
    let (mut game, zealot, prey, life) = zealot_board();

    attack_unblocked(&mut game, zealot);
    settle(&mut game, false, "Sedge Troll");
    game.deal_combat_damage();

    assert!(on_battlefield(&game, prey), "nothing was dealt to it");
    assert_eq!(game.players[PlayerId::Two.index()].life, life - 2);
    assert!(on_battlefield(&game, zealot));
}

/// The constraint is on this creature, not on the attack: a second attacker
/// connects for its own damage either way.
#[test]
fn another_attacker_still_connects() {
    let (mut game, zealot, _, life) = zealot_board();
    let ally = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One);
    let ally_id = ally.card.id;
    game.battlefield.push(ally);

    attack_unblocked(&mut game, zealot);
    for permanent in &mut game.battlefield {
        if permanent.card.id == ally_id {
            permanent.attacking = true;
            permanent.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        }
    }
    settle(&mut game, true, "Sedge Troll");
    game.deal_combat_damage();

    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        life - 2,
        "the Lions' two, and none of the Zealot's"
    );
}

/// The constraint lasts the turn rather than the combat, so a second combat
/// phase gets nothing out of it either.
#[test]
fn the_constraint_outlasts_the_combat() {
    let (mut game, zealot, _, life) = zealot_board();

    attack_unblocked(&mut game, zealot);
    settle(&mut game, true, "Sedge Troll");
    game.deal_combat_damage();
    game.clear_combat();

    attack_unblocked(&mut game, zealot);
    game.deal_combat_damage();

    assert_eq!(game.players[PlayerId::Two.index()].life, life);
}

/// Floral Spuzzem pays the same price for an artifact.
#[test]
fn the_spuzzem_trades_its_swing_for_an_artifact() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let spuzzem = creature(10_000, cards::FLORAL_SPUZZEM, PlayerId::One);
    let spuzzem_id = spuzzem.card.id;
    game.battlefield.push(spuzzem);
    let ring = creature(10_001, cards::SOL_RING, PlayerId::Two);
    let ring_id = ring.card.id;
    game.battlefield.push(ring);
    let life = game.players[PlayerId::Two.index()].life;

    attack_unblocked(&mut game, spuzzem_id);
    settle(&mut game, true, "Sol Ring");
    game.deal_combat_damage();

    assert!(!on_battlefield(&game, ring_id), "the artifact is destroyed");
    assert_eq!(game.players[PlayerId::Two.index()].life, life);
}
