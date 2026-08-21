use super::*;

static FIRST_DUPLICATE_MIRACLE_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::spell("Do nothing.", EffectDef::None),
    AbilityDef::alternative_cast(
        ManaCost::new(0, 1),
        AlternativeCastKindDef::Miracle,
        None,
        EffectDef::None,
    ),
];
static SECOND_DUPLICATE_MIRACLE_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::spell("Do nothing else.", EffectDef::None),
    AbilityDef::alternative_cast(
        ManaCost::new(3, 1),
        AlternativeCastKindDef::Miracle,
        None,
        EffectDef::None,
    ),
];

fn miracle_game(
    definition: CardDefinitionId,
    mana_source: CardDefinitionId,
    sources: usize,
) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library = vec![card(20_000, definition, PlayerId::One)];
    for _ in 0..sources {
        game.put_onto_battlefield(PlayerId::One, mana_source)
            .expect("the mana source is cataloged");
    }
    game.turn = 2;
    game.step = Step::Draw;
    game.priority = PlayerId::One;
    game.cards_drawn_this_turn = [0; 2];
    game.drawn_this_turn = [Vec::new(), Vec::new()];
    game
}

fn answer_draw_action_window(game: &mut Game, reveal: bool) {
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the first draw has an action window");
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: reveal.then_some(1).into_iter().collect(),
        },
    )
    .expect("the draw action is legal");
}

fn protocol_observation(game: &Game, viewer: PlayerId) -> serde_json::Value {
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    )
}

fn resolve_miracle_trigger(game: &mut Game) {
    assert_eq!(
        game.stack
            .iter()
            .filter(|object| object.kind == StackObjectKind::TriggeredAbility)
            .count(),
        1,
        "revealing puts Miracle's linked trigger on the stack"
    );
    assert!(
        game.pending_decisions.is_empty(),
        "the cast offer waits for that trigger to resolve"
    );
    pass_until_decision(game);
    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::MayCastAlternative {
                ability: AbilityOrigin::Printed { .. },
                ..
            })
        ),
        "resolving the linked trigger creates the one-shot Miracle offer"
    );
}

fn selected_alternative(game: &Game, action: &Action) -> Option<AlternativeCastKindDef> {
    let Action::CastSpell { card, choices, .. } = action else {
        return None;
    };
    let instance = game.players[0]
        .hand
        .iter()
        .find(|candidate| candidate.id == *card)?;
    let definition = game.catalog.get(instance.definition)?;
    let option = definition.play_option(choices.play_option())?;
    game.selected_alternative_kind(definition, option, *card, choices.costs())
}

#[test]
fn miracle_reveal_uses_the_real_definition_and_creates_a_linked_trigger() {
    let mut game = miracle_game(cards::TERMINUS, cards::PLAINS, 1);
    let drawn = game.draw_card(PlayerId::One).expect("Terminus is drawn");
    let reveal = game
        .observe(PlayerId::One)
        .decision
        .expect("the drawing player sees the private reveal");
    assert_eq!(reveal.visibility, DecisionVisibility::Private);
    assert_eq!((reveal.minimum, reveal.maximum), (0, 1));
    assert_eq!(reveal.preference, DecisionPreference::PreferOption(1));
    assert_eq!(reveal.options.len(), 1);
    assert_eq!(reveal.options[0].id, 1);
    assert_eq!(
        reveal.options[0].card,
        Some((drawn, cards::TERMINUS)),
        "the option carries the card's actual catalog identity"
    );
    assert!(
        game.observe(PlayerId::Two).decision.is_none(),
        "the opponent does not see the reveal choice"
    );

    answer_draw_action_window(&mut game, true);

    assert!(game.events.iter().any(|event| {
        matches!(
            event,
            GameEvent::CardRevealed {
                player: PlayerId::One,
                card,
                definition: cards::TERMINUS,
            } if *card == drawn
        )
    }));
    assert!(game.pending_decisions.is_empty());
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].source, Some(drawn));
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
}

#[test]
fn only_the_first_card_drawn_in_a_turn_offers_miracle() {
    let mut game = miracle_game(cards::TERMINUS, cards::PLAINS, 1);
    game.players[0].library = vec![
        card(20_010, cards::TERMINUS, PlayerId::One),
        card(20_011, cards::PLAINS, PlayerId::One),
    ];

    let decision_before = game.next_decision_id;
    game.draw_card(PlayerId::One);
    assert!(
        game.pending_decisions.is_empty(),
        "an ordinary first draw resolves its empty action atomically"
    );
    assert_eq!(
        game.next_decision_id,
        decision_before.saturating_add(1),
        "the hidden draw-action check still allocates its decision ID"
    );
    game.draw_card(PlayerId::One);
    assert!(
        game.pending_decisions.is_empty(),
        "a Miracle card drawn second has no reveal choice"
    );
}

#[test]
fn an_ordinary_draw_and_a_declined_miracle_are_identical_to_the_opponent() {
    let mut ordinary = miracle_game(cards::PLAINS, cards::PLAINS, 0);
    let mut miracle = miracle_game(cards::TERMINUS, cards::PLAINS, 0);

    ordinary.draw_card(PlayerId::One);
    miracle.draw_card(PlayerId::One);

    assert!(ordinary.pending_decisions.is_empty());
    assert_eq!(miracle.pending_decisions[0].observation.options.len(), 1);
    assert_eq!(
        ordinary.next_decision_id, miracle.next_decision_id,
        "both hidden draw-action paths allocate the same decision ID"
    );
    assert_eq!(
        ordinary.events_for(PlayerId::Two),
        miracle.events_for(PlayerId::Two),
        "the private prompt itself emits no distinguishing opponent event"
    );

    answer_draw_action_window(&mut miracle, false);

    assert_eq!(
        ordinary.observe(PlayerId::Two),
        miracle.observe(PlayerId::Two),
        "an empty ordinary answer and a declined Miracle settle identically"
    );
    assert_eq!(
        protocol_observation(&ordinary, PlayerId::Two),
        protocol_observation(&miracle, PlayerId::Two),
        "settled bot observations retain no trace of the hidden option"
    );
    assert_eq!(
        ordinary.events_for(PlayerId::Two),
        miracle.events_for(PlayerId::Two),
        "declining does not leave a distinguishing opponent event"
    );
}

#[test]
fn declining_the_reveal_creates_neither_reveal_event_nor_trigger() {
    let mut game = miracle_game(cards::TERMINUS, cards::PLAINS, 1);
    let drawn = game.draw_card(PlayerId::One).expect("Terminus is drawn");
    answer_draw_action_window(&mut game, false);

    assert!(game.pending_decisions.is_empty());
    assert!(game.stack.is_empty());
    assert!(
        !game
            .events
            .iter()
            .any(|event| matches!(event, GameEvent::CardRevealed { card, .. } if *card == drawn))
    );
}

#[test]
fn a_multi_card_draw_finishes_before_the_miracle_trigger_can_offer_a_cast() {
    let mut game = miracle_game(cards::TERMINUS, cards::PLAINS, 1);
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (20_100, cards::TERMINUS),
            (20_101, cards::LIGHTNING_BOLT),
            (20_102, cards::SAVANNAH_LIONS),
        ],
    );

    game.draw_cards(PlayerId::One, 3);
    assert_eq!(game.players[0].hand.len(), 1);
    answer_draw_action_window(&mut game, true);

    assert_eq!(
        game.players[0].hand.len(),
        3,
        "the suspended draw instruction completed before priority"
    );
    assert!(game.pending_decisions.is_empty());
    assert_eq!(game.stack.len(), 1, "only then is the trigger stacked");
}

#[test]
fn simultaneous_miracles_keep_both_linked_triggers_in_apnap_order() {
    let mut game = ready_game();
    game.battlefield.clear();
    for player in [PlayerId::One, PlayerId::Two] {
        game.players[player.index()].hand.clear();
        game.players[player.index()].library = vec![card(
            20_200 + u32::try_from(player.index()).unwrap_or(0),
            cards::TERMINUS,
            player,
        )];
        game.put_onto_battlefield(player, cards::PLAINS)
            .expect("cataloged");
    }
    game.turn = 2;
    game.step = Step::Draw;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.cards_drawn_this_turn = [0; 2];
    game.drawn_this_turn = [Vec::new(), Vec::new()];

    game.draw_cards_simultaneously([1, 1]);
    answer_draw_action_window(&mut game, true);
    assert_eq!(
        game.decision_player(),
        Some(PlayerId::Two),
        "the simultaneous instruction reaches the other player's reveal"
    );
    answer_draw_action_window(&mut game, true);

    assert_eq!(
        game.stack
            .iter()
            .map(|object| object.controller)
            .collect::<Vec<_>>(),
        vec![PlayerId::One, PlayerId::Two],
        "the nonactive player's trigger is on top"
    );
    pass_until_decision(&mut game);
    assert!(matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::MayCastAlternative {
            player: PlayerId::Two,
            ..
        }
    ));
    let first_offer = game.pending_decisions[0].observation.clone();
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: first_offer.id,
            options: vec![0],
        },
    )
    .unwrap();
    pass_until_decision(&mut game);
    assert!(matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::MayCastAlternative {
            player: PlayerId::One,
            ..
        }
    ));
}

#[test]
fn terminus_offer_contains_only_its_miracle_cost_and_survives_revalidation() {
    let mut game = miracle_game(cards::TERMINUS, cards::PLAINS, 6);
    game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    let drawn = game.draw_card(PlayerId::One).expect("Terminus is drawn");
    answer_draw_action_window(&mut game, true);
    resolve_miracle_trigger(&mut game);

    let casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == drawn))
        .collect::<Vec<_>>();
    assert!(!casts.is_empty(), "the one-white Miracle cast is offered");
    assert!(
        casts
            .iter()
            .all(|action| selected_alternative(&game, action)
                == Some(AlternativeCastKindDef::Miracle)),
        "the affordable printed cost does not leak into the one-shot offer"
    );

    game.apply(PlayerId::One, casts[0].clone())
        .expect("the offer remains authoritative through cast revalidation");
    drain_pending(&mut game);
    assert!(game.pending_decisions.is_empty());
    assert!(!game.battlefield.iter().any(|permanent| {
        matches!(
            permanent.card.definition,
            cards::SAVANNAH_LIONS | cards::SERRA_ANGEL
        )
    }));
}

#[test]
fn miracle_offer_names_the_exact_linked_clause_across_same_kind_play_options() {
    let definition_id = CardDefinitionId(10_300);
    let first_rules = CardRules::new_sorcery(ManaCost::new(6, 1))
        .with_abilities(&FIRST_DUPLICATE_MIRACLE_ABILITIES);
    let second_rules = CardRules::new_sorcery(ManaCost::new(5, 1))
        .with_abilities(&SECOND_DUPLICATE_MIRACLE_ABILITIES);
    let mut definition = CardDefinition::new(
        definition_id,
        "Two Miracles",
        CardSet::AvacynRestored,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = first_rules;
    definition.parts = vec![
        CardPart::new(CardPartId::PRIMARY, "First Miracle", first_rules),
        CardPart::new(CardPartId(1), "Second Miracle", second_rules),
    ];
    definition.structure = CardStructure::Split {
        parts: vec![CardPartId::PRIMARY, CardPartId(1)],
        fused: None,
    };
    definition.play_options = vec![
        PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "First Miracle",
            SpellForm::Part(CardPartId::PRIMARY),
            ManaCost::new(6, 1),
            CardEffectStatus::Implemented,
        )
        .with_alternative_cast_costs(&first_rules),
        PlayOptionDef::cast(
            PlayOptionId(1),
            "Second Miracle",
            SpellForm::Part(CardPartId(1)),
            ManaCost::new(5, 1),
            CardEffectStatus::Implemented,
        )
        .with_alternative_cast_costs(&second_rules),
    ];

    let mut game = miracle_game(definition_id, cards::MOUNTAIN, 5);
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the split Miracle fixture is valid");

    let drawn = game
        .draw_card(PlayerId::One)
        .expect("the split card is drawn");
    answer_draw_action_window(&mut game, true);
    resolve_miracle_trigger(&mut game);

    assert!(matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::MayCastAlternative {
            ability: AbilityOrigin::Printed {
                definition,
                part: CardPartId::PRIMARY,
                ability: AbilityId(1),
            },
            ..
        } if definition == definition_id
    ));
    let casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == drawn))
        .collect::<Vec<_>>();
    assert_eq!(casts.len(), 1, "only the linked Miracle clause is offered");
    assert!(matches!(
        &casts[0],
        Action::CastSpell { choices, .. }
            if choices.play_option() == PlayOptionId::DEFAULT
                && choices.costs().alternative() == Some(AlternativeCostId(1))
    ));

    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 10_301)
        .expect("the exact linked Miracle offer reconstructs");
    let mut wrong_linked_clause = wire;
    wrong_linked_clause["checkpoint"]["decisionState"]["continuation"]["ability"]["partId"] =
        serde_json::json!(1);
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wrong_linked_clause,
        &hidden,
        10_302,
    )
    .expect_err("a second valid Miracle clause cannot replace the linked one");
    assert!(
        error.contains("is not the card's linked Miracle clause"),
        "unexpected error: {error}"
    );
}

#[test]
fn declining_the_standing_offer_ends_only_the_miracle_permission() {
    let mut game = miracle_game(cards::TERMINUS, cards::PLAINS, 6);
    let drawn = game.draw_card(PlayerId::One).expect("Terminus is drawn");
    answer_draw_action_window(&mut game, true);
    resolve_miracle_trigger(&mut game);
    let decline = game.pending_decisions[0].observation.clone();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decline.id,
            options: vec![0],
        },
    )
    .expect("declining is legal");

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .all(|action| !matches!(action, Action::CastSpell { card, .. } if *card == drawn)),
        "the card cannot be cast in the draw step after declining"
    );
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let ordinary = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == drawn))
        .expect("its affordable printed cost remains available at ordinary timing");
    assert_eq!(selected_alternative(&game, &ordinary), None);
}

#[test]
fn bonfire_cast_for_its_miracle_cost_still_chooses_x() {
    let mut game = miracle_game(cards::BONFIRE_OF_THE_DAMNED, cards::MOUNTAIN, 2);
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.draw_card(PlayerId::One);
    answer_draw_action_window(&mut game, true);
    resolve_miracle_trigger(&mut game);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { choices, .. }
                if choices.x() == 1
                    && choices.iter_targets().any(|target| *target == Target::Player(PlayerId::Two)))
        })
        .expect("the Miracle cost is payable with X of one");
    assert_eq!(
        selected_alternative(&game, &cast),
        Some(AlternativeCastKindDef::Miracle)
    );
    game.apply(PlayerId::One, cast).unwrap();
    drain_pending(&mut game);

    assert_eq!(game.players[1].life, 19);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS)
    );
}
