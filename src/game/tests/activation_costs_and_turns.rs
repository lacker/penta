use super::*;

#[test]
fn duplicate_source_counter_costs_are_aggregated_before_an_activation_is_offered() {
    static COSTS: [AbilityCostDef; 2] = [
        AbilityCostDef::RemoveCountersFromSource {
            kind: CounterKind::Charge,
            amount: 1,
        },
        AbilityCostDef::RemoveCountersFromSource {
            kind: CounterKind::Charge,
            amount: 1,
        },
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
        "Remove two charge counters from this artifact: You gain 1 life.",
        &COSTS,
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )];
    let definition_id = CardDefinitionId::new(10_090);
    let mut definition = CardDefinition::new(
        definition_id,
        "Aggregate counter cost test",
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
    let mut source = creature(10_000, definition_id, PlayerId::One);
    source.counters[CounterKind::Charge.index()] = 1;
    let source_id = source.card.id;
    game.battlefield.push(source);
    let action = Action::ActivateAbility {
        source: source_id,
        ability: activated_ability_for(&game, source_id, 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };

    assert!(!game.legal_actions(PlayerId::One).contains(&action));
    game.battlefield[0].counters[CounterKind::Charge.index()] = 2;
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.battlefield[0].counters(CounterKind::Charge), 0);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].life, 21);
}

#[test]
fn a_counter_only_mana_ability_is_offered_and_pays_its_counter_cost() {
    static COSTS: [AbilityCostDef; 1] = [AbilityCostDef::RemoveCountersFromSource {
        kind: CounterKind::Charge,
        amount: 1,
    }];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
        "Remove a charge counter from this artifact: Add {C}.",
        &COSTS,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
    )];
    let definition_id = CardDefinitionId::new(10_092);
    let mut definition = CardDefinition::new(
        definition_id,
        "Counter mana cost test",
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
    let mut source = creature(10_000, definition_id, PlayerId::One);
    source.counters[CounterKind::Charge.index()] = 1;
    let source_id = source.card.id;
    game.battlefield.push(source);
    let action = Action::ActivateManaAbility {
        source: source_id,
        ability: mana_ability_for(&game, source_id, ManaColor::Colorless),
        color: ManaColor::Colorless,
        counters_removed: None,
        cost_object: None,
        combination: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.battlefield[0].counters(CounterKind::Charge), 0);
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 1);
    assert!(!game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateManaAbility { source, .. } if *source == source_id)
    ));
}

#[test]
fn source_counters_are_removed_before_a_source_sacrifice_cost_regardless_of_printed_order() {
    static COSTS: [AbilityCostDef; 2] = [
        AbilityCostDef::SacrificeSource,
        AbilityCostDef::RemoveCountersFromSource {
            kind: CounterKind::Charge,
            amount: 1,
        },
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
        "Sacrifice this artifact and remove a charge counter from it: You gain 1 life.",
        &COSTS,
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )];
    let definition_id = CardDefinitionId::new(10_091);
    let mut definition = CardDefinition::new(
        definition_id,
        "Counter and sacrifice cost test",
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
    let mut source = creature(10_000, definition_id, PlayerId::One);
    source.counters[CounterKind::Charge.index()] = 1;
    let source_id = source.card.id;
    game.battlefield.push(source);
    let action = Action::ActivateAbility {
        source: source_id,
        ability: activated_ability_for(&game, source_id, 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != source_id)
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == definition_id)
    );
    assert_eq!(game.stack.len(), 1);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].life, 21);
}

#[test]
fn a_generic_source_sacrifice_waits_for_its_tap_and_counter_costs() {
    static COSTS: [AbilityCostDef; 3] = [
        AbilityCostDef::SacrificePermanent {
            object: ObjectPredicateDef::Source,
            controller: PlayerRelation::You,
        },
        AbilityCostDef::TapSource,
        AbilityCostDef::RemoveCountersFromSource {
            kind: CounterKind::Charge,
            amount: 1,
        },
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
        "Sacrifice this artifact, tap it, and remove a charge counter from it: You gain 1 life.",
        &COSTS,
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )];
    let definition_id = CardDefinitionId::new(10_093);
    let mut definition = CardDefinition::new(
        definition_id,
        "Generic source sacrifice ordering test",
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
    let mut source = creature(10_000, definition_id, PlayerId::One);
    source.counters[CounterKind::Charge.index()] = 1;
    let source_id = source.card.id;
    game.battlefield.push(source);
    let action = Action::ActivateAbility {
        source: source_id,
        ability: primary_ability(definition_id),
        targets: Vec::new(),
        cost_objects: vec![source_id],
        x: 0,
        modes: Vec::new(),
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();

    assert!(game.battlefield.is_empty());
    assert_eq!(
        game.current_or_last_known_counters(source_id, CounterKind::Charge),
        0
    );
    assert!(matches!(
        game.retired_objects.get(&source_id),
        Some(RetiredObject::Permanent { permanent, .. }) if permanent.tapped
    ));
    assert_eq!(game.stack.len(), 1);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].life, 21);
}

#[test]
fn separate_source_sacrifice_costs_require_separate_permanents() {
    static COSTS: [AbilityCostDef; 2] = [
        AbilityCostDef::SacrificeSource,
        AbilityCostDef::SacrificePermanent {
            object: ObjectPredicateDef::HasType(CardType::Artifact),
            controller: PlayerRelation::You,
        },
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
        "Sacrifice this artifact and another artifact: You gain 1 life.",
        &COSTS,
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )];
    let definition_id = CardDefinitionId::new(10_094);
    let mut definition = CardDefinition::new(
        definition_id,
        "Distinct sacrifice cost test",
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
    let source_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(source_id.0, definition_id, PlayerId::One));
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == source_id)
        ),
        "the source cannot satisfy both sacrifice costs by itself",
    );

    let other_id = CardInstanceId(10_001);
    game.battlefield
        .push(creature(other_id.0, cards::ICY_MANIPULATOR, PlayerId::One));
    let action = Action::ActivateAbility {
        source: source_id,
        ability: primary_ability(definition_id),
        targets: Vec::new(),
        cost_objects: vec![other_id],
        x: 0,
        modes: Vec::new(),
    };
    let illegal_double_payment = Action::ActivateAbility {
        source: source_id,
        ability: primary_ability(definition_id),
        targets: Vec::new(),
        cost_objects: vec![source_id],
        x: 0,
        modes: Vec::new(),
    };
    let actions = game.legal_actions(PlayerId::One);
    assert!(actions.contains(&action));
    assert!(!actions.contains(&illegal_double_payment));

    game.apply(PlayerId::One, action).unwrap();
    assert!(game.battlefield.is_empty());
    assert_eq!(game.stack.len(), 1);
}

#[test]
fn duplicate_source_sacrifice_costs_are_never_offered() {
    static COSTS: [AbilityCostDef; 2] = [
        AbilityCostDef::SacrificeSource,
        AbilityCostDef::SacrificeSource,
    ];
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::activated(
            "Sacrifice this artifact twice: You gain 1 life.",
            &COSTS,
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "Sacrifice this artifact twice: Add {C}.",
            &COSTS,
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
    ];
    let definition_id = CardDefinitionId::new(10_095);
    let mut definition = CardDefinition::new(
        definition_id,
        "Duplicate source sacrifice test",
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
    let source_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(source_id.0, definition_id, PlayerId::One));

    assert!(!game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source, .. } if *source == source_id)
    ));
    assert!(!game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateManaAbility { source, .. } if *source == source_id)
    ));
}

#[test]
fn javelineers_on_the_stack_retain_the_sources_last_known_color() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let mut javelineers = creature(10_000, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    javelineers.counters[CounterKind::Javelin.index()] = 1;
    let source = javelineers.card.id;
    let target = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield = vec![javelineers, target];

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source,
            ability: activated_ability_for(&game, source, 0),
            targets: activated_targets(Target::Permanent(target_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    game.destroy_permanent_without_regeneration(source);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == target_id)
        .expect("the target remains on the battlefield")
        .temporary_keywords
        .push(protection_keyword(ManaColor::White));

    pass_priority_pair(&mut game);

    let target = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == target_id)
        .expect("protection prevents the damage");
    assert_eq!(target.damage, 0);
}

#[test]
fn goblin_king_buffs_other_goblins_and_grants_mountainwalk() {
    let mut game = ready_game();
    let king = creature(10_000, cards::GOBLIN_KING, PlayerId::One);
    let mut flarg = creature(10_001, cards::GOBLINS_OF_THE_FLARG, PlayerId::One);
    flarg.attacking = true;
    let mountain = creature(10_002, cards::MOUNTAIN, PlayerId::Two);
    let blocker = creature(10_003, cards::IRONCLAW_ORCS, PlayerId::Two);
    let flarg_id = flarg.card.id;
    game.battlefield = vec![king, flarg, mountain, blocker];
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;

    let flarg = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == flarg_id)
        .unwrap();
    assert_eq!(game.power(flarg), Some(2));
    assert!(
        game.legal_actions(PlayerId::Two)
            .iter()
            .all(|action| !matches!(
                action,
                Action::DeclareBlocker { attacker, .. } if *attacker == flarg_id
            ))
    );
}

#[test]
fn erhnam_djinn_upkeep_targets_a_creature_for_forestwalk() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::ERHNAM_DJINN, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::JUZAM_DJINN, PlayerId::Two));
    // A Wall is never a candidate, and neither is the Djinn's own side.
    game.battlefield
        .push(creature(10_002, cards::WALL_OF_STONE, PlayerId::Two));
    game.battlefield
        .push(creature(10_003, cards::SAVANNAH_LIONS, PlayerId::One));
    game.turn = 2;
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();
    drain_pending(&mut game);

    let target = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_001))
        .expect("still there");
    assert!(
        game.has_forestwalk(target),
        "the only legal target got the gift"
    );
    for spared in [GameObjectId(10_002), GameObjectId(10_003)] {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == spared)
            .expect("still there");
        assert!(!game.has_forestwalk(permanent));
    }

    // It lasts through the opponent's turn and ends when the Djinn's
    // controller comes back around.
    game.finish_cleanup();
    game.start_next_turn();
    let target = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_001))
        .expect("still there");
    assert!(
        game.has_forestwalk(target),
        "an until-your-next-upkeep grant outlives cleanup"
    );
    game.finish_cleanup();
    game.start_next_turn();
    let target = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_001))
        .expect("still there");
    assert!(
        !game.has_forestwalk(target),
        "and ends when that upkeep arrives"
    );
}

#[test]
fn wheel_discards_both_hands_and_draws_seven() {
    let mut game = ready_game();
    let wheel = card(10_000, cards::WHEEL_OF_FORTUNE, PlayerId::One);
    game.players[0].hand.push(wheel.clone());
    game.players[0]
        .hand
        .push(card(10_001, cards::MOUNTAIN, PlayerId::One));
    game.players[1]
        .hand
        .push(card(10_002, cards::MOUNTAIN, PlayerId::Two));
    game.players[0].mana_pool.red = 3;

    game.apply(
        PlayerId::One,
        cast_action(wheel.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].hand.len(), 7);
    assert_eq!(game.players[1].hand.len(), 7);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| backing_cards(&card.backing) == vec![PhysicalCardId(10_001)])
    );
}

#[test]
fn wheel_and_timetwister_resolve_as_shared_declarative_spells() {
    let game = ready_game();
    for definition in [cards::WHEEL_OF_FORTUNE, cards::TIMETWISTER] {
        let card = game
            .catalog
            .get(definition)
            .expect("card is in the catalog");
        let [ability] = card.rules.ability_clauses() else {
            panic!("the spell has one printed clause")
        };
        assert!(matches!(
            ability.definition,
            DeclarativeAbilityDef::Spell(_)
        ));
        assert_eq!(ability.effect.execution, EffectExecutionDef::Declarative);
        assert!(matches!(
            ability.declarative_effect(),
            Some(EffectDef::Sequence(_))
        ));
        assert_eq!(ability.custom_behavior(), None);
    }

    let wheel = game
        .catalog
        .get(cards::WHEEL_OF_FORTUNE)
        .expect("Wheel is in the catalog");
    let [ability] = wheel.rules.ability_clauses() else {
        panic!("Wheel has one printed clause")
    };
    assert!(matches!(
        ability.declarative_effect(),
        Some(EffectDef::Sequence([
            EffectDef::Discard {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(i32::MAX),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(7),
            },
        ]))
    ));
}

#[test]
fn wheel_draws_in_active_player_order_when_cast_by_the_nonactive_player() {
    let mut game = ready_game();
    let wheel = card(10_000, cards::WHEEL_OF_FORTUNE, PlayerId::One);
    game.players[0].hand.push(wheel.clone());
    game.players[0].mana_pool.red = 3;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    game.sorcery_flash_grants[0] = 1;
    let event_start = game.events.len();

    game.apply(
        PlayerId::One,
        cast_action(wheel.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let draw_order = game.events[event_start..]
        .iter()
        .filter_map(|event| match event {
            GameEvent::CardDrawn { player, .. } => Some(*player),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        draw_order,
        [[PlayerId::Two; 7].as_slice(), [PlayerId::One; 7].as_slice(),].concat(),
        "CR 121.2c makes the active player complete all seven draws first"
    );
}

#[test]
fn a_wheel_that_decks_only_one_player_still_deals_the_other_a_full_hand() {
    // The loser draws what is left before losing, and the survivor still gets
    // all seven. The old shortcut checked library sizes first and dealt
    // nobody anything.
    let mut game = ready_game();
    let wheel = card(10_000, cards::WHEEL_OF_FORTUNE, PlayerId::One);
    game.players[0].hand.push(wheel.clone());
    game.players[0].mana_pool.red = 3;
    game.players[1].library.truncate(3);

    game.apply(
        PlayerId::One,
        cast_action(wheel.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentTriedToDrawFromEmptyLibrary,
        })
    );
    assert_eq!(game.players[0].hand.len(), 7, "the survivor drew all seven");
    assert_eq!(
        game.players[1].hand.len(),
        3,
        "and the loser drew the three they had before running out"
    );
}

#[test]
fn a_wheel_that_decks_both_players_is_a_draw() {
    // One spell, two empty libraries. Whoever the loop happens to reach first
    // must not win the game for it.
    let mut game = ready_game();
    let wheel = card(10_000, cards::WHEEL_OF_FORTUNE, PlayerId::One);
    game.players[0].hand.push(wheel.clone());
    game.players[0].mana_pool.red = 3;
    game.players[0].library.truncate(2);
    game.players[1].library.truncate(5);

    game.apply(
        PlayerId::One,
        cast_action(wheel.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.result, Some(GameResult::Draw));
}

#[test]
fn a_timetwister_that_decks_both_players_is_a_draw() {
    // Timetwister shuffles hands and graveyards back first, so the libraries
    // have to be short even after that to run out.
    let mut game = ready_game();
    let twister = card(10_000, cards::TIMETWISTER, PlayerId::One);
    game.players[0].hand.push(twister.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;
    game.players[0].library.truncate(1);
    game.players[1].library.truncate(1);

    game.apply(
        PlayerId::One,
        cast_action(twister.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.result, Some(GameResult::Draw));
}

#[test]
fn timetwister_shuffles_hands_and_graveyards_but_not_itself_then_draws_seven() {
    let mut game = ready_game();
    let twister = card(10_000, cards::TIMETWISTER, PlayerId::One);
    let one_hand = card(10_001, cards::MOUNTAIN, PlayerId::One);
    let one_graveyard = card(10_002, cards::MOUNTAIN, PlayerId::One);
    let two_hand = card(10_003, cards::MOUNTAIN, PlayerId::Two);
    let two_graveyard = card(10_004, cards::MOUNTAIN, PlayerId::Two);
    game.players[0].hand.extend([twister.clone(), one_hand]);
    game.players[0].graveyard.push(one_graveyard);
    game.players[1].hand.push(two_hand);
    game.players[1].graveyard.push(two_graveyard);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;

    game.apply(
        PlayerId::One,
        cast_action(twister.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].hand.len(), 7);
    assert_eq!(game.players[1].hand.len(), 7);
    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .flat_map(|card| backing_cards(&card.backing))
            .collect::<Vec<_>>(),
        vec![PhysicalCardId(10_000)],
        "Timetwister stays on the stack during its effect and goes to the graveyard afterward"
    );
    assert!(game.players[1].graveyard.is_empty());

    for (player, returned) in [
        (
            PlayerId::One,
            [PhysicalCardId(10_001), PhysicalCardId(10_002)],
        ),
        (
            PlayerId::Two,
            [PhysicalCardId(10_003), PhysicalCardId(10_004)],
        ),
    ] {
        let shuffled = game.players[player.index()]
            .hand
            .iter()
            .chain(&game.players[player.index()].library)
            .flat_map(|card| backing_cards(&card.backing))
            .collect::<Vec<_>>();
        for card in returned {
            assert!(
                shuffled.contains(&card),
                "the player's former hand and graveyard are in their shuffled library or new hand"
            );
        }
    }
}

#[test]
fn a_seats_event_stream_withholds_the_seed() {
    // Decklists are public. Hand a seat the seed and they can shuffle both
    // libraries themselves, which is the opponent's hand and every draw
    // either player will make. It has to stay out of anything a seat is sent.
    let game = ready_game();
    assert!(
        game.events()
            .iter()
            .any(|event| matches!(event, GameEvent::GameStarted { .. })),
        "the raw log records it, which is why the projection has work to do"
    );
    for seat in [PlayerId::One, PlayerId::Two] {
        assert!(
            !game
                .events_for(seat)
                .iter()
                .any(|event| matches!(event, GameEvent::GameStarted { .. })),
            "{seat:?} must not be handed the seed"
        );
    }
    assert_eq!(
        game.events_for(PlayerId::One).len(),
        game.events().len() - 1,
        "and nothing else is withheld yet"
    );
}

#[test]
fn extra_turn_scheduler_preserves_a_nonactive_players_regular_turn() {
    let mut game = ready_game();
    let first_turn = game.turn;

    game.schedule_extra_turns([PlayerId::Two]);
    game.start_next_turn();

    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.next_regular_player, PlayerId::Two);
    assert_eq!(game.turn, first_turn + 1);

    game.start_next_turn();

    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.next_regular_player, PlayerId::One);
    assert_eq!(game.turn, first_turn + 2);
}

#[test]
fn extra_turn_scheduler_takes_multiple_extra_turns_lifo() {
    let mut game = ready_game();

    game.schedule_extra_turns([PlayerId::One]);
    game.schedule_extra_turns([PlayerId::Two]);

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.next_regular_player, PlayerId::Two);

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
    assert_eq!(game.next_regular_player, PlayerId::Two);

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.next_regular_player, PlayerId::One);
}

#[test]
fn extra_turn_scheduler_deduplicates_simultaneous_recipients_in_apnap_order() {
    let mut game = ready_game();

    game.schedule_extra_turns([PlayerId::Two, PlayerId::One, PlayerId::Two]);
    assert_eq!(game.extra_turns, vec![PlayerId::One, PlayerId::Two]);

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.next_regular_player, PlayerId::Two);

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
    assert_eq!(game.next_regular_player, PlayerId::Two);

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.next_regular_player, PlayerId::One);
}

#[test]
fn cleanup_without_a_discard_advances_without_priority() {
    let mut game = ready_game();
    game.step = Step::End;
    let first_turn = game.turn;

    pass_priority_pair(&mut game);

    assert_eq!(game.turn, first_turn + 1);
    assert_eq!(game.step, Step::Upkeep);
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.observe(PlayerId::One).active_turn, 1);
    assert_eq!(game.decision_player(), Some(PlayerId::Two));
}

#[test]
fn cleanup_discard_advances_directly_to_the_next_upkeep() {
    let mut game = ready_game();
    game.step = Step::End;
    for id in 10_000..10_008 {
        game.players[0]
            .hand
            .push(card(id, cards::MOUNTAIN, PlayerId::One));
    }

    pass_priority_pair(&mut game);
    assert_eq!(game.step, Step::Cleanup);
    let discard = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::DiscardCards { .. }))
        .unwrap();
    game.apply(PlayerId::One, discard).unwrap();

    assert_eq!(game.turn, 2);
    assert_eq!(game.step, Step::Upkeep);
    assert_eq!(game.active_player, PlayerId::Two);
    assert_eq!(game.decision_player(), Some(PlayerId::Two));
}
