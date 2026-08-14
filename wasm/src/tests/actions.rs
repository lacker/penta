use super::*;

#[test]
fn cast_action_labels_distinguish_normal_flashback_and_overload() {
    let game = WebGame::new(
        "Briksza Naya Midrange",
        "Greer G/R Aggro",
        "Handcrafted",
        true,
        2,
        Some("isd-rtr-standard".into()),
    )
    .unwrap();
    let mut observation = game.session.engine().observe(game.human);
    let normal = CardInstanceId(90_000);
    let flashback = CardInstanceId(90_001);
    let overload = CardInstanceId(90_002);
    let granted_flashback = CardInstanceId(90_003);
    observation
        .hand
        .push((normal, penta::card::cards::THINK_TWICE));
    observation.graveyards[game.human.index()].push((flashback, penta::card::cards::THINK_TWICE));
    observation.graveyards[game.human.index()]
        .push((granted_flashback, penta::card::cards::THINK_TWICE));
    observation
        .hand
        .push((overload, penta::card::cards::MIZZIUM_MORTARS));

    let normal = Action::CastSpell {
        card: normal,
        choices: penta::CastChoices::default(),
        sacrifices: Vec::new(),
    };
    let flashback = Action::CastSpell {
        card: flashback,
        choices: penta::CastChoices::default().with_costs(penta::CostConfiguration::new(
            Some(penta::AlternativeCostId(1)),
            Vec::new(),
        )),
        sacrifices: Vec::new(),
    };
    let overload = Action::CastSpell {
        card: overload,
        choices: penta::CastChoices::default().with_costs(penta::CostConfiguration::new(
            Some(penta::AlternativeCostId(1)),
            Vec::new(),
        )),
        sacrifices: Vec::new(),
    };
    let granted_flashback = Action::CastSpell {
        card: granted_flashback,
        choices: penta::CastChoices::default().with_costs(penta::CostConfiguration::new(
            Some(penta::AlternativeCostId(u8::MAX)),
            Vec::new(),
        )),
        sacrifices: Vec::new(),
    };

    assert_eq!(game.action_label(&observation, &normal), "Cast Think Twice");
    assert_eq!(
        game.action_label(&observation, &flashback),
        "Cast Think Twice via Flashback {2}{U}"
    );
    assert_eq!(
        game.action_label(&observation, &overload),
        "Cast Mizzium Mortars via Overload {3}{R}{R}{R}"
    );
    assert_eq!(
        game.action_label(&observation, &granted_flashback),
        "Cast Think Twice via Flashback {1}{U}"
    );
}

#[test]
fn activated_action_labels_distinguish_exact_ability_origins() {
    let mut game = WebGame::new(
        "The Deck",
        "Robots",
        "Handcrafted",
        true,
        2,
        Some("old-school-93-94".into()),
    )
    .unwrap();
    let source = game
        .session
        .engine_mut()
        .put_onto_battlefield(game.human, penta::card::cards::MISHRA_S_FACTORY)
        .expect("Mishra's Factory enters the test battlefield");
    let mut observation = game.session.engine_mut().observe(game.human);
    assert!(
        observation
            .battlefield
            .iter()
            .any(|permanent| permanent.id == source),
        "the formatter resolves the ability from the real game object",
    );
    let animate = Action::ActivateAbility {
        source,
        ability: AbilityOrigin::Printed {
            definition: penta::card::cards::MISHRA_S_FACTORY,
            part: penta::CardPartId::PRIMARY,
            ability: penta::AbilityId(1),
        },
        targets: Vec::new(),
        cost_object: None,
        x: 0,
    };
    let pump = Action::ActivateAbility {
        source,
        ability: AbilityOrigin::Printed {
            definition: penta::card::cards::MISHRA_S_FACTORY,
            part: penta::CardPartId::PRIMARY,
            ability: penta::AbilityId(2),
        },
        targets: vec![penta::TargetSelection::single(
            penta::TargetSlotId(0),
            Target::Permanent(source),
        )],
        cost_object: None,
        x: 0,
    };

    // Multiple target/X/sacrifice variants of one origin remain one
    // relevant ability and retain the compact source-only label.
    observation.legal_actions = vec![animate.clone(), animate.clone()];
    assert_eq!(
        game.action_ability_label(&observation, &animate),
        Some("Activate Mishra's Factory".into()),
    );

    observation.legal_actions = vec![animate.clone(), pump.clone()];
    let animate_label = "Mishra's Factory — {1}: This land becomes a 2/2 Assembly-Worker artifact creature until end of turn. It's still a land.";
    let pump_label =
        "Mishra's Factory — {T}: Target Assembly-Worker creature gets +1/+1 until end of turn.";
    assert_eq!(
        game.action_ability_label(&observation, &animate),
        Some(animate_label.into()),
    );
    assert_eq!(
        game.action_ability_label(&observation, &pump),
        Some(pump_label.into()),
    );
    assert_eq!(game.action_label(&observation, &animate), animate_label);
    assert_eq!(
        game.opponent_action_label(&observation, &pump),
        format!("{pump_label} → Mishra's Factory"),
    );
    assert_eq!(
        game.action_ability_label(&observation, &Action::PassPriority),
        None,
    );
}

#[test]
fn activated_action_labels_show_distinct_x_and_every_selected_target() {
    let mut game = WebGame::new(
        "Briksza Naya Midrange",
        "Greer G/R Aggro",
        "Handcrafted",
        true,
        2,
        Some("isd-rtr-standard".into()),
    )
    .unwrap();
    let source = game
        .session
        .engine_mut()
        .put_onto_battlefield(game.human, penta::card::cards::KESSIG_WOLF_RUN)
        .expect("Kessig Wolf Run enters the test battlefield");
    let pilgrim = game
        .session
        .engine_mut()
        .put_onto_battlefield(game.human, penta::card::cards::AVACYNS_PILGRIM)
        .expect("Avacyn's Pilgrim enters the test battlefield");
    let thragtusk = game
        .session
        .engine_mut()
        .put_onto_battlefield(game.human, penta::card::cards::THRAGTUSK)
        .expect("Thragtusk enters the test battlefield");
    let ability = AbilityOrigin::Printed {
        definition: penta::card::cards::KESSIG_WOLF_RUN,
        part: penta::CardPartId::PRIMARY,
        ability: penta::AbilityId(1),
    };
    let action = |x| Action::ActivateAbility {
        source,
        ability,
        targets: vec![penta::TargetSelection::new(
            penta::TargetSlotId(0),
            vec![Target::Permanent(pilgrim), Target::Permanent(thragtusk)],
        )],
        cost_object: None,
        x,
    };
    let zero = action(0);
    let two = action(2);
    let mut observation = game.session.engine_mut().observe(game.human);
    observation.legal_actions = vec![zero.clone(), two.clone()];

    assert_eq!(
        game.action_ability_label(&observation, &zero),
        Some("Activate Kessig Wolf Run".into()),
        "the grouping label is independent of X and targets",
    );
    assert_eq!(
        game.action_ability_label(&observation, &two),
        game.action_ability_label(&observation, &zero),
    );
    assert_eq!(
        game.action_label(&observation, &zero),
        "Activate Kessig Wolf Run (X=0) → Avacyn's Pilgrim, Thragtusk",
    );
    assert_eq!(
        game.action_label(&observation, &two),
        "Activate Kessig Wolf Run (X=2) → Avacyn's Pilgrim, Thragtusk",
    );

    observation.legal_actions = vec![two.clone()];
    assert_eq!(
        game.action_label(&observation, &two),
        "Activate Kessig Wolf Run → Avacyn's Pilgrim, Thragtusk",
        "a sole legal X value needs no disambiguating suffix",
    );
}

#[test]
fn stack_signature_json_preserves_forms_modes_costs_and_target_slots() {
    let signature = penta::CastSignature::from_validated_choices(
        penta::SpellForm::Combined(vec![penta::CardPartId(0), penta::CardPartId(1)]),
        penta::CastChoices::new(penta::PlayOptionId(2))
            .with_modes(vec![penta::ModeId(3)])
            .with_costs(penta::CostConfiguration::new(
                Some(penta::AlternativeCostId(4)),
                vec![penta::AdditionalCostId(5)],
            ))
            .with_x(6)
            .with_targets(vec![penta::TargetSelection::new(
                penta::TargetSlotId(7),
                vec![
                    Target::Permanent(penta::GameObjectId(8)),
                    Target::Player(PlayerId::Two),
                    Target::Spell(penta::GameObjectId(9)),
                ],
            )]),
    );

    assert_eq!(
        cast_signature_value(&signature, PlayerId::One),
        json!({
            "playOptionId": 2,
            "form": { "kind": "combined", "partIds": [0, 1] },
            "modeIds": [3],
            "alternativeCostId": 4,
            "additionalCostIds": [5],
            "x": 6,
            "targetSelections": [{
                "slotId": 7,
                "amounts": [],
                "targetCardIds": [8],
                "targetPlayers": ["opponent"],
                "targetStackIds": [9],
            }],
        })
    );

    let part_signature = penta::CastSignature::from_validated_choices(
        penta::SpellForm::Part(penta::CardPartId::PRIMARY),
        penta::CastChoices::default(),
    );
    assert_eq!(
        cast_signature_value(&part_signature, PlayerId::One)["form"],
        json!({ "kind": "part", "partId": 0 })
    );
}

#[test]
fn stack_presentation_uses_the_locked_split_card_form() {
    let catalog = card::catalog().expect("catalog builds");
    let turn_burn = catalog
        .get(penta::card::cards::TURN_BURN)
        .expect("Turn // Burn is cataloged");
    let burn_signature = penta::CastSignature::from_validated_choices(
        penta::SpellForm::Part(penta::CardPartId(1)),
        penta::CastChoices::new(penta::PlayOptionId(1)),
    );

    let burn = stack_card_presentation(Some(turn_burn), Some(&burn_signature));
    assert_eq!(burn.name, "Burn");
    assert_eq!(burn.kind, "instant");
    assert_eq!(burn.type_line, "Instant");
    assert_eq!(
        burn.implementation_status,
        penta::ImplementationStatus::Complete
    );
    assert_eq!(
        burn.mana_cost,
        Some(penta::ManaCost::colored(1, 0, 0, 0, 1, 0))
    );
    assert!(burn.rules_text.starts_with("Burn deals 2 damage"));
    assert_eq!((burn.power, burn.toughness), (None, None));

    let fused_signature = penta::CastSignature::from_validated_choices(
        penta::SpellForm::Combined(vec![penta::CardPartId::PRIMARY, penta::CardPartId(1)]),
        penta::CastChoices::new(penta::PlayOptionId(2)),
    );
    let fused = stack_card_presentation(Some(turn_burn), Some(&fused_signature));
    assert_eq!(fused.name, "Turn // Burn");
    assert_eq!(fused.kind, "instant");
    assert_eq!(fused.type_line, "Instant");
    assert_eq!(
        fused.mana_cost,
        Some(penta::ManaCost::colored(3, 0, 1, 0, 1, 0))
    );
    assert!(fused.rules_text.contains("Turn — Until end of turn"));
    assert!(fused.rules_text.contains("Burn — Burn deals 2 damage"));
    assert_eq!(
        fused.implementation_status,
        penta::ImplementationStatus::Complete
    );
}

#[test]
fn stack_presentation_preserves_legacy_composite_kinds_and_land_membership() {
    let catalog = card::catalog().expect("catalog builds");
    let juggernaut = catalog
        .get(penta::card::cards::JUGGERNAUT)
        .expect("Juggernaut is cataloged");
    let artifact_creature = StackCardPresentation::from_rules(
        juggernaut.name.clone(),
        &juggernaut.rules,
        juggernaut.rules.mana_cost(),
    );

    assert_eq!(artifact_creature.kind, "artifactcreature");
    assert!(!artifact_creature.is_land);

    let mountain = catalog
        .get(penta::card::cards::MOUNTAIN)
        .expect("Mountain is cataloged");
    let land = StackCardPresentation::from_rules(
        mountain.name.clone(),
        &mountain.rules,
        mountain.rules.mana_cost(),
    );

    assert_eq!(land.kind, "land");
    assert!(land.is_land);
}

#[test]
fn visible_card_coverage_comes_from_ability_implementations() {
    let game = WebGame::new(
        "Briksza Naya Midrange",
        "Greer G/R Aggro",
        "Handcrafted",
        true,
        2,
        Some("isd-rtr-standard".into()),
    )
    .unwrap();
    let snapshot = game.snapshot_value(false);
    let hand = snapshot["human"]["hand"].as_array().expect("hand array");
    let find = |name: &str| {
        hand.iter()
            .find(|card| card["name"] == name)
            .unwrap_or_else(|| panic!("{name} is in the deterministic hand"))
    };

    assert_eq!(
        find("Avacyn's Pilgrim")["implementationStatus"],
        "complete",
        "the fully modeled creature and mana ability must not inherit its legacy play gate",
    );
    assert_eq!(
        find("Bonfire of the Damned")["implementationStatus"],
        "complete"
    );
    assert!(
        hand.iter().all(|card| card.get("metadataOnly").is_none()),
        "the WASM surface exposes the derived status, not its former boolean projection",
    );
}

#[test]
fn blocker_actions_expose_the_attacker_as_their_board_target() {
    let attacker = CardInstanceId(7);
    let blocker = CardInstanceId(8);
    let action = Action::DeclareBlocker { blocker, attacker };
    assert_eq!(action_card(&action), Some(blocker));
    assert_eq!(action_target_card(&action), Some(attacker));
}

fn assert_animated_as_ability(action: &Action) {
    assert!(should_animate_action(action));
    assert_eq!(crate::action_view::animated_action_kind(action), "ability");
}

#[test]
fn ability_actions_expose_their_stable_origins() {
    let action = Action::ActivateAbility {
        source: CardInstanceId(8),
        ability: penta::AbilityOrigin::Printed {
            definition: penta::poc::cards::MISHRA_S_FACTORY,
            part: penta::CardPartId::PRIMARY,
            ability: penta::AbilityId::PRIMARY,
        },
        targets: vec![
            penta::TargetSelection::single(penta::TargetSlotId(3), Target::Player(PlayerId::Two)),
            penta::TargetSelection::new(
                penta::TargetSlotId(7),
                vec![
                    Target::Permanent(CardInstanceId(11)),
                    Target::Spell(CardInstanceId(12)),
                ],
            ),
        ],
        cost_object: None,
        x: 0,
    };
    assert_eq!(
        action_ability_origin(&action),
        Some(json!({
            "kind": "printed",
            "definition": penta::poc::cards::MISHRA_S_FACTORY.0,
            "partId": 0,
            "abilityId": 0,
        }))
    );
    assert_eq!(action_target_card(&action), Some(CardInstanceId(11)));
    assert_eq!(
        action_target_player(&action, PlayerId::One),
        Some("opponent")
    );
    assert_eq!(action_target_stack(&action), Some(12));
    assert_eq!(action_target_cards(&action), vec![11]);
    assert_eq!(
        action_target_players(&action, PlayerId::One),
        vec!["opponent"]
    );
    assert_eq!(action_target_stacks(&action), vec![12]);

    let mana_action = Action::ActivateManaAbility {
        source: CardInstanceId(9),
        ability: penta::AbilityOrigin::IntrinsicBasicLand(penta::BasicLandType::Mountain),
        color: penta::ManaColor::Red,
    };
    assert_eq!(
        action_ability_origin(&mana_action),
        Some(json!({
            "kind": "intrinsicBasicLand",
            "landType": "mountain",
        }))
    );

    let granted_action = Action::ActivateAbility {
        source: CardInstanceId(10),
        ability: penta::AbilityOrigin::Granted {
            source: CardInstanceId(9),
            source_definition: penta::CardDefinitionId(8),
            source_part: penta::CardPartId(1),
            source_ability: penta::AbilityId(2),
            grant: penta::GrantId(3),
        },
        targets: Vec::new(),
        cost_object: None,
        x: 0,
    };
    assert_eq!(
        action_ability_origin(&granted_action),
        Some(json!({
            "kind": "granted",
            "source": 9,
            "sourceDefinition": 8,
            "sourcePartId": 1,
            "sourceAbilityId": 2,
            "grantId": 3,
        }))
    );

    let special_action = Action::TakeSpecialAction {
        source: CardInstanceId(13),
        ability: penta::AbilityOrigin::Printed {
            definition: penta::CardDefinitionId(14),
            part: penta::CardPartId::PRIMARY,
            ability: penta::AbilityId(2),
        },
        effect_id: Some(12),
    };
    assert_eq!(action_card(&special_action), Some(CardInstanceId(13)));
    assert_eq!(
        action_ability_origin(&special_action),
        Some(json!({
            "kind": "printed",
            "definition": 14,
            "partId": 0,
            "abilityId": 2,
        }))
    );
    assert_animated_as_ability(&special_action);
}
