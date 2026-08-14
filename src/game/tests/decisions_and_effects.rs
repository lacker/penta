use super::*;

fn resolve_demonic_tutor(game: &mut Game, tutor: &StackObject) {
    let effect = game
        .catalog
        .get(cards::DEMONIC_TUTOR)
        .expect("Demonic Tutor is cataloged")
        .rules
        .ability_clauses()[0]
        .effect
        .definition;
    game.resolve_effect_def(
        ScopedEffect::primary(effect),
        tutor,
        TriggerContext::empty(),
    );
}

#[test]
fn crusade_declaratively_buffs_every_white_creature() {
    let mut game = ready_game();
    game.battlefield = vec![
        creature(10_000, cards::CRUSADE, PlayerId::One),
        creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One),
        creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two),
        creature(10_003, cards::LOXODON_SMITER, PlayerId::Two),
        creature(10_004, cards::ARBOR_ELF, PlayerId::One),
    ];

    assert_eq!(
        game.catalog
            .get(cards::CRUSADE)
            .unwrap()
            .rules
            .special_behavior(),
        None
    );
    assert_eq!(
        (
            game.power(&game.battlefield[1]),
            game.toughness(&game.battlefield[1])
        ),
        (Some(3), Some(2)),
        "a white creature controlled by Crusade's controller gets +1/+1",
    );
    assert_eq!(
        (
            game.power(&game.battlefield[2]),
            game.toughness(&game.battlefield[2])
        ),
        (Some(3), Some(2)),
        "an opponent's white creature gets +1/+1 too",
    );
    assert_eq!(
        (
            game.power(&game.battlefield[3]),
            game.toughness(&game.battlefield[3])
        ),
        (Some(5), Some(5)),
        "a multicolored white creature qualifies",
    );
    assert_eq!(
        (
            game.power(&game.battlefield[4]),
            game.toughness(&game.battlefield[4])
        ),
        (Some(1), Some(1)),
        "a nonwhite creature is unchanged",
    );
}

#[test]
fn demonic_tutor_exposes_a_library_choice_then_shuffles() {
    let mut game = ready_game();
    game.players[0]
        .library
        .push(card(10_001, cards::JUZAM_DJINN, PlayerId::One));
    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);

    resolve_demonic_tutor(&mut game, &tutor);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.minimum, 1);
    assert_eq!(decision.maximum, 1);
    let option = decision
        .options
        .iter()
        .find(|option| option.card == Some((CardInstanceId(10_001), cards::JUZAM_DJINN)))
        .unwrap();
    let choice = Action::ChooseDecision {
        decision: decision.id,
        options: vec![option.id],
    };
    game.apply(PlayerId::One, choice).unwrap();

    assert_eq!(game.players[0].hand[0].definition, cards::JUZAM_DJINN);
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn demonic_tutor_must_find_a_card_when_the_library_is_not_empty() {
    // CR 701.23d: because Demonic Tutor asks simply for "a card", rather than
    // a card with a stated quality, it must find one whenever one is present.
    let mut game = ready_game();
    for (index, definition) in [cards::JUZAM_DJINN, cards::BLACK_LOTUS]
        .into_iter()
        .enumerate()
    {
        let id = 10_001 + u32::try_from(index).unwrap();
        game.players[0]
            .library
            .push(card(id, definition, PlayerId::One));
    }
    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    resolve_demonic_tutor(&mut game, &tutor);

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.minimum, 1, "an unqualified search is compulsory");
    assert_eq!(decision.maximum, 1);
    assert!(
        !decision.cancellable,
        "failing to find is a resolution, not a way out of the spell"
    );

    assert!(
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: Vec::new(),
            },
        )
        .is_err(),
        "choosing no card is not a legal Demonic Tutor resolution"
    );
}

#[test]
fn the_handcrafted_policy_finds_for_demonic_tutor() {
    use crate::{HandcraftedPolicy, Policy};

    let mut game = ready_game();
    game.players[0]
        .library
        .push(card(10_001, cards::BLACK_LOTUS, PlayerId::One));
    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    resolve_demonic_tutor(&mut game, &tutor);

    let mut policy = HandcraftedPolicy::new(poc::catalog().unwrap());
    let action = policy
        .choose_action(&game.observe(PlayerId::One))
        .expect("the policy answers the search");
    let Action::ChooseDecision { options, .. } = &action else {
        panic!("expected a decision, got {action:?}");
    };
    assert_eq!(options.len(), 1, "the policy searched and found a card");

    game.apply(PlayerId::One, action.clone()).expect("legal");
    assert_eq!(game.players[0].hand.len(), 1, "the card reached hand");
}

#[test]
fn demonic_tutor_shuffles_the_remaining_library() {
    let mut game = ready_game();
    let before: Vec<_> = game.players[0].library.iter().map(|card| card.id).collect();
    assert!(
        before.len() > 10,
        "the deck's library is long enough for a shuffle to be observable"
    );

    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    resolve_demonic_tutor(&mut game, &tutor);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    let chosen = decision.options[0].clone();
    let chosen_card = chosen.card.expect("a library card").0;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![chosen.id],
        },
    )
    .expect("the mandatory search resolves");

    let after: Vec<_> = game.players[0].library.iter().map(|card| card.id).collect();
    assert_eq!(before.len() - 1, after.len(), "one card left the library");
    let before_without_chosen = before
        .into_iter()
        .filter(|card| *card != chosen_card)
        .collect::<Vec<_>>();
    assert_ne!(
        before_without_chosen, after,
        "the remaining library was shuffled"
    );
}

#[test]
fn a_tutor_with_nothing_to_find_leaves_a_legal_action() {
    // An empty library used to produce a decision asking for exactly one of
    // zero options, and not cancellable. `is_legal` rejects a ChooseDecision
    // carrying fewer than `minimum` options, so no legal action existed and
    // the game deadlocked -- every policy stalls, having nothing to return.
    let mut game = ready_game();
    game.players[0].library.clear();
    let tutor = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);

    resolve_demonic_tutor(&mut game, &tutor);

    let observation = game.observe(PlayerId::One);
    assert!(
        observation.decision.is_none(),
        "an impossible search finishes without an empty decision"
    );
    assert!(
        !observation.legal_actions.is_empty(),
        "an empty library must still leave the player something to do"
    );

    assert!(game.pending_decisions.is_empty());
    assert!(game.players[0].hand.is_empty(), "nothing was found");
}

#[test]
fn a_qualified_hidden_zone_search_may_fail_to_find() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0].library.extend([
        card(10_001, cards::BLACK_LOTUS, PlayerId::One),
        card(10_002, cards::JUZAM_DJINN, PlayerId::One),
        card(10_003, cards::SAVANNAH_LIONS, PlayerId::One),
        card(10_004, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_005, cards::MOUNTAIN, PlayerId::One),
    ]);
    let before = game.players[0]
        .library
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    let source = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasType(CardType::Artifact),
            minimum: 0,
            maximum: 1,
            reveal: false,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
        }),
        &source,
        TriggerContext::empty(),
    );

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.minimum, 0);
    assert!(game.observe(PlayerId::Two).decision.is_none());
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("a qualified hidden-zone search may find nothing");

    assert!(game.players[0].hand.is_empty());
    let after = game.players[0]
        .library
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    assert_eq!(after.len(), before.len());
    assert_ne!(after, before, "a failed library search still shuffles");
}

#[test]
fn search_zone_moves_multiple_selected_cards_in_one_resolution() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0].library.extend([
        card(10_001, cards::MOUNTAIN, PlayerId::One),
        card(10_002, cards::PLAINS, PlayerId::One),
        card(10_003, cards::FOREST, PlayerId::One),
        card(10_004, cards::LIGHTNING_BOLT, PlayerId::One),
    ]);
    let source = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::Any,
            minimum: 0,
            maximum: 3,
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
        }),
        &source,
        TriggerContext::empty(),
    );

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!((decision.minimum, decision.maximum), (0, 3));
    let selected = decision.options[..3]
        .iter()
        .map(|option| option.id)
        .collect::<Vec<_>>();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: selected,
        },
    )
    .unwrap();

    assert_eq!(game.players[0].hand.len(), 3);
    assert_eq!(game.players[0].library.len(), 1);
    assert_eq!(
        game.events
            .iter()
            .filter(|event| matches!(event, GameEvent::CardRevealed { .. }))
            .count(),
        3
    );
}

#[test]
fn searching_to_library_top_reveals_and_preserves_the_card_object() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0].library.extend([
        card(10_001, cards::JUZAM_DJINN, PlayerId::One),
        card(10_002, cards::BLACK_LOTUS, PlayerId::One),
        card(10_003, cards::SAVANNAH_LIONS, PlayerId::One),
        card(10_004, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_005, cards::MOUNTAIN, PlayerId::One),
        card(10_006, cards::SERRA_ANGEL, PlayerId::One),
    ]);
    let remainder_before = game.players[0]
        .library
        .iter()
        .filter(|card| card.id != CardInstanceId(10_002))
        .map(|card| card.id)
        .collect::<Vec<_>>();
    let source = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasType(CardType::Artifact),
            minimum: 0,
            maximum: 1,
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: true,
        }),
        &source,
        TriggerContext::empty(),
    );

    let decision = game.observe(PlayerId::One).decision.unwrap();
    let lotus = decision
        .options
        .iter()
        .find(|option| option.card == Some((CardInstanceId(10_002), cards::BLACK_LOTUS)))
        .expect("Black Lotus matches the search");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![lotus.id],
        },
    )
    .unwrap();

    let top = game.players[0].library.last().expect("a top card");
    assert_eq!(top.id, CardInstanceId(10_002));
    assert_eq!(top.definition, cards::BLACK_LOTUS);
    let remainder_after = game.players[0].library[..game.players[0].library.len() - 1]
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    assert_ne!(
        remainder_after, remainder_before,
        "the found card is excluded while the remaining library is shuffled"
    );
    assert!(game.events.iter().any(|event| {
        matches!(
            event,
            GameEvent::CardRevealed {
                player: PlayerId::One,
                card: CardInstanceId(10_002),
                definition,
            } if *definition == cards::BLACK_LOTUS
        )
    }));
}

#[test]
fn search_zone_can_move_a_public_graveyard_card_to_hand() {
    let mut game = ready_game();
    game.players[0].graveyard.clear();
    game.players[0]
        .graveyard
        .push(card(10_001, cards::LIGHTNING_BOLT, PlayerId::One));
    let source = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Graveyard,
            object: ObjectPredicateDef::Any,
            minimum: 1,
            maximum: 1,
            reveal: false,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: false,
        }),
        &source,
        TriggerContext::empty(),
    );

    let decision = game
        .observe(PlayerId::Two)
        .decision
        .expect("a public-zone search is publicly observable");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[0].id],
        },
    )
    .unwrap();

    assert!(game.players[0].graveyard.is_empty());
    let moved = game.players[0].hand.last().expect("the searched-for card");
    assert_eq!(moved.definition, cards::LIGHTNING_BOLT);
    assert_ne!(moved.id, CardInstanceId(10_001));
}

#[test]
fn search_zone_supports_private_hands_and_public_exile() {
    let source = spell(10_000, cards::DEMONIC_TUTOR, PlayerId::One, 0);

    let mut hand_game = ready_game();
    hand_game.players[0].hand.clear();
    hand_game.players[0]
        .hand
        .push(card(10_101, cards::LIGHTNING_BOLT, PlayerId::One));
    hand_game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Hand,
            object: ObjectPredicateDef::Any,
            minimum: 1,
            maximum: 1,
            reveal: false,
            destination: ZoneKind::Graveyard,
            placement: ZonePlacement::Top,
            shuffle: false,
        }),
        &source,
        TriggerContext::empty(),
    );
    let decision = hand_game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.visibility, DecisionVisibility::Private);
    assert!(hand_game.observe(PlayerId::Two).decision.is_none());
    hand_game
        .apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![decision.options[0].id],
            },
        )
        .unwrap();
    let moved = hand_game.players[0].graveyard.last().unwrap();
    assert_eq!(moved.definition, cards::LIGHTNING_BOLT);
    assert_ne!(moved.id, GameObjectId(10_101));

    let mut exile_game = ready_game();
    exile_game.players[0]
        .exile
        .push(card(10_102, cards::SERRA_ANGEL, PlayerId::One));
    exile_game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Exile,
            object: ObjectPredicateDef::Any,
            minimum: 1,
            maximum: 1,
            reveal: false,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: false,
        }),
        &source,
        TriggerContext::empty(),
    );
    let decision = exile_game
        .observe(PlayerId::Two)
        .decision
        .expect("an exile search is public");
    assert_eq!(decision.visibility, DecisionVisibility::Public);
    exile_game
        .apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![decision.options[0].id],
            },
        )
        .unwrap();
    let moved = exile_game.players[0].hand.last().unwrap();
    assert_eq!(moved.definition, cards::SERRA_ANGEL);
    assert_ne!(moved.id, GameObjectId(10_102));
}

#[test]
fn armageddon_destroys_every_land_but_not_creatures() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::CITY_OF_BRASS, PlayerId::One),
        creature(10_001, cards::SWAMP, PlayerId::Two),
        creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);
    let armageddon = spell(10_003, cards::ARMAGEDDON, PlayerId::One, 0);

    let effect = game
        .catalog
        .get(cards::ARMAGEDDON)
        .expect("Armageddon is in the catalog")
        .rules
        .ability_clauses()[0]
        .effect
        .definition;
    game.resolve_effect_def(
        ScopedEffect::primary(effect),
        &armageddon,
        TriggerContext::empty(),
    );

    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(game.battlefield[0].card.definition, cards::SAVANNAH_LIONS);
}

#[test]
fn recall_discards_and_returns_as_it_resolves() {
    let mut game = ready_game();
    game.players[0].hand.extend([
        card(10_000, cards::RECALL, PlayerId::One),
        card(10_001, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_002, cards::BALANCE, PlayerId::One),
    ]);
    game.players[0].mana_pool = ManaPool {
        blue: 1,
        colorless: 4,
        ..ManaPool::default()
    };

    game.cast_spell(
        PlayerId::One,
        CardInstanceId(10_000),
        &cast_choices(Vec::new(), 2),
        &[],
    );
    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "nothing is discarded to cast it, so there is no cost decision"
    );
    assert_eq!(game.players[0].graveyard.len(), 0);

    pass_priority_pair(&mut game);
    let discard = game.observe(PlayerId::One).decision.unwrap();
    assert!(
        !discard.cancellable,
        "a resolving spell cannot be taken back"
    );
    assert_eq!(discard.minimum, 2);
    let discard_action = Action::ChooseDecision {
        decision: discard.id,
        options: discard
            .options
            .iter()
            .take(discard.minimum)
            .map(|option| option.id)
            .collect(),
    };
    game.apply(PlayerId::One, discard_action).unwrap();

    let return_decision = game.observe(PlayerId::One).decision.unwrap();
    assert!(!return_decision.cancellable);
    assert_eq!(return_decision.minimum, 2);
    let return_action = Action::ChooseDecision {
        decision: return_decision.id,
        options: return_decision
            .options
            .iter()
            .take(return_decision.minimum)
            .map(|option| option.id)
            .collect(),
    };
    game.apply(PlayerId::One, return_action).unwrap();

    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(game.players[0].exile[0].definition, cards::RECALL);
}

#[test]
fn recall_x_may_exceed_the_hand_and_discards_what_it_can() {
    // X is chosen when Recall is cast and the discard happens on resolution,
    // so nothing caps X at the hand size. A short hand just discards, and
    // returns, fewer.
    let mut game = ready_game();
    game.players[0].hand.extend([
        card(10_000, cards::RECALL, PlayerId::One),
        card(10_001, cards::LIGHTNING_BOLT, PlayerId::One),
    ]);
    game.players[0]
        .graveyard
        .push(card(10_002, cards::BALANCE, PlayerId::One));
    game.players[0].mana_pool = ManaPool {
        blue: 1,
        colorless: 6,
        ..ManaPool::default()
    };

    assert!(
        game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == CardInstanceId(10_000) && choices.x() == 3)
        }),
        "three is affordable even though only one other card is in hand"
    );

    game.cast_spell(
        PlayerId::One,
        CardInstanceId(10_000),
        &cast_choices(Vec::new(), 3),
        &[],
    );
    pass_priority_pair(&mut game);
    let discard = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(discard.minimum, 1, "only one card is there to discard");
    choose_all_offered(&mut game, PlayerId::One);

    let returns = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(returns.minimum, 1, "and so only one comes back");
    choose_all_offered(&mut game, PlayerId::One);
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn a_countered_recall_costs_no_cards() {
    // The discard used to be an additional cost, so countering Recall was a
    // two-for-one that also stripped the caster's hand.
    let mut game = ready_game();
    game.players[0].hand.extend([
        card(10_000, cards::RECALL, PlayerId::One),
        card(10_001, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_002, cards::BALANCE, PlayerId::One),
    ]);
    game.players[1]
        .hand
        .push(card(10_003, cards::COUNTERSPELL, PlayerId::Two));
    game.players[0].mana_pool = ManaPool {
        blue: 1,
        colorless: 4,
        ..ManaPool::default()
    };
    game.players[1].mana_pool = ManaPool {
        blue: 2,
        ..ManaPool::default()
    };

    game.cast_spell(
        PlayerId::One,
        CardInstanceId(10_000),
        &cast_choices(Vec::new(), 2),
        &[],
    );
    acceptance_attempt_counterspell(&mut game, CardInstanceId(10_003));
    drain_pending(&mut game);

    assert_eq!(
        game.players[0].hand.len(),
        2,
        "Bolt and Balance are still in hand"
    );
    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::RECALL],
        "only the countered Recall is there; nothing was discarded, and the \
         exile happens on resolution, which never came"
    );
}

#[test]
fn balance_counts_an_animated_land_in_both_phases() {
    // Balance settles lands, then hands, then creatures, recounting each time.
    // An animated Mishra's Factory is a land and a creature at once, so it
    // has to be counted twice -- and the land phase running first is what
    // decides whether it is still there for the creature count.
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::MISHRA_S_FACTORY, PlayerId::One),
        creature(10_001, cards::SWAMP, PlayerId::Two),
        creature(10_002, cards::FOREST, PlayerId::Two),
    ]);
    game.battlefield[0].animation = Some(ResolvedAnimation {
        definition: &abilities::MISHRAS_FACTORY_ANIMATION,
        timestamp: game.battlefield[0].timestamp,
    });

    game.resolve_balance(PlayerId::One);
    let mut prompts = Vec::new();
    while let Some(player) = game.decision_player() {
        let Some(decision) = game.observe(player).decision else {
            break;
        };
        prompts.push((player, decision.prompt.clone()));
        choose_all_offered(&mut game, player);
    }

    assert_eq!(
        prompts,
        vec![
            (
                PlayerId::Two,
                "Choose 1 land(s) to sacrifice to Balance".into()
            ),
            (
                PlayerId::One,
                "Choose 1 creature(s) to sacrifice to Balance".into()
            ),
        ],
        "the Factory kept its controller's land count level, then lost the \
         creature count outright"
    );
    assert_eq!(
        game.battlefield.len(),
        1,
        "one land each was two lands, and the Factory was one of them"
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(10_000)),
        "and the Factory is what went"
    );
}

#[test]
fn balance_requests_public_sacrifices_and_private_discards() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::PLAINS, PlayerId::One),
        creature(10_001, cards::CITY_OF_BRASS, PlayerId::One),
        creature(10_002, cards::SWAMP, PlayerId::Two),
    ]);
    game.players[0].hand.extend([
        card(10_003, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_004, cards::BALANCE, PlayerId::One),
    ]);
    game.players[1]
        .hand
        .push(card(10_005, cards::TERROR, PlayerId::Two));

    game.resolve_balance(PlayerId::One);
    assert_eq!(
        game.observe(PlayerId::Two).decision.unwrap().visibility,
        DecisionVisibility::Public
    );
    let decision_player = game.decision_player().unwrap();
    let pending_actions = game.legal_actions(decision_player);
    assert_eq!(pending_actions.len(), 2);
    assert!(matches!(
        &pending_actions[1],
        Action::ChooseDecision {
            decision: _,
            options
        } if options.is_empty()
    ));
    while let Some(player) = game.decision_player() {
        let Some(decision) = game.observe(player).decision else {
            break;
        };
        let action = Action::ChooseDecision {
            decision: decision.id,
            options: decision
                .options
                .iter()
                .take(decision.minimum)
                .map(|option| option.id)
                .collect(),
        };
        game.apply(player, action).unwrap();
    }

    let land_counts = [PlayerId::One, PlayerId::Two].map(|player| {
        game.battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && game
                        .permanent_types(permanent)
                        .is_some_and(|types| types.contains(CardType::Land))
            })
            .count()
    });
    assert_eq!(land_counts, [1, 1]);
    assert_eq!(game.players[0].hand.len(), game.players[1].hand.len());
}

#[test]
fn balance_recounts_creatures_after_loxodon_smiter_replaces_its_discard() {
    let mut game = ready_game();
    let balance = card(10_010, cards::BALANCE, PlayerId::One);
    game.players[0].hand.push(balance.clone());
    game.players[1]
        .hand
        .push(card(10_011, cards::LOXODON_SMITER, PlayerId::Two));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    game.apply(
        PlayerId::One,
        cast_action(balance.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let discard = game
        .observe(PlayerId::Two)
        .decision
        .expect("Balance makes player two discard down to zero");
    assert_eq!(discard.visibility, DecisionVisibility::Private);
    let smiter = discard
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(_, definition)| definition == cards::LOXODON_SMITER)
        })
        .expect("Loxodon Smiter is the discard choice")
        .id;
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: discard.id,
            options: vec![smiter],
        },
    )
    .unwrap();

    let sacrifice = game
        .observe(PlayerId::Two)
        .decision
        .expect("the creature step is counted after the discard step");
    assert_eq!(sacrifice.visibility, DecisionVisibility::Public);
    assert!(sacrifice.prompt.contains("creature"));
    assert_eq!(sacrifice.options.len(), 1);
    assert!(
        sacrifice.options[0]
            .card
            .is_some_and(|(_, definition)| definition == cards::LOXODON_SMITER)
    );
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: sacrifice.id,
            options: vec![sacrifice.options[0].id],
        },
    )
    .unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::LOXODON_SMITER)
    );
    assert_eq!(
        game.players[1]
            .graveyard
            .iter()
            .filter(|card| card.definition == cards::LOXODON_SMITER)
            .count(),
        1,
    );
    assert!(game.events.iter().any(|event| matches!(
        event,
        GameEvent::CardsDiscarded {
            player: PlayerId::Two,
            cards,
        } if cards.iter().any(|(_, definition)| *definition == cards::LOXODON_SMITER)
    )));
}

#[test]
fn balance_defers_one_apnap_trigger_batch_until_its_decisions_finish() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::SU_CHI, PlayerId::One),
        creature(10_001, cards::SU_CHI, PlayerId::One),
        creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
        creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two),
    ]);
    game.players[0].hand.extend([
        card(10_004, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_005, cards::MOUNTAIN, PlayerId::One),
    ]);

    game.resolve_balance(PlayerId::One);
    let discard = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(discard.kind, DecisionKind::Choice);
    assert!(discard.prompt.contains("discard"));
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: discard.id,
            options: discard.options.iter().map(|option| option.id).collect(),
        },
    )
    .unwrap();

    let sacrifice = game.observe(PlayerId::One).decision.unwrap();
    let su_chi = sacrifice
        .options
        .iter()
        .filter(|option| {
            option
                .card
                .is_some_and(|(_, definition)| definition == cards::SU_CHI)
        })
        .map(|option| option.id)
        .collect::<Vec<_>>();
    assert_eq!(su_chi.len(), 2);
    assert!(sacrifice.prompt.contains("creature"));
    assert!(game.stack.is_empty());
    assert!(game.pending_triggers.is_empty());
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: sacrifice.id,
            options: su_chi,
        },
    )
    .unwrap();

    let order = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(order.kind, DecisionKind::TriggerOrder);
    assert_eq!(order.options.len(), 2);
    assert!(game.stack.is_empty());
    assert!(game.pending_triggers.is_empty());

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: order.id,
            options: order.options.iter().map(|option| option.id).collect(),
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 2);
    assert!(
        game.stack
            .iter()
            .all(|object| object.kind == StackObjectKind::TriggeredAbility)
    );
}
