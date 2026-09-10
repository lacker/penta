//! Forage is one semantic action, whether instructed by an effect or paid as a cost.

use super::*;

#[test]
fn named_costs_cast_selection_uses_the_payment_source_not_each_candidate() {
    const ID: crate::card::MechanicId = crate::card::MechanicId::from_name("test:source-cost");
    for object in [
        ObjectPredicateDef::Source,
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ] {
        for sacrifice in [false, true] {
            let cost = Box::leak(Box::new(if sacrifice {
                CostDef::sacrifice(object, crate::card::CostQuantityDef::Fixed(1))
            } else {
                CostDef::exile(
                    object,
                    ZoneKind::Graveyard,
                    crate::card::CostQuantityDef::Fixed(1),
                )
            }));
            let (mut game, source) = super::cost_lists::game_with_cost_rules(
                &CardRules::new_sorcery(mana_cost!("{0}")).with_ability(
                    AbilityDef::spell_with_additional_cost(
                        "Pay a named object cost.",
                        &[],
                        CostDef::named(ID, cost),
                        EffectDef::None,
                    ),
                ),
            );
            let payer = GameObjectId(181_100);
            if sacrifice {
                game.battlefield
                    .push(creature(payer.0, cards::GRIZZLY_BEARS, PlayerId::One));
            } else {
                game.players[0]
                    .graveyard
                    .push(card(payer.0, cards::FOREST, PlayerId::One));
            }
            let casts = game
                .legal_actions(PlayerId::One)
                .into_iter()
                .filter(
                    |action| matches!(action, Action::CastSpell { card, .. } if *card == source),
                )
                .collect::<Vec<_>>();
            if object == ObjectPredicateDef::Source {
                assert!(
                    casts.is_empty(),
                    "the spell itself is not an eligible payment object"
                );
            } else {
                assert_eq!(casts.len(), 1, "a different object can pay the cost");
                game.apply(PlayerId::One, casts[0].clone()).unwrap();
                assert_eq!(
                    game.stack.len(),
                    1,
                    "the advertised plan commits successfully"
                );
                assert!(
                    !game
                        .battlefield
                        .iter()
                        .any(|permanent| permanent.card.id == payer)
                );
                assert_eq!(game.players[0].exile.len(), usize::from(!sacrifice));
            }
        }
    }
}

#[test]
fn named_costs_use_authored_identity_filter_and_quantity() {
    const RECLAIM: crate::card::MechanicId = crate::card::MechanicId::from_name("test:reclaim");
    static RULES: [AbilityDef; 2] = [
        AbilityDef::triggered(
            "You may reclaim two lands",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::named(
                    RECLAIM,
                    &CostDef::Exile {
                        object: ObjectPredicateDef::HasType(CardType::Land),
                        from: ZoneKind::Graveyard,
                        quantity: crate::card::CostQuantityDef::Fixed(2),
                    },
                )],
                &EffectDef::None,
            )),
        ),
        AbilityDef::triggered(
            "Whenever you reclaim, gain 4 life",
            TriggerEventDef::MechanicPerformed {
                mechanic: RECLAIM,
                player: PlayerRelation::You,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ),
    ];
    for prepared in [false, true] {
        let (mut game, _) = super::composed_mechanic_programs::staged(&RULES);
        game.set_prepared_engine_enabled(prepared);
        for (index, definition) in [cards::FOREST, cards::SWAMP, cards::LIGHTNING_BOLT]
            .into_iter()
            .enumerate()
        {
            game.players[0].graveyard.push(card(
                181_000 + u32::try_from(index).unwrap(),
                definition,
                PlayerId::One,
            ));
        }
        super::composed_mechanic_programs::start(&mut game);
        game = reconstruct(&game);
        game.set_prepared_engine_enabled(prepared);
        choose(&mut game, vec![1]);
        assert_eq!(game.pending_decisions[0].observation.options.len(), 2);
        assert_eq!(game.pending_decisions[0].observation.minimum, 2);
        game = reconstruct(&game);
        game.set_prepared_engine_enabled(prepared);
        choose(&mut game, vec![0, 1]);
        drain_pending(&mut game);
        assert_eq!(
            game.players[0].life, 24,
            "one named occurrence, not one per object"
        );
        assert_eq!(game.players[0].graveyard.len(), 1);
        assert_eq!(
            game.players[0].graveyard[0].definition,
            cards::LIGHTNING_BOLT
        );
        assert_eq!(game.players[0].exile.len(), 2);
    }
}

fn staged(graveyard_cards: u32) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let cultivator = game
        .put_onto_battlefield(PlayerId::One, cards::CORPSEBERRY_CULTIVATOR)
        .unwrap();
    let opponent = game
        .put_onto_battlefield(PlayerId::Two, cards::CORPSEBERRY_CULTIVATOR)
        .unwrap();
    for index in 0..graveyard_cards {
        game.players[0]
            .graveyard
            .push(card(180_000 + index, cards::FOREST, PlayerId::One));
    }
    drain_pending(&mut game);
    (game, cultivator, opponent)
}

fn counters(game: &Game, source: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .unwrap()
        .counters(CounterKind::PlusOnePlusOne)
}

fn choose(game: &mut Game, options: Vec<u32>) {
    let decision = game.pending_decisions[0].observation.clone();
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .unwrap();
}

fn begin_combat(game: &mut Game) {
    game.step = Step::PrecombatMain;
    game.advance_step();
    assert_eq!(game.step, Step::BeginningOfCombat);
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        game.apply(game.priority, Action::PassPriority).unwrap();
    }
    assert!(matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::NamedCost { branch: None, .. }
    ));
}

fn reconstruct(game: &Game) -> Game {
    let viewer = game.decision_player().unwrap_or(game.priority);
    let (wire, hidden) = checkpoint_fixture(game, viewer);
    Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 180_500)
        .expect("forage choices and triggers reconstruct")
}

#[test]
fn combat_forage_selects_exactly_three_with_linear_options_and_a_separate_counter_trigger() {
    let (mut game, cultivator, opponent) = staged(40);
    game.players[1]
        .graveyard
        .push(card(180_100, cards::SWAMP, PlayerId::Two));
    begin_combat(&mut game);
    game = reconstruct(&game);
    choose(&mut game, vec![1]);
    let decision = game.pending_decisions[0].observation.clone();
    assert_eq!((decision.minimum, decision.maximum), (3, 3));
    assert_eq!(decision.options.len(), 40);
    assert_eq!(decision.source, Some(cultivator));
    assert!(
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![0, 1],
            }
        )
        .is_err()
    );
    assert_eq!(game.players[0].graveyard.len(), 40);
    game = reconstruct(&game);
    choose(&mut game, vec![0, 7, 39]);
    assert_eq!(game.players[0].graveyard.len(), 37);
    assert_eq!(game.players[0].exile.len(), 3);
    assert_eq!(game.players[1].graveyard.len(), 1);
    assert_eq!(counters(&game, cultivator), 0, "the counter uses the stack");
    assert_eq!(game.stack.len(), 1);
    game = reconstruct(&game);
    drain_pending(&mut game);
    assert_eq!(
        counters(&game, cultivator),
        1,
        "one forage, not three exiles"
    );
    assert_eq!(counters(&game, opponent), 0);
}

#[test]
fn optional_forage_can_be_declined_and_never_spends_an_incomplete_payment() {
    for count in [0, 2, 3] {
        let (mut game, cultivator, _) = staged(count);
        game.create_token(PlayerId::Two, tokens::food());
        begin_combat(&mut game);
        let choices = &game.pending_decisions[0].observation.options;
        assert_eq!(
            choices.iter().map(|option| option.id).collect::<Vec<_>>(),
            if count == 3 { vec![0, 1] } else { vec![0] }
        );
        choose(&mut game, vec![0]);
        drain_pending(&mut game);
        assert_eq!(game.players[0].graveyard.len(), count as usize);
        assert_eq!(counters(&game, cultivator), 0);
    }
}

#[test]
fn only_the_active_controllers_cultivator_offers_combat_forage() {
    let (mut game, cultivator, opponent) = staged(3);
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    begin_combat(&mut game);
    let decision = &game.pending_decisions[0].observation;
    assert_eq!(decision.player, PlayerId::Two);
    assert_eq!(decision.source, Some(opponent));
    assert_eq!(decision.options.len(), 1);
    choose(&mut game, vec![0]);
    assert!(game.stack.is_empty());
    assert_eq!(counters(&game, cultivator), 0);
}

#[test]
fn food_forage_uses_only_your_food_and_preserves_sacrifice_events() {
    let (mut game, cultivator, _) = staged(0);
    let food = game.create_token_from(PlayerId::One, tokens::food(), None);
    game.create_token(PlayerId::Two, tokens::food());
    game.tap_permanent(food).unwrap();
    begin_combat(&mut game);
    choose(&mut game, vec![2]);
    let decision = &game.pending_decisions[0].observation;
    assert_eq!(decision.options.len(), 1);
    assert_eq!(decision.options[0].card.map(|(id, _)| id), Some(food));
    game = reconstruct(&game);
    choose(&mut game, vec![0]);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == food)
    );
    assert_eq!(
        game.players[0].life, 20,
        "foraging does not activate Food's life gain"
    );
    assert_eq!(counters(&game, cultivator), 0);
    drain_pending(&mut game);
    assert_eq!(counters(&game, cultivator), 1);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.controller == PlayerId::Two
                && is_token_with(permanent, tokens::food()))
    );
}

#[test]
fn ordinary_exile_and_food_activation_do_not_count_as_foraging() {
    let (mut game, cultivator, _) = staged(3);
    let cards = game.players[0]
        .graveyard
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    game.exile_graveyard_cards(PlayerId::One, &cards);
    let food = game.create_token_from(PlayerId::One, tokens::food(), None);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == food))
        .unwrap();
    game.apply(PlayerId::One, activation).unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, 23);
    assert_eq!(counters(&game, cultivator), 0);
}

#[test]
fn spell_cost_forage_triggers_all_your_cultivators_but_the_mana_alternative_does_not() {
    for spent_objects in [0, 1, 3] {
        let (mut game, cultivator, opponent) = staged(3);
        let second = game
            .put_onto_battlefield(PlayerId::One, cards::CORPSEBERRY_CULTIVATOR)
            .unwrap();
        let victim = game
            .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
        game.create_token(PlayerId::One, tokens::food());
        let feed = card(180_200, cards::FEED_THE_CYCLE, PlayerId::One);
        let feed_id = feed.id;
        game.players[0].hand.push(feed);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 3);
        let cast = game.legal_actions(PlayerId::One).into_iter().find(|action| {
            matches!(action, Action::CastSpell { card, sacrifices, choices, .. }
                if *card == feed_id && sacrifices.len() == spent_objects
                    && choices.targets().iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Permanent(victim)))
        }).expect("each additional-cost alternative is legal");
        game.apply(PlayerId::One, cast).unwrap();
        drain_pending(&mut game);
        let expected = u16::from(spent_objects != 0);
        assert_eq!(counters(&game, cultivator), expected);
        assert_eq!(counters(&game, second), expected);
        assert_eq!(counters(&game, opponent), 0);
        assert!(
            !game
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == victim)
        );
    }
}

#[test]
fn food_forage_waits_for_exit_replacements_before_publishing_its_event() {
    let (mut game, cultivator, _) = staged(0);
    for id in [180_300, 180_301] {
        game.battlefield
            .push(creature(id, cards::REST_IN_PEACE, PlayerId::Two));
    }
    let food = game.create_token_from(PlayerId::One, tokens::food(), None);
    begin_combat(&mut game);
    choose(&mut game, vec![2]);
    choose(&mut game, vec![0]);
    assert!(matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::BattlefieldExitReplacement { .. }
    ));
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == food)
    );
    assert!(
        game.pending_triggers.is_empty(),
        "forage is not finished yet"
    );
    drain_pending(&mut game);
    assert_eq!(
        counters(&game, cultivator),
        1,
        "a replaced sacrifice still forages"
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == food)
    );
}
