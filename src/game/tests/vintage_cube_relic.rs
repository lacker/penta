//! Coalition Relic: charge counters cashed in for mana of chosen colours.

use super::*;

/// Answers a pending colour choice with `color`, then settles the stack.
fn answer_colors(game: &mut Game, colors: &[ManaColor]) {
    let mut wanted = colors.iter();
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let label = wanted.next().map_or("White", |color| match color {
                ManaColor::White => "White",
                ManaColor::Blue => "Blue",
                ManaColor::Black => "Black",
                ManaColor::Red => "Red",
                ManaColor::Green => "Green",
                ManaColor::Colorless => "Colorless",
            });
            let option = decision
                .options
                .iter()
                .find(|option| option.label == label)
                .unwrap_or_else(|| panic!("{label} is on offer"));
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![option.id],
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

/// The Relic on the battlefield, already untapped and unsick, holding
/// `charges` charge counters.
fn staged(charges: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let relic = game
        .put_onto_battlefield(PlayerId::One, cards::COALITION_RELIC)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == relic)
    {
        permanent
            .counters
            .set(CounterKind::named("charge"), charges);
    }
    (game, relic)
}

fn begin_main(game: &mut Game) {
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::PrecombatMain,
        player: PlayerId::One,
    });
    game.finish_rules_procedure();
}

/// The second ability banks a counter rather than making mana.
#[test]
fn the_second_ability_puts_a_charge_counter_on_instead_of_making_mana() {
    let (mut game, relic) = staged(0);
    let ability = activated_ability_for(&game, relic, 0);

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: relic,
            ability,
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .expect("the ability activates");
    pass_priority_pair(&mut game);
    drain_pending(&mut game);

    let relic = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == relic)
        .expect("still there");
    assert_eq!(relic.counters(CounterKind::named("charge")), 1);
    assert_eq!(game.players[0].mana_pool.total(), 0, "no mana yet");
}

/// Two counters cash in for two mana, and each one is named separately, so
/// they can be different colours.
#[test]
fn the_counters_cash_in_for_one_mana_each_of_chosen_colors() {
    let (mut game, relic) = staged(2);

    begin_main(&mut game);
    answer_colors(&mut game, &[ManaColor::Blue, ManaColor::Red]);
    drain_pending(&mut game);

    assert_eq!(game.players[0].mana_pool.blue, 1);
    assert_eq!(game.players[0].mana_pool.red, 1);
    assert_eq!(game.players[0].mana_pool.total(), 2);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == relic)
            .map(|permanent| permanent.counters(CounterKind::named("charge"))),
        Some(0),
        "and the counters are gone",
    );
}

/// Both mana may be the same colour, which is the ordinary case.
#[test]
fn both_mana_may_be_the_same_color() {
    let (mut game, _relic) = staged(2);

    begin_main(&mut game);
    answer_colors(&mut game, &[ManaColor::Green, ManaColor::Green]);
    drain_pending(&mut game);

    assert_eq!(game.players[0].mana_pool.green, 2);
    assert_eq!(game.players[0].mana_pool.total(), 2);
}

/// With no counters the trigger asks nothing and adds nothing.
#[test]
fn no_counters_means_no_question_and_no_mana() {
    let (mut game, _relic) = staged(0);

    begin_main(&mut game);
    answer_colors(&mut game, &[]);
    drain_pending(&mut game);

    assert!(game.pending_decisions.is_empty(), "nothing to choose");
    assert_eq!(game.players[0].mana_pool.total(), 0);
}
