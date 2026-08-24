//! Static Prison: energy counters, and a jail with a two-turn lease.

use super::*;

/// Answers every pending decision with the first option it offered, then
/// resolves whatever is left on the stack.
fn settle_paying(game: &mut Game, pay: bool) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            // Chosen by label rather than by position: the payment branch
            // is the one that says so.
            let paying = decision
                .options
                .iter()
                .find(|option| option.label.starts_with("Pay "));
            let wanted = match (pay, paying) {
                (true, Some(option)) => Some(option),
                (true, None) => decision.options.first(),
                (false, _) => decision
                    .options
                    .iter()
                    .find(|option| !option.label.starts_with("Pay "))
                    .or_else(|| decision.options.first()),
            };
            let options = wanted.map(|option| vec![option.id]).unwrap_or_default();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// A Prison on the battlefield with the opponent's Angel already exiled.
fn jailed() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let angel = creature(93_100, cards::SERRA_ANGEL, PlayerId::Two);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    let prison = game
        .put_onto_battlefield(PlayerId::One, cards::STATIC_PRISON)
        .expect("cataloged");
    settle_paying(&mut game, true);
    drain_pending(&mut game);
    (game, prison, angel_id)
}

/// The entry exiles their creature and pays out two energy.
#[test]
fn the_prison_exiles_a_permanent_and_gives_two_energy() {
    let (game, _prison, angel_id) = jailed();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != angel_id),
        "the creature is exiled",
    );
    assert_eq!(
        game.observe(PlayerId::One).energy_counters[0],
        2,
        "and the two energy came with it",
    );
}

/// Paying the energy keeps the jail shut, and spends one.
#[test]
fn paying_the_energy_keeps_the_prison_and_the_prisoner() {
    let (mut game, prison, angel_id) = jailed();

    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::PrecombatMain,
        player: PlayerId::One,
    });
    game.finish_rules_procedure();
    settle_paying(&mut game, true);
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == prison),
        "the Prison is still standing",
    );
    assert_eq!(
        game.observe(PlayerId::One).energy_counters[0],
        1,
        "one energy of the two is gone",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != angel_id),
        "and the prisoner is still exiled",
    );
}

/// Declining sacrifices the Prison, and the prisoner walks out.
#[test]
fn declining_frees_the_prisoner() {
    let (mut game, prison, _angel_id) = jailed();

    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::PrecombatMain,
        player: PlayerId::One,
    });
    game.finish_rules_procedure();
    settle_paying(&mut game, false);
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != prison),
        "the Prison was sacrificed",
    );
    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SERRA_ANGEL)
        .expect("the prisoner is back on the battlefield");
    assert_eq!(angel.controller, PlayerId::Two, "under its owner's control");
    assert_eq!(
        game.observe(PlayerId::One).energy_counters[0],
        2,
        "and nothing was spent",
    );
}

/// Energy is spent in full or not at all: with none left, the payment is not
/// even offered and the Prison goes.
#[test]
fn a_player_out_of_energy_cannot_pay_at_all() {
    let (mut game, prison, _angel_id) = jailed();
    game.players[0].counters.set(CounterKind::Energy, 0);

    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::PrecombatMain,
        player: PlayerId::One,
    });
    game.finish_rules_procedure();
    settle_paying(&mut game, true);
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != prison),
        "there was nothing to pay with",
    );
}

/// The tax is on your own first main phase, not the opponent's.
#[test]
fn the_opponents_main_phase_costs_nothing() {
    let (mut game, prison, _angel_id) = jailed();

    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::PrecombatMain,
        player: PlayerId::Two,
    });
    game.finish_rules_procedure();
    settle_paying(&mut game, true);
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == prison),
        "no trigger, so nothing to pay",
    );
    assert_eq!(game.observe(PlayerId::One).energy_counters[0], 2);
}
