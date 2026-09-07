use super::*;
use crate::card::{
    AggregateOperationDef, CostQuantityDef, ObjectSetValueAtLeastDef, ObjectSetValueDef,
    ObjectValueDef,
};

const GAIN: EffectDef = EffectDef::GainLife {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::Constant(3),
};
static ACTIVATIONS: [AbilityDef; 3] = [
    AbilityDef::activated(
        "Pay and sacrifice two creatures.",
        &[
            CostDef::Mana(mana_cost!("{1}")),
            CostDef::TapSource,
            CostDef::PayLife(1),
            CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(2),
            ),
        ],
        GAIN,
    ),
    AbilityDef::activated(
        "Pay and discard two cards.",
        &[
            CostDef::Mana(mana_cost!("{1}")),
            CostDef::TapSource,
            CostDef::PayLife(1),
            CostDef::discard(ObjectPredicateDef::Any, CostQuantityDef::Fixed(2)),
        ],
        GAIN,
    ),
    AbilityDef::activated(
        "Pay and exile two cards from your hand.",
        &[
            CostDef::Mana(mana_cost!("{1}")),
            CostDef::TapSource,
            CostDef::PayLife(1),
            CostDef::exile(
                ObjectPredicateDef::Any,
                ZoneKind::Hand,
                CostQuantityDef::Fixed(2),
            ),
        ],
        GAIN,
    ),
];

pub(super) fn fixture(abilities: &'static [AbilityDef], power: i16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    let id = CardDefinitionId::new(160_500);
    let mut definition = CardDefinition::new(
        id,
        "Object cost fixture",
        CardSet::Magic2014,
        CardRules::new_creature(ManaCost::default(), &[], power, 5).with_abilities(abilities),
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
    let source = creature(160_501, id, PlayerId::One);
    let source_id = source.card.id;
    game.battlefield.push(source);
    (game, source_id)
}

pub(super) fn activate(game: &mut Game, source_id: GameObjectId, index: usize) {
    game.priority = PlayerId::One;
    let action = game.legal_actions(PlayerId::One).into_iter().filter(|action|
        matches!(action, Action::ActivateAbility { source, .. } if *source == source_id)
    ).nth(index).expect("the complete cost is payable");
    game.apply(PlayerId::One, action).unwrap();
}

pub(super) fn select(game: &mut Game, objects: &[GameObjectId]) {
    let decision = game.pending_decisions[0].observation.clone();
    let options = objects
        .iter()
        .map(|object| {
            decision
                .options
                .iter()
                .find(|option| option.card.is_some_and(|(id, _)| id == *object))
                .unwrap()
                .id
        })
        .collect();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .unwrap();
}

#[test]
fn object_costs_collect_before_mana_tap_life_or_zone_mutation_and_cancel_cleanly() {
    for prepared in [false, true] {
        for ability in 0..3 {
            for cancel in [false, true] {
                let (mut game, source) = fixture(&ACTIVATIONS, 1);
                game.set_prepared_engine_enabled(prepared);
                let payers = [GameObjectId(160_510), GameObjectId(160_511)];
                for id in payers {
                    game.battlefield
                        .push(creature(id.0, cards::GRIZZLY_BEARS, PlayerId::One));
                    game.players[0]
                        .hand
                        .push(card(id.0 + 10, cards::ISLAND, PlayerId::One));
                }
                game.players[0].mana_pool.colorless = 1;
                activate(&mut game, source, ability);
                let decision = game.pending_decisions[0].observation.clone();
                assert_eq!((decision.minimum, decision.maximum), (2, 2));
                assert_eq!(
                    decision.visibility,
                    if ability == 0 {
                        DecisionVisibility::Public
                    } else {
                        DecisionVisibility::Private
                    }
                );
                if ability != 0 {
                    assert!(
                        game.observe(PlayerId::Two).decision.is_none(),
                        "opponents cannot see hand candidates"
                    );
                }
                assert_eq!(game.players[0].life, 20);
                assert_eq!(game.players[0].mana_pool.colorless, 1);
                assert_eq!(game.players[0].hand.len(), 2);
                assert_eq!(game.battlefield.len(), 3);
                assert!(!game.battlefield[0].tapped);
                assert!(game.stack.is_empty());
                if cancel {
                    game.apply(
                        PlayerId::One,
                        Action::CancelDecision {
                            decision: decision.id,
                        },
                    )
                    .unwrap();
                    assert_eq!(game.players[0].life, 20);
                    assert_eq!(game.players[0].mana_pool.colorless, 1);
                    assert_eq!(game.players[0].hand.len(), 2);
                    assert_eq!(game.battlefield.len(), 3);
                    assert!(!game.battlefield[0].tapped);
                    assert!(game.stack.is_empty());
                } else {
                    let selected = if ability == 0 {
                        payers
                    } else {
                        payers.map(|id| GameObjectId(id.0 + 10))
                    };
                    select(&mut game, &selected);
                    assert_eq!(game.players[0].life, 19);
                    assert_eq!(game.players[0].mana_pool.colorless, 0);
                    assert!(game.battlefield[0].tapped);
                    assert_eq!(game.stack.len(), 1);
                    assert_eq!(
                        game.players[0].exile.len(),
                        if ability == 2 { 2 } else { 0 }
                    );
                    assert_eq!(
                        game.players[0].graveyard.len(),
                        if ability == 2 { 0 } else { 2 }
                    );
                    drain_pending(&mut game);
                    assert_eq!(game.players[0].life, 22);
                }
            }
        }
    }
}

#[test]
fn object_costs_reserve_sacrifice_members_but_allow_tap_for_mana_before_sacrifice() {
    for consumes in [false, true] {
        let (mut game, source) = fixture(&ACTIVATIONS, 1);
        let payer = creature(
            160_520,
            if consumes {
                cards::SKIRK_PROSPECTOR
            } else {
                cards::LLANOWAR_ELVES
            },
            PlayerId::One,
        );
        let payer_id = payer.card.id;
        let other = creature(160_521, cards::GRIZZLY_BEARS, PlayerId::One);
        let other_id = other.card.id;
        game.battlefield.extend([payer, other]);
        activate(&mut game, source, 0);
        let decision = game.pending_decisions[0].observation.clone();
        let options = decision
            .options
            .iter()
            .filter(|option| {
                option
                    .card
                    .is_some_and(|(id, _)| [payer_id, other_id].contains(&id))
            })
            .map(|option| option.id)
            .collect();
        let result = game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options,
            },
        );
        if consumes {
            assert!(
                result.is_err(),
                "Skirk Prospector cannot sacrifice itself twice"
            );
            assert_eq!(game.pending_decisions.len(), 1);
            assert_eq!(game.battlefield.len(), 3);
            assert_eq!(game.players[0].life, 20);
            assert!(game.stack.is_empty());
        } else {
            result.unwrap();
            assert_eq!(game.battlefield.len(), 1);
            assert_eq!(game.players[0].graveyard.len(), 2);
            assert_eq!(game.stack.len(), 1);
        }
    }
}

#[test]
fn object_costs_aggregate_selection_counts_negative_power_and_allows_extra_members() {
    for prepared in [false, true] {
        let (mut game, negative) = fixture(&[], -1);
        game.set_prepared_engine_enabled(prepared);
        let angels = [160_530, 160_531, 160_532].map(|id| {
            game.battlefield
                .push(creature(id, cards::SERRA_ANGEL, PlayerId::One));
            GameObjectId(id)
        });
        let extras = [160_533, 160_534].map(|id| {
            game.battlefield
                .push(creature(id, cards::GRIZZLY_BEARS, PlayerId::One));
            GameObjectId(id)
        });
        let dread = game
            .put_onto_battlefield(PlayerId::One, cards::PHYREXIAN_DREADNOUGHT)
            .unwrap();
        game.finish_rules_procedure();
        game.resolve_stack_top();
        select(&mut game, &[negative]);
        for angel in angels {
            select(&mut game, &[angel]);
        }
        assert!(
            !game.pending_decisions[0]
                .observation
                .options
                .iter()
                .any(|option| option.id == 0),
            "12 minus 1 does not reach 12"
        );
        assert_eq!(
            game.battlefield.len(),
            7,
            "tentative choices sacrifice nothing"
        );
        for extra in extras {
            select(&mut game, &[extra]);
        }
        choose_decision_by_label(&mut game, PlayerId::One, "Pay selected objects");
        drain_pending(&mut game);
        assert_eq!(game.battlefield.len(), 1);
        assert_eq!(game.battlefield[0].card.id, dread);
        assert_eq!(game.players[0].graveyard.len(), 6);
    }
}

#[test]
fn object_costs_selection_validates_source_identity_uniqueness_and_signed_threshold() {
    static REQUIREMENT: ObjectSetValueAtLeastDef = ObjectSetValueAtLeastDef {
        value: ObjectSetValueDef::Aggregate {
            select: ObjectValueDef::Power,
            operation: AggregateOperationDef::Sum,
        },
        minimum: 4,
    };
    let (mut game, source) = fixture(&[], -1);
    let angel = creature(160_550, cards::SERRA_ANGEL, PlayerId::One);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    let source_cost = CostDef::sacrifice(ObjectPredicateDef::Source, CostQuantityDef::Fixed(1));
    assert_eq!(
        game.object_cost_candidates(PlayerId::One, source, source_cost),
        [source]
    );
    let quantity = CostQuantityDef::ObjectSetValueAtLeast(&REQUIREMENT);
    assert!(game.object_selection_is_valid(&[source, angel_id], &[angel_id], quantity));
    assert!(!game.object_selection_is_valid(&[source, angel_id], &[source, angel_id], quantity));
    assert!(!game.object_selection_is_valid(&[source, angel_id], &[angel_id, angel_id], quantity));
    assert!(!game.object_selection_is_valid(&[source], &[angel_id], quantity));
    game.battlefield
        .push(creature(160_551, cards::ISLAND, PlayerId::One));
    assert_eq!(
        game.object_set_value(
            &[source, GameObjectId(160_551)],
            ObjectSetValueDef::CardTypeCount
        ),
        2
    );
}

#[test]
fn object_costs_activation_sacrifice_is_one_batch_for_one_or_more_listeners() {
    static ABILITIES: [AbilityDef; 3] = [
        AbilityDef::activated(
            "Sacrifice two creatures.",
            &[CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(2),
            )],
            EffectDef::None,
        ),
        AbilityDef::triggered(
            "Whenever you sacrifice a permanent.",
            TriggerEventDef::mechanic_performed_on(
                abilities::SACRIFICE,
                ObjectPredicateDef::Any,
                PlayerRelation::You,
            ),
            EffectDef::None,
        ),
        AbilityDef::triggered(
            "Whenever you sacrifice one or more permanents.",
            TriggerEventDef::MechanicPerformed {
                mechanic: abilities::SACRIFICE,
                player: PlayerRelation::You,
                object: Some(ObjectPredicateDef::Any),
                one_or_more: true,
            },
            EffectDef::None,
        ),
    ];
    let (mut game, source) = fixture(&ABILITIES, 1);
    let payers = [160_560, 160_561].map(|id| {
        game.battlefield
            .push(creature(id, cards::GRIZZLY_BEARS, PlayerId::One));
        GameObjectId(id)
    });
    activate(&mut game, source, 0);
    select(&mut game, &payers);
    let DecisionContinuation::TriggerOrder { batch, .. } = &game.pending_decisions[0].continuation
    else {
        panic!("simultaneous sacrifice triggers");
    };
    assert_eq!(
        batch.triggers.len(),
        3,
        "two per-object occurrences and one batch occurrence"
    );
}
