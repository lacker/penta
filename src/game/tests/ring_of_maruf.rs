use super::*;

fn set_outside_game(
    game: &mut Game,
    player: PlayerId,
    definitions: &[CardDefinitionId],
) -> Vec<CardInstance> {
    let cards = game.build_zone(player, definitions).unwrap();
    game.players[player.index()].outside_game = cards.clone();
    cards
}

fn resolve_ring_activation(game: &mut Game) -> GameObjectId {
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::RING_OF_MARUF)
        .unwrap();
    game.players[PlayerId::One.index()].mana_pool.colorless = 5;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source)
        })
        .expect("Ring's shared activated ability should be offered");

    game.apply(PlayerId::One, action).unwrap();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != source),
        "exiling Ring is a cost paid before its ability resolves"
    );
    let exiled = game.players[PlayerId::One.index()]
        .exile
        .iter()
        .find(|card| card.definition == cards::RING_OF_MARUF)
        .expect("Ring should be in exile after paying the cost")
        .id;
    assert_eq!(game.stack.len(), 1);

    pass_priority_pair(game);
    assert!(game.stack.is_empty());
    assert_eq!(game.draw_replacements[PlayerId::One.index()].len(), 1);
    exiled
}

fn choose_option_from_zone(game: &mut Game, zone: DecisionZone) {
    let decision = game.observe(PlayerId::One).decision.unwrap();
    let option = decision
        .options
        .iter()
        .find(|option| option.zone == zone)
        .unwrap_or_else(|| panic!("Ring did not offer a card from {zone:?}"))
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .unwrap();
}

#[test]
fn old_school_ring_offers_owned_exile_and_sideboard_then_resumes_the_draws() {
    let mut game = ready_game();
    let outside = set_outside_game(&mut game, PlayerId::One, &[cards::SERRA_ANGEL]);
    let outside_id = outside[0].id;
    let outside_backing = outside[0].backing.clone();
    let exiled_ring = resolve_ring_activation(&mut game);
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (12_000, cards::LIGHTNING_BOLT),
            (12_001, cards::MOUNTAIN),
            (12_002, cards::BLACK_LOTUS),
        ],
    );
    let event_start = game.events().len();

    game.draw_cards(PlayerId::One, 3);

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.visibility, DecisionVisibility::Private);
    assert_eq!((decision.minimum, decision.maximum), (1, 1));
    assert!(game.observe(PlayerId::Two).decision.is_none());
    assert!(decision.options.iter().any(|option| {
        option.zone == DecisionZone::Exile
            && option.card
                == Some((
                    exiled_ring,
                    ObjectCharacteristics::card(cards::RING_OF_MARUF, CardPartId::PRIMARY),
                ))
    }));
    assert!(decision.options.iter().any(|option| {
        option.zone == DecisionZone::OutsideGame
            && option.card
                == Some((
                    outside_id,
                    ObjectCharacteristics::card(cards::SERRA_ANGEL, CardPartId::PRIMARY),
                ))
    }));
    assert_eq!(game.players[0].library.len(), 3);
    assert_eq!(game.cards_drawn_this_turn[0], 0);

    choose_option_from_zone(&mut game, DecisionZone::OutsideGame);

    assert!(game.players[0].outside_game.is_empty());
    let imported = game.players[0]
        .hand
        .iter()
        .find(|card| card.definition == cards::SERRA_ANGEL)
        .expect("the sideboard card should enter the game in hand");
    assert_ne!(
        imported.id, outside_id,
        "entering the game creates an object"
    );
    assert_eq!(imported.backing, outside_backing);
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.id == exiled_ring)
    );
    assert_eq!(game.players[0].library.len(), 1);
    assert_eq!(game.cards_drawn_this_turn[0], 2);
    assert_eq!(game.drawn_this_turn[0].len(), 2);
    assert_eq!(
        game.events()[event_start..]
            .iter()
            .filter(|event| matches!(event, GameEvent::CardDrawn { .. }))
            .count(),
        2
    );
    assert!(game.result.is_none());

    let _ = game.draw_card(PlayerId::One);
    assert_eq!(
        game.cards_drawn_this_turn[0], 3,
        "only one draw was replaced"
    );
}

#[test]
fn non_old_school_ring_uses_oracle_outside_game_source_only() {
    let mut game = ready_game();
    game.format = Format::IsdM14Standard;
    set_outside_game(&mut game, PlayerId::One, &[cards::SERRA_ANGEL]);
    let exiled_ring = resolve_ring_activation(&mut game);

    let _ = game.draw_card(PlayerId::One);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert!(
        decision
            .options
            .iter()
            .all(|option| option.zone == DecisionZone::OutsideGame)
    );
    assert!(
        decision
            .options
            .iter()
            .all(|option| { option.card.is_none_or(|(card, _)| card != exiled_ring) })
    );

    choose_option_from_zone(&mut game, DecisionZone::OutsideGame);
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.id == exiled_ring)
    );
}

#[test]
fn an_impossible_ring_choice_still_replaces_the_draw_without_decking() {
    let mut game = ready_game();
    game.format = Format::IsdM14Standard;
    game.players[0].outside_game.clear();
    game.players[0].library.clear();
    resolve_ring_activation(&mut game);
    let event_start = game.events().len();

    assert_eq!(game.draw_card(PlayerId::One), None);
    assert!(game.pending_decisions.is_empty());
    assert!(game.result.is_none());
    assert_eq!(game.cards_drawn_this_turn[0], 0);
    assert!(
        game.events()[event_start..]
            .iter()
            .all(|event| !matches!(event, GameEvent::CardDrawn { .. }))
    );

    let _ = game.draw_card(PlayerId::One);
    assert!(game.result.is_none(), "empty-library loss waits for SBAs");
    game.check_state_based_actions();
    assert!(matches!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            reason: WinReason::OpponentTriedToDrawFromEmptyLibrary,
        })
    ));
}

#[test]
fn ring_suspends_both_players_simultaneous_draws_until_the_choice_finishes() {
    let mut game = ready_game();
    set_outside_game(&mut game, PlayerId::One, &[cards::SERRA_ANGEL]);
    resolve_ring_activation(&mut game);
    game.players[0].library = game
        .build_zone(PlayerId::One, &[cards::MOUNTAIN; 6])
        .unwrap();
    game.players[1].library = game
        .build_zone(PlayerId::Two, &[cards::MOUNTAIN; 7])
        .unwrap();

    let event_start = game.events().len();
    game.draw_cards_simultaneously([7, 7]);

    assert!(game.observe(PlayerId::One).decision.is_some());
    assert!(game.players[0].hand.is_empty());
    assert!(game.players[1].hand.is_empty());
    assert_eq!(game.players[0].library.len(), 6);
    assert_eq!(game.players[1].library.len(), 7);

    choose_option_from_zone(&mut game, DecisionZone::OutsideGame);

    assert_eq!(game.cards_drawn_this_turn, [6, 7]);
    assert!(game.players[0].library.is_empty());
    assert!(game.players[1].library.is_empty());
    assert!(!game.defer_empty_library_loss);
    assert!(game.pending_procedures.is_empty());
    assert!(game.result.is_none());
    let draw_order = game.events()[event_start..]
        .iter()
        .filter_map(|event| match event {
            GameEvent::CardDrawn { player, .. } => Some(*player),
            _ => None,
        })
        .collect::<Vec<_>>();
    let expected = [PlayerId::One; 6]
        .into_iter()
        .chain([PlayerId::Two; 7])
        .collect::<Vec<_>>();
    assert_eq!(
        draw_order, expected,
        "the active player completes every draw before the other player"
    );
}

#[test]
fn ring_suspends_the_remaining_clauses_of_a_declarative_effect_sequence() {
    static EFFECTS: [EffectDef; 2] = [
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    ];

    let mut game = ready_game();
    set_outside_game(&mut game, PlayerId::One, &[cards::SERRA_ANGEL]);
    resolve_ring_activation(&mut game);
    game.players[0].library.clear();
    stack_library(&mut game, &[(12_050, cards::MOUNTAIN)]);
    let source = spell(12_051, cards::SIGN_IN_BLOOD, PlayerId::One, 0);

    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Sequence(&EFFECTS)),
        &source,
        TriggerContext::empty(),
    );

    assert!(game.observe(PlayerId::One).decision.is_some());
    assert_eq!(game.players[0].life, 20, "later clauses have not run");
    assert_eq!(game.players[0].library.len(), 1);

    choose_option_from_zone(&mut game, DecisionZone::OutsideGame);

    assert_eq!(game.cards_drawn_this_turn[0], 1);
    assert!(game.players[0].library.is_empty());
    assert_eq!(game.players[0].life, 18, "the sequence tail resumed");
    assert!(game.pending_procedures.is_empty());
}

#[test]
fn replacement_effect_tail_finishes_before_later_draws_and_outer_effects() {
    static SOURCES: [CardChoiceSourceDef; 1] = [CardChoiceSourceDef::OutsideGame];
    static REPLACEMENT_EFFECTS: [EffectDef; 2] = [
        EffectDef::ChooseCards {
            player: EffectRecipientDef::Controller,
            sources: &SOURCES,
            object: ObjectPredicateDef::Any,
            minimum: 1,
            maximum: 1,
            reveal: false,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ];
    static OUTER_EFFECTS: [EffectDef; 2] = [
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    ];

    let mut game = ready_game();
    set_outside_game(&mut game, PlayerId::One, &[cards::SERRA_ANGEL]);
    game.players[0].library.clear();
    stack_library(&mut game, &[(12_075, cards::MOUNTAIN)]);
    let source = spell(12_076, cards::SIGN_IN_BLOOD, PlayerId::One, 0);
    game.draw_replacements[0].push_back(DrawReplacement {
        optional: false,
        installed: true,
        object: Box::new(source.clone()),
        context: TriggerContext::empty().into(),
        effect: ScopedEffect::primary(EffectDef::Sequence(&REPLACEMENT_EFFECTS)),
    });
    let event_start = game.events().len();

    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Sequence(&OUTER_EFFECTS)),
        &source,
        TriggerContext::empty(),
    );
    choose_option_from_zone(&mut game, DecisionZone::OutsideGame);

    let order = game.events()[event_start..]
        .iter()
        .filter_map(|event| match event {
            GameEvent::LifeLost {
                player: PlayerId::One,
                amount: 1,
            } => Some("replacement tail"),
            GameEvent::CardDrawn {
                player: PlayerId::One,
                ..
            } => Some("remaining draw"),
            GameEvent::LifeLost {
                player: PlayerId::One,
                amount: 2,
            } => Some("outer tail"),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        order,
        vec!["replacement tail", "remaining draw", "outer tail"]
    );
    assert!(game.pending_procedures.is_empty());
}

#[test]
fn the_affected_player_chooses_between_multiple_draw_replacements() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(&mut game, &[(12_090, cards::MOUNTAIN)]);
    let source = spell(12_091, cards::RING_OF_MARUF, PlayerId::One, 0);
    game.draw_replacements[0].extend([
        DrawReplacement {
            optional: false,
            installed: true,
            object: Box::new(source.clone()),
            context: TriggerContext::empty().into(),
            effect: ScopedEffect::primary(EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            }),
        },
        DrawReplacement {
            optional: false,
            installed: true,
            object: Box::new(source),
            context: TriggerContext::empty().into(),
            effect: ScopedEffect::primary(EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            }),
        },
    ]);

    assert_eq!(game.draw_card(PlayerId::One), None);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.visibility, DecisionVisibility::Public);
    assert_eq!(decision.options.len(), 2);
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![2],
        },
    )
    .unwrap();

    assert_eq!(game.players[0].life, 22);
    assert_eq!(game.draw_replacements[0].len(), 1);
    assert_eq!(game.players[0].library.len(), 1);
    assert_eq!(game.draw_card(PlayerId::One), None);
    assert_eq!(game.players[0].life, 21);
    assert!(game.draw_replacements[0].is_empty());
    assert_eq!(game.players[0].library.len(), 1);
}

#[test]
fn a_chosen_replacement_finishes_its_draw_before_the_original_instruction_resumes() {
    static OUTER_EFFECTS: [EffectDef; 2] = [
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    ];

    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(&mut game, &[(12_095, cards::MOUNTAIN)]);
    let source = spell(12_096, cards::RING_OF_MARUF, PlayerId::One, 0);
    game.draw_replacements[0].extend([
        DrawReplacement {
            optional: false,
            installed: true,
            object: Box::new(source.clone()),
            context: TriggerContext::empty().into(),
            effect: ScopedEffect::primary(EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            }),
        },
        DrawReplacement {
            optional: false,
            installed: true,
            object: Box::new(source.clone()),
            context: TriggerContext::empty().into(),
            effect: ScopedEffect::primary(EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            }),
        },
    ]);
    let event_start = game.events().len();

    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Sequence(&OUTER_EFFECTS)),
        &source,
        TriggerContext::empty(),
    );
    let decision = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            // The chosen replacement draws one. The unchosen replacement
            // applies to that nested draw and consumes it before the original
            // instruction's second draw can happen.
            options: vec![1],
        },
    )
    .unwrap();

    let order = game.events()[event_start..]
        .iter()
        .filter_map(|event| match event {
            GameEvent::LifeLost {
                player: PlayerId::One,
                amount: 3,
            } => Some("nested draw replacement"),
            GameEvent::CardDrawn {
                player: PlayerId::One,
                ..
            } => Some("original remaining draw"),
            GameEvent::LifeLost {
                player: PlayerId::One,
                amount: 2,
            } => Some("outer tail"),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        order,
        vec![
            "nested draw replacement",
            "original remaining draw",
            "outer tail",
        ]
    );
    assert_eq!(game.cards_drawn_this_turn[0], 1);
    assert!(game.draw_replacements[0].is_empty());
    assert!(game.pending_procedures.is_empty());
}

#[test]
fn the_draw_step_finishes_only_after_ring_is_answered_and_replacements_expire() {
    let mut game = ready_game();
    set_outside_game(&mut game, PlayerId::One, &[cards::SERRA_ANGEL]);
    resolve_ring_activation(&mut game);
    game.players[0].library.clear();
    stack_library(&mut game, &[(12_100, cards::MOUNTAIN)]);
    game.turn = 2;
    game.step = Step::Upkeep;
    let event_start = game.events().len();

    game.advance_step();
    assert_eq!(game.step, Step::Draw);
    assert!(game.observe(PlayerId::One).decision.is_some());
    assert!(game.events()[event_start..].iter().all(|event| !matches!(
        event,
        GameEvent::StepChanged {
            step: Step::Draw,
            ..
        }
    )));

    choose_option_from_zone(&mut game, DecisionZone::OutsideGame);
    assert!(game.events()[event_start..].iter().any(|event| matches!(
        event,
        GameEvent::StepChanged {
            step: Step::Draw,
            ..
        }
    )));
    assert_eq!(game.players[0].library.len(), 1);

    // A second Ring effect that is never used lapses in cleanup.
    game.draw_replacements[0].push_back(DrawReplacement {
        optional: false,
        installed: true,
        object: Box::new(spell(12_200, cards::RING_OF_MARUF, PlayerId::One, 0)),
        context: TriggerContext::empty().into(),
        effect: ScopedEffect::primary(EffectDef::None),
    });
    game.finish_cleanup();
    assert!(game.draw_replacements.iter().all(VecDeque::is_empty));
}
