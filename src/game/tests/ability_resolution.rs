use super::*;

#[test]
fn copied_grant_source_definition_is_part_of_the_granted_ability_origin() {
    let (mut game, grantor, receiver, definition_a, definition_b) = copied_grant_source_game();
    let first_origin = sole_granted_origin(&game, receiver);
    assert_eq!(first_origin, copied_grant_origin(grantor, definition_a));
    assert_eq!(
        game.ability_for_origin(receiver, first_origin)
            .map(|ability| ability.text),
        Some("Gain 1 life."),
    );
    let stale_action = Action::ActivateAbility {
        source: receiver,
        ability: first_origin,
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&stale_action));

    game.battlefield[0].copy_effect = Some(copied_characteristics(definition_b));
    let second_origin = sole_granted_origin(&game, receiver);
    assert_eq!(second_origin, copied_grant_origin(grantor, definition_b));
    assert_ne!(first_origin, second_origin);
    assert_eq!(game.ability_for_origin(receiver, first_origin), None);
    assert_eq!(
        game.ability_for_origin(receiver, second_origin)
            .map(|ability| ability.text),
        Some("Lose 1 life."),
    );
    let current_actions = game.legal_actions(PlayerId::One);
    assert!(
        !current_actions.contains(&stale_action),
        "a stale action must not alias a same-position grant from different copied rules",
    );
    assert!(current_actions.contains(&Action::ActivateAbility {
        source: receiver,
        ability: second_origin,
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    }));
}

static MULTI_SLOT_ACTIVATION_TARGETS: [AbilityTargetDef; 2] = [
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent)),
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    }),
];
static MULTI_SLOT_ACTIVATION_EFFECTS: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(1),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex(1)),
        amount: ValueDef::Constant(1),
    },
];
static MULTI_SLOT_ACTIVATION_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_with_targets(
    "Sacrifice this artifact: It deals 1 damage to target opponent and 1 damage to target creature that player controls.",
    &[AbilityCostDef::SacrificeSource],
    &MULTI_SLOT_ACTIVATION_TARGETS,
    EffectDef::Sequence(&MULTI_SLOT_ACTIVATION_EFFECTS),
)];

#[test]
fn declarative_activation_preserves_multiple_slots_before_sacrificing_its_source() {
    let definition_id = CardDefinitionId::new(10_063);
    let mut definition = CardDefinition::new(
        definition_id,
        "Multi-slot activation test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::new(0, 0))
        .with_abilities(&MULTI_SLOT_ACTIVATION_ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = CardInstanceId(10_000);
    let creature_target = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(source.0, definition_id, PlayerId::One),
        creature(creature_target.0, cards::SERRA_ANGEL, PlayerId::Two),
    ]);
    let targets = vec![
        TargetSelection::single(TargetSlotId(0), Target::Player(PlayerId::Two)),
        TargetSelection::single(TargetSlotId(1), Target::Permanent(creature_target)),
    ];
    let activation = Action::ActivateAbility {
        source,
        ability: primary_ability(definition_id),
        targets: targets.clone(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };

    let invalid_slots = Action::ActivateAbility {
        source,
        ability: primary_ability(definition_id),
        targets: vec![
            TargetSelection::single(TargetSlotId(1), Target::Player(PlayerId::Two)),
            TargetSelection::single(TargetSlotId(0), Target::Permanent(creature_target)),
        ],
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };
    assert!(game.apply(PlayerId::One, invalid_slots).is_err());
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == source),
        "slot validation must happen before sacrificing the source",
    );
    assert!(game.stack.is_empty());

    assert!(
        game.legal_actions(PlayerId::One).contains(&activation),
        "declarative action generation must retain abilities with multiple target slots",
    );
    game.apply(PlayerId::One, activation).unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != source),
        "the source was sacrificed as an activation cost",
    );
    let payload = game.stack[0]
        .ability
        .as_ref()
        .expect("the activated ability has a frozen payload");
    assert_eq!(payload.target_defs, &MULTI_SLOT_ACTIVATION_TARGETS);
    assert_eq!(payload.targets, targets);

    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 19);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == creature_target)
            .is_some_and(|permanent| permanent.damage == 1),
    );
}

#[test]
fn one_ability_target_slot_resolves_for_every_selected_legal_target() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef {
        predicate: AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            zones: &[ZoneKind::Battlefield],
            controller: Some(PlayerRelation::Opponent),
            owner: None,
        },
        minimum: 1,
        maximum: 2,
        divided_total: None,
        another: false,
        excludes_source: false,
        chooser: TargetChooserDef::Controller,
    }];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_with_targets(
        "Deal 1 damage to up to two target creatures an opponent controls.",
        &[],
        &TARGETS,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
        },
    )];

    let definition_id = CardDefinitionId::new(10_064);
    let mut definition = CardDefinition::new(
        definition_id,
        "Multi-target slot test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::new(0, 0)).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = CardInstanceId(10_000);
    let first_target = CardInstanceId(10_001);
    let second_target = CardInstanceId(10_002);
    game.battlefield.extend([
        creature(source.0, definition_id, PlayerId::One),
        creature(first_target.0, cards::SERRA_ANGEL, PlayerId::Two),
        creature(second_target.0, cards::SERRA_ANGEL, PlayerId::Two),
    ]);
    let action = Action::ActivateAbility {
        source,
        ability: primary_ability(definition_id),
        targets: vec![TargetSelection::new(
            TargetSlotId(0),
            vec![
                Target::Permanent(first_target),
                Target::Permanent(second_target),
            ],
        )],
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    pass_priority_pair(&mut game);

    for target in [first_target, second_target] {
        assert!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == target)
                .is_some_and(|permanent| permanent.damage == 1),
            "every legal target selected in the slot receives the effect",
        );
    }
}

#[test]
fn granted_ability_keeps_its_frozen_resolver_when_the_source_changes() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Any,
            zones: &[ZoneKind::Battlefield],
            controller: None,
            owner: None,
        },
    )];
    static GRANTED_ABILITY: AbilityDef = AbilityDef::activated_with_targets(
        "{T}: Tap target permanent.",
        &[AbilityCostDef::TapSource],
        &TARGETS,
        EffectDef::Tap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )
    .with_effect_execution(EffectExecutionDef::Custom(
        CardBehavior::LibraryOfAlexandria,
    ))
    .with_coverage(AbilityCoverageDef::explained_complete(
        "The test intentionally grants a custom resolver.",
    ));
    static SOURCE_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "This permanent has the test ability.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(&GRANTED_ABILITY),
        },
    )];
    let definition_id = CardDefinitionId::new(10_061);
    let mut definition = CardDefinition::new(
        definition_id,
        "Granted resolver test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new_artifact(ManaCost::new(0, 0)).with_abilities(&SOURCE_ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    game.battlefield.extend([
        creature(10_000, definition_id, PlayerId::One),
        creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two),
    ]);
    let source = CardInstanceId(10_000);
    let target = CardInstanceId(10_001);
    let source_card = game.battlefield[0].card.clone();
    let origin = AbilityOrigin::Granted {
        source,
        source_definition: definition_id,
        source_part: CardPartId::PRIMARY,
        source_ability: AbilityId::PRIMARY,
        grant: GrantId::PRIMARY,
    };
    let frozen = game.freeze_activated_ability(&game.battlefield[0], origin);

    game.push_activated_ability(
        source,
        &source_card,
        PlayerId::One,
        frozen,
        activated_targets(Target::Permanent(target)),
        vec![target],
    );
    assert_eq!(game.stack[0].ability_origin(), Some(origin));
    assert!(matches!(
        game.stack[0]
            .ability
            .as_ref()
            .map(|ability| ability.resolver),
        Some(StackAbilityResolver::Custom(
            CardBehavior::LibraryOfAlexandria
        ))
    ));

    // This models a continuous/copy effect changing the effective rules of a
    // source after activation. The origin remains provenance, while the stack
    // object's executable payload must remain the Library procedure.
    game.battlefield[0].copy_effect = Some(copied_characteristics(cards::JAYEMDAE_TOME));
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        1,
        "resolution must not rediscover a different handler from the changed source",
    );
}

#[test]
fn declarative_clause_uses_its_own_resolver_on_a_card_with_custom_behavior() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::AnyTarget,
    )];
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::activated_with_targets(
            "Deal 1 damage to any target.",
            &[],
            &TARGETS,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::custom_full(
            "A separate custom clause.",
            CardBehavior::Fireball,
            "The test keeps one explicitly custom clause beside the declarative clause.",
        ),
    ];
    let definition_id = CardDefinitionId::new(10_060);
    let mut definition = CardDefinition::new(
        definition_id,
        "Mixed resolver test card",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::new(0, 0)).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_060, definition_id, PlayerId::One));
    let source = CardInstanceId(10_060);
    let source_card = game.battlefield[0].card.clone();
    let origin = primary_ability(definition_id);
    let frozen = game.freeze_activated_ability(&game.battlefield[0], origin);

    game.push_activated_ability(
        source,
        &source_card,
        PlayerId::One,
        frozen,
        activated_targets(Target::Player(PlayerId::Two)),
        Vec::new(),
    );
    assert!(matches!(
        game.stack[0]
            .ability
            .as_ref()
            .map(|ability| ability.resolver),
        Some(StackAbilityResolver::Declarative(ScopedEffect {
            effect: EffectDef::DealDamage { .. },
            ..
        }))
    ));

    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[1].life, 19,
        "the selected definition must not dispatch through Fireball's unrelated hook",
    );
}

#[test]
fn legacy_activated_clauses_keep_their_own_origins() {
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::activated(
            "{T}: Draw a card. Activate only if you have exactly seven cards in hand.",
            &[AbilityCostDef::TapSource],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )
        .with_effect_execution(EffectExecutionDef::Custom(
            CardBehavior::LibraryOfAlexandria,
        ))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The test uses the Library of Alexandria resolver.",
        ))
        .with_legacy_procedure(),
        AbilityDef::activated(
            "{T}: Draw a card. Activate only if you have exactly seven cards in hand.",
            &[AbilityCostDef::TapSource],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )
        .with_effect_execution(EffectExecutionDef::Custom(
            CardBehavior::LibraryOfAlexandria,
        ))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The test uses the Library of Alexandria resolver.",
        ))
        .with_legacy_procedure(),
    ];
    let definition_id = CardDefinitionId::new(10_096);
    let mut definition = CardDefinition::new(
        definition_id,
        "Multiple legacy activation test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::default()).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = CardInstanceId(10_000);
    game.battlefield
        .push(creature(source.0, definition_id, PlayerId::One));
    game.players[PlayerId::One.index()]
        .hand
        .extend((0..7).map(|offset| card(10_001 + offset, cards::MOUNTAIN, PlayerId::One)));
    let first_origin = activated_ability_for(&game, source, 0);
    let second_origin = activated_ability_for(&game, source, 1);
    let first = plain_activation(source, first_origin);
    let second = plain_activation(source, second_origin);
    let actions = game.legal_actions(PlayerId::One);
    assert!(actions.contains(&first));
    assert!(actions.contains(&second));
    assert_ne!(first_origin, second_origin);

    game.apply(PlayerId::One, second).unwrap();
    assert_eq!(game.stack[0].ability_origin(), Some(second_origin));
    assert_eq!(
        game.stack[0]
            .ability
            .as_ref()
            .map(|ability| ability.resolver),
        Some(StackAbilityResolver::Custom(
            CardBehavior::LibraryOfAlexandria,
        )),
    );
    pass_priority_pair(&mut game);
    assert!(game.battlefield[0].tapped);
    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 8);
}

#[test]
fn a_legacy_activation_after_a_shared_clause_keeps_its_own_origin() {
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::activated(
            "You gain 1 life.",
            &[],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{T}: Draw a card. Activate only if you have exactly seven cards in hand.",
            &[AbilityCostDef::TapSource],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )
        .with_effect_execution(EffectExecutionDef::Custom(
            CardBehavior::LibraryOfAlexandria,
        ))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The test uses the Library of Alexandria resolver.",
        ))
        .with_legacy_procedure(),
    ];
    let definition_id = CardDefinitionId::new(10_097);
    let mut definition = CardDefinition::new(
        definition_id,
        "Mixed shared and legacy activation test",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules =
        CardRules::new_creature(ManaCost::default(), &[], 1, 1).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    let source = CardInstanceId(10_000);
    game.battlefield
        .push(creature(source.0, definition_id, PlayerId::One));
    game.players[PlayerId::One.index()]
        .hand
        .extend((0..7).map(|offset| card(10_001 + offset, cards::MOUNTAIN, PlayerId::One)));
    let legacy_origin = activated_ability_for(&game, source, 1);
    let action = Action::ActivateAbility {
        source,
        ability: legacy_origin,
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.stack[0].ability_origin(), Some(legacy_origin));
    assert_eq!(
        game.stack[0]
            .ability
            .as_ref()
            .map(|ability| ability.resolver),
        Some(StackAbilityResolver::Custom(
            CardBehavior::LibraryOfAlexandria,
        )),
    );
    assert_eq!(game.players[PlayerId::One.index()].life, 20);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 8);
    assert!(game.stack.is_empty());
}

static TWO_SLOT_TARGETS: [AbilityTargetDef; 2] = [
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    }),
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    }),
];
static TWO_SLOT_EFFECTS: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(1),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex(1)),
        amount: ValueDef::Constant(1),
    },
];

#[test]
fn resolving_ability_masks_an_illegal_target_in_each_frozen_slot() {
    let mut game = ready_game();
    let source = CardInstanceId(10_000);
    let first = CardInstanceId(10_001);
    let second = CardInstanceId(10_002);
    game.battlefield.extend([
        creature(source.0, cards::ANKH_OF_MISHRA, PlayerId::One),
        creature(first.0, cards::SERRA_ANGEL, PlayerId::One),
        creature(second.0, cards::SERRA_ANGEL, PlayerId::One),
    ]);
    game.stack.push(StackObject {
        id: StackObjectId(20_000),
        kind: StackObjectKind::TriggeredAbility,
        card: card(20_000, cards::ANKH_OF_MISHRA, PlayerId::One).into(),
        source: Some(source),
        ability: Some(StackAbilityPayload {
            origin: primary_ability(cards::ANKH_OF_MISHRA),
            definition: None,
            presentation: ObjectCharacteristics::card(cards::ANKH_OF_MISHRA, CardPartId::PRIMARY),
            text: Some("Test two-slot trigger"),
            target_defs: TWO_SLOT_TARGETS.to_vec(),
            targets: vec![
                TargetSelection::single(TargetSlotId(0), Target::Permanent(first)),
                TargetSelection::single(TargetSlotId(1), Target::Permanent(second)),
            ],
            context: TriggerContext {
                object: None,
                zone_change_result: None,
                object_controller: None,
                event_player: None,
                amount: None,
                damaged_object: None,
                cast_from_zone: None,
            }
            .into(),
            resolver: StackAbilityResolver::Declarative(ScopedEffect::primary(
                EffectDef::Sequence(&TWO_SLOT_EFFECTS),
            )),
            condition: None,
            mode_effects: Vec::new(),
            resolution_destination: None,
            x: 0,
            sacrificed_mana_value: 0,
        }),
        controller: PlayerId::One,
        signature: None,
        chosen_permanents: Vec::new(),
        applied_effects: Vec::new(),
        text_changes: Vec::new(),
        colors: None,
        cast_via_flashback: false,
        cast_via_suspend: false,
        cast_at_instant_speed: false,
        cast_from_zone: None,
        face_down: None,
        colors_of_mana_spent: ColorSet::empty(),
        phyrexian_symbols_paid_with_life: 0,
        is_copy: false,
    });

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == first)
        .unwrap()
        .controller = PlayerId::Two;
    pass_priority_pair(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == first)
            .unwrap()
            .damage,
        0,
        "an illegal target in one slot is ignored",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == second)
            .unwrap()
            .damage,
        1,
        "the legal target in the other slot still receives its effect",
    );
}

#[test]
fn copy_artifact_copies_declarative_mana_abilities_without_a_behavior_hook() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SOL_RING, PlayerId::Two));
    let copy = card(10_001, cards::COPY_ARTIFACT, PlayerId::One);
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
    let ability = mana_ability_for(&game, copied_id, ManaColor::Colorless);
    assert_eq!(ability, primary_ability(cards::SOL_RING));
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: copied_id,
            ability,
            color: ManaColor::Colorless,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .unwrap();

    assert_eq!(game.players[0].mana_pool.colorless, 2);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == copied_id)
            .is_some_and(|permanent| permanent.tapped)
    );
}

#[test]
fn dust_to_dust_exiles_two_artifacts_and_hurkyls_recall_returns_them() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::SOL_RING, PlayerId::Two),
        creature(10_001, cards::BLACK_VISE, PlayerId::Two),
    ]);
    let dust = spell(10_002, cards::DUST_TO_DUST, PlayerId::One, 0);
    dust_to_dust_targets(&mut game, dust);
    assert_eq!(game.players[0].exile.len(), 0);
    assert_eq!(game.players[1].exile.len(), 2);

    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::SOL_RING, PlayerId::Two),
        creature(10_001, cards::BLACK_VISE, PlayerId::Two),
    ]);
    let recall = card(10_002, cards::HURKYLS_RECALL, PlayerId::One);
    game.players[0].hand.push(recall.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
    let action = acceptance_cast_action_targeting(
        &game,
        PlayerId::One,
        recall.id,
        Target::Player(PlayerId::Two),
    );
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[1].hand.len(), 2);
    assert!(game.battlefield.is_empty());
}

#[test]
fn hurkyls_recall_follows_ownership_rather_than_control() {
    let mut game = ready_game();
    // An artifact its owner has lost control of still goes home to them.
    let mut stolen = creature(10_000, cards::SOL_RING, PlayerId::Two);
    stolen.controller = PlayerId::One;
    game.battlefield.push(stolen);
    // And one the targeted player controls but does not own stays put.
    let mut borrowed = creature(10_001, cards::BLACK_VISE, PlayerId::One);
    borrowed.controller = PlayerId::Two;
    game.battlefield.push(borrowed);

    let recall = card(10_002, cards::HURKYLS_RECALL, PlayerId::One);
    game.players[0].hand.push(recall.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
    let action = acceptance_cast_action_targeting(
        &game,
        PlayerId::One,
        recall.id,
        Target::Player(PlayerId::Two),
    );
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);

    assert_eq!(
        game.players[1]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::SOL_RING],
        "the artifact they own came back even from across the table"
    );
    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.definition)
            .collect::<Vec<_>>(),
        vec![cards::BLACK_VISE],
        "and the one they only control was left alone"
    );
}
