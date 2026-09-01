use super::*;

fn intrinsic_mana_colors(game: &Game, permanent: &Permanent) -> Vec<ManaColor> {
    let mut colors = game
        .effective_abilities(permanent)
        .into_iter()
        .filter_map(|effective| {
            let AbilityOrigin::IntrinsicBasicLand(land_type) = effective.origin else {
                return None;
            };
            Some(land_type.mana_color())
        })
        .collect::<Vec<_>>();
    colors.sort_unstable();
    colors
}

fn resolve_applied_effect_on_permanent(
    game: &mut Game,
    target: CardInstanceId,
    effect: AppliedEffectDef,
    duration: ResolvedEffectDurationDef,
    stack_id: u32,
) {
    let object = spell_with_targets(
        stack_id,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
        vec![Target::Permanent(target)],
        0,
    );
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect,
            duration,
        }),
        &object,
        TriggerContext::empty(),
    );
}

#[test]
fn factory_and_sorceress_queen_base_setters_follow_timestamp_without_losing_types() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.turns_started[PlayerId::One.index()] = 1;
    let factory = GameObjectId(10_000);
    let queen = GameObjectId(10_001);
    game.battlefield.extend([
        creature(factory.0, cards::MISHRA_S_FACTORY, PlayerId::One),
        creature(queen.0, cards::SORCERESS_QUEEN, PlayerId::One),
    ]);

    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: factory,
            ability: activated_ability_for(&game, factory, 0),
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    drain_pending(&mut game);

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: queen,
            ability: activated_ability_for(&game, queen, 0),
            targets: activated_targets(Target::Permanent(factory)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    drain_pending(&mut game);

    let affected = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == factory)
        .expect("Factory remains on the battlefield")
        .clone();
    let types = game
        .permanent_types(&affected)
        .expect("Factory has card types");
    for card_type in [CardType::Land, CardType::Artifact, CardType::Creature] {
        assert!(types.contains(card_type));
    }
    assert!(
        game.effective_subtypes(&affected)
            .contains(&"Assembly-Worker")
    );
    assert_eq!(
        (game.power(&affected), game.toughness(&affected)),
        (Some(0), Some(2))
    );

    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: factory,
            ability: activated_ability_for(&game, factory, 0),
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    drain_pending(&mut game);
    let affected = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == factory)
        .expect("Factory remains on the battlefield");
    assert_eq!(
        (game.power(affected), game.toughness(affected)),
        (Some(2), Some(2))
    );
}

#[test]
fn turn_preserves_land_subtypes_and_only_later_ability_grants() {
    static EARLIER_FLYING: AbilityDef = abilities::flying();
    static LATER_TRAMPLE: AbilityDef = abilities::trample();
    static FOREST_ANIMATION: [AppliedEffectDef; 2] = [
        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
        AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
    ];
    static TURN_CHARACTERISTICS: [AppliedEffectDef; 5] = [
        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Weird"])),
        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
        AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Red])),
        AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(0), ValueDef::Constant(1)),
    ];

    let mut game = ready_game();
    let forest = GameObjectId(10_000);
    game.battlefield
        .push(creature(forest.0, cards::FOREST, PlayerId::One));
    resolve_applied_effect_on_permanent(
        &mut game,
        forest,
        AppliedEffectDef::Composite(&FOREST_ANIMATION),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_000,
    );
    resolve_applied_effect_on_permanent(
        &mut game,
        forest,
        AppliedEffectDef::add_ability(&EARLIER_FLYING),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_001,
    );
    assert_eq!(
        intrinsic_mana_colors(&game, &game.battlefield[0]),
        vec![ManaColor::Green]
    );
    assert!(game.has_flying(&game.battlefield[0]));

    resolve_applied_effect_on_permanent(
        &mut game,
        forest,
        AppliedEffectDef::Composite(&TURN_CHARACTERISTICS),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_002,
    );
    assert!(intrinsic_mana_colors(&game, &game.battlefield[0]).is_empty());
    assert!(
        game.mana_ability_activations(&game.battlefield[0])
            .is_empty()
    );
    assert!(!game.has_flying(&game.battlefield[0]));

    resolve_applied_effect_on_permanent(
        &mut game,
        forest,
        AppliedEffectDef::add_ability(&LATER_TRAMPLE),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_003,
    );
    let affected = &game.battlefield[0];
    assert!(game.has_trample(affected));
    let subtypes = game.effective_subtypes(affected);
    assert!(subtypes.contains(&"Forest"));
    assert!(subtypes.contains(&"Weird"));
    assert_eq!(
        (game.power(affected), game.toughness(affected)),
        (Some(0), Some(1))
    );
}

#[test]
fn end_of_turn_base_setter_reveals_an_earlier_permanent_setter_at_cleanup() {
    let mut game = ready_game();
    let target = GameObjectId(10_000);
    game.battlefield
        .push(creature(target.0, cards::SAVANNAH_LIONS, PlayerId::One));
    resolve_applied_effect_on_permanent(
        &mut game,
        target,
        AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(5), ValueDef::Constant(5)),
        ResolvedEffectDurationDef::Permanent,
        20_000,
    );
    resolve_applied_effect_on_permanent(
        &mut game,
        target,
        AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_001,
    );
    assert_eq!(
        (
            game.power(&game.battlefield[0]),
            game.toughness(&game.battlefield[0])
        ),
        (Some(1), Some(1))
    );

    game.finish_cleanup();
    assert_eq!(
        (
            game.power(&game.battlefield[0]),
            game.toughness(&game.battlefield[0])
        ),
        (Some(5), Some(5)),
        "cleanup removes only the later EOT setter"
    );
    assert_eq!(game.battlefield[0].resolved_continuous_effects.len(), 1);
    assert_eq!(
        game.battlefield[0].resolved_continuous_effects[0].expiration,
        ContinuousEffectExpiration::Never
    );
}

#[test]
fn same_timestamp_composite_removes_then_adds_abilities_in_component_order() {
    static LATER_TRAMPLE: AbilityDef = abilities::trample();
    static REMOVE_THEN_ADD: [AppliedEffectDef; 2] = [
        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
        AppliedEffectDef::add_ability(&LATER_TRAMPLE),
    ];

    let mut game = ready_game();
    let target = GameObjectId(10_000);
    game.battlefield
        .push(creature(target.0, cards::SERRA_ANGEL, PlayerId::One));
    resolve_applied_effect_on_permanent(
        &mut game,
        target,
        AppliedEffectDef::Composite(&REMOVE_THEN_ADD),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_000,
    );

    let affected = &game.battlefield[0];
    assert!(!game.has_flying(affected));
    assert!(!game.permanent_has_executable_keyword(affected, KeywordAbility::Vigilance));
    assert!(game.has_trample(affected));
    assert_eq!(game.effective_abilities(affected).len(), 1);
    let operations = affected
        .resolved_continuous_effects
        .iter()
        .filter(|effect| matches!(effect.kind, ResolvedContinuousEffectKind::Abilities(_)))
        .collect::<Vec<_>>();
    assert_eq!(operations.len(), 2);
    assert_eq!(operations[0].timestamp, operations[1].timestamp);
    assert_eq!(
        [operations[0].component_order, operations[1].component_order],
        [0, 1]
    );
}

#[test]
fn urborg_and_yavimaya_add_types_and_intrinsic_mana_to_every_land() {
    for sources in [
        [
            cards::URBORG_TOMB_OF_YAWGMOTH,
            cards::YAVIMAYA_CRADLE_OF_GROWTH,
        ],
        [
            cards::YAVIMAYA_CRADLE_OF_GROWTH,
            cards::URBORG_TOMB_OF_YAWGMOTH,
        ],
    ] {
        let mut game = ready_game();
        game.battlefield.extend([
            creature(10_000, sources[0], PlayerId::One),
            creature(10_001, sources[1], PlayerId::Two),
            creature(10_002, cards::ISLAND, PlayerId::One),
            creature(10_003, cards::THESPIANS_STAGE, PlayerId::One),
        ]);

        for permanent in &game.battlefield {
            assert_eq!(
                game.effective_land_types(permanent),
                if permanent.card.definition == cards::ISLAND {
                    [false, true, true, false, true]
                } else {
                    [false, false, true, false, true]
                },
            );
        }
        assert_eq!(
            intrinsic_mana_colors(&game, &game.battlefield[2]),
            vec![ManaColor::Blue, ManaColor::Black, ManaColor::Green],
        );
        assert_eq!(
            intrinsic_mana_colors(&game, &game.battlefield[3]),
            vec![ManaColor::Black, ManaColor::Green],
        );
        assert!(
            game.mana_ability_activations(&game.battlefield[3])
                .iter()
                .any(|activation| activation.color == ManaColor::Colorless),
            "adding land types does not remove Stage's printed mana ability",
        );
    }
}

#[test]
fn blood_moon_disables_urborg_and_yavimaya_regardless_of_timestamp() {
    for sources in [
        [
            cards::BLOOD_MOON,
            cards::URBORG_TOMB_OF_YAWGMOTH,
            cards::YAVIMAYA_CRADLE_OF_GROWTH,
        ],
        [
            cards::BLOOD_MOON,
            cards::YAVIMAYA_CRADLE_OF_GROWTH,
            cards::URBORG_TOMB_OF_YAWGMOTH,
        ],
        [
            cards::URBORG_TOMB_OF_YAWGMOTH,
            cards::BLOOD_MOON,
            cards::YAVIMAYA_CRADLE_OF_GROWTH,
        ],
        [
            cards::URBORG_TOMB_OF_YAWGMOTH,
            cards::YAVIMAYA_CRADLE_OF_GROWTH,
            cards::BLOOD_MOON,
        ],
        [
            cards::YAVIMAYA_CRADLE_OF_GROWTH,
            cards::BLOOD_MOON,
            cards::URBORG_TOMB_OF_YAWGMOTH,
        ],
        [
            cards::YAVIMAYA_CRADLE_OF_GROWTH,
            cards::URBORG_TOMB_OF_YAWGMOTH,
            cards::BLOOD_MOON,
        ],
    ] {
        let mut game = ready_game();
        game.battlefield.extend([
            creature(10_000, sources[0], PlayerId::One),
            creature(10_001, sources[1], PlayerId::Two),
            creature(10_002, sources[2], PlayerId::One),
            creature(10_003, cards::ISLAND, PlayerId::One),
            creature(10_004, cards::THESPIANS_STAGE, PlayerId::One),
        ]);

        let island = &game.battlefield[3];
        assert_eq!(
            game.effective_land_types(island),
            [false, true, false, false, false]
        );
        assert_eq!(intrinsic_mana_colors(&game, island), vec![ManaColor::Blue]);

        let stage = &game.battlefield[4];
        assert_eq!(
            game.effective_land_types(stage),
            [false, false, false, true, false]
        );
        assert_eq!(intrinsic_mana_colors(&game, stage), vec![ManaColor::Red]);
        assert!(game.effective_abilities(stage).iter().all(|effective| {
            !matches!(
                effective.ability.definition,
                DeclarativeAbilityDef::Activated(_)
            )
        }));

        for definition in [
            cards::URBORG_TOMB_OF_YAWGMOTH,
            cards::YAVIMAYA_CRADLE_OF_GROWTH,
        ] {
            let source = game
                .battlefield
                .iter()
                .find(|permanent| permanent.card.definition == definition)
                .unwrap();
            assert_eq!(
                game.effective_land_types(source),
                [false, false, false, true, false]
            );
            assert_eq!(intrinsic_mana_colors(&game, source), vec![ManaColor::Red]);
            assert!(game.effective_abilities(source).iter().all(|effective| {
                !matches!(
                    effective.ability.definition,
                    DeclarativeAbilityDef::Static(_)
                )
            }));
        }
    }
}

#[test]
fn stage_copying_a_basic_land_stays_basic_through_blood_moon() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let stage_id = CardInstanceId(10_000);
    let island_id = CardInstanceId(10_001);
    let urborg_id = CardInstanceId(10_002);
    let yavimaya_id = CardInstanceId(10_003);
    let moon_id = CardInstanceId(10_004);
    game.battlefield.extend([
        creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(island_id.0, cards::ISLAND, PlayerId::Two),
        creature(urborg_id.0, cards::URBORG_TOMB_OF_YAWGMOTH, PlayerId::One),
        creature(
            yavimaya_id.0,
            cards::YAVIMAYA_CRADLE_OF_GROWTH,
            PlayerId::Two,
        ),
    ]);
    let copy_ability = activated_ability_for(&game, stage_id, 0);
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage_id,
            ability: copy_ability,
            targets: activated_targets(Target::Permanent(island_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let copied = &game.battlefield[0];
    assert!(
        game.effective_rules(copied)
            .unwrap()
            .has_supertype(CardSupertype::Basic)
    );
    assert_eq!(
        game.effective_land_types(copied),
        [false, true, true, false, true],
    );
    assert_eq!(
        intrinsic_mana_colors(&game, copied),
        vec![ManaColor::Blue, ManaColor::Black, ManaColor::Green],
    );
    assert_eq!(activated_ability_for(&game, stage_id, 0), copy_ability);

    game.battlefield
        .push(creature(moon_id.0, cards::BLOOD_MOON, PlayerId::Two));
    let copied = &game.battlefield[0];
    assert_eq!(
        game.effective_land_types(copied),
        [false, true, false, false, false],
    );
    assert_eq!(intrinsic_mana_colors(&game, copied), vec![ManaColor::Blue]);
    assert_eq!(activated_ability_for(&game, stage_id, 0), copy_ability);

    game.destroy_permanent(moon_id);
    let copied = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == stage_id)
        .unwrap();
    assert_eq!(
        game.effective_land_types(copied),
        [false, true, true, false, true],
    );
}

#[test]
fn stage_activation_already_on_the_stack_resolves_through_blood_moon() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let stage_id = CardInstanceId(10_000);
    let island_id = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(island_id.0, cards::ISLAND, PlayerId::Two),
    ]);
    let copy_ability = activated_ability_for(&game, stage_id, 0);
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage_id,
            ability: copy_ability,
            targets: activated_targets(Target::Permanent(island_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    game.battlefield
        .push(creature(10_002, cards::BLOOD_MOON, PlayerId::Two));
    assert_eq!(
        game.effective_land_types(&game.battlefield[0]),
        [false, false, false, true, false],
    );

    pass_priority_pair(&mut game);
    let copied = &game.battlefield[0];
    assert!(copied.tapped);
    assert!(
        game.effective_rules(copied)
            .unwrap()
            .has_supertype(CardSupertype::Basic)
    );
    assert_eq!(
        game.effective_land_types(copied),
        [false, true, false, false, false],
    );
    assert_eq!(intrinsic_mana_colors(&game, copied), vec![ManaColor::Blue]);
    assert_eq!(activated_ability_for(&game, stage_id, 0), copy_ability);
}

#[test]
fn stage_copying_a_nonbasic_land_is_masked_but_persists_through_blood_moon() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let stage_id = CardInstanceId(10_000);
    let yavimaya_id = CardInstanceId(10_001);
    let moon_id = CardInstanceId(10_002);
    game.battlefield.extend([
        creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(
            yavimaya_id.0,
            cards::YAVIMAYA_CRADLE_OF_GROWTH,
            PlayerId::Two,
        ),
    ]);
    let copy_ability = activated_ability_for(&game, stage_id, 0);
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage_id,
            ability: copy_ability,
            targets: activated_targets(Target::Permanent(yavimaya_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();

    game.battlefield
        .push(creature(moon_id.0, cards::BLOOD_MOON, PlayerId::Two));
    pass_priority_pair(&mut game);

    let copied = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == stage_id)
        .unwrap();
    assert_eq!(
        copied.copy_effect.as_ref().map(|copy| copy.base),
        Some(ObjectCharacteristics::card(
            cards::YAVIMAYA_CRADLE_OF_GROWTH,
            CardPartId::PRIMARY,
        )),
        "the already-stacked activation resolves even though Moon masks its source",
    );
    assert_eq!(
        game.effective_land_types(copied),
        [false, false, false, true, false],
    );
    assert_eq!(intrinsic_mana_colors(&game, copied), vec![ManaColor::Red]);
    assert_eq!(game.effective_abilities(copied).len(), 1);

    game.destroy_permanent(moon_id);
    let copied = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == stage_id)
        .unwrap();
    assert_eq!(
        game.effective_permanent_name(copied).as_deref(),
        Some("Yavimaya, Cradle of Growth"),
    );
    assert!(
        game.effective_rules(copied)
            .unwrap()
            .has_supertype(CardSupertype::Legendary),
    );
    assert!(
        !game
            .effective_rules(copied)
            .unwrap()
            .has_supertype(CardSupertype::Basic),
    );
    assert_eq!(
        game.effective_land_types(copied),
        [false, false, false, false, true],
    );
    assert_eq!(intrinsic_mana_colors(&game, copied), vec![ManaColor::Green]);
    assert_eq!(activated_ability_for(&game, stage_id, 0), copy_ability);
}

#[test]
fn blood_moon_preserves_external_grants_but_later_ability_removal_removes_them() {
    static GRANTED_FLYING: AbilityDef = abilities::flying();

    let mut game = ready_game();
    let stage_id = CardInstanceId(10_000);
    game.battlefield.extend([
        creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(10_001, cards::BLOOD_MOON, PlayerId::Two),
    ]);
    resolve_applied_effect_on_permanent(
        &mut game,
        stage_id,
        AppliedEffectDef::add_ability(&GRANTED_FLYING),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_000,
    );

    let stage = &game.battlefield[0];
    assert!(game.has_flying(stage));
    assert_eq!(intrinsic_mana_colors(&game, stage), vec![ManaColor::Red]);
    assert_eq!(
        game.effective_abilities(stage).len(),
        2,
        "Blood Moon removes Stage's rules abilities, not independently granted abilities",
    );

    resolve_applied_effect_on_permanent(
        &mut game,
        stage_id,
        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_001,
    );
    assert!(game.effective_abilities(&game.battlefield[0]).is_empty());
    assert!(
        game.mana_ability_activations(&game.battlefield[0])
            .is_empty()
    );

    game.finish_cleanup();
    let stage = &game.battlefield[0];
    assert!(!game.has_flying(stage));
    assert_eq!(intrinsic_mana_colors(&game, stage), vec![ManaColor::Red]);
}

#[test]
fn resolved_ability_additions_and_removals_are_ordered_and_expire() {
    static GRANTED_ACTIVATED: AbilityDef = AbilityDef::activated(
        "Draw a card.",
        &[],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    );
    static GRANTED_FLYING: AbilityDef = abilities::flying();

    let mut game = ready_game();
    let target = CardInstanceId(10_000);
    game.battlefield
        .push(creature(target.0, cards::SERRA_ANGEL, PlayerId::One));
    resolve_applied_effect_on_permanent(
        &mut game,
        target,
        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_000,
    );
    assert!(game.effective_abilities(&game.battlefield[0]).is_empty());

    resolve_applied_effect_on_permanent(
        &mut game,
        target,
        AppliedEffectDef::add_ability(&GRANTED_ACTIVATED),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_001,
    );
    assert!(
        game.effective_abilities(&game.battlefield[0])
            .iter()
            .any(|effective| matches!(
                effective.ability.definition,
                DeclarativeAbilityDef::Activated(_)
            ))
    );
    game.finish_cleanup();
    assert!(game.has_flying(&game.battlefield[0]));

    resolve_applied_effect_on_permanent(
        &mut game,
        target,
        AppliedEffectDef::add_ability(&GRANTED_FLYING),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_002,
    );
    resolve_applied_effect_on_permanent(
        &mut game,
        target,
        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(KeywordAbility::Flying)),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_003,
    );
    assert!(!game.has_flying(&game.battlefield[0]));
    assert!(
        game.permanent_has_executable_keyword(&game.battlefield[0], KeywordAbility::Vigilance),
        "selective removal leaves unrelated abilities alone",
    );

    game.finish_cleanup();
    assert!(game.has_flying(&game.battlefield[0]));
    assert!(
        game.effective_abilities(&game.battlefield[0])
            .iter()
            .all(|effective| !matches!(
                effective.ability.definition,
                DeclarativeAbilityDef::Activated(_)
            ))
    );
}

#[test]
fn resolved_keyword_changes_are_visible_to_object_predicates() {
    static GRANTED_FLYING: AbilityDef = abilities::flying();

    let mut game = ready_game();
    let target = CardInstanceId(10_000);
    game.battlefield
        .push(creature(target.0, cards::SAVANNAH_LIONS, PlayerId::One));
    let has_flying = |game: &Game| {
        let event = game.trigger_event_object(&game.battlefield[0]);
        game.trigger_object_matches(
            ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
            &event,
            target,
            false,
        )
    };
    assert!(!has_flying(&game));

    resolve_applied_effect_on_permanent(
        &mut game,
        target,
        AppliedEffectDef::add_ability(&GRANTED_FLYING),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_000,
    );
    assert!(has_flying(&game));
    resolve_applied_effect_on_permanent(
        &mut game,
        target,
        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(KeywordAbility::Flying)),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_001,
    );
    assert!(!has_flying(&game));
}

#[test]
fn blood_moon_strips_printed_keywords_from_object_predicates() {
    let definition_id = CardDefinitionId::new(10_090);
    let mut definition = CardDefinition::new(
        definition_id,
        "Flying Gate",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_creature_without_mana_cost(&["Gate", "Bird"], 1, 1)
        .with_type(CardType::Land)
        .with_ability(abilities::flying());
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let blood_moon = game.catalog.get(cards::BLOOD_MOON).unwrap().clone();
    game.catalog = CardCatalog::new([blood_moon, definition]).unwrap();
    game.battlefield.extend([
        creature(10_000, cards::BLOOD_MOON, PlayerId::One),
        creature(10_001, definition_id, PlayerId::Two),
    ]);
    let event = game.trigger_event_object(&game.battlefield[1]);
    assert!(!game.trigger_object_matches(
        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
        &event,
        game.battlefield[1].card.id,
        false,
    ));
}

#[test]
fn resolved_ability_removal_suppresses_declarative_abilities_until_it_expires() {
    let mut game = ready_game();
    let ape = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(10_000, cards::TAIGA, PlayerId::One),
        creature(ape.0, cards::KIRD_APE, PlayerId::One),
    ]);
    assert_eq!(game.power(&game.battlefield[1]), Some(2));
    assert_eq!(game.toughness(&game.battlefield[1]), Some(3));

    resolve_applied_effect_on_permanent(
        &mut game,
        ape,
        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
        ResolvedEffectDurationDef::UntilEndOfTurn,
        20_000,
    );
    assert_eq!(game.power(&game.battlefield[1]), Some(1));
    assert_eq!(game.toughness(&game.battlefield[1]), Some(1));

    game.finish_cleanup();
    assert_eq!(game.power(&game.battlefield[1]), Some(2));
    assert_eq!(game.toughness(&game.battlefield[1]), Some(3));
}

#[test]
fn static_ability_additions_and_removals_follow_source_timestamps() {
    static FLYING: AbilityDef = abilities::flying();
    static GRANT: [AbilityDef; 1] = [AbilityDef::static_ability(
        "Creatures have flying.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::add_ability(&FLYING),
        },
    )];
    static REMOVE: [AbilityDef; 1] = [AbilityDef::static_ability(
        "Creatures lose all abilities.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
        },
    )];
    let grant_id = CardDefinitionId::new(10_090);
    let remove_id = CardDefinitionId::new(10_091);
    let mut grant = CardDefinition::new(
        grant_id,
        "Static ability grant test",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    grant.rules = CardRules::new_enchantment(ManaCost::new(0, 0)).with_abilities(&GRANT);
    synchronize_single_part_definition(&mut grant);
    let mut remove = CardDefinition::new(
        remove_id,
        "Static ability removal test",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    remove.rules = CardRules::new_enchantment(ManaCost::new(0, 0)).with_abilities(&REMOVE);
    synchronize_single_part_definition(&mut remove);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.extend([grant.clone(), remove.clone()]);
    game.catalog = CardCatalog::new(definitions).unwrap();
    game.battlefield.extend([
        creature(10_000, grant_id, PlayerId::One),
        creature(10_001, remove_id, PlayerId::Two),
        creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);
    assert!(!game.has_flying(&game.battlefield[2]));
    game.destroy_permanent(CardInstanceId(10_001));
    assert!(game.has_flying(&game.battlefield[1]));

    let mut reverse = ready_game();
    let mut definitions = reverse
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.extend([grant, remove]);
    reverse.catalog = CardCatalog::new(definitions).unwrap();
    reverse.battlefield.extend([
        creature(10_000, remove_id, PlayerId::Two),
        creature(10_001, grant_id, PlayerId::One),
        creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
    ]);
    assert!(reverse.has_flying(&reverse.battlefield[2]));
}

/// A keyword a live static effect grants or removes reaches object predicates,
/// so target legality and the combat rules read one ability set rather than
/// two. Lord of Atlantis hands out islandwalk and Gravity Sphere takes flying
/// away; both answers have to match `permanent_has_executable_keyword`.
#[test]
fn static_keyword_grants_and_removals_reach_object_predicates() {
    let matches = |game: &Game, index: usize, keyword: KeywordAbility| {
        let permanent = &game.battlefield[index];
        let predicate = game.trigger_object_matches(
            ObjectPredicateDef::HasKeyword(keyword),
            &game.trigger_event_object(permanent),
            permanent.card.id,
            false,
        );
        assert_eq!(
            predicate,
            game.permanent_has_executable_keyword(permanent, keyword),
            "the predicate and the rules query disagree about {keyword:?}"
        );
        predicate
    };

    let islandwalk = KeywordAbility::Landwalk(BasicLandType::Island);
    let mut granted = ready_game();
    granted
        .battlefield
        .push(creature(10_000, cards::VODALIAN_MAGE, PlayerId::One));
    assert!(!matches(&granted, 0, islandwalk));
    granted
        .battlefield
        .push(creature(10_001, cards::LORD_OF_ATLANTIS, PlayerId::One));
    assert!(matches(&granted, 0, islandwalk));

    let mut removed = ready_game();
    removed
        .battlefield
        .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::One));
    assert!(matches(&removed, 0, KeywordAbility::Flying));
    removed
        .battlefield
        .push(creature(10_001, cards::GRAVITY_SPHERE, PlayerId::Two));
    assert!(!matches(&removed, 0, KeywordAbility::Flying));
}

include!("land_and_ability_layers/static_dependencies.rs");
