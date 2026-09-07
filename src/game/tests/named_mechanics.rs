use super::*;

fn staged(graveyard: usize, food: bool) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    for player in &mut game.players {
        player.hand.clear();
        player.graveyard.clear();
        player.exile.clear();
    }
    let cultivator = game
        .put_onto_battlefield(PlayerId::One, cards::CORPSEBERRY_CULTIVATOR)
        .unwrap();
    if food {
        game.create_token(PlayerId::One, tokens::food());
    }
    for index in 0..graveyard {
        game.players[0].graveyard.push(card(
            150_000 + u32::try_from(index).unwrap(),
            cards::LIGHTNING_BOLT,
            PlayerId::One,
        ));
    }
    drain_pending(&mut game);
    (game, cultivator)
}

fn begin_combat(game: &mut Game) {
    game.step = Step::BeginningOfCombat;
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: crate::card::TurnStepDef::BeginningOfCombat,
        player: game.active_player,
    });
    game.finish_rules_procedure();
    game.resolve_stack_top();
}

fn choose_objects(game: &mut Game, count: usize) {
    let decision = game.pending_decisions.last().unwrap().observation.clone();
    if count == 0 {
        game.apply(
            decision.player,
            Action::CancelDecision {
                decision: decision.id,
            },
        )
        .unwrap();
        return;
    }
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: decision
                .options
                .iter()
                .take(count)
                .map(|option| option.id)
                .collect(),
        },
    )
    .unwrap();
}

fn counters(game: &Game, object: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == object)
        .unwrap()
        .counters(CounterKind::PlusOnePlusOne)
}

#[test]
fn forage_payment_window_selects_then_commits_once_in_both_engines() {
    for prepared in [false, true] {
        for exile in [false, true] {
            let (mut game, cultivator) = staged(5, true);
            game.set_prepared_engine_enabled(prepared);
            let sacrifice_watcher = game
                .put_onto_battlefield(PlayerId::One, cards::BLOOD_ASPIRANT)
                .unwrap();
            begin_combat(&mut game);
            choose_decision_by_label(
                &mut game,
                PlayerId::One,
                if exile {
                    "Exile 3 card(s) from your graveyard"
                } else {
                    "Sacrifice 1 permanent(s)"
                },
            );
            let offer = &game.pending_decisions.last().unwrap().observation;
            assert_eq!(offer.minimum, if exile { 3 } else { 1 });
            assert_eq!(
                offer.options.len(),
                if exile { 5 } else { 1 },
                "linear candidates, not combinations"
            );
            assert_eq!(
                game.players[0].graveyard.len(),
                5,
                "choosing a branch pays nothing"
            );
            assert_eq!(counters(&game, cultivator), 0);
            choose_objects(&mut game, if exile { 3 } else { 1 });
            drain_pending(&mut game);
            assert_eq!(
                counters(&game, cultivator),
                1,
                "one forage, regardless of objects paid"
            );
            assert_eq!(
                counters(&game, sacrifice_watcher),
                u16::from(!exile),
                "the ordinary sacrifice event is preserved"
            );
            assert_eq!(game.players[0].exile.len(), if exile { 3 } else { 0 });
        }
    }
}

#[test]
fn forage_decline_cancel_and_unaffordable_do_not_mutate_or_trigger() {
    for graveyard in [0, 2, 3] {
        let (mut game, cultivator) = staged(graveyard, false);
        begin_combat(&mut game);
        if graveyard == 3 {
            choose_decision_by_label(&mut game, PlayerId::One, "Decline");
            begin_combat(&mut game);
            choose_decision_by_label(
                &mut game,
                PlayerId::One,
                "Exile 3 card(s) from your graveyard",
            );
            choose_objects(&mut game, 0);
        }
        assert!(game.pending_decisions.is_empty());
        assert_eq!(game.players[0].graveyard.len(), graveyard);
        assert!(game.players[0].exile.is_empty());
        assert_eq!(counters(&game, cultivator), 0);
    }
}

#[test]
fn forage_stale_or_duplicate_selection_is_rejected_before_payment() {
    let (mut game, cultivator) = staged(3, false);
    begin_combat(&mut game);
    choose_decision_by_label(
        &mut game,
        PlayerId::One,
        "Exile 3 card(s) from your graveyard",
    );
    let pending = game.pending_decisions.last().unwrap().clone();
    let first = pending.observation.options[0].id;
    assert!(
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: pending.observation.id,
                options: vec![first; 3]
            }
        )
        .is_err()
    );
    let DecisionContinuation::CostPayment(window) = pending.continuation else {
        panic!("payment window")
    };
    game.players[0].graveyard.pop();
    game.resolve_cost_payment_window(
        *window,
        &pending
            .observation
            .options
            .iter()
            .map(|option| option.id)
            .collect::<Vec<_>>(),
        &pending.observation.options,
    );
    assert_eq!(game.players[0].graveyard.len(), 2);
    assert!(game.players[0].exile.is_empty());
    assert_eq!(counters(&game, cultivator), 0);
}

#[test]
fn forage_from_spell_cost_triggers_before_resolution_but_mana_alternative_does_not() {
    for selected_count in [0, 1, 3] {
        let (mut game, cultivator) = staged(3, true);
        let watcher = game
            .put_onto_battlefield(PlayerId::One, cards::BLOOD_ASPIRANT)
            .unwrap();
        let victim = game
            .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
        let spell = card(150_200, cards::FEED_THE_CYCLE, PlayerId::One);
        let spell_id = spell.id;
        game.players[0].hand.push(spell);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 3);
        let action = game.legal_actions(PlayerId::One).into_iter().find(|action| matches!(action,
            Action::CastSpell { card, sacrifices, choices, .. } if *card == spell_id && sacrifices.len() == selected_count && choices.iter_targets().any(|target| *target == Target::Permanent(victim))
        )).expect("payable Feed the Cycle");
        game.apply(PlayerId::One, action).unwrap();
        // Only the triggered abilities resolve here; the removal spell still
        // waits below them and could subsequently be countered.
        while game
            .stack
            .last()
            .is_some_and(|object| object.kind != StackObjectKind::Spell)
            || !game.pending_decisions.is_empty()
        {
            if let Some(decision) = game.pending_decisions.last() {
                let decision = decision.observation.clone();
                game.apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options: decision
                            .options
                            .iter()
                            .take(decision.minimum)
                            .map(|option| option.id)
                            .collect(),
                    },
                )
                .unwrap();
            } else {
                game.resolve_stack_top();
            }
            game.finish_rules_procedure();
        }
        assert_eq!(counters(&game, cultivator), u16::from(selected_count != 0));
        assert_eq!(counters(&game, watcher), u16::from(selected_count == 1));
        assert!(
            game.battlefield
                .iter()
                .any(|permanent| permanent.card.id == victim)
        );
        assert_eq!(game.stack.last().unwrap().kind, StackObjectKind::Spell);
    }
}

#[test]
fn ordinary_sacrifice_and_opponents_forage_do_not_count_as_your_forage() {
    let (mut game, cultivator) = staged(0, true);
    let watcher = game
        .put_onto_battlefield(PlayerId::One, cards::BLOOD_ASPIRANT)
        .unwrap();
    let food = game
        .battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, tokens::food()))
        .unwrap()
        .card
        .id;
    game.sacrifice_permanents(&[food]);
    let forage = match game
        .catalog
        .get(cards::CORPSEBERRY_CULTIVATOR)
        .unwrap()
        .rules
        .ability_clauses()[1]
        .definition
    {
        DeclarativeAbilityDef::Triggered(definition) => match definition.event {
            TriggerEventDef::MechanicPerformed { mechanic, .. } => mechanic,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    game.capture_battlefield_triggers(&CommittedTriggerEvent::MechanicPerformed {
        mechanics: vec![forage],
        player: PlayerId::Two,
        object: None,
    });
    drain_pending(&mut game);
    assert_eq!(counters(&game, watcher), 1);
    assert_eq!(counters(&game, cultivator), 0);
}

#[test]
fn named_mechanics_sacrifice_batches_preserve_each_and_one_or_more_listeners() {
    static ABILITIES: [AbilityDef; 2] = [
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
    let (mut game, _) = staged(0, false);
    let mut definition = CardDefinition::new(
        CardDefinitionId::new(160_400),
        "Sacrifice batch watcher",
        CardSet::Magic2014,
        CardRules::new_creature(ManaCost::default(), &[], 2, 2).with_abilities(&ABILITIES),
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
    let watcher = game
        .put_onto_battlefield(PlayerId::One, CardDefinitionId::new(160_400))
        .unwrap();
    let first = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let second = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    game.sacrifice_permanents(&[first, second]);
    assert_eq!(
        game.pending_triggers.len(),
        3,
        "two per-object occurrences and one batch occurrence"
    );
    game.pending_triggers.clear();
    let payers = (0..3)
        .map(|_| {
            game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
                .unwrap()
        })
        .collect::<Vec<_>>();
    let dread = card(160_410, cards::DREAD_RETURN, PlayerId::One);
    let dread_id = dread.id;
    game.players[0].graveyard.push(dread);
    let cast = game.legal_actions(PlayerId::One).into_iter().find(|action| matches!(action,
        Action::CastSpell { card, sacrifices, .. } if *card == dread_id && sacrifices.len() == 3
            && sacrifices.iter().all(|object| payers.contains(object))
    )).expect("three creatures can pay the flashback cost");
    game.apply(PlayerId::One, cast).unwrap();
    let DecisionContinuation::TriggerOrder { batch, .. } = &game.pending_decisions[0].continuation
    else {
        panic!("the simultaneous sacrifice triggers are ordered together");
    };
    assert_eq!(
        batch.triggers.len(),
        4,
        "three per-object triggers and one batch trigger from a cost"
    );
    drain_pending(&mut game);
    game.sacrifice_permanents(&[watcher]);
    assert_eq!(
        game.pending_triggers.len(),
        2,
        "a departing listener still sees its own sacrifice"
    );
}
