use super::*;

#[test]
fn trigger_placement_preserves_the_nonactive_players_priority() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::CITY_OF_BRASS, PlayerId::Two));

    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    assert_eq!(game.priority, PlayerId::Two);
    game.apply(
        PlayerId::Two,
        Action::ActivateManaAbility {
            source: CardInstanceId(10_000),
            ability: mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Blue),
            color: ManaColor::Blue,
            counters_removed: None,
            cost_object: None,
            combination: None,
        },
    )
    .unwrap();

    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.priority, PlayerId::Two);
    assert!(
        game.legal_actions(PlayerId::Two)
            .contains(&Action::PassPriority)
    );
}

#[test]
fn ankh_trigger_can_be_answered_by_bolt_before_it_resolves() {
    let mut game = ready_game();
    game.players[0].life = 2;
    game.players[1].life = 3;
    game.battlefield
        .push(creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::Two));
    let mountain = card(10_001, cards::MOUNTAIN, PlayerId::One);
    let bolt = card(10_002, cards::LIGHTNING_BOLT, PlayerId::One);
    game.players[0]
        .hand
        .extend([mountain.clone(), bolt.clone()]);

    let play_land = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == mountain.id))
        .expect("Mountain is a legal land play");
    game.apply(PlayerId::One, play_land).unwrap();

    assert_eq!(game.players[0].life, 2);
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
    assert_eq!(game.stack[0].source, Some(CardInstanceId(10_000)));
    assert_eq!(
        game.stack[0].ability_origin(),
        Some(primary_ability(cards::ANKH_OF_MISHRA))
    );

    let cast_bolt = cast_action(bolt.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast_bolt));
    game.apply(PlayerId::One, cast_bolt).unwrap();
    assert_eq!(game.stack.len(), 2);
    assert_eq!(game.stack.last().unwrap().kind, StackObjectKind::Spell);

    pass_priority_pair(&mut game);
    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentLostAllLife,
        })
    );
    assert_eq!(game.players[0].life, 2);
    assert_eq!(game.stack.len(), 1, "Ankh never got to resolve");
}

#[test]
fn ankh_damages_the_entering_lands_controller_not_its_owner() {
    let mut game = ready_game();
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.battlefield
        .push(creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::One));
    let borrowed_mountain = card(10_001, cards::MOUNTAIN, PlayerId::One);
    game.players[1].hand.push(borrowed_mountain.clone());

    let play_land = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(action, Action::PlayLand { card, .. } if *card == borrowed_mountain.id)
        })
        .expect("the active player may play the land they currently hold");
    game.apply(PlayerId::Two, play_land).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].life, 20, "the physical owner is unharmed");
    assert_eq!(game.players[1].life, 18, "the land's controller takes 2");
}

#[test]
fn city_trigger_can_be_answered_when_mana_was_floated_first() {
    let mut game = ready_game();
    game.players[0].life = 1;
    game.players[1].life = 3;
    let city = creature(10_000, cards::CITY_OF_BRASS, PlayerId::One);
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::One);
    game.battlefield.push(city);
    game.players[0].hand.push(bolt.clone());

    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: CardInstanceId(10_000),
            ability: mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Red),
            color: ManaColor::Red,
            counters_removed: None,
            cost_object: None,
            combination: None,
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.players[0].life, 1);

    game.apply(
        PlayerId::One,
        cast_action(bolt.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    assert_eq!(game.stack.last().unwrap().kind, StackObjectKind::Spell);
    pass_priority_pair(&mut game);

    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentLostAllLife,
        })
    );
}

#[test]
fn city_trigger_is_above_a_spell_when_city_pays_during_casting() {
    let mut game = ready_game();
    game.players[0].life = 1;
    game.players[1].life = 3;
    game.battlefield
        .push(creature(10_000, cards::CITY_OF_BRASS, PlayerId::One));
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::One);
    game.players[0].hand.push(bolt.clone());

    let cast = cast_action(bolt.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();

    assert_eq!(game.stack.len(), 2);
    assert_eq!(game.stack[0].kind, StackObjectKind::Spell);
    assert_eq!(game.stack[1].kind, StackObjectKind::TriggeredAbility);
    pass_priority_pair(&mut game);
    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            reason: WinReason::OpponentLostAllLife,
        })
    );
    assert_eq!(game.players[1].life, 3, "Bolt never resolved");
}

#[test]
fn a_resolving_tap_effect_uses_the_same_city_trigger_path() {
    let mut game = ready_game();
    game.players[0].mana_pool.colorless = 1;
    game.battlefield.extend([
        creature(10_000, cards::ICY_MANIPULATOR, PlayerId::One),
        creature(10_001, cards::CITY_OF_BRASS, PlayerId::Two),
    ]);
    let activation = Action::ActivateAbility {
        source: CardInstanceId(10_000),
        ability: activated_ability_for(&game, CardInstanceId(10_000), 0),
        targets: activated_targets(Target::Permanent(CardInstanceId(10_001))),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    assert!(game.legal_actions(PlayerId::One).contains(&activation));
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield[1].tapped);
    assert_eq!(game.players[1].life, 20);
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
    assert_eq!(game.stack[0].source, Some(CardInstanceId(10_001)));
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 19);
}

#[test]
fn controller_chooses_resolution_order_for_simultaneous_triggers() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::One),
        creature(10_001, cards::ANKH_OF_MISHRA, PlayerId::One),
    ]);
    let mountain = card(10_002, cards::MOUNTAIN, PlayerId::One);
    game.players[0].hand.push(mountain.clone());
    let play = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == mountain.id))
        .unwrap();
    game.apply(PlayerId::One, play).unwrap();

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.kind, DecisionKind::TriggerOrder);
    assert_eq!(
        decision.order_semantics,
        Some(DecisionOrderSemantics::Resolution)
    );
    assert!(decision.options.iter().all(|option| {
        option
            .ability_text
            .as_deref()
            .is_some_and(|text| text.contains("Whenever a land enters"))
    }));
    let first = decision.options[0].id;
    let second = decision.options[1].id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![second, first],
        },
    )
    .unwrap();

    assert_eq!(game.stack.len(), 2);
    assert_eq!(
        game.stack.last().unwrap().source,
        Some(CardInstanceId(10_001))
    );
    assert!(game.stack.iter().all(|object| {
        object.ability_origin() == Some(primary_ability(cards::ANKH_OF_MISHRA))
            && object.ability_text().is_some()
    }));
}

#[test]
fn simultaneous_triggers_are_put_on_the_stack_in_apnap_order() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::One),
        creature(10_001, cards::ANKH_OF_MISHRA, PlayerId::Two),
    ]);
    let mountain = card(10_002, cards::MOUNTAIN, PlayerId::One);
    game.players[0].hand.push(mountain.clone());
    let play = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == mountain.id))
        .expect("Mountain is a legal land play");
    game.apply(PlayerId::One, play).unwrap();

    assert_eq!(game.stack.len(), 2);
    assert_eq!(
        game.stack[0].source,
        Some(CardInstanceId(10_000)),
        "the active player's trigger is put on the stack first"
    );
    assert_eq!(
        game.stack[1].source,
        Some(CardInstanceId(10_001)),
        "the nonactive player's trigger is on top and resolves first"
    );
}

#[test]
fn targeted_trigger_chooses_public_targets_while_being_put_on_stack() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            zones: &[ZoneKind::Battlefield],
            controller: Some(PlayerRelation::Opponent),
            owner: None,
        },
    )];
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::ANKH_OF_MISHRA, PlayerId::One),
        creature(10_001, cards::SU_CHI, PlayerId::Two),
    ]);
    game.capture_trigger(&TriggerCapture {
        source: AbilitySourceRef {
            object: CardInstanceId(10_000),
            ability: primary_ability(cards::ANKH_OF_MISHRA),
        },
        presentation: ObjectCharacteristics::card(cards::ANKH_OF_MISHRA, CardPartId::PRIMARY),
        owner: PlayerId::One,
        controller: PlayerId::One,
        text: "Deal 2 damage to target creature an opponent controls.",
        target_defs: TARGETS.to_vec(),
        targets: Vec::new(),
        effect: EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
        resolver: StackAbilityResolver::Declarative(ScopedEffect::primary(EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        })),
        context: TriggerContext {
            object: None,
            object_controller: None,
            event_player: None,
            amount: None,
        }
        .into(),
        condition: None,
        x: 0,
    });
    game.finish_rules_procedure();

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.kind, DecisionKind::TriggerPlacement);
    assert_eq!(decision.visibility, DecisionVisibility::Public);
    assert_eq!(decision.minimum, 1);
    assert_eq!(decision.maximum, 1);
    assert_eq!(decision.options.len(), 1);
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[0].id],
        },
    )
    .unwrap();

    assert_eq!(
        game.stack[0].targets(),
        vec![Target::Permanent(CardInstanceId(10_001))]
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.battlefield[1].damage, 2);
}

#[test]
fn nonbattlefield_card_targets_are_zone_incarnations() {
    static INSTANT_OR_SORCERY: [ObjectPredicateDef; 2] = [
        ObjectPredicateDef::HasType(CardType::Instant),
        ObjectPredicateDef::HasType(CardType::Sorcery),
    ];
    let predicate = AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&INSTANT_OR_SORCERY),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    };
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let stone_rain = card(10_001, cards::STONE_RAIN, PlayerId::One);
    let mountain = card(10_002, cards::MOUNTAIN, PlayerId::One);
    let opposing_bolt = card(10_003, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[0]
        .graveyard
        .extend([bolt.clone(), stone_rain.clone(), mountain]);
    game.players[1].graveyard.push(opposing_bolt);

    let targets = game.ability_targets_matching(
        predicate,
        PlayerId::One,
        GameObjectId(99_999),
        TriggerContext::empty(),
    );
    assert_eq!(
        targets,
        vec![Target::Card(bolt.id), Target::Card(stone_rain.id)]
    );

    let old_bolt = game.players[0].graveyard.remove(0);
    let (new_bolt, zone_change) = game.zone_change_card(old_bolt);
    game.players[0].hand.push(new_bolt);
    assert_eq!(zone_change.previous, bolt.id);
    assert_eq!(
        game.ability_targets_matching(
            predicate,
            PlayerId::One,
            GameObjectId(99_999),
            TriggerContext::empty(),
        ),
        vec![Target::Card(stone_rain.id)],
        "a target does not follow the physical card to its new zone object",
    );
}

#[test]
fn su_chi_mana_and_source_power_use_ordinary_stack_and_lki() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SU_CHI, PlayerId::One));
    game.destroy_permanent(CardInstanceId(10_000));
    assert_eq!(game.players[0].mana_pool.colorless, 0);
    game.finish_rules_procedure();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].source, Some(CardInstanceId(10_000)));
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].mana_pool.colorless, 4);

    let mut game = ready_game();
    let source = creature(10_010, cards::SAVANNAH_LIONS, PlayerId::One);
    game.battlefield.push(source);
    attach_constant_resolved_characteristics(
        &mut game,
        GameObjectId(10_010),
        &[AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(3),
            ValueDef::Constant(0),
        )],
        ContinuousEffectExpiration::Never,
    );
    game.capture_trigger(&TriggerCapture {
        source: AbilitySourceRef {
            object: CardInstanceId(10_010),
            ability: primary_ability(cards::SAVANNAH_LIONS),
        },
        presentation: ObjectCharacteristics::card(cards::SAVANNAH_LIONS, CardPartId::PRIMARY),
        owner: PlayerId::One,
        controller: PlayerId::One,
        text: "Deal damage equal to this creature's power.",
        target_defs: Vec::new(),
        targets: Vec::new(),
        effect: EffectDef::DealDamage {
            recipient: EffectRecipientDef::Opponent,
            amount: ValueDef::SourcePower,
        },
        resolver: StackAbilityResolver::Declarative(ScopedEffect::primary(EffectDef::DealDamage {
            recipient: EffectRecipientDef::Opponent,
            amount: ValueDef::SourcePower,
        })),
        context: TriggerContext {
            object: Some(CardInstanceId(10_010)),
            object_controller: Some(PlayerId::One),
            event_player: Some(PlayerId::One),
            amount: None,
        }
        .into(),
        condition: None,
        x: 0,
    });
    game.destroy_permanent(CardInstanceId(10_010));
    game.finish_rules_procedure();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 15, "last known power was five");
}

#[test]
fn workshop_mana_is_three_individual_values_restricted_to_artifact_spells() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::MISHRA_S_WORKSHOP, PlayerId::One));
    let ability = mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Colorless);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: CardInstanceId(10_000),
            ability,
            color: ManaColor::Colorless,
            counters_removed: None,
            cost_object: None,
            combination: None,
        },
    )
    .unwrap();

    assert_eq!(game.players[0].mana_pool.colorless, 3);
    assert_eq!(game.players[0].mana.len(), 3);
    assert!(game.players[0].mana.iter().all(|mana| {
        mana.color == ManaColor::Colorless
            && mana.source
                == Some(ManaSource {
                    object: CardInstanceId(10_000),
                    ability,
                })
            && mana.restrictions
                == [ManaRestrictionDef::CastSpell(ObjectPredicateDef::HasType(
                    CardType::Artifact,
                ))]
    }));
}

#[test]
fn explicitly_tagged_triggered_mana_ability_resolves_without_the_stack() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered_mana(
        "Whenever this is tapped for mana, add {C}.",
        TriggerEventDef::tapped_for_mana(ObjectPredicateDef::Source),
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
    )];
    let definition_id = CardDefinitionId::new(10_050);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test triggered mana source",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_artifact(ManaCost::new(0, 0)).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_050, definition_id, PlayerId::One));

    let _ = game.tap_permanent_for_mana(CardInstanceId(10_050));

    assert_eq!(game.players[0].mana_pool.colorless, 1);
    assert_eq!(game.players[0].mana.len(), 1);
    assert!(game.pending_triggers.is_empty());
    assert!(game.stack.is_empty());
}

#[test]
fn a_mana_spend_rider_attaches_to_the_paid_spell_with_its_source() {
    static RIDERS: [ManaSpendEffectDef; 1] = [ManaSpendEffectDef::ApplyToPaidSpell(
        crate::AppliedEffectDef::Rule(crate::AppliedRuleDef::CannotBeCountered),
    )];
    let mut object = spell(77, cards::SAVANNAH_LIONS, PlayerId::One, 0);
    let mana = Mana::from_ability(
        ManaColor::White,
        ManaSource {
            object: CardInstanceId(10_000),
            ability: AbilityOrigin::Printed {
                definition: cards::SAVANNAH_LIONS,
                part: CardPartId::PRIMARY,
                ability: crate::AbilityId(1),
            },
        },
        &[],
        &RIDERS,
    );

    Game::apply_spent_mana_to_spell(&mut object, &[mana]);

    assert_eq!(object.applied_effects.len(), 1);
    assert_eq!(object.applied_effects[0].source, mana.source);
    assert_eq!(
        object.applied_effects[0].effect,
        crate::AppliedEffectDef::Rule(crate::AppliedRuleDef::CannotBeCountered)
    );
}
