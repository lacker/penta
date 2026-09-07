use super::object_costs::{activate, fixture, select};
use super::*;
use crate::card::{CostQuantityDef, EffectPaymentDef, PayOrDef, PlayerRefDef};

const DISCARD: CostDef = CostDef::discard(ObjectPredicateDef::Any, CostQuantityDef::Fixed(1));
const GAIN: EffectDef = EffectDef::GainLife {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::Constant(3),
};
static PROGRAMS: [AbilityDef; 3] = [
    AbilityDef::activated(
        "Pay the complete bundle.",
        &[],
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef {
                payer: crate::card::PlayerSetDef::One(PlayerRefDef::EffectController),
                cost: CostDef::All(&[
                    CostDef::PayLife(1),
                    CostDef::Mana(mana_cost!("{1}")),
                    DISCARD,
                    DISCARD,
                ]),
            },
            &GAIN,
        )),
    ),
    AbilityDef::activated(
        "Pay either cost twice.",
        &[],
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef {
                payer: crate::card::PlayerSetDef::One(PlayerRefDef::EffectController),
                cost: CostDef::Repeat {
                    cost: &CostDef::Choice(&[CostDef::PayLife(1), DISCARD]),
                    times: ValueDef::Constant(2),
                },
            },
            &GAIN,
        )),
    ),
    AbilityDef::activated(
        "Pay mana, life, and sacrifice.",
        &[],
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef {
                payer: crate::card::PlayerSetDef::One(PlayerRefDef::EffectController),
                cost: CostDef::All(&[
                    CostDef::Mana(mana_cost!("{1}")),
                    CostDef::PayLife(1),
                    CostDef::sacrifice(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        CostQuantityDef::Fixed(1),
                    ),
                ]),
            },
            &GAIN,
        )),
    ),
];

fn begin(game: &mut Game, source: GameObjectId, ability: usize) {
    activate(game, source, ability);
    game.resolve_stack_top();
}

#[test]
fn payment_programs_bundle_collects_all_selections_before_spending_and_cancels_privately() {
    for prepared in [false, true] {
        for cancel in [false, true] {
            let (mut game, source) = fixture(&PROGRAMS, 1);
            game.set_prepared_engine_enabled(prepared);
            let cards = [GameObjectId(161_010), GameObjectId(161_011)];
            for id in cards {
                game.players[0]
                    .hand
                    .push(card(id.0, cards::ISLAND, PlayerId::One));
            }
            game.players[0].mana_pool.colorless = 1;
            begin(&mut game, source, 0);
            select(&mut game, &cards[..1]);
            assert_eq!(game.players[0].hand.len(), 2);
            assert_eq!(game.players[0].life, 20);
            assert_eq!(game.players[0].mana_pool.colorless, 1);
            let decision = game.pending_decisions[0].observation.clone();
            assert_eq!(decision.visibility, DecisionVisibility::Private);
            assert!(game.observe(PlayerId::Two).decision.is_none());
            assert_eq!(
                decision.options.len(),
                1,
                "one card cannot pay two discard costs"
            );
            if cancel {
                game.apply(
                    PlayerId::One,
                    Action::CancelDecision {
                        decision: decision.id,
                    },
                )
                .unwrap();
                assert_eq!(game.players[0].hand.len(), 2);
                assert_eq!(game.players[0].life, 20);
                assert_eq!(game.players[0].mana_pool.colorless, 1);
            } else {
                select(&mut game, &cards[1..]);
                assert!(game.players[0].hand.is_empty());
                assert_eq!(game.players[0].graveyard.len(), 2);
                assert_eq!(game.players[0].life, 22);
                assert_eq!(game.players[0].mana_pool.colorless, 0);
            }
        }
    }
}

#[test]
fn payment_programs_repeated_alternatives_are_independent_and_preflight_total_life() {
    for first_life in [false, true] {
        let (mut game, source) = fixture(&PROGRAMS, 1);
        let card_id = GameObjectId(161_020);
        game.players[0]
            .hand
            .push(card(card_id.0, cards::ISLAND, PlayerId::One));
        begin(&mut game, source, 1);
        for life in [first_life, !first_life] {
            choose_decision_by_label(
                &mut game,
                PlayerId::One,
                if life {
                    "Pay the cost"
                } else {
                    "Discard matching cards"
                },
            );
            if !life {
                select(&mut game, &[card_id]);
            }
            if !game.pending_decisions.is_empty() {
                assert_eq!(game.players[0].life, 20);
            }
        }
        if !game.pending_decisions.is_empty() {
            choose_decision_by_label(&mut game, PlayerId::One, "Pay the cost");
        }
        assert_eq!(game.players[0].life, 22);
        assert!(game.players[0].hand.is_empty());
    }
    let (mut game, source) = fixture(&PROGRAMS, 1);
    game.players[0].life = 1;
    begin(&mut game, source, 1);
    choose_decision_by_label(&mut game, PlayerId::One, "Pay the cost");
    let decision = &game.pending_decisions[0].observation;
    assert!(
        decision
            .options
            .iter()
            .all(|option| option.label == "Decline" || option.label == "Start selections over")
    );
    assert_eq!(
        game.players[0].life, 1,
        "the first repetition is still tentative"
    );
    choose_decision_by_label(&mut game, PlayerId::One, "Decline");
    assert_eq!(game.players[0].life, 1);
}

#[test]
fn payment_programs_herald_selects_distinct_lands_together_without_combination_enumeration() {
    for prepared in [false, true] {
        for cancel in [false, true] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            let mut herald = creature(161_030, cards::HERALD_OF_LESHRAC, PlayerId::One);
            let source = herald.card.id;
            herald.set_counters(CounterKind::named("age"), 1);
            game.battlefield.push(herald);
            for id in 161_031..161_037 {
                game.battlefield
                    .push(creature(id, cards::ISLAND, PlayerId::Two));
            }
            game.step = Step::Upkeep;
            game.handle_upkeep_triggers();
            game.finish_rules_procedure();
            game.resolve_stack_top();
            let decision = game.pending_decisions[0].observation.clone();
            assert_eq!(
                (decision.minimum, decision.maximum, decision.options.len()),
                (2, 2, 6)
            );
            assert!(
                game.battlefield
                    .iter()
                    .filter(|permanent| permanent.card.id != source)
                    .all(|permanent| permanent.controller == PlayerId::Two)
            );
            if cancel {
                game.apply(
                    PlayerId::One,
                    Action::CancelDecision {
                        decision: decision.id,
                    },
                )
                .unwrap();
                assert!(
                    game.battlefield
                        .iter()
                        .all(|permanent| permanent.controller == PlayerId::Two)
                );
                assert!(
                    game.players[0]
                        .graveyard
                        .iter()
                        .any(|card| card.definition == cards::HERALD_OF_LESHRAC)
                );
            } else {
                select(&mut game, &[GameObjectId(161_031), GameObjectId(161_032)]);
                assert_eq!(
                    game.battlefield
                        .iter()
                        .filter(|permanent| permanent.controller == PlayerId::One)
                        .count(),
                    3
                );
                assert_eq!(game.battlefield[0].counters(CounterKind::named("age")), 2);
            }
        }
    }
}

#[test]
fn payment_programs_repeated_life_gain_preserves_each_ordinary_event() {
    let mut game = ready_game();
    let mut wall = creature(161_040, cards::WALL_OF_SHARDS, PlayerId::One);
    wall.set_counters(CounterKind::named("age"), 1);
    game.battlefield.push(wall);
    let pridemate = creature(161_041, cards::ARCHANGEL_OF_THUNE, PlayerId::Two);
    let watcher = pridemate.card.id;
    game.battlefield.push(pridemate);
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    game.resolve_stack_top();
    choose_decision_by_label(&mut game, PlayerId::One, "Have an opponent gain 2 life");
    drain_pending(&mut game);
    assert_eq!(game.players[1].life, 22);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == watcher)
            .unwrap()
            .counters(CounterKind::PlusOnePlusOne),
        2
    );
}

#[test]
fn payment_programs_joint_mana_reservations_allow_tap_then_sacrifice_but_not_two_sacrifices() {
    for consumes in [false, true] {
        let (mut game, source) = fixture(&PROGRAMS, 1);
        let payer = creature(
            161_050,
            if consumes {
                cards::SKIRK_PROSPECTOR
            } else {
                cards::LLANOWAR_ELVES
            },
            PlayerId::One,
        );
        let id = payer.card.id;
        game.battlefield.push(payer);
        begin(&mut game, source, 2);
        select(&mut game, &[id]);
        if consumes {
            assert_eq!(game.battlefield.len(), 2);
            assert_eq!(game.players[0].life, 20);
            assert!(
                game.pending_decisions[0]
                    .observation
                    .options
                    .iter()
                    .all(|option| option.label != "Pay the cost")
            );
            choose_decision_by_label(&mut game, PlayerId::One, "Start selections over");
            assert_eq!(game.pending_decisions[0].observation.options.len(), 2);
            let decision = game.pending_decisions[0].observation.id;
            game.apply(PlayerId::One, Action::CancelDecision { decision })
                .unwrap();
            assert_eq!(game.battlefield.len(), 2);
        } else {
            assert_eq!(game.battlefield.len(), 1);
            assert_eq!(game.players[0].graveyard.len(), 1);
            assert_eq!(game.players[0].life, 22);
        }
    }
}

#[test]
fn payment_programs_upkeep_is_a_tagged_ordinary_program_and_checks_its_source() {
    let ability = abilities::cumulative_upkeep!(CostDef::PayLife(1));
    assert_eq!(ability.mechanics, &[abilities::CUMULATIVE_UPKEEP]);
    let Some(EffectDef::IfCondition {
        then: EffectDef::Sequence(steps),
        ..
    }) = ability.declarative_effect()
    else {
        panic!("upkeep is a guarded ordinary sequence");
    };
    assert!(matches!(
        steps,
        [EffectDef::AddCounters { .. }, EffectDef::PayOr(_)]
    ));
    let mut game = ready_game();
    let source = creature(161_060, cards::GALLOWBRAID, PlayerId::One);
    let id = source.card.id;
    game.battlefield.push(source);
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    game.sacrifice_permanents(&[id]);
    game.resolve_stack_top();
    assert!(game.pending_decisions.is_empty());
    assert_eq!(game.players[0].life, 20);
}

#[test]
fn payment_programs_reserve_life_owed_by_the_bundle_from_channel() {
    for life in [1, 2] {
        let (mut game, source) = fixture(&PROGRAMS, 1);
        resolve_channel(&mut game);
        game.players[0].life = life;
        for id in 161_070..161_072 {
            game.players[0]
                .hand
                .push(card(id, cards::ISLAND, PlayerId::One));
        }
        begin(&mut game, source, 0);
        select(&mut game, &[GameObjectId(161_070)]);
        select(&mut game, &[GameObjectId(161_071)]);
        if life == 1 {
            assert_eq!(game.players[0].life, 1);
            assert_eq!(game.players[0].hand.len(), 2);
            assert!(
                game.pending_decisions[0]
                    .observation
                    .options
                    .iter()
                    .all(|option| option.label != "Pay the cost")
            );
            choose_decision_by_label(&mut game, PlayerId::One, "Decline");
        } else {
            assert!(game.players[0].hand.is_empty());
            assert_eq!(
                game.players[0].life, 3,
                "one life for Channel, one for the cost, then gain three"
            );
        }
    }
}
