use super::*;

fn faebloom_game() -> (Game, GameObjectId) {
    let mut game = ready_game();
    let spell = card(10_000, cards::FAEBLOOM_TRICK, PlayerId::One);
    let id = spell.id;
    game.players[0].hand.push(spell);
    for id in 10_001..10_004 {
        game.battlefield
            .push(creature(id, cards::ISLAND, PlayerId::One));
    }
    (game, id)
}

fn assert_faeries(game: &Game, count: usize) {
    let tokens = game
        .battlefield
        .iter()
        .filter(|p| p.card.definition.is_token())
        .collect::<Vec<_>>();
    assert_eq!(tokens.len(), count);
    for token in tokens {
        assert_eq!(token.controller, PlayerId::One);
        assert_eq!(
            (game.power(token), game.toughness(token)),
            (Some(1), Some(1))
        );
        assert!(!token.tapped);
        let rules = Game::effective_rules_source(token)
            .token_characteristics()
            .unwrap()
            .rules();
        assert_eq!(rules.subtypes(), &["Faerie"]);
        assert_eq!(
            game.permanent_colors(token),
            [false, true, false, false, false]
        );
        assert!(game.permanent_has_executable_keyword(token, KeywordAbility::Flying));
    }
}

fn choose_target(game: &mut Game, target: GameObjectId) {
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("fresh target choice");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(id, _)| id == target))
        .expect("opposing creature is offered")
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

fn rebuild(game: &Game) -> Game {
    let (wire, hidden) = checkpoint_fixture(game, PlayerId::One);
    Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
        .expect("reflexive trigger checkpoint reconstructs")
}

#[test]
fn faebloom_casts_as_an_instant_without_any_target() {
    let (mut game, spell) = faebloom_game();
    game.active_player = PlayerId::Two;
    let cast = cast_action(spell, Vec::new(), Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    assert!(game.stack[0].ability.as_ref().unwrap().targets.is_empty());
    pass_priority_pair(&mut game);
    assert_faeries(&game, 2);
    assert!(game.stack.is_empty());
    assert!(game.pending_decisions.is_empty());
    assert!(game.installed_triggers.is_empty());
}

#[test]
fn faebloom_chooses_after_creation_and_preserves_both_checkpoint_windows() {
    let (mut game, spell) = faebloom_game();
    game.battlefield
        .push(creature(10_010, cards::SAVANNAH_LIONS, PlayerId::Two));
    game.apply(PlayerId::One, cast_action(spell, Vec::new(), Vec::new(), 0))
        .unwrap();
    // This creature did not exist when the spell was cast.
    game.battlefield
        .push(creature(10_011, cards::GRIZZLY_BEARS, PlayerId::Two));
    pass_priority_pair(&mut game);
    assert_faeries(&game, 2);
    assert!(
        game.stack.is_empty(),
        "the spell finishes before targets are chosen"
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::FAEBLOOM_TRICK)
    );

    let target = GameObjectId(10_011);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(
        decision.options.len(),
        2,
        "only opposing creatures are targets"
    );
    let mut restored_choice = rebuild(&game);
    for game in [&mut game, &mut restored_choice] {
        choose_target(game, target);
        assert_eq!(game.stack.len(), 1);
        assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
        assert!(
            !game
                .battlefield
                .iter()
                .find(|p| p.card.id == target)
                .unwrap()
                .tapped
        );
    }
    let mut restored_stack = rebuild(&game);
    for game in [&mut game, &mut restored_choice, &mut restored_stack] {
        pass_priority_pair(game);
        assert!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == target)
                .unwrap()
                .tapped
        );
        assert!(
            !game
                .battlefield
                .iter()
                .find(|p| p.card.id == GameObjectId(10_010))
                .unwrap()
                .tapped
        );
        assert_faeries(game, 2);
    }
}

#[test]
fn faebloom_trigger_can_be_answered_without_undoing_tokens() {
    let (mut game, spell) = faebloom_game();
    let target = GameObjectId(10_010);
    game.battlefield
        .push(creature(target.0, cards::GRIZZLY_BEARS, PlayerId::Two));
    game.battlefield
        .push(creature(10_011, cards::ISLAND, PlayerId::Two));
    game.players[1]
        .hand
        .push(card(10_012, cards::UNSUMMON, PlayerId::Two));
    game.apply(PlayerId::One, cast_action(spell, Vec::new(), Vec::new(), 0))
        .unwrap();
    pass_priority_pair(&mut game);
    choose_target(&mut game, target);
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    game.apply(
        PlayerId::Two,
        cast_action(
            GameObjectId(10_012),
            vec![Target::Permanent(target)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);
    assert_faeries(&game, 2);
    assert!(game.stack.is_empty());
    assert!(!game.battlefield.iter().any(|p| p.card.id == target));
}

#[test]
fn faebloom_token_doubling_creates_one_reflexive_trigger() {
    let (mut game, spell) = faebloom_game();
    game.battlefield.push(creature(
        10_010,
        cards::MONDRAK_GLORY_DOMINUS_346,
        PlayerId::One,
    ));
    game.battlefield
        .push(creature(10_011, cards::GRIZZLY_BEARS, PlayerId::Two));
    game.apply(PlayerId::One, cast_action(spell, Vec::new(), Vec::new(), 0))
        .unwrap();
    pass_priority_pair(&mut game);
    assert_faeries(&game, 4);
    choose_target(&mut game, GameObjectId(10_011));
    assert_eq!(game.stack.len(), 1);
    game.counter_spell(game.stack[0].id);
    assert_faeries(&game, 4);
    assert!(game.installed_triggers.is_empty());
}

#[test]
fn reflexive_trigger_retains_results_but_resets_targets_and_finishes_the_creator() {
    static TRIGGER: AbilityDef = AbilityDef::triggered_with_targets(
        "When you do, target player loses that much life.",
        TriggerEventDef::Reflexive,
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Sum(&crate::card::SumValueDef {
                left: ValueDef::BoundObjectCount(crate::Binding!("created")),
                right: ValueDef::ChosenX,
            }),
        },
    );
    const PROGRAM: EffectDef = EffectDef::Sequence(&[
        EffectDef::ReflexiveTrigger(&TRIGGER),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(3),
        },
    ]);
    let (mut game, spell) = faebloom_game();
    game.apply(PlayerId::One, cast_action(spell, Vec::new(), Vec::new(), 0))
        .unwrap();
    let mut creator = game.stack.pop().unwrap();
    creator.ability.as_mut().unwrap().x = 4;
    creator.ability.as_mut().unwrap().targets = vec![TargetSelection::new(
        TargetSlotId(0),
        vec![Target::Permanent(GameObjectId(10_001))],
    )];
    let mut context = EffectResolutionContext::empty();
    context.bind_object_group(
        crate::Binding!("created"),
        vec![
            Target::Permanent(GameObjectId(10_001)),
            Target::Permanent(GameObjectId(10_002)),
        ],
    );
    game.resolve_effect_def(ScopedEffect::at(PROGRAM, 1), &creator, context);
    assert_eq!(
        game.players[0].life, 23,
        "the creator's remaining effect resolves first"
    );
    assert_eq!(game.pending_triggers.len(), 1);
    assert!(game.pending_triggers[0].targets.is_empty());
    game.finish_rules_procedure();
    let label = game.target_label(PlayerId::One, Target::Player(PlayerId::Two));
    choose_decision_by_label(&mut game, PlayerId::One, &label);
    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[1].life, 14,
        "bound results and X survive the independent trigger"
    );
}

#[test]
fn faebloom_reference_and_prepared_execution_agree() {
    let run = |prepared| {
        let (mut game, spell) = faebloom_game();
        game.set_prepared_engine_enabled(prepared);
        game.battlefield
            .push(creature(10_010, cards::GRIZZLY_BEARS, PlayerId::Two));
        game.apply(PlayerId::One, cast_action(spell, Vec::new(), Vec::new(), 0))
            .unwrap();
        pass_priority_pair(&mut game);
        choose_target(&mut game, GameObjectId(10_010));
        pass_priority_pair(&mut game);
        game
    };
    let reference = run(false);
    let prepared = run(true);
    assert_eq!(reference.players, prepared.players);
    assert_eq!(reference.battlefield, prepared.battlefield);
    assert!(reference.stack.is_empty());
    assert!(prepared.stack.is_empty());
    assert_eq!(reference.events, prepared.events);
    assert_eq!(reference.pending_triggers, prepared.pending_triggers);
    assert!(reference.pending_decisions.is_empty());
    assert!(prepared.pending_decisions.is_empty());
}

#[test]
fn reflexive_trigger_is_not_created_for_an_empty_token_result() {
    let (mut game, spell) = faebloom_game();
    game.battlefield
        .push(creature(10_010, cards::GRIZZLY_BEARS, PlayerId::Two));
    game.apply(PlayerId::One, cast_action(spell, Vec::new(), Vec::new(), 0))
        .unwrap();
    let creator = game.stack.pop().unwrap();
    let EffectDef::CreateToken(mut creation) = creator
        .ability
        .as_ref()
        .unwrap()
        .resolver
        .declarative_reference()
        .unwrap()
        .effect
    else {
        panic!("Faebloom's spell creates tokens")
    };
    creation.count = ValueDef::Constant(0);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::CreateToken(creation)),
        &creator,
        EffectResolutionContext::empty(),
    );
    game.finish_rules_procedure();
    assert!(game.pending_triggers.is_empty());
    assert!(game.pending_decisions.is_empty());
    assert!(game.stack.is_empty());
    assert!(game.installed_triggers.is_empty());
}

#[test]
fn faebloom_trigger_survives_tokens_dying_before_target_selection() {
    let (mut game, spell) = faebloom_game();
    let norn = GameObjectId(10_010);
    game.battlefield.push(creature(
        norn.0,
        cards::ELESH_NORN_GRAND_CENOBITE,
        PlayerId::Two,
    ));
    game.apply(PlayerId::One, cast_action(spell, Vec::new(), Vec::new(), 0))
        .unwrap();
    pass_priority_pair(&mut game);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|p| p.card.definition.is_token())
    );
    choose_target(&mut game, norn);
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == norn)
            .unwrap()
            .tapped
    );
}
