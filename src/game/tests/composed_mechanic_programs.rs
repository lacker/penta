use super::*;
use crate::card::sets;
use crate::card::{AbilityLabel, AbilityPredicateDef, PayOrDef};

const MAINTENANCE: AbilityLabel = AbilityLabel("test maintenance");
static PAY: EffectDef = EffectDef::PayOr(
    PayOrDef::optional_or(
        &[CostDef::repeated(
            &[CostDef::Parameter],
            &ValueDef::Constant(2),
        )],
        &EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(3),
        },
        &EffectDef::None,
    )
    .labeled(MAINTENANCE)
    .with_visibility(ChoiceVisibilityDef::Public),
);

pub(in crate::game) static TWO_PAYMENTS: [AbilityDef; 1] = [AbilityDef::triggered(
    "Two payments with separate cost scopes",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::Sequence(&[
        EffectDef::WithCosts {
            costs: &[CostDef::PayLife(1)],
            effect: &PAY,
        },
        EffectDef::WithCosts {
            costs: &[CostDef::PayLife(2)],
            effect: &PAY,
        },
    ]),
)
.labeled(MAINTENANCE)];

pub(in crate::game) fn staged(abilities: &'static [AbilityDef]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.set_prepared_engine_enabled(false);
    game.step = Step::Upkeep;
    let definition_id = CardDefinitionId::from_uuid("00000000-0000-0000-0000-00000000276b");
    let mut definition = CardDefinition::new(
        definition_id,
        "Composed payment fixture",
        sets::magic_2014::SET,
        CardRules::new_creature(ManaCost::new(1, 0), &[], 2, 2).with_abilities(abilities),
    );
    synchronize_single_part_definition(&mut definition);
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = creature(31_100, definition_id, PlayerId::One);
    let id = source.card.id;
    game.battlefield.push(source);
    (game, id)
}

pub(in crate::game) fn start(game: &mut Game) {
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    game.resolve_stack_top();
}

#[test]
fn composed_mechanic_programs_preserve_cost_scope_between_suspended_payments() {
    let (mut game, _) = staged(&TWO_PAYMENTS);
    start(&mut game);
    assert!(matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::PayOr {
            payment: ResolvedEffectPayment::Life(2),
            ..
        }
    ));
    choose_decision_by_label(&mut game, PlayerId::One, "Pay 2 life");
    assert_eq!(game.players[0].life, 21);
    assert!(matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::PayOr {
            payment: ResolvedEffectPayment::Life(4),
            ..
        }
    ));
    choose_decision_by_label(&mut game, PlayerId::One, "Pay 4 life");
    assert_eq!(game.players[0].life, 20);
}

#[test]
fn composed_mechanic_programs_do_not_partially_pay_a_repeated_cost() {
    let (mut game, _) = staged(&TWO_PAYMENTS);
    game.players[0].life = 1;
    start(&mut game);
    assert!(game.pending_decisions.is_empty());
    assert_eq!(game.players[0].life, 1);
}

pub(in crate::game) static PARTIALLY_REPEATED: [AbilityDef; 1] = [AbilityDef::triggered(
    "Pay once plus a repeated sub-list",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::WithCosts {
        costs: &[CostDef::PayLife(2)],
        effect: &EffectDef::PayOr(PayOrDef::optional(
            &[CostDef::All(&[
                CostDef::PayLife(1),
                CostDef::repeated(
                    &[CostDef::Parameter],
                    &ValueDef::CountersOnSource(CounterKind::named("age")),
                ),
            ])],
            &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        )),
    },
)];

#[test]
fn composed_mechanic_programs_repeat_only_the_selected_cost_subtree() {
    for (life, can_pay) in [(4, false), (10, true)] {
        let (mut game, id) = staged(&PARTIALLY_REPEATED);
        game.players[0].life = life;
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == id)
            .unwrap()
            .set_counters(CounterKind::named("age"), 2);
        start(&mut game);
        if can_pay {
            choose_decision_by_label(&mut game, PlayerId::One, "Pay 5 life");
            assert_eq!(game.players[0].life, life - 5 + 3);
        } else {
            assert_eq!(game.pending_decisions[0].observation.options.len(), 1);
            choose_decision_by_label(&mut game, PlayerId::One, "Decline");
            assert_eq!(
                game.players[0].life, life,
                "the unrepeated sibling must not be paid on its own"
            );
        }
    }
}

static NESTED_REPETITION: [AbilityDef; 1] = [AbilityDef::triggered(
    "Repeat nested cost choices",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::WithCosts {
        costs: &[CostDef::Mana(ManaCost::new(1, 0)), CostDef::PayLife(1)],
        effect: &EffectDef::PayOr(PayOrDef::optional(
            &[CostDef::repeated(
                &[CostDef::Choice(&[
                    CostDef::repeated(&[CostDef::Parameter], &ValueDef::Constant(2)),
                    CostDef::PayLife(5),
                ])],
                &ValueDef::Constant(2),
            )],
            &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        )),
    },
)];

#[test]
fn composed_mechanic_programs_choose_independently_through_nested_repeated_costs() {
    let (mut game, _) = staged(&NESTED_REPETITION);
    game.players[0].life = 8;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    start(&mut game);
    // Only one outer repetition can choose the mana/life branch. The other
    // chooses five life; repeating the same selected branch cannot be paid.
    choose_decision_by_label(&mut game, PlayerId::One, "Pay {2}, Pay 7 life");
    assert_eq!(game.players[0].life, 4);
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

static ZERO_REPETITION: [AbilityDef; 1] = [AbilityDef::triggered(
    "Pay life and repeat a mana cost zero times",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::PayOr(PayOrDef::optional(
        &[
            CostDef::PayLife(1),
            CostDef::repeated(
                &[CostDef::Mana(ManaCost::new(1, 0))],
                &ValueDef::Constant(0),
            ),
        ],
        &EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(3),
        },
    )),
)];

#[test]
fn composed_mechanic_programs_zero_repetition_introduces_no_mana_payment() {
    let (mut game, _) = staged(&ZERO_REPETITION);
    start(&mut game);
    assert!(matches!(
        &game.pending_decisions[0].continuation,
        DecisionContinuation::PayOr {
            payment: ResolvedEffectPayment::All(payments),
            ..
        } if matches!(payments.as_slice(), [ResolvedEffectPayment::Life(1)])
    ));
    choose_decision_by_label(&mut game, PlayerId::One, "Pay 1 life");
    assert_eq!(game.players[0].life, 22);
}

static TWO_UPKEEPS: [AbilityDef; 2] = [
    abilities::cumulative_upkeep(&[CostDef::PayLife(1)]).override_text("First upkeep"),
    abilities::cumulative_upkeep(&[CostDef::PayLife(2)]).override_text("Second upkeep"),
];

#[test]
fn composed_mechanic_programs_keep_multiple_named_ability_instances() {
    let (mut game, id) = staged(&TWO_UPKEEPS);
    game.handle_upkeep_triggers();
    assert_eq!(game.pending_triggers.len(), 2);
    assert_ne!(
        game.pending_triggers[0].source.ability,
        game.pending_triggers[1].source.ability
    );
    for ability in TWO_UPKEEPS {
        assert!(AbilityPredicateDef::Label(&AbilityLabel::CUMULATIVE_UPKEEP).matches(&ability));
    }
    assert!(
        game.pending_triggers
            .iter()
            .all(|trigger| trigger.source.object == id)
    );
    game.finish_rules_procedure();
    for _ in 0..12 {
        if let Some(decision) = game.pending_decisions.first() {
            let observation = decision.observation.clone();
            let options = if matches!(decision.continuation, DecisionContinuation::PayOr { .. }) {
                vec![
                    observation
                        .options
                        .iter()
                        .find(|option| option.id != 0)
                        .unwrap()
                        .id,
                ]
            } else {
                observation
                    .options
                    .iter()
                    .take(observation.minimum)
                    .map(|option| option.id)
                    .collect()
            };
            game.apply(
                observation.player,
                Action::ChooseDecision {
                    decision: observation.id,
                    options,
                },
            )
            .unwrap();
        } else if !game.stack.is_empty() {
            game.resolve_stack_top();
        } else {
            break;
        }
    }
    let source = game.battlefield.iter().find(|p| p.card.id == id).unwrap();
    assert_eq!(source.counters(CounterKind::named("age")), 2);
    assert!(
        matches!(game.players[0].life, 15 | 16),
        "both separately scaled payments were made"
    );
}

pub(in crate::game) static DRAW_PAYMENT: [AbilityDef; 1] = [AbilityDef::triggered(
    "Pay by drawing twice",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::WithCosts {
        costs: &[CostDef::DrawCards(1), CostDef::PayLife(1)],
        effect: &PAY,
    },
)];

#[test]
fn composed_mechanic_programs_prepared_fallback_preserves_the_program() {
    let (mut reference, _) = staged(&TWO_PAYMENTS);
    let (mut prepared, _) = staged(&TWO_PAYMENTS);
    prepared.set_prepared_engine_enabled(true);
    for game in [&mut reference, &mut prepared] {
        start(game);
        choose_decision_by_label(game, PlayerId::One, "Pay 2 life");
        choose_decision_by_label(game, PlayerId::One, "Pay 4 life");
    }
    assert_eq!(reference.players, prepared.players);
    assert_eq!(reference.events, prepared.events);
    assert_eq!(reference.pending_procedures, prepared.pending_procedures);
    assert_eq!(reference.pending_events, prepared.pending_events);
    assert_eq!(
        reference.checkpoint_json(PlayerId::One),
        prepared.checkpoint_json(PlayerId::One)
    );
}

pub(in crate::game) static CHOICE_BATCH: [AbilityDef; 1] = [AbilityDef::triggered(
    "Pay mana or life for each repetition",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::WithCosts {
        costs: &[CostDef::Choice(&[
            CostDef::Mana(ManaCost::new(1, 0)),
            CostDef::PayLife(2),
        ])],
        effect: &PAY,
    },
)];

static MIXED_BATCH: [AbilityDef; 1] = [AbilityDef::triggered(
    "Pay mana and life for every repetition",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::WithCosts {
        costs: &[CostDef::Mana(ManaCost::new(1, 0)), CostDef::PayLife(1)],
        effect: &PAY,
    },
)];

#[test]
fn composed_mechanic_programs_choose_separately_for_each_payment_repetition() {
    let (mut game, _) = staged(&CHOICE_BATCH);
    game.players[0].life = 3;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    start(&mut game);
    assert_eq!(
        game.pending_decisions[0].observation.options.len(),
        2,
        "only the mixed plan can pay the complete obligation"
    );
    choose_decision_by_label(&mut game, PlayerId::One, "Pay {1}, Pay 2 life");
    assert_eq!(
        game.players[0].life, 4,
        "two life paid, then the paid branch gains three"
    );
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

#[test]
fn composed_mechanic_programs_reserve_life_shared_with_mana_abilities() {
    for starting_life in [3, 5] {
        let (mut game, _) = staged(&MIXED_BATCH);
        game.step = Step::PrecombatMain;
        resolve_channel(&mut game);
        game.step = Step::Upkeep;
        game.players[0].life = starting_life;
        start(&mut game);
        if starting_life == 3 {
            assert!(
                game.pending_decisions.is_empty(),
                "no partial plan is offered"
            );
            assert_eq!(game.players[0].life, 3);
            assert_eq!(game.players[0].mana_pool.total(), 0);
        } else {
            choose_decision_by_label(&mut game, PlayerId::One, "Pay {2}, Pay 2 life");
            assert_eq!(
                game.players[0].life, 4,
                "two life plus two Channel activations, then gain three"
            );
        }
    }
}

static MIXED_OBJECT_BATCH: [AbilityDef; 1] = [AbilityDef::triggered(
    "Pay life and discard for every repetition",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::WithCosts {
        costs: &[CostDef::PayLife(2), CostDef::DiscardCards(1)],
        effect: &PAY,
    },
)];

#[test]
fn composed_mechanic_programs_require_the_entire_repeated_object_and_life_list() {
    for card_count in [1, 2] {
        let (mut game, _) = staged(&MIXED_OBJECT_BATCH);
        game.players[0].life = 5;
        game.players[0].hand = (0..card_count)
            .map(|i| card(31_150 + i, cards::PLAINS, PlayerId::One))
            .collect();
        game.players[0].graveyard.clear();
        start(&mut game);
        if card_count == 1 {
            assert!(game.pending_decisions.is_empty());
            assert_eq!(
                game.players[0].life, 5,
                "an incomplete group cannot spend life"
            );
            assert_eq!(game.players[0].hand.len(), 1);
            assert!(game.players[0].graveyard.is_empty());
        } else {
            let decision = game.observe(PlayerId::One).decision.unwrap();
            assert_eq!(decision.options.len(), 2);
            assert_eq!(decision.options[1].members.len(), 2);
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![decision.options[1].id],
                },
            )
            .unwrap();
            assert_eq!(game.players[0].life, 4, "pay four life, then gain three");
            assert!(game.players[0].hand.is_empty());
            assert_eq!(game.players[0].graveyard.len(), 2);
        }
    }
}

#[test]
fn composed_mechanic_programs_combine_choices_across_the_whole_cost_list() {
    const ALTERNATIVE: CostDef =
        CostDef::Choice(&[CostDef::Mana(ManaCost::new(1, 0)), CostDef::PayLife(2)]);
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
        "Two independent cost choices, each repeated twice",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::WithCosts {
            costs: &[ALTERNATIVE, ALTERNATIVE],
            effect: &PAY,
        },
    )];
    let (mut game, _) = staged(&ABILITIES);
    game.players[0].life = 5;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    start(&mut game);
    assert_eq!(
        game.pending_decisions[0].observation.options.len(),
        2,
        "equivalent complete alternatives appear once and reserve the entire life cost"
    );
    choose_decision_by_label(&mut game, PlayerId::One, "Pay {2}, Pay 4 life");
    assert_eq!(game.players[0].life, 4);
    assert_eq!(game.players[0].mana_pool.total(), 0);
}
