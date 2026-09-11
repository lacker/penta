use super::composed_mechanic_programs::{staged, start};
use super::*;
use crate::card::{GameActionChoiceDef, GameActionDef, actions};

pub(in crate::game) const DISCARD_THREE: GameActionDef = actions::choose_discard(3);

pub(in crate::game) static DISCARD_COST: [AbilityDef; 1] = [AbilityDef::triggered(
    "Discard three as a cost",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::PayOr(PayOrDef::optional(
        &[DISCARD_THREE.as_cost()],
        &EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(5),
        },
    )),
)];

pub(in crate::game) static DISCARD_EFFECT: [AbilityDef; 1] = [AbilityDef::triggered(
    "Discard three as an effect",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    DISCARD_THREE.as_effect(),
)];

pub(in crate::game) static SACRIFICE_COST: [AbilityDef; 1] = [AbilityDef::triggered(
    "Sacrifice a land as a cost",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::PayOr(PayOrDef::optional(
        &[GameActionDef::Choose(GameActionChoiceDef {
            binding: Binding!("objects"),
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerSetDef::Related(PlayerRelation::You),
            )),
            amount: ValueDef::Constant(1),
            visibility: ChoiceVisibilityDef::Public,
            then: &GameActionDef::SacrificeYours {
                object: EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("objects"))),
            },
        })
        .as_cost()],
        &EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(5),
        },
    )),
)];

pub(in crate::game) fn add_hand(game: &mut Game, count: u32) {
    game.players[0].hand = (0..count)
        .map(|offset| card(31_200 + offset, cards::ISLAND, PlayerId::One))
        .collect();
}

#[test]
fn game_action_programs_effect_discards_available_cards_but_cost_requires_three() {
    for count in 0..=2 {
        let (mut effect, _) = staged(&DISCARD_EFFECT);
        add_hand(&mut effect, count);
        start(&mut effect);
        assert!(effect.players[0].hand.is_empty());
        assert_eq!(effect.players[0].graveyard.len(), count as usize);
        let (mut payment, _) = staged(&DISCARD_COST);
        add_hand(&mut payment, count);
        start(&mut payment);
        assert_eq!(payment.players[0].hand.len(), count as usize);
        assert!(payment.players[0].graveyard.is_empty());
        assert_eq!(payment.pending_decisions[0].observation.options.len(), 1);
        choose_decision_by_label(&mut payment, PlayerId::One, "Decline");
        assert_eq!(payment.players[0].life, 20);
    }
}

#[test]
fn game_action_programs_cost_commits_only_a_complete_selection() {
    let (mut game, _) = staged(&DISCARD_COST);
    add_hand(&mut game, 4);
    start(&mut game);
    assert_eq!(game.players[0].hand.len(), 4);
    assert_eq!(game.players[0].life, 20);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert!(
        decision
            .options
            .iter()
            .skip(1)
            .all(|option| option.members.len() == 3)
    );
    choose_decision_by_label(&mut game, PlayerId::One, "Discard Island, Island, Island");
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].graveyard.len(), 3);
    assert_eq!(game.players[0].life, 25);
}

#[test]
fn game_action_programs_sacrifice_payment_waits_for_replacement_choices() {
    let (mut game, _) = staged(&SACRIFICE_COST);
    game.battlefield.extend([
        creature(31_210, cards::ISLAND, PlayerId::One),
        creature(31_211, cards::REST_IN_PEACE, PlayerId::Two),
        creature(31_212, cards::REST_IN_PEACE, PlayerId::Two),
    ]);
    start(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Sacrifice Island");
    assert_eq!(
        game.players[0].life, 20,
        "paid branch waits for replacement execution"
    );
    assert!(
        game.pending_procedures
            .iter()
            .any(|procedure| matches!(procedure, PendingProcedure::CompletePayment { .. }))
    );
    let decision = game.observe(PlayerId::One).decision.unwrap();
    let option = decision.options[0].id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .unwrap();
    assert!(game.players[0].graveyard.is_empty());
    assert_eq!(game.players[0].exile.len(), 1);
    assert_eq!(
        game.players[0].life, 25,
        "replacement destination does not negate payment"
    );
}

pub(in crate::game) static DRAW_THEN_ACTION: [AbilityDef; 1] = [AbilityDef::triggered(
    "Draw and sacrifice to gain life",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::PayOr(PayOrDef::optional(
        &[
            CostDef::repeated(&[CostDef::DrawCards(1)], &ValueDef::Constant(1)),
            actions::choose_sacrifice(1)
                .matching(ObjectPredicateDef::HasType(CardType::Land))
                .as_cost(),
        ],
        &EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(5),
        },
    )),
)];

static SEQUENTIAL_COST: [AbilityDef; 1] = [AbilityDef::triggered(
    "Sacrifice enchantments, then a land",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::PayOr(PayOrDef::optional(
        &[actions::sequence(&[
            actions::choose_sacrifice(2)
                .matching(ObjectPredicateDef::HasType(CardType::Enchantment)),
            actions::choose_sacrifice(1).matching(ObjectPredicateDef::HasType(CardType::Land)),
        ])
        .as_cost()],
        &EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(5),
        },
    )),
)];

#[test]
fn game_action_programs_cost_sequence_finishes_replacements_before_next_action() {
    let (mut game, _) = staged(&SEQUENTIAL_COST);
    game.battlefield.extend([
        creature(31_240, cards::REST_IN_PEACE, PlayerId::One),
        creature(31_241, cards::REST_IN_PEACE, PlayerId::One),
        creature(31_242, cards::ISLAND, PlayerId::One),
    ]);
    start(&mut game);
    choose_decision_by_label(
        &mut game,
        PlayerId::One,
        "Sacrifice Rest in Peace, Rest in Peace, Sacrifice Island",
    );
    assert_eq!(game.players[0].life, 20);
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.id == GameObjectId(31_242))
    );
    while let Some(decision) = game.observe(PlayerId::One).decision {
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![decision.options[0].id],
            },
        )
        .unwrap();
    }
    assert_eq!(game.players[0].exile.len(), 2);
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "land sacrifice follows the completed enchantment sacrifice"
    );
    assert_eq!(game.players[0].life, 25);
}

static OPPONENT_DISCARDS: [AbilityDef; 1] = [AbilityDef::triggered(
    "Opponent may discard three",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::PayOr(
        PayOrDef::optional(
            &[DISCARD_THREE.as_cost()],
            &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
        )
        .with_payer(PlayerSetDef::Related(PlayerRelation::Opponent)),
    ),
)];

#[test]
fn game_action_programs_use_the_payer_for_selection_and_execution() {
    let (mut game, _) = staged(&OPPONENT_DISCARDS);
    add_hand(&mut game, 3);
    game.players[1].hand = (0..3)
        .map(|offset| card(31_260 + offset, cards::FOREST, PlayerId::Two))
        .collect();
    start(&mut game);
    choose_decision_by_label(&mut game, PlayerId::Two, "Discard Forest, Forest, Forest");
    assert_eq!(game.players[0].hand.len(), 3);
    assert!(game.players[1].hand.is_empty());
    assert_eq!(game.players[1].graveyard.len(), 3);
    assert_eq!(
        game.players[0].life, 25,
        "the paid branch keeps the original effect controller"
    );
}

#[test]
fn game_action_programs_prepared_fallback_matches_reference_execution() {
    let (mut reference, _) = staged(&DISCARD_COST);
    let (mut prepared, _) = staged(&DISCARD_COST);
    prepared.set_prepared_engine_enabled(true);
    for game in [&mut reference, &mut prepared] {
        add_hand(game, 4);
        start(game);
        choose_decision_by_label(game, PlayerId::One, "Discard Island, Island, Island");
    }
    assert_eq!(reference.players, prepared.players);
    assert_eq!(reference.events, prepared.events);
    assert_eq!(
        reference.checkpoint_json(PlayerId::One),
        prepared.checkpoint_json(PlayerId::One)
    );
}

const CUSTOM_DISCARD: GameActionDef = actions::choose(
    Binding!("cards"),
    ObjectSetDef::Query(ObjectQueryDef::owned_by(
        ObjectPredicateDef::Any,
        &[ZoneKind::Hand],
        PlayerSetDef::Related(PlayerRelation::You),
    )),
    &actions::discard_cards(EffectRecipientDef::objects(ObjectSetDef::Binding(
        Binding!("cards"),
    ))),
)
.matching(ObjectPredicateDef::HasType(CardType::Land))
.with_amount(ValueDef::SourcePower)
.with_visibility(ChoiceVisibilityDef::Private);

static CUSTOM_DISCARD_COST: [AbilityDef; 1] = [AbilityDef::triggered(
    "Discard lands equal to this creature's power to gain life",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::PayOr(PayOrDef::optional(
        &[CUSTOM_DISCARD.as_cost()],
        &EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(5),
        },
    )),
)];

static CUSTOM_DISCARD_EFFECT: [AbilityDef; 1] = [AbilityDef::triggered(
    "An opponent chooses lands from your hand equal to this creature's power to discard",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    CUSTOM_DISCARD
        .with_chooser(PlayerRefDef::Opponent)
        .as_effect(),
)];

#[test]
fn game_action_programs_custom_cost_preserves_filter_binding_and_computed_amount() {
    for count in 1..=2 {
        let (mut game, _) = staged(&CUSTOM_DISCARD_COST);
        add_hand(&mut game, count);
        game.players[0]
            .hand
            .push(card(31_300, cards::GIANT_GROWTH, PlayerId::One));
        start(&mut game);
        assert_eq!(game.players[0].hand.len(), count as usize + 1);
        assert_eq!(game.players[0].life, 20);
        if count == 1 {
            assert_eq!(game.pending_decisions[0].observation.options.len(), 1);
            choose_decision_by_label(&mut game, PlayerId::One, "Decline");
            assert!(game.players[0].graveyard.is_empty());
        } else {
            choose_decision_by_label(&mut game, PlayerId::One, "Discard Island, Island");
            assert_eq!(game.players[0].graveyard.len(), 2);
            assert_eq!(game.players[0].hand.len(), 1);
            assert_eq!(game.players[0].hand[0].id, GameObjectId(31_300));
            assert_eq!(game.players[0].life, 25);
        }
    }
}

#[test]
fn game_action_programs_custom_effect_keeps_candidates_independent_of_chooser() {
    let (mut game, _) = staged(&CUSTOM_DISCARD_EFFECT);
    add_hand(&mut game, 3);
    game.players[0]
        .hand
        .push(card(31_300, cards::GIANT_GROWTH, PlayerId::One));
    game.players[1]
        .hand
        .push(card(31_301, cards::FOREST, PlayerId::Two));
    start(&mut game);
    let decision = game.observe(PlayerId::Two).decision.unwrap();
    assert_eq!(decision.options.len(), 3, "only your lands are candidates");
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: decision
                .options
                .iter()
                .take(2)
                .map(|option| option.id)
                .collect(),
        },
    )
    .unwrap();
    assert_eq!(game.players[0].graveyard.len(), 2);
    assert_eq!(game.players[0].hand.len(), 2);
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.id == GameObjectId(31_300))
    );
    assert_eq!(game.players[1].hand.len(), 1);
    assert!(game.players[1].graveyard.is_empty());
}

#[test]
fn game_action_programs_zone_moves_sequence_through_exact_successors() {
    static ROUND_TRIP: [AbilityDef; 1] = [AbilityDef::triggered(
        "Return after changing zones",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        actions::sequence(&[
            actions::move_to_zone(
                EffectRecipientDef::Source,
                ZoneKind::Graveyard,
                ZonePlacement::Top,
            ),
            actions::move_to_zone(
                EffectRecipientDef::SourceZoneChangeSuccessor,
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        ])
        .as_effect(),
    )];
    let (mut game, source) = staged(&ROUND_TRIP);
    let definition = game.battlefield[0].card.definition;
    start(&mut game);
    assert!(game.players[0].graveyard.is_empty());
    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(game.battlefield[0].card.definition, definition);
    assert_ne!(game.battlefield[0].card.id, source);
}
