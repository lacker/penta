use super::*;

#[test]
fn stage_copies_dryad_arbors_copiable_values_but_not_hack_or_presence() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let stage_id = CardInstanceId(10_000);
    let arbor_id = CardInstanceId(10_001);
    let aura_id = CardInstanceId(10_002);
    let stage = creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One);
    let mut arbor = creature(arbor_id.0, cards::DRYAD_ARBOR, PlayerId::One);
    arbor.text_changes.push(BasicLandTypeChange {
        from: BasicLandType::Forest,
        to: BasicLandType::Island,
    });
    let mut presence = creature(aura_id.0, cards::NYLEAS_PRESENCE, PlayerId::One);
    presence.attached_to = Some(arbor_id);
    game.battlefield.extend([stage, arbor, presence]);
    assert_eq!(game.effective_land_types(&game.battlefield[1]), [true; 5]);

    let copy_ability = activated_ability_for(&game, stage_id, 0);
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage_id,
            ability: copy_ability,
            targets: activated_targets(Target::Permanent(arbor_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let stage = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == stage_id)
        .unwrap();
    let types = game.permanent_types(stage).unwrap();
    assert!(types.contains(CardType::Land));
    assert!(types.contains(CardType::Creature));
    assert_eq!(
        game.effective_subtypes(stage).as_ref(),
        &["Forest", "Dryad"]
    );
    assert_eq!(
        (game.power(stage), game.toughness(stage)),
        (Some(1), Some(1))
    );
    assert_eq!(
        game.effective_rules(stage).unwrap().colors(),
        [false, false, false, false, true]
    );
    assert!(stage.tapped, "copying does not untap or reenter Stage");
    assert_eq!(activated_ability_for(&game, stage_id, 0), copy_ability);

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == stage_id)
        .unwrap()
        .tapped = false;
    assert_eq!(
        game.mana_ability_activations(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == stage_id)
                .unwrap(),
        )
        .into_iter()
        .map(|activation| (activation.ability, activation.color))
        .collect::<Vec<_>>(),
        vec![(
            AbilityOrigin::IntrinsicBasicLand(BasicLandType::Forest),
            ManaColor::Green,
        )],
    );

    game.destroy_permanent(aura_id);
    let arbor = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == arbor_id)
        .unwrap();
    assert_eq!(
        game.effective_subtypes(arbor).as_ref(),
        &["Island", "Dryad"],
        "removing Presence reveals the earlier text change",
    );
}

#[test]
fn a_new_stage_can_copy_dryad_arbor_but_the_result_is_summoning_sick() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 0;
    let stage_id = CardInstanceId(10_000);
    let arbor_id = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(arbor_id.0, cards::DRYAD_ARBOR, PlayerId::Two),
    ]);
    let copy_ability = activated_ability_for(&game, stage_id, 0);
    game.players[0].mana_pool.colorless = 2;

    let copy = Action::ActivateAbility {
        source: stage_id,
        ability: copy_ability,
        targets: activated_targets(Target::Permanent(arbor_id)),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };
    assert!(
        game.legal_actions(PlayerId::One).contains(&copy),
        "Stage is not a creature while it pays the tap cost",
    );
    game.apply(PlayerId::One, copy).unwrap();
    pass_priority_pair(&mut game);
    game.battlefield[0].tapped = false;

    assert!(
        game.mana_ability_activations(&game.battlefield[0])
            .is_empty(),
        "the copied creature cannot use a tap ability in its controller's first turn",
    );
    game.turns_started[PlayerId::One.index()] = 1;
    assert_eq!(
        game.mana_ability_activations(&game.battlefield[0])
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Green],
    );
}

#[test]
fn stage_copying_stage_does_not_duplicate_indistinguishable_legal_actions() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let copying_stage = CardInstanceId(10_000);
    let copied_stage = CardInstanceId(10_001);
    let mountain = CardInstanceId(10_002);
    game.battlefield.extend([
        creature(copying_stage.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(copied_stage.0, cards::THESPIANS_STAGE, PlayerId::Two),
        creature(mountain.0, cards::MOUNTAIN, PlayerId::Two),
    ]);
    let copy_ability = activated_ability_for(&game, copying_stage, 0);
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: copying_stage,
            ability: copy_ability,
            targets: activated_targets(Target::Permanent(copied_stage)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    game.battlefield[0].tapped = false;
    game.players[0].mana_pool.colorless = 2;

    assert_eq!(
        game.effective_abilities(&game.battlefield[0])
            .iter()
            .filter(|effective| {
                effective.origin == copy_ability
                    && matches!(
                        effective.ability.definition,
                        DeclarativeAbilityDef::Activated(_)
                    )
            })
            .count(),
        2,
        "both copiable Stage abilities remain part of the permanent",
    );

    let copy_mountain = Action::ActivateAbility {
        source: copying_stage,
        ability: copy_ability,
        targets: activated_targets(Target::Permanent(mountain)),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };
    assert_eq!(
        game.legal_actions(PlayerId::One)
            .iter()
            .filter(|action| **action == copy_mountain)
            .count(),
        1,
        "the two rules-identical Stage abilities produce one external action",
    );
}

#[test]
fn stage_keeps_a_resolved_factory_animation_after_copying_another_land() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let stage_id = CardInstanceId(10_000);
    let factory_id = CardInstanceId(10_001);
    let mountain_id = CardInstanceId(10_002);
    game.battlefield.extend([
        creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(factory_id.0, cards::MISHRA_S_FACTORY, PlayerId::One),
        creature(mountain_id.0, cards::MOUNTAIN, PlayerId::Two),
    ]);

    let original_copy_ability = activated_ability_for(&game, stage_id, 0);
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage_id,
            ability: original_copy_ability,
            targets: activated_targets(Target::Permanent(factory_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    game.battlefield[0].tapped = false;

    let animate = Action::ActivateAbility {
        source: stage_id,
        ability: activated_ability_for(&game, stage_id, 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };
    game.players[0].mana_pool.colorless = 1;
    assert!(
        game.legal_actions(PlayerId::One).contains(&animate),
        "the copied Factory animation coexists with Stage's added copy ability",
    );
    game.apply(PlayerId::One, animate).unwrap();
    drain_pending(&mut game);
    let animated_stage = game.battlefield[0].clone();
    let types = game.permanent_types(&animated_stage).expect("types");
    for card_type in [CardType::Land, CardType::Artifact, CardType::Creature] {
        assert!(types.contains(card_type));
    }
    assert_eq!(game.power(&animated_stage), Some(2));
    assert_eq!(game.toughness(&animated_stage), Some(2));
    assert!(
        !animated_stage.resolved_continuous_effects.is_empty(),
        "the animation is represented by resolved characteristic leaves"
    );

    let added_copy_ability = activated_ability_for(&game, stage_id, 2);
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage_id,
            ability: added_copy_ability,
            targets: activated_targets(Target::Permanent(mountain_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let stage = &game.battlefield[0];
    let types = game.permanent_types(stage).unwrap();
    assert!(types.contains(CardType::Land));
    assert!(types.contains(CardType::Artifact));
    assert!(types.contains(CardType::Creature));
    assert_eq!(
        (game.power(stage), game.toughness(stage)),
        (Some(2), Some(2))
    );

    let pump = Action::ActivateAbility {
        source: factory_id,
        ability: activated_ability_for(&game, factory_id, 1),
        targets: activated_targets(Target::Permanent(stage_id)),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };
    assert!(
        game.legal_actions(PlayerId::One).contains(&pump),
        "the still-animated object remains an Assembly-Worker pump target",
    );
}

/// Casts a Copy Artifact already in hand and answers the entry choice with
/// the named permanent. The copy is chosen as the enchantment enters, so
/// there is no target to pick at cast time.
pub(super) fn resolve_copy_artifact(game: &mut Game, copy: GameObjectId, copied: GameObjectId) {
    game.apply(PlayerId::One, cast_action(copy, Vec::new(), Vec::new(), 0))
        .unwrap();
    pass_priority_pair(game);
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("entering asks what to copy");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(id, _)| id == copied))
        .expect("the permanent is on the menu")
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
fn stage_does_not_copy_a_land_that_leaves_before_the_ability_resolves() {
    let mut game = ready_game();
    let stage_id = CardInstanceId(10_000);
    let target_id = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(target_id.0, cards::DRYAD_ARBOR, PlayerId::Two),
    ]);
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage_id,
            ability: activated_ability_for(&game, stage_id, 0),
            targets: activated_targets(Target::Permanent(target_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    game.destroy_permanent(target_id);
    pass_priority_pair(&mut game);
    game.battlefield[0].tapped = false;

    let stage = &game.battlefield[0];
    assert!(stage.copy_effect.is_none());
    assert_eq!(
        game.mana_ability_activations(stage)
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Colorless],
    );
}

#[test]
fn copy_artifact_copies_an_artifact_creature() {
    let mut game = ready_game();
    let source = creature(10_000, cards::TETRAVUS, PlayerId::Two);
    game.battlefield.push(source);
    let copy = card(10_001, cards::COPY_ARTIFACT, PlayerId::One);
    game.players[0].hand.push(copy.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    assert!(
        game.legal_actions(PlayerId::One).contains(&cast_action(
            copy.id,
            Vec::new(),
            Vec::new(),
            0
        )),
        "it is cast without naming what it copies"
    );
    resolve_copy_artifact(&mut game, copy.id, CardInstanceId(10_000));
    let copied = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::COPY_ARTIFACT)
        .unwrap();
    assert_eq!(
        copied.copy_effect.as_ref().map(|copy| copy.base),
        Some(ObjectCharacteristics::card(
            cards::TETRAVUS,
            CardPartId::PRIMARY,
        ))
    );
    assert_eq!(copied.presented, CardPartId::PRIMARY);
    assert_eq!(
        game.effective_rules(copied),
        game.catalog
            .get(cards::TETRAVUS)
            .map(|definition| definition.rules),
    );
    let copied_types = game.permanent_types(copied).unwrap();
    assert!(copied_types.contains(CardType::Artifact));
    assert!(copied_types.contains(CardType::Creature));
    assert!(
        copied_types.contains(CardType::Enchantment),
        "Copy Artifact retains its copy-process type exception",
    );
    assert_eq!(game.power(copied), Some(4));
    assert!(game.has_flying(copied));
}

#[test]
fn token_nature_is_independent_from_the_characteristics_being_copied() {
    let mut game = ready_game();
    game.battlefield.clear();

    game.create_token_copy(
        PlayerId::One,
        copied_characteristics(cards::SERRA_ANGEL),
        None,
        CardPartId::PRIMARY,
    );
    drain_pending(&mut game);
    let card_copy_token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition.is_token())
        .expect("the card-characteristics copy arrived as a token");
    assert_eq!(
        Game::effective_rules_source(card_copy_token),
        ObjectCharacteristics::card(cards::SERRA_ANGEL, CardPartId::PRIMARY),
    );
    assert_eq!(
        (game.power(card_copy_token), game.toughness(card_copy_token)),
        (Some(4), Some(4))
    );
    let token_id = card_copy_token.card.id;
    game.return_permanent_to_hand(token_id);
    assert!(
        game.players[PlayerId::One.index()].hand.is_empty(),
        "a token copying a card still ceases instead of becoming a card in hand",
    );

    let food = token_permanent(10_100, tokens::food(), PlayerId::Two);
    let food_id = food.card.id;
    game.battlefield.push(food);
    let copy = card(10_101, cards::COPY_ARTIFACT, PlayerId::One);
    game.players[PlayerId::One.index()].hand.push(copy.clone());
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    resolve_copy_artifact(&mut game, copy.id, food_id);

    let token_characteristics_copy = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::COPY_ARTIFACT)
        .expect("Copy Artifact resolved as its printed card object");
    assert!(!token_characteristics_copy.card.definition.is_token());
    assert_eq!(
        token_characteristics_copy
            .copy_effect
            .as_ref()
            .map(|copy| copy.base),
        Some(ObjectCharacteristics::token(
            tokens::food(),
            CardPartId::PRIMARY,
        )),
        "a card can copy inline token characteristics without becoming a token",
    );
}

#[test]
fn copy_artifact_resolves_a_copied_icy_manipulator_ability_from_its_frozen_origin() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::ICY_MANIPULATOR, PlayerId::Two),
        creature(10_001, cards::MOUNTAIN, PlayerId::Two),
    ]);
    let copy = card(10_002, cards::COPY_ARTIFACT, PlayerId::One);
    game.players[0].hand.push(copy.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    resolve_copy_artifact(&mut game, copy.id, CardInstanceId(10_000));

    let copied_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::COPY_ARTIFACT)
        .expect("Copy Artifact resolved")
        .card
        .id;
    let target_id = CardInstanceId(10_001);
    let ability = activated_ability_for(&game, copied_id, 0);
    assert_eq!(ability, primary_ability(cards::ICY_MANIPULATOR));

    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: copied_id,
            ability,
            targets: activated_targets(Target::Permanent(target_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].card.definition, ObjectKind::Ability);
    assert_eq!(
        game.stack[0].ability_origin(),
        Some(primary_ability(cards::ICY_MANIPULATOR))
    );
    assert_eq!(
        game.observe(PlayerId::One).stack[0].characteristics,
        ObjectCharacteristics::card(cards::ICY_MANIPULATOR, CardPartId::PRIMARY),
        "stack presentation follows the frozen copied ability definition",
    );

    game.destroy_permanent(copied_id);
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == target_id)
            .is_some_and(|permanent| permanent.tapped),
        "the copied Icy ability resolves after its physical source leaves play",
    );
}

include!("copy_effects/granted_origins.rs");
