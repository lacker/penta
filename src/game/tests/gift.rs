use super::*;

fn gift_game(definition: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    let spell = game
        .build_zone(PlayerId::One, &[definition])
        .unwrap()
        .remove(0);
    let id = spell.id;
    game.players[0].hand.push(spell);
    for color in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
        ManaColor::Colorless,
    ] {
        game.add_unrestricted_mana(PlayerId::One, color, 10);
    }
    (game, id)
}

fn cast_gift(game: &mut Game, card_id: GameObjectId, promised: bool) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == card_id && !choices.costs().additional().is_empty() == promised)
        })
        .expect("gift choice is a legal cast");
    game.apply(PlayerId::One, action).unwrap();
}

#[test]
fn gift_dawns_truce_promises_before_resolution_and_protects_permanents() {
    for promised in [false, true] {
        let (mut game, spell) = gift_game(cards::DAWN_S_TRUCE);
        game.battlefield
            .push(creature(180_001, cards::GRIZZLY_BEARS, PlayerId::One));
        let hand = game.players[1].hand.len();
        cast_gift(&mut game, spell, promised);
        assert_eq!(game.players[1].hand.len(), hand);
        assert_eq!(
            game.stack[0].cast.as_ref().unwrap().gift_recipient,
            promised.then_some(PlayerId::Two)
        );
        game.resolve_stack_top();
        assert_eq!(game.players[1].hand.len(), hand + usize::from(promised));
        let bear = &game.battlefield[0];
        assert!(game.permanent_has_executable_keyword(bear, KeywordAbility::Hexproof));
        assert_eq!(
            game.permanent_has_executable_keyword(bear, KeywordAbility::Indestructible),
            promised
        );
    }
}

fn rebuild(game: &Game) -> Game {
    let (wire, hidden) = checkpoint_fixture(game, PlayerId::One);
    Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 900_000)
        .expect("gift state reconstructs")
}

#[test]
fn gift_spell_copy_keeps_the_chosen_opponent_after_controller_changes_and_checkpoint() {
    let (mut game, spell) = gift_game(cards::DAWN_S_TRUCE);
    cast_gift(&mut game, spell, true);
    game.push_copy(game.stack[0].clone(), PlayerId::Two, Vec::new());
    let mut game = rebuild(&game);
    assert_eq!(
        game.stack
            .last()
            .unwrap()
            .cast
            .as_ref()
            .unwrap()
            .gift_recipient,
        Some(PlayerId::Two)
    );
    let hands = [game.players[0].hand.len(), game.players[1].hand.len()];
    game.resolve_stack_top();
    assert_eq!(game.players[0].hand.len(), hands[0]);
    assert_eq!(game.players[1].hand.len(), hands[1] + 1);
    game.resolve_stack_top();
    assert_eq!(game.players[1].hand.len(), hands[1] + 2);
}

#[test]
fn gift_is_not_given_when_countered_or_all_targets_are_illegal() {
    for countered in [false, true] {
        let (mut game, spell) = gift_game(cards::BLOOMING_BLAST);
        game.battlefield
            .push(creature(180_101, cards::GRIZZLY_BEARS, PlayerId::Two));
        game.battlefield
            .push(creature(180_102, cards::JOLLY_GERBILS, PlayerId::One));
        cast_gift(&mut game, spell, true);
        if countered {
            game.counter_spell(game.stack[0].id);
        } else {
            // Remove the sole announced target before resolution.
            let victim = game.stack[0].first_target().unwrap();
            if let Target::Permanent(id) = victim {
                game.battlefield.retain(|permanent| permanent.card.id != id);
            }
            game.resolve_stack_top();
        }
        assert!(
            !game
                .battlefield
                .iter()
                .any(|permanent| permanent.card.definition.is_token())
        );
        assert!(game.pending_triggers.is_empty());
    }
}

#[test]
fn gift_modifies_required_target_counts_and_restrictions_before_payment() {
    let (mut game, spell) = gift_game(cards::WEAR_DOWN);
    for id in [180_201, 180_202] {
        game.battlefield
            .push(creature(id, cards::SOL_RING, PlayerId::Two));
    }
    for action in game.legal_actions(PlayerId::One) {
        if let Action::CastSpell { card, choices, .. } = action {
            if card == spell {
                assert_eq!(
                    choices.iter_targets().count(),
                    if choices.costs().additional().is_empty() {
                        1
                    } else {
                        2
                    }
                );
            }
        }
    }
    let (mut game, spell) = gift_game(cards::INTO_THE_FLOOD_MAW);
    game.battlefield
        .push(creature(180_203, cards::SOL_RING, PlayerId::Two));
    let casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .collect::<Vec<_>>();
    assert!(!casts.is_empty());
    assert!(casts.iter().all(|action| matches!(action, Action::CastSpell { choices, .. } if choices.costs().additional().len() == 1)));
    cast_gift(&mut game, spell, true);
    game.resolve_stack_top();
    let fish = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition.is_token())
        .unwrap();
    assert_eq!(fish.controller, PlayerId::Two);
    assert!(fish.tapped);
}

#[test]
fn gift_additional_targets_are_absent_without_a_promise() {
    let (mut game, spell) = gift_game(cards::VALLEY_RALLY);
    // An empty battlefield permits the ordinary cast but no promised cast.
    let casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .collect::<Vec<_>>();
    assert_eq!(casts.len(), 1);
    assert!(
        matches!(&casts[0], Action::CastSpell { choices, .. } if choices.iter_targets().count() == 0 && choices.costs().additional().is_empty())
    );
    game.battlefield
        .push(creature(180_301, cards::GRIZZLY_BEARS, PlayerId::One));
    assert!(game.legal_actions(PlayerId::One).iter().any(|action| matches!(action, Action::CastSpell { card, choices, .. } if *card == spell && !choices.costs().additional().is_empty() && choices.iter_targets().count() == 1)));
}

#[test]
fn gift_permanent_uses_an_enters_trigger_and_uncast_arrivals_have_no_promise() {
    let (mut game, spell) = gift_game(cards::SCRAPSHOOTER);
    let hand = game.players[1].hand.len();
    // Promising is legal even without a target for Scrapshooter's other trigger.
    cast_gift(&mut game, spell, true);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].hand.len(), hand);
    let order = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: order.id,
            options: order.options.iter().map(|option| option.id).collect(),
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 1, "the gift trigger uses the stack");
    let mut game = rebuild(&game);
    assert_eq!(
        game.battlefield[0].cast.as_ref().unwrap().gift_recipient,
        Some(PlayerId::Two)
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].hand.len(), hand + 1);
    game.put_onto_battlefield(PlayerId::One, cards::SCRAPSHOOTER)
        .unwrap();
    assert!(
        game.pending_triggers.is_empty(),
        "an uncast arrival makes no promise"
    );
}

#[test]
fn gift_completion_triggers_jolly_gerbils_once_per_gift() {
    let (mut game, spell) = gift_game(cards::DAWN_S_TRUCE);
    game.battlefield
        .push(creature(180_401, cards::JOLLY_GERBILS, PlayerId::One));
    cast_gift(&mut game, spell, true);
    let hand = game.players[0].hand.len();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), hand);
    assert_eq!(
        game.stack.len(),
        1,
        "one give-gift event queues one Gerbils trigger"
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), hand + 1);
}

#[test]
fn gift_draw_replacement_resumes_before_other_effects_and_still_gives_one_gift() {
    let (mut game, spell) = gift_game(cards::DAWN_S_TRUCE);
    game.battlefield
        .push(creature(180_501, cards::JOLLY_GERBILS, PlayerId::One));
    game.battlefield
        .push(creature(180_502, cards::ISLAND_SANCTUARY, PlayerId::Two));
    game.active_player = PlayerId::Two;
    game.step = Step::Draw;
    cast_gift(&mut game, spell, true);
    let hands = [game.players[0].hand.len(), game.players[1].hand.len()];
    pass_priority_pair(&mut game);
    assert!(game.observe(PlayerId::Two).decision.is_some());
    assert!(!game.permanent_has_executable_keyword(&game.battlefield[0], KeywordAbility::Hexproof));
    assert!(
        game.pending_triggers.is_empty(),
        "the gift has not finished resolving"
    );
    let mut game = rebuild(&game);
    let decision = game.observe(PlayerId::Two).decision.unwrap();
    let skip = decision
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
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![skip],
        },
    )
    .unwrap();
    assert!(game.permanent_has_executable_keyword(&game.battlefield[0], KeywordAbility::Hexproof));
    drain_pending(&mut game);
    assert_eq!(
        game.players[1].hand.len(),
        hands[1],
        "the promised draw was replaced"
    );
    assert_eq!(
        game.players[0].hand.len(),
        hands[0] + 1,
        "Gerbils still sees exactly one gift"
    );
}

#[test]
fn gift_and_mandatory_discard_are_separate_cast_choices() {
    let (mut game, spell) = gift_game(cards::SAZACAP_S_BREW);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
    );
    game.players[0]
        .hand
        .push(card(180_601, cards::MOUNTAIN, PlayerId::One));
    game.battlefield
        .push(creature(180_602, cards::GRIZZLY_BEARS, PlayerId::One));
    cast_gift(&mut game, spell, true);
    assert_eq!(game.players[0].graveyard.len(), 1);
    assert_eq!(game.players[0].graveyard[0].definition, cards::MOUNTAIN);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition.is_token()),
        "promising did not create the Fish while paying"
    );
    drain_pending(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition.is_token()
                && permanent.controller == PlayerId::Two
                && permanent.tapped)
    );
}

#[test]
fn gift_starfall_returns_only_a_creature_actually_destroyed_into_your_graveyard() {
    let (mut game, spell) = gift_game(cards::STARFALL_INVOCATION);
    game.battlefield
        .push(creature(180_701, cards::SAVANNAH_LIONS, PlayerId::One));
    game.battlefield
        .push(creature(180_702, cards::GRIZZLY_BEARS, PlayerId::Two));
    game.battlefield
        .push(creature(180_704, cards::SERRA_ANGEL, PlayerId::One));
    game.players[0]
        .graveyard
        .push(card(180_703, cards::SHIVAN_DRAGON, PlayerId::One));
    cast_gift(&mut game, spell, true);
    pass_priority_pair(&mut game);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(
        decision.options.len(),
        2,
        "only your creatures destroyed this way are eligible"
    );
    assert!(decision.options.iter().all(|option| {
        option.label.contains("Savannah Lions") || option.label.contains("Serra Angel")
    }));
    let mut game = rebuild(&game);
    drain_pending(&mut game);
    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(game.battlefield[0].card.definition, cards::SAVANNAH_LIONS);
    assert_eq!(game.battlefield[0].controller, PlayerId::One);
}

#[test]
fn gift_coiling_rebirth_copies_the_returned_creature_as_one_one() {
    let (mut game, spell) = gift_game(cards::COILING_REBIRTH);
    game.players[0]
        .graveyard
        .push(card(180_801, cards::SAVANNAH_LIONS, PlayerId::One));
    cast_gift(&mut game, spell, true);
    drain_pending(&mut game);
    assert_eq!(game.battlefield.len(), 2);
    let token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition.is_token())
        .unwrap();
    assert_eq!(game.power(token), Some(1));
    assert_eq!(game.toughness(token), Some(1));
}

#[test]
fn gift_enters_trigger_retains_recipient_after_its_source_leaves() {
    let (mut game, spell) = gift_game(cards::SCRAPSHOOTER);
    cast_gift(&mut game, spell, true);
    pass_priority_pair(&mut game);
    let source = game.battlefield[0].card.id;
    game.destroy_permanent(source);
    let mut game = rebuild(&game);
    let hand = game.players[1].hand.len();
    drain_pending(&mut game);
    assert_eq!(game.players[1].hand.len(), hand + 1);
}

#[test]
fn gift_kitnap_controls_and_taps_the_creature_with_stun_only_without_a_promise() {
    for promised in [false, true] {
        let (mut game, spell) = gift_game(cards::KITNAP);
        game.battlefield
            .push(creature(180_901, cards::GRIZZLY_BEARS, PlayerId::Two));
        let hand = game.players[1].hand.len();
        cast_gift(&mut game, spell, promised);
        drain_pending(&mut game);
        let bear = game
            .battlefield
            .iter()
            .find(|p| p.card.id == GameObjectId(180_901))
            .unwrap();
        assert_eq!(bear.controller, PlayerId::One);
        assert!(bear.tapped);
        assert_eq!(
            bear.counters(CounterKind::Stun),
            if promised { 0 } else { 3 }
        );
        assert_eq!(game.players[1].hand.len(), hand + usize::from(promised));
    }
}

#[test]
fn gift_parting_gust_returns_only_without_a_promise_after_checkpoint() {
    for promised in [false, true] {
        let (mut game, spell) = gift_game(cards::PARTING_GUST);
        game.battlefield
            .push(creature(181_001, cards::GRIZZLY_BEARS, PlayerId::Two));
        cast_gift(&mut game, spell, promised);
        drain_pending(&mut game);
        assert!(
            !game
                .battlefield
                .iter()
                .any(|p| p.card.definition == cards::GRIZZLY_BEARS)
        );
        let mut game = rebuild(&game);
        game.step = Step::End;
        game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
            step: TurnStepDef::End,
            player: PlayerId::One,
        });
        drain_pending(&mut game);
        let bear = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::GRIZZLY_BEARS);
        assert_eq!(bear.is_some(), !promised);
        if let Some(bear) = bear {
            assert_eq!(bear.controller, PlayerId::Two);
            assert_eq!(bear.counters(CounterKind::PlusOnePlusOne), 1);
        }
    }
}

#[test]
fn gift_prepared_and_reference_resolution_have_identical_results() {
    for promised in [false, true] {
        let run = |prepared| {
            let (mut game, spell) = gift_game(cards::BLOOMING_BLAST);
            game.set_prepared_engine_enabled(prepared);
            game.battlefield
                .push(creature(181_101, cards::GRIZZLY_BEARS, PlayerId::Two));
            cast_gift(&mut game, spell, promised);
            drain_pending(&mut game);
            game
        };
        let prepared = run(true);
        let reference = run(false);
        assert_eq!(prepared.players, reference.players);
        assert_eq!(prepared.battlefield, reference.battlefield);
        assert_eq!(prepared.events, reference.events);
        assert_eq!(prepared.pending_events, reference.pending_events);
        assert_eq!(prepared.pending_procedures, reference.pending_procedures);
    }
}
