//! Reaper from the Abyss and the shared damaged-creature death trigger.

use super::*;
use crate::ImplementationStatus;

fn board() -> (Game, GameObjectId, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::Two;

    let reaper = creature(10_000, cards::REAPER_FROM_THE_ABYSS, PlayerId::One);
    let reaper_id = reaper.card.id;
    let sengir = creature(10_001, cards::SENGIR_VAMPIRE, PlayerId::One);
    let sengir_id = sengir.card.id;
    let victim = creature(10_002, cards::GRIZZLY_BEARS, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.extend([reaper, sengir, victim]);
    game.priority = PlayerId::Two;
    (game, reaper_id, sengir_id, victim_id)
}

fn record_a_death(game: &mut Game) {
    let fodder = creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two);
    let fodder_id = fodder.card.id;
    game.battlefield.push(fodder);
    game.destroy_permanent(fodder_id);
    drain_pending(game);
}

fn begin_end_step(game: &mut Game) {
    game.step = Step::End;
    game.begin_step_triggers();
    game.finish_rules_procedure();
}

fn choose_trigger_target(game: &mut Game, target: GameObjectId) {
    let pending = game
        .pending_decisions
        .first()
        .expect("Reaper's controller chooses a target");
    let target_index = match &pending.continuation {
        DecisionContinuation::TriggerPlacement { candidates, .. } => candidates
            .iter()
            .position(|candidate| *candidate == Target::Permanent(target))
            .expect("the requested non-Demon is a legal target"),
        other => panic!("expected trigger placement, found {other:?}"),
    };
    let decision = pending.observation.clone();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[target_index].id],
        },
    )
    .expect("the target is chosen");
}

fn permanent_exists(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

#[test]
fn quiet_turn_creates_no_trigger_or_target_choice() {
    let (mut game, reaper, _sengir, victim) = board();

    begin_end_step(&mut game);

    assert!(game.pending_decisions.is_empty());
    assert!(game.stack.is_empty());
    assert!(permanent_exists(&game, reaper));
    assert!(permanent_exists(&game, victim));
}

#[test]
fn a_death_turn_destroys_a_non_demon_at_each_players_end_step() {
    let (mut game, reaper, _sengir, victim) = board();
    record_a_death(&mut game);

    begin_end_step(&mut game);

    let candidates = match &game.pending_decisions[0].continuation {
        DecisionContinuation::TriggerPlacement { candidates, .. } => candidates,
        other => panic!("expected trigger placement, found {other:?}"),
    };
    assert!(
        !candidates.contains(&Target::Permanent(reaper)),
        "Reaper is a Demon and cannot be targeted",
    );
    choose_trigger_target(&mut game, victim);
    drain_pending(&mut game);

    assert!(!permanent_exists(&game, victim));
    assert!(permanent_exists(&game, reaper));
}

#[test]
fn reaper_kill_triggers_sengir_for_a_creature_it_damaged() {
    let (mut game, _reaper, sengir, victim) = board();
    record_a_death(&mut game);
    game.damage_target_from(Some(sengir), Some(Target::Permanent(victim)), 1);

    begin_end_step(&mut game);
    choose_trigger_target(&mut game, victim);
    drain_pending(&mut game);

    assert!(!permanent_exists(&game, victim));
    let sengir = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == sengir)
        .expect("Sengir remains");
    assert_eq!(sengir.counters(CounterKind::PlusOnePlusOne), 1);
}

#[test]
fn reaper_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    let reaper = catalog
        .get(cards::REAPER_FROM_THE_ABYSS)
        .expect("Reaper is cataloged");
    assert_eq!(
        reaper.rules.implementation_status(),
        ImplementationStatus::Complete,
    );
}
