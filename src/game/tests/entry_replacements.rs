use super::*;

/// Starts a shock-land play and returns its pre-entry replacement choice.
fn begin_shock_land_play(game: &mut Game, definition: CardDefinitionId) -> DecisionObservation {
    game.players[0]
        .hand
        .push(card(10_500, definition, PlayerId::One));
    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: CardInstanceId(10_500),
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();
    game.observe(PlayerId::One)
        .decision
        .expect("a payable shock land asks its controller whether to pay")
}

fn answer_shock_land_choice(game: &mut Game, decision: u32, pay: bool) {
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision,
            options: vec![u32::from(pay)],
        },
    )
    .unwrap();
}

#[test]
fn effect_life_payments_are_preflighted_and_paid_atomically() {
    let payment = ResolvedEffectPayment::Life(4);
    let mut game = ready_game();

    game.players[0].life = 3;
    assert!(!game.can_pay_effect_payment(PlayerId::One, payment));
    assert!(!game.pay_effect_payment(PlayerId::One, payment));
    assert_eq!(game.players[0].life, 3);

    game.players[0].life = 4;
    let event_start = game.events().len();
    assert!(game.can_pay_effect_payment(PlayerId::One, payment));
    assert_eq!(Game::effect_payment_label(payment), "Pay 4 life");
    assert!(game.pay_effect_payment(PlayerId::One, payment));
    assert_eq!(game.players[0].life, 0);
    assert_eq!(
        game.events()[event_start..]
            .iter()
            .filter(|event| matches!(event, GameEvent::LifeLost { amount: 4, .. }))
            .count(),
        1
    );
}

#[test]
fn resolved_grants_participate_in_external_entry_replacement_discovery() {
    let source_definition = CardDefinitionId::new(10_501);
    let land_definition = CardDefinitionId::new(10_502);
    let mut source = CardDefinition::new(
        source_definition,
        "Test resolved replacement source",
        CardSet::Gatecrash,
        crate::card::CardRules::unsupported(),
    );
    source.rules = CardRules::new_enchantment(ManaCost::default());
    synchronize_single_part_definition(&mut source);
    let mut land = CardDefinition::new(
        land_definition,
        "Test entering land",
        CardSet::Gatecrash,
        crate::card::CardRules::unsupported(),
    );
    land.rules = CardRules::new_land(&[]);
    synchronize_single_part_definition(&mut land);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([source, land]).unwrap();
    let source_id = GameObjectId(10_501);
    game.battlefield
        .push(creature(source_id.0, source_definition, PlayerId::Two));
    attach_constant_resolved_characteristics(
        &mut game,
        source_id,
        &[AppliedEffectDef::add_ability(
            &TEST_OPPONENT_LANDS_ENTER_TAPPED_ABILITY[0],
        )],
        ContinuousEffectExpiration::Never,
    );

    let entering = card(10_502, land_definition, PlayerId::One);
    game.players[0].hand.push(entering.clone());
    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: entering.id,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.definition == land_definition)
            .expect("the land committed after replacement discovery")
            .tapped,
        "a resolved layer-6 replacement grant must pass the entry scan's optimization gate",
    );
}

#[test]
fn a_shock_land_is_not_committed_until_its_replacement_choice_is_made() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::Two));
    let event_start = game.events().len();

    let decision = begin_shock_land_play(&mut game, cards::HALLOWED_FOUNTAIN);

    assert_eq!(decision.kind, DecisionKind::Choice);
    assert_eq!(decision.visibility, DecisionVisibility::Public);
    assert_eq!((decision.minimum, decision.maximum), (1, 1));
    assert!(!decision.cancellable);
    assert_eq!(
        decision
            .options
            .iter()
            .map(|option| option.id)
            .collect::<Vec<_>>(),
        vec![0, 1],
        "decline remains the stable first option and pay the second"
    );
    assert!(
        game.players[0]
            .hand
            .iter()
            .all(|card| card.id != CardInstanceId(10_500)),
        "the proposed zone change has removed the card from its old zone"
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::HALLOWED_FOUNTAIN),
        "the prospective permanent is not observable before replacements finish"
    );
    assert!(game.pending_triggers.is_empty());
    assert!(game.stack.is_empty());
    assert!(
        game.events()[event_start..].iter().all(|event| !matches!(
            event,
            GameEvent::LandPlayed { .. } | GameEvent::AbilityTriggered { .. }
        )),
        "neither the committed land play nor entry-derived triggers exist yet"
    );
}

#[test]
fn shock_land_payment_or_decline_is_applied_before_ankh_observes_the_entry() {
    for (pay, tapped, life) in [(true, false, 18), (false, true, 20)] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::Two));
        let event_start = game.events().len();
        let decision = begin_shock_land_play(&mut game, cards::HALLOWED_FOUNTAIN);
        answer_shock_land_choice(&mut game, decision.id, pay);

        let entered = game
            .battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::HALLOWED_FOUNTAIN)
            .collect::<Vec<_>>();
        assert_eq!(entered.len(), 1, "the proposed entry commits exactly once");
        assert_eq!(entered[0].tapped, tapped);
        assert_ne!(
            entered[0].card.id,
            CardInstanceId(10_500),
            "the committed zone change creates the battlefield object"
        );
        assert_eq!(game.players[0].life, life);
        assert!(game.pending_decisions.is_empty());

        let events = &game.events()[event_start..];
        let land_played = events
            .iter()
            .position(|event| {
                matches!(
                    event,
                    GameEvent::LandPlayed {
                        player: PlayerId::One,
                        definition: cards::HALLOWED_FOUNTAIN,
                        ..
                    }
                )
            })
            .expect("the completed event is logged once");
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, GameEvent::LandPlayed { .. }))
                .count(),
            1
        );
        let ankh_triggered = events
            .iter()
            .position(|event| {
                matches!(
                    event,
                    GameEvent::AbilityTriggered {
                        presentation: ObjectCharacteristics::Card {
                            definition: cards::ANKH_OF_MISHRA,
                            ..
                        },
                        ..
                    }
                )
            })
            .expect("Ankh observes the committed battlefield entry");
        assert!(land_played < ankh_triggered);
        assert_eq!(game.stack.len(), 1, "the entry trigger is now on the stack");

        let life_lost = events.iter().position(|event| {
            matches!(
                event,
                GameEvent::LifeLost {
                    player: PlayerId::One,
                    amount: 2
                }
            )
        });
        if pay {
            assert!(
                life_lost.expect("paying logs life loss") < land_played,
                "the replacement payment happens before the entry commits"
            );
        } else {
            assert!(life_lost.is_none(), "declining does not lose life");
        }
    }
}

#[test]
fn replacement_effects_are_ordered_and_re_evaluated_before_entry_commits() {
    let external_definition = CardDefinitionId::new(10_501);
    let mut external = CardDefinition::new(
        external_definition,
        "Test entry restriction",
        CardSet::Gatecrash,
        crate::card::CardRules::unsupported(),
    );
    external.rules = CardRules::new_enchantment(ManaCost::new(2, 0))
        .with_abilities(&TEST_OPPONENT_LANDS_ENTER_TAPPED_ABILITY);
    synchronize_single_part_definition(&mut external);

    let mut game = ready_game();
    let shock = game
        .catalog
        .get(cards::HALLOWED_FOUNTAIN)
        .expect("the real shock-land definition is cataloged")
        .clone();
    game.catalog = CardCatalog::new([external, shock]).unwrap();
    game.battlefield
        .push(creature(10_501, external_definition, PlayerId::Two));
    game.players[0]
        .hand
        .push(card(10_500, cards::HALLOWED_FOUNTAIN, PlayerId::One));

    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: CardInstanceId(10_500),
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();

    let order = game
        .observe(PlayerId::One)
        .decision
        .expect("the affected player orders the two applicable replacements");
    assert_eq!(order.kind, DecisionKind::Choice);
    assert_eq!(order.options.len(), 2);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::HALLOWED_FOUNTAIN)
    );
    let enter_tapped = order
        .options
        .iter()
        .find(|option| option.ability_text.as_deref() == Some(TEST_OPPONENT_LAND_ENTRY_TEXT))
        .expect("the external replacement is one of the ordered effects")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: order.id,
            options: vec![enter_tapped],
        },
    )
    .unwrap();

    let payment = game
        .observe(PlayerId::One)
        .decision
        .expect("re-evaluation finds the shock land's remaining replacement");
    assert_eq!(payment.kind, DecisionKind::Choice);
    assert_eq!(
        payment
            .options
            .iter()
            .map(|option| option.id)
            .collect::<Vec<_>>(),
        vec![0, 1]
    );
    answer_shock_land_choice(&mut game, payment.id, true);

    let entered = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::HALLOWED_FOUNTAIN)
        .expect("the fully replaced event committed");
    assert!(
        entered.tapped,
        "paying the shock-land cost does not undo another replacement's tapped modification"
    );
    assert_eq!(game.players[0].life, 18);
}

#[test]
fn nested_replacement_effects_keep_their_source_controller_context() {
    let external_definition = CardDefinitionId::new(10_501);
    let mut external = CardDefinition::new(
        external_definition,
        "Test source-relative entry replacement",
        CardSet::Gatecrash,
        crate::card::CardRules::unsupported(),
    );
    external.rules = CardRules::new_enchantment(ManaCost::new(2, 0))
        .with_abilities(&TEST_EXTERNAL_CONTEXT_ABILITY);
    synchronize_single_part_definition(&mut external);

    let mut game = ready_game();
    let plains = game.catalog.get(cards::PLAINS).unwrap().clone();
    let stage = game.catalog.get(cards::THESPIANS_STAGE).unwrap().clone();
    game.catalog = CardCatalog::new([external, plains, stage]).unwrap();
    game.battlefield.extend([
        creature(10_501, external_definition, PlayerId::Two),
        creature(10_502, cards::PLAINS, PlayerId::Two),
    ]);
    let stage = card(10_500, cards::THESPIANS_STAGE, PlayerId::One);
    game.players[0].hand.push(stage.clone());

    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: stage.id,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();

    let payment = game
        .observe(PlayerId::Two)
        .decision
        .expect("the replacement source's controller is asked to pay");
    assert_eq!(payment.player, PlayerId::Two);
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: payment.id,
            options: vec![1],
        },
    )
    .unwrap();

    assert_eq!(game.players[0].life, i16::from(rules::STARTING_LIFE));
    assert_eq!(game.players[1].life, i16::from(rules::STARTING_LIFE) - 2);
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::THESPIANS_STAGE)
            .expect("the land committed after the source-relative choice")
            .tapped
    );
}

#[test]
fn a_shock_land_asks_nothing_when_the_life_is_not_there() {
    // You may pay life down to zero, but you cannot pay more than you have.
    let mut game = ready_game();
    game.players[0].life = 1;
    game.players[0]
        .hand
        .push(card(10_500, cards::STEAM_VENTS, PlayerId::One));
    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: CardInstanceId(10_500),
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();

    assert!(
        game.pending_decisions.is_empty(),
        "no prompt whose only real answer is no"
    );
    assert!(game.battlefield[0].tapped);
    assert_eq!(game.players[0].life, 1);
    assert!(game.events().iter().all(|event| !matches!(
        event,
        GameEvent::LifeLost {
            player: PlayerId::One,
            ..
        }
    )));
}

#[test]
fn paying_for_a_shock_land_at_exactly_two_life_loses_the_game() {
    let mut game = ready_game();
    game.players[0].life = 2;
    let event_start = game.events().len();
    let decision = begin_shock_land_play(&mut game, cards::TEMPLE_GARDEN);
    answer_shock_land_choice(&mut game, decision.id, true);

    assert_eq!(game.players[0].life, 0);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::TEMPLE_GARDEN)
            .count(),
        1,
        "the land commits before state-based actions end the game"
    );
    let events = &game.events()[event_start..];
    let life_lost = events
        .iter()
        .position(|event| matches!(event, GameEvent::LifeLost { amount: 2, .. }))
        .expect("the payment is logged");
    let land_played = events
        .iter()
        .position(|event| matches!(event, GameEvent::LandPlayed { .. }))
        .expect("the land commits");
    let game_ended = events
        .iter()
        .position(|event| matches!(event, GameEvent::GameEnded { .. }))
        .expect("state-based actions end the game");
    assert!(life_lost < land_played && land_played < game_ended);
    assert!(matches!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            ..
        })
    ));
}
