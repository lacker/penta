use super::*;

#[test]
fn sylvan_library_triggers_onto_the_stack_and_may_be_declined() {
    let mut game = ready_game();
    game.turn = 2;
    game.step = Step::Upkeep;
    game.battlefield
        .push(creature(10_000, cards::SYLVAN_LIBRARY, PlayerId::One));
    game.players[0].library = vec![
        card(10_001, cards::PLAINS, PlayerId::One),
        card(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
        card(10_003, cards::SWORDS_TO_PLOWSHARES, PlayerId::One),
    ];

    game.advance_step();
    assert_eq!(
        game.players[0].hand.len(),
        1,
        "the draw step draws one; the extras wait on the ability"
    );
    assert_eq!(game.pending_triggers.len(), 1, "the ability triggered");

    pass_priority_pair(&mut game);
    assert_eq!(game.stack.len(), 1, "and it went on the stack");
    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "so the opponent had a window before any of it happened"
    );

    pass_until_decision(&mut game);
    let offer = game.observe(PlayerId::One).decision.unwrap();
    assert!(
        offer
            .prompt
            .starts_with("At the beginning of your draw step")
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
            options: vec![0],
        },
    )
    .unwrap();

    assert_eq!(game.players[0].hand.len(), 1, "declining draws nothing");
    assert_eq!(game.players[0].life, 20, "and costs nothing");
    assert!(game.observe(PlayerId::One).decision.is_none());
}

#[test]
fn sylvan_library_may_draw_from_an_empty_library_before_state_based_actions() {
    let mut game = ready_game();
    game.turn = 2;
    game.step = Step::Upkeep;
    game.battlefield
        .push(creature(10_000, cards::SYLVAN_LIBRARY, PlayerId::One));
    game.players[0].library = vec![card(10_001, cards::PLAINS, PlayerId::One)];

    game.advance_step();
    pass_priority_pair(&mut game);
    pass_until_decision(&mut game);

    let offer = game.observe(PlayerId::One).decision.unwrap();
    assert!(
        offer
            .prompt
            .starts_with("At the beginning of your draw step")
    );
    game.choose_decision(PlayerId::One, offer.id, &[1]);

    assert_eq!(
        game.result, None,
        "choosing the draw only records the failed attempts during resolution"
    );
    game.finish_rules_procedure();
    let settlement = game.observe(PlayerId::One).decision.unwrap();
    game.choose_decision(PlayerId::One, settlement.id, &[0]);
    assert_eq!(
        game.result, None,
        "settling the only available drawn card is still inside the resolution"
    );
    game.finish_rules_procedure();
    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            reason: WinReason::OpponentTriedToDrawFromEmptyLibrary,
        })
    );
}

#[test]
fn sylvan_library_pays_life_or_puts_each_chosen_card_back() {
    let mut game = ready_game();
    game.turn = 2;
    game.step = Step::Upkeep;
    game.battlefield
        .push(creature(10_000, cards::SYLVAN_LIBRARY, PlayerId::One));
    game.players[0].library = vec![
        card(10_001, cards::PLAINS, PlayerId::One),
        card(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
        card(10_003, cards::SWORDS_TO_PLOWSHARES, PlayerId::One),
    ];

    game.advance_step();
    pass_priority_pair(&mut game);
    pass_until_decision(&mut game);
    let offer = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
            options: vec![1],
        },
    )
    .unwrap();
    assert_eq!(game.players[0].hand.len(), 3, "one drawn plus two more");

    let selection = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(selection.options.len(), 3);
    let selected = selection
        .options
        .iter()
        .take(2)
        .map(|option| option.id)
        .collect::<Vec<_>>();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: selection.id,
            options: selected,
        },
    )
    .unwrap();

    for mode in [1, 0] {
        let decision = game.observe(PlayerId::One).decision.unwrap();
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![mode],
            },
        )
        .unwrap();
    }

    assert_eq!(game.players[0].life, 16, "four life for the one kept");
    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(
        game.players[0].library.len(),
        1,
        "the other went back on top"
    );
}

#[test]
fn sylvan_library_chooses_all_available_cards_when_replacements_reduce_its_draws() {
    let mut game = ready_game();
    game.turn = 2;
    game.step = Step::Upkeep;
    game.battlefield
        .push(creature(10_000, cards::SYLVAN_LIBRARY, PlayerId::One));
    game.players[0].library = vec![
        card(10_001, cards::PLAINS, PlayerId::One),
        card(10_002, cards::MOUNTAIN, PlayerId::One),
        card(10_003, cards::FOREST, PlayerId::One),
    ];

    game.advance_step();
    let source = spell(10_010, cards::RING_OF_MARUF, PlayerId::One, 0);
    game.draw_replacements[0].push_back(DrawReplacement {
        object: Box::new(source),
        context: TriggerContext::empty().into(),
        effect: ScopedEffect::primary(EffectDef::None),
        optional: false,
        installed: true,
    });
    pass_priority_pair(&mut game);
    pass_until_decision(&mut game);
    let offer = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
            options: vec![1],
        },
    )
    .unwrap();

    assert_eq!(game.players[0].hand.len(), 2);
    assert!(game.draw_replacements[0].is_empty());
    let ordering = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(ordering.options.len(), 2);
    assert_eq!(
        ordering.order_semantics,
        Some(DecisionOrderSemantics::Resolution)
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: ordering.id,
            options: ordering
                .options
                .iter()
                .rev()
                .map(|option| option.id)
                .collect(),
        },
    )
    .unwrap();
    let settlement = game.observe(PlayerId::One).decision.unwrap();
    assert!(
        settlement
            .options
            .iter()
            .any(|option| option.label == "Decline")
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: settlement.id,
            options: vec![0],
        },
    )
    .unwrap();
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn island_sanctuary_replaces_each_draw_in_its_controllers_draw_step() {
    let mut game = ready_game();
    game.step = Step::Draw;
    game.active_player = PlayerId::One;
    game.battlefield
        .push(creature(10_000, cards::ISLAND_SANCTUARY, PlayerId::One));
    game.players[0].library = vec![
        card(10_001, cards::PLAINS, PlayerId::One),
        card(10_002, cards::MOUNTAIN, PlayerId::One),
    ];

    for expected_library_size in [2, 2] {
        assert_eq!(game.draw_card(PlayerId::One), None);
        let decision = game.observe(PlayerId::One).decision.unwrap();
        assert!(
            decision
                .options
                .iter()
                .any(|option| option.label == "Draw the card")
        );
        let skip = decision
            .options
            .iter()
            .find(|option| option.ability_text.is_some())
            .expect("Island Sanctuary offers its replacement")
            .id;
        game.choose_decision(PlayerId::One, decision.id, &[skip]);
        assert_eq!(game.players[0].library.len(), expected_library_size);
    }

    assert_eq!(game.resolved_attack_restrictions.len(), 2);
    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.draw_card(PlayerId::One), None);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    let draw = decision
        .options
        .iter()
        .find(|option| option.label == "Draw the card")
        .unwrap()
        .id;
    game.choose_decision(PlayerId::One, decision.id, &[draw]);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].library.len(), 1);
    assert_eq!(game.resolved_attack_restrictions.len(), 2);
}

#[test]
fn island_sanctuary_does_not_replace_draws_outside_its_controllers_draw_step() {
    let mut game = ready_game();
    game.step = Step::PrecombatMain;
    game.battlefield
        .push(creature(10_000, cards::ISLAND_SANCTUARY, PlayerId::One));
    game.players[0].library = vec![card(10_001, cards::PLAINS, PlayerId::One)];
    game.players[1].library = vec![card(10_002, cards::MOUNTAIN, PlayerId::Two)];

    assert!(game.draw_card(PlayerId::One).is_some());
    game.step = Step::Draw;
    game.active_player = PlayerId::One;
    assert!(game.draw_card(PlayerId::Two).is_some());
    assert!(game.pending_decisions.is_empty());
    assert!(game.resolved_attack_restrictions.is_empty());
}

#[test]
fn declining_island_sanctuary_keeps_the_miracle_draw_window() {
    let mut game = ready_game();
    game.step = Step::Draw;
    game.active_player = PlayerId::One;
    game.battlefield
        .push(creature(10_000, cards::ISLAND_SANCTUARY, PlayerId::One));
    game.players[0].library = vec![card(10_001, cards::TERMINUS, PlayerId::One)];

    assert_eq!(game.draw_card(PlayerId::One), None);
    let sanctuary = game.observe(PlayerId::One).decision.unwrap();
    let draw = sanctuary
        .options
        .iter()
        .find(|option| option.label == "Draw the card")
        .unwrap()
        .id;
    game.choose_decision(PlayerId::One, sanctuary.id, &[draw]);

    let miracle = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(miracle.visibility, DecisionVisibility::Private);
    assert!(
        miracle
            .options
            .iter()
            .any(|option| option.label == "Reveal Terminus")
    );
    assert_eq!(game.cards_drawn_this_turn[0], 1);
}

#[test]
fn island_sanctuary_competes_with_and_preserves_an_unchosen_draw_replacement() {
    let mut game = ready_game();
    game.step = Step::Draw;
    game.active_player = PlayerId::One;
    game.battlefield
        .push(creature(10_000, cards::ISLAND_SANCTUARY, PlayerId::One));
    let source = spell(10_001, cards::RING_OF_MARUF, PlayerId::One, 0);
    game.draw_replacements[0].push_back(DrawReplacement {
        object: Box::new(source),
        context: TriggerContext::empty().into(),
        effect: ScopedEffect::primary(EffectDef::None),
        optional: false,
        installed: true,
    });

    assert_eq!(game.draw_card(PlayerId::One), None);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert!(
        decision
            .options
            .iter()
            .all(|option| option.label != "Draw the card"),
        "a mandatory replacement means drawing unchanged is not an option"
    );
    let sanctuary = decision
        .options
        .iter()
        .find(|option| {
            option
                .ability_text
                .as_deref()
                .is_some_and(|text| text.contains("skip that draw"))
        })
        .unwrap()
        .id;
    game.choose_decision(PlayerId::One, decision.id, &[sanctuary]);

    assert_eq!(game.draw_replacements[0].len(), 1);
    assert_eq!(game.resolved_attack_restrictions.len(), 1);
}

#[test]
fn island_sanctuary_blocks_ground_attacks_but_not_fliers_islandwalk_or_planeswalkers() {
    let mut game = ready_game();
    game.step = Step::Draw;
    game.active_player = PlayerId::One;
    let sanctuary = creature(10_000, cards::ISLAND_SANCTUARY, PlayerId::One);
    let sanctuary_id = sanctuary.card.id;
    game.battlefield.push(sanctuary);
    game.players[0].library = vec![card(10_001, cards::PLAINS, PlayerId::One)];

    assert_eq!(game.draw_card(PlayerId::One), None);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    let skip = decision
        .options
        .iter()
        .find(|option| option.ability_text.is_some())
        .unwrap()
        .id;
    game.choose_decision(PlayerId::One, decision.id, &[skip]);
    game.battlefield
        .retain(|permanent| permanent.card.id != sanctuary_id);

    let ground = creature(10_010, cards::GRIZZLY_BEARS, PlayerId::Two);
    let ground_id = ground.card.id;
    let flying = creature(10_011, cards::SERRA_ANGEL, PlayerId::Two);
    let flying_id = flying.card.id;
    let islandwalk = creature(10_012, cards::DEVOURING_DEEP, PlayerId::Two);
    let islandwalk_id = islandwalk.card.id;
    let planeswalker = creature(10_013, cards::VRASKA_THE_UNSEEN, PlayerId::One);
    let planeswalker_id = planeswalker.card.id;
    game.battlefield
        .extend([ground, flying, islandwalk, planeswalker]);
    game.turns_started[PlayerId::Two.index()] = 1;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;

    let actions = game.legal_actions(PlayerId::Two);
    assert!(!actions.contains(&Action::DeclareAttacker {
        attacker: ground_id,
        defender: AttackDefender::Player(PlayerId::One),
    }));
    assert!(actions.contains(&Action::DeclareAttacker {
        attacker: flying_id,
        defender: AttackDefender::Player(PlayerId::One),
    }));
    assert!(actions.contains(&Action::DeclareAttacker {
        attacker: islandwalk_id,
        defender: AttackDefender::Player(PlayerId::One),
    }));
    assert!(actions.contains(&Action::DeclareAttacker {
        attacker: ground_id,
        defender: AttackDefender::Planeswalker(planeswalker_id),
    }));

    game.commit_next_turn(PlayerId::Two, Vec::new());
    assert_eq!(game.resolved_attack_restrictions.len(), 1);
    game.commit_next_turn(PlayerId::One, Vec::new());
    assert!(game.resolved_attack_restrictions.is_empty());
}
