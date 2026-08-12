use super::*;

fn lilianas_shade_may_decision(game: &mut Game) -> DecisionObservation {
    game.put_onto_battlefield(PlayerId::One, cards::LILIANAS_SHADE)
        .expect("Liliana's Shade is cataloged");
    for _ in 0..12 {
        if let Some(decision) = game.observe(PlayerId::One).decision {
            return decision;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority).unwrap();
    }
    panic!("Liliana's Shade never offered its optional search");
}

#[test]
fn enlightened_tutor_filters_reveals_and_puts_the_same_object_on_top() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0].library.extend([
        card(13_000, cards::SERRA_ANGEL, PlayerId::One),
        card(13_001, cards::CRUSADE, PlayerId::One),
        card(13_002, cards::BLACK_LOTUS, PlayerId::One),
    ]);
    let tutor = card(13_100, cards::ENLIGHTENED_TUTOR, PlayerId::One);
    game.players[0].hand.push(tutor.clone());
    game.players[0].mana_pool.white = 1;
    let event_start = game.events().len();

    game.apply(
        PlayerId::One,
        cast_action(tutor.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.visibility, DecisionVisibility::Private);
    assert_eq!((decision.minimum, decision.maximum), (0, 1));
    assert!(game.observe(PlayerId::Two).decision.is_none());
    let offered = decision
        .options
        .iter()
        .filter_map(|option| option.card.map(|(_, definition)| definition))
        .collect::<Vec<_>>();
    assert!(offered.contains(&cards::BLACK_LOTUS));
    assert!(offered.contains(&cards::CRUSADE));
    assert!(!offered.contains(&cards::SERRA_ANGEL));
    let lotus = decision
        .options
        .iter()
        .find(|option| option.card == Some((GameObjectId(13_002), cards::BLACK_LOTUS)))
        .unwrap()
        .id;

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![lotus],
        },
    )
    .unwrap();

    let top = game.players[0].library.last().unwrap();
    assert_eq!(
        (top.id, top.definition),
        (GameObjectId(13_002), cards::BLACK_LOTUS)
    );
    assert!(game.events()[event_start..].iter().any(|event| {
        matches!(
            event,
            GameEvent::CardRevealed {
                player: PlayerId::One,
                card: GameObjectId(13_002),
                definition,
            } if *definition == cards::BLACK_LOTUS
        )
    }));
    assert!(game
        .events_for(PlayerId::Two)
        .iter()
        .any(|event| matches!(event, GameEvent::CardRevealed { definition, .. } if *definition == cards::BLACK_LOTUS)));
}

#[test]
fn lilianas_shade_decline_skips_the_search_reveal_and_shuffle() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0].library.extend([
        card(13_110, cards::SAVANNAH_LIONS, PlayerId::One),
        card(13_111, cards::SWAMP, PlayerId::One),
        card(13_112, cards::LIGHTNING_BOLT, PlayerId::One),
        card(13_113, cards::BLACK_LOTUS, PlayerId::One),
        card(13_114, cards::SERRA_ANGEL, PlayerId::One),
    ]);
    let before = game.players[0]
        .library
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    let event_start = game.events().len();
    let may = lilianas_shade_may_decision(&mut game);
    let decline = may
        .options
        .iter()
        .find(|option| option.label == "Decline")
        .expect("the optional search can be declined")
        .id;

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: may.id,
            options: vec![decline],
        },
    )
    .unwrap();

    assert_eq!(
        game.players[0]
            .library
            .iter()
            .map(|card| card.id)
            .collect::<Vec<_>>(),
        before,
        "declining skips the search's shuffle"
    );
    assert!(
        !game.events()[event_start..]
            .iter()
            .any(|event| matches!(event, GameEvent::CardRevealed { .. }))
    );
}

#[test]
fn lilianas_shade_acceptance_still_allows_qualified_fail_to_find() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0].library.extend([
        card(13_120, cards::SAVANNAH_LIONS, PlayerId::One),
        card(13_121, cards::SWAMP, PlayerId::One),
        card(13_122, cards::LIGHTNING_BOLT, PlayerId::One),
        card(13_123, cards::BLACK_LOTUS, PlayerId::One),
        card(13_124, cards::SERRA_ANGEL, PlayerId::One),
        card(13_125, cards::CRUSADE, PlayerId::One),
    ]);
    let before = game.players[0]
        .library
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    let event_start = game.events().len();
    let may = lilianas_shade_may_decision(&mut game);
    let accept = may
        .options
        .iter()
        .find(|option| option.label != "Decline")
        .expect("the optional search can be accepted")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: may.id,
            options: vec![accept],
        },
    )
    .unwrap();

    let search = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!((search.minimum, search.maximum), (0, 1));
    assert_eq!(search.options.len(), 1);
    assert_eq!(
        search.options[0].card.map(|(_, card)| card),
        Some(cards::SWAMP)
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: search.id,
            options: Vec::new(),
        },
    )
    .unwrap();

    assert_eq!(game.players[0].library.len(), before.len());
    assert_ne!(
        game.players[0]
            .library
            .iter()
            .map(|card| card.id)
            .collect::<Vec<_>>(),
        before,
        "accepting and failing to find still shuffles"
    );
    assert!(
        !game.events()[event_start..]
            .iter()
            .any(|event| matches!(event, GameEvent::CardRevealed { .. }))
    );
}

#[test]
fn seek_the_horizon_reveals_and_moves_three_basics_then_shuffles() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0].library.extend([
        card(13_130, cards::SAVANNAH_LIONS, PlayerId::One),
        card(13_131, cards::PLAINS, PlayerId::One),
        card(13_132, cards::LIGHTNING_BOLT, PlayerId::One),
        card(13_133, cards::ISLAND, PlayerId::One),
        card(13_134, cards::BLACK_LOTUS, PlayerId::One),
        card(13_135, cards::SWAMP, PlayerId::One),
        card(13_136, cards::SERRA_ANGEL, PlayerId::One),
        card(13_137, cards::CRUSADE, PlayerId::One),
    ]);
    let selected_ids = [
        GameObjectId(13_131),
        GameObjectId(13_133),
        GameObjectId(13_135),
    ];
    let remainder_before = game.players[0]
        .library
        .iter()
        .filter(|card| !selected_ids.contains(&card.id))
        .map(|card| card.id)
        .collect::<Vec<_>>();
    let seek = card(13_140, cards::SEEK_THE_HORIZON, PlayerId::One);
    game.players[0].hand.push(seek.clone());
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 3;
    let event_start = game.events().len();

    game.apply(
        PlayerId::One,
        cast_action(seek.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let search = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!((search.minimum, search.maximum), (0, 3));
    assert_eq!(search.options.len(), 3);
    let choices = search.options.iter().map(|option| option.id).collect();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: search.id,
            options: choices,
        },
    )
    .unwrap();

    let hand = game.players[0]
        .hand
        .iter()
        .map(|card| card.definition)
        .collect::<Vec<_>>();
    for basic in [cards::PLAINS, cards::ISLAND, cards::SWAMP] {
        assert!(hand.contains(&basic), "the selected basic moved to hand");
        assert!(game.events()[event_start..].iter().any(|event| {
            matches!(event, GameEvent::CardRevealed { definition, .. } if *definition == basic)
        }));
    }
    let remainder_after = game.players[0]
        .library
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    assert_eq!(remainder_after.len(), remainder_before.len());
    assert_ne!(
        remainder_after, remainder_before,
        "the remaining library shuffled"
    );
}

#[test]
fn each_onslaught_fetch_land_pays_its_cost_and_finds_the_named_land_types() {
    let cases = [
        (cards::BLOODSTAINED_MIRE, cards::BADLANDS, cards::TUNDRA),
        (cards::FLOODED_STRAND, cards::TUNDRA, cards::BADLANDS),
        (cards::POLLUTED_DELTA, cards::UNDERGROUND_SEA, cards::TAIGA),
        (
            cards::WINDSWEPT_HEATH,
            cards::SAVANNAH,
            cards::VOLCANIC_ISLAND,
        ),
        (cards::WOODED_FOOTHILLS, cards::TAIGA, cards::TUNDRA),
    ];

    for (fetch, matching, off_pair) in cases {
        let mut game = ready_game();
        let source = game.put_onto_battlefield(PlayerId::One, fetch).unwrap();
        game.players[0].library.clear();
        game.players[0].library.extend([
            card(13_200, off_pair, PlayerId::One),
            card(13_201, matching, PlayerId::One),
        ]);
        let event_start = game.events().len();
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source)
            })
            .unwrap_or_else(|| panic!("fetch ability was not offered for {fetch:?}"));

        game.apply(PlayerId::One, action).unwrap();

        assert_eq!(game.players[0].life, 19);
        assert!(
            game.battlefield
                .iter()
                .all(|permanent| permanent.card.id != source)
        );
        assert!(
            game.players[0]
                .graveyard
                .iter()
                .any(|card| card.definition == fetch)
        );
        assert!(game.events()[event_start..].iter().any(|event| {
            matches!(
                event,
                GameEvent::LifeLost {
                    player: PlayerId::One,
                    amount: 1,
                }
            )
        }));
        assert_eq!(game.stack.len(), 1, "costs are paid before resolution");

        pass_priority_pair(&mut game);
        let decision = game.observe(PlayerId::One).decision.unwrap();
        assert_eq!(decision.visibility, DecisionVisibility::Private);
        assert_eq!((decision.minimum, decision.maximum), (0, 1));
        assert_eq!(
            decision
                .options
                .iter()
                .filter_map(|option| option.card.map(|(_, definition)| definition))
                .collect::<Vec<_>>(),
            vec![matching]
        );
        let option = decision.options[0].id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![option],
            },
        )
        .unwrap();

        assert!(game.battlefield.iter().any(|permanent| {
            permanent.controller == PlayerId::One && permanent.card.definition == matching
        }));
        assert_eq!(game.players[0].library.len(), 1);
        assert_eq!(game.players[0].library[0].definition, off_pair);
    }
}

#[test]
fn a_fetch_finishes_the_lands_as_enters_choice_before_shuffling() {
    let mut game = ready_game();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::FLOODED_STRAND)
        .unwrap();
    game.players[0].library = (13_300..13_312)
        .map(|id| card(id, cards::MOUNTAIN, PlayerId::One))
        .chain(std::iter::once(card(
            13_312,
            cards::HALLOWED_FOUNTAIN,
            PlayerId::One,
        )))
        .collect();
    let remaining_before = game.players[0].library[..12]
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source)
        })
        .expect("Flooded Strand's ability is offered");

    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);
    let search = game.observe(PlayerId::One).decision.unwrap();
    let fountain = search
        .options
        .iter()
        .find(|option| option.card == Some((GameObjectId(13_312), cards::HALLOWED_FOUNTAIN)))
        .expect("Flooded Strand finds a Plains Island")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: search.id,
            options: vec![fountain],
        },
    )
    .unwrap();

    let entry = game.observe(PlayerId::One).decision.unwrap();
    assert!(entry.prompt.contains("Hallowed Fountain"));
    assert_eq!(
        game.players[0]
            .library
            .iter()
            .map(|card| card.id)
            .collect::<Vec<_>>(),
        remaining_before,
        "the search has not shuffled while the land is still entering"
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: entry.id,
            options: vec![0],
        },
    )
    .unwrap();

    let fountain = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::HALLOWED_FOUNTAIN)
        .expect("Hallowed Fountain finished entering");
    assert!(fountain.tapped, "declining the life payment taps the land");
    let remaining_after = game.players[0]
        .library
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    assert_eq!(remaining_after.len(), remaining_before.len());
    assert_ne!(
        remaining_after, remaining_before,
        "the search shuffles immediately after entry finishes"
    );
    assert!(game.pending_procedures.is_empty());
}
