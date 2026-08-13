use super::*;

#[test]
fn green_creatures_get_their_land_bonuses_and_llanowar_elves_make_green() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::TAIGA, PlayerId::One),
        creature(10_001, cards::KIRD_APE, PlayerId::One),
        creature(10_002, cards::LLANOWAR_ELVES, PlayerId::One),
    ]);
    assert_eq!(game.power(&game.battlefield[1]), Some(2));
    assert_eq!(game.toughness(&game.battlefield[1]), Some(3));
    assert_eq!(
        game.mana_ability_activations(&game.battlefield[2])
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Green]
    );
}

#[test]
fn lands_derive_intrinsic_mana_in_effective_subtype_order() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::PLAINS, PlayerId::One),
        creature(10_001, cards::ISLAND, PlayerId::One),
        creature(10_002, cards::SWAMP, PlayerId::One),
        creature(10_003, cards::MOUNTAIN, PlayerId::One),
        creature(10_004, cards::FOREST, PlayerId::One),
        creature(10_005, cards::TAIGA, PlayerId::One),
    ]);

    for (index, (land_type, color)) in [
        (BasicLandType::Plains, ManaColor::White),
        (BasicLandType::Island, ManaColor::Blue),
        (BasicLandType::Swamp, ManaColor::Black),
        (BasicLandType::Mountain, ManaColor::Red),
        (BasicLandType::Forest, ManaColor::Green),
    ]
    .into_iter()
    .enumerate()
    {
        let activations = game.mana_ability_activations(&game.battlefield[index]);
        assert_eq!(activations.len(), 1);
        assert_eq!(activations[0].color, color);
        assert_eq!(
            activations[0].ability,
            AbilityOrigin::IntrinsicBasicLand(land_type)
        );
    }

    let taiga = game.mana_ability_activations(&game.battlefield[5]);
    assert_eq!(
        taiga
            .iter()
            .map(|activation| (activation.ability, activation.color))
            .collect::<Vec<_>>(),
        vec![
            (
                AbilityOrigin::IntrinsicBasicLand(BasicLandType::Forest),
                ManaColor::Green,
            ),
            (
                AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
                ManaColor::Red,
            ),
        ]
    );
}

#[test]
fn a_basic_land_subtype_only_grants_mana_to_a_land() {
    let definition_id = CardDefinitionId(10_000);
    let mut definition = CardDefinition::new(
        definition_id,
        "Forest creature",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_creature(ManaCost::default(), &["Forest"], 1, 1);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_000, definition_id, PlayerId::One));

    assert_eq!(game.effective_land_types(&game.battlefield[0]), [false; 5]);
    assert!(
        game.mana_ability_activations(&game.battlefield[0])
            .is_empty()
    );
}

#[test]
fn printed_and_intrinsic_mana_abilities_coexist() {
    static ABILITIES: [AbilityDef; 1] = [abilities::tap_for(ManaColor::Green)];
    let definition_id = CardDefinitionId(10_000);
    let mut definition = CardDefinition::new(
        definition_id,
        "Forest with printed mana",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_land(&["Forest"]).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_000, definition_id, PlayerId::One));

    assert_eq!(
        game.mana_ability_activations(&game.battlefield[0])
            .into_iter()
            .map(|activation| (activation.ability, activation.color))
            .collect::<Vec<_>>(),
        vec![
            (
                AbilityOrigin::Printed {
                    definition: definition_id,
                    part: CardPartId::PRIMARY,
                    ability: AbilityId::PRIMARY,
                },
                ManaColor::Green,
            ),
            (
                AbilityOrigin::IntrinsicBasicLand(BasicLandType::Forest),
                ManaColor::Green,
            ),
        ]
    );
}

#[test]
fn direct_and_composite_land_type_effects_grant_intrinsic_mana_in_order() {
    static DIRECT_TYPES: [BasicLandType; 1] = [BasicLandType::Mountain];
    static FIRST_COMPOSITE_TYPES: [BasicLandType; 1] = [BasicLandType::Forest];
    static SECOND_COMPOSITE_TYPES: [BasicLandType; 1] = [BasicLandType::Island];
    static COMPONENTS: [AppliedEffectDef; 2] = [
        AppliedEffectDef::AddLandTypes(&FIRST_COMPOSITE_TYPES),
        AppliedEffectDef::AddLandTypes(&SECOND_COMPOSITE_TYPES),
    ];
    static EFFECTS: [EffectDef; 2] = [
        EffectDef::Apply {
            recipient: EffectRecipientDef::AttachedPermanent,
            effect: AppliedEffectDef::AddLandTypes(&DIRECT_TYPES),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
        EffectDef::Apply {
            recipient: EffectRecipientDef::AttachedPermanent,
            effect: AppliedEffectDef::Composite(&COMPONENTS),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    ];
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
        "Enchanted land is a Mountain, Forest, and Island in addition to its other types.",
        EffectDef::Sequence(&EFFECTS),
    )];

    let definition_id = CardDefinitionId(10_081);
    let mut definition = CardDefinition::new(
        definition_id,
        "Composite land-type test Aura",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_enchantment(ManaCost::new(0, 0)).with_abilities(&ABILITIES);
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
    let land_id = CardInstanceId(10_000);
    let mut aura = creature(10_001, definition_id, PlayerId::One);
    aura.attached_to = Some(land_id);
    game.battlefield.extend([
        creature(land_id.0, cards::THESPIANS_STAGE, PlayerId::One),
        aura,
    ]);

    assert_eq!(
        game.effective_subtypes(&game.battlefield[0]).as_ref(),
        &["Mountain", "Forest", "Island"],
    );
    assert_eq!(
        game.mana_ability_activations(&game.battlefield[0])
            .into_iter()
            .filter_map(|activation| match activation.ability {
                AbilityOrigin::IntrinsicBasicLand(land_type) => {
                    Some((land_type, activation.color))
                }
                AbilityOrigin::Printed { .. } | AbilityOrigin::Granted { .. } => None,
            })
            .collect::<Vec<_>>(),
        vec![
            (BasicLandType::Mountain, ManaColor::Red),
            (BasicLandType::Forest, ManaColor::Green),
            (BasicLandType::Island, ManaColor::Blue),
        ],
    );
}

#[test]
fn blood_moon_replaces_nonbasic_land_abilities_with_intrinsic_red_mana() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::BLOOD_MOON, PlayerId::One),
        creature(10_001, cards::CITY_OF_BRASS, PlayerId::One),
        creature(10_002, cards::MISHRA_S_WORKSHOP, PlayerId::One),
        creature(10_003, cards::TAIGA, PlayerId::One),
    ]);

    for permanent in &game.battlefield[1..] {
        assert_eq!(
            game.effective_land_types(permanent),
            [false, false, false, true, false]
        );
        let activations = game.mana_ability_activations(permanent);
        assert_eq!(activations.len(), 1);
        assert_eq!(activations[0].color, ManaColor::Red);
        assert_eq!(
            activations[0].ability,
            AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain)
        );
        assert!(activations[0].effect.restrictions.is_empty());
        assert!(
            game.effective_behavior(permanent).is_none(),
            "Blood Moon grants intrinsic rules, not a special-behavior hook"
        );
    }
}

#[test]
fn blood_moon_suppresses_nonbasic_lands_own_entry_replacements() {
    for definition in [cards::TEMPLE_GARDEN, cards::CLIFFTOP_RETREAT] {
        let mut game = ready_game();
        game.catalog = crate::card::catalog().unwrap();
        game.battlefield
            .push(creature(9_999, cards::BLOOD_MOON, PlayerId::Two));
        let land = card(10_000, definition, PlayerId::One);
        game.players[0].hand.push(land.clone());
        let event_start = game.events().len();

        game.apply(
            PlayerId::One,
            Action::PlayLand {
                card: land.id,
                option: PlayOptionId::DEFAULT,
            },
        )
        .unwrap();

        assert!(
            game.pending_decisions.is_empty(),
            "Blood Moon removes the printed as-enters ability before it applies"
        );
        assert_eq!(game.players[0].life, i16::from(rules::STARTING_LIFE));
        assert!(
            game.events()[event_start..]
                .iter()
                .all(|event| !matches!(event, GameEvent::LifeLost { .. }))
        );
        let entered = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == definition)
            .expect("the nonbasic land committed");
        assert!(!entered.tapped);
        assert_eq!(
            game.effective_land_types(entered),
            [false, false, false, true, false]
        );
        assert_eq!(
            game.mana_ability_activations(entered)
                .into_iter()
                .map(|activation| (activation.ability, activation.color))
                .collect::<Vec<_>>(),
            vec![(
                AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
                ManaColor::Red,
            )]
        );
    }
}

#[test]
fn blood_moon_preserves_nonland_subtypes_on_a_land_creature() {
    let definition_id = CardDefinitionId(10_000);
    let mut definition = CardDefinition::new(
        definition_id,
        "Forest Dryad",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_creature_without_mana_cost(
        &["Forest", "Gate", "Cave", "Locus", "Dryad"],
        1,
        1,
    )
    .with_type(CardType::Land);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let blood_moon = game.catalog.get(cards::BLOOD_MOON).unwrap().clone();
    game.catalog = CardCatalog::new([blood_moon, definition]).unwrap();
    game.turns_started[PlayerId::One.index()] = 1;
    game.battlefield.extend([
        creature(10_000, cards::BLOOD_MOON, PlayerId::One),
        creature(10_001, definition_id, PlayerId::One),
    ]);

    let permanent = &game.battlefield[1];
    let event = game.trigger_event_object(permanent);
    assert!(event.types.contains(CardType::Land));
    assert!(event.types.contains(CardType::Creature));
    assert_eq!(event.subtypes.as_ref(), &["Mountain", "Dryad"]);
    assert_eq!(
        game.mana_ability_activations(permanent)
            .into_iter()
            .map(|activation| (activation.ability, activation.color))
            .collect::<Vec<_>>(),
        vec![(
            AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
            ManaColor::Red,
        )]
    );
}

#[test]
fn dryad_arbor_is_a_green_land_creature_with_summoning_sick_intrinsic_mana() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 0;
    let arbor_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(arbor_id.0, cards::DRYAD_ARBOR, PlayerId::One));

    let arbor = &game.battlefield[0];
    let types = game.permanent_types(arbor).unwrap();
    assert!(types.contains(CardType::Land));
    assert!(types.contains(CardType::Creature));
    assert_eq!(
        game.effective_subtypes(arbor).as_ref(),
        &["Forest", "Dryad"]
    );
    assert_eq!(
        game.effective_rules(arbor).unwrap().colors(),
        [false, false, false, false, true]
    );
    assert_eq!(
        (game.power(arbor), game.toughness(arbor)),
        (Some(1), Some(1))
    );
    assert!(
        game.mana_ability_activations(arbor).is_empty(),
        "Dryad Arbor's intrinsic tap ability observes summoning sickness",
    );

    game.turns_started[PlayerId::One.index()] = 1;
    assert_eq!(
        game.mana_ability_activations(&game.battlefield[0])
            .into_iter()
            .map(|activation| (activation.ability, activation.color))
            .collect::<Vec<_>>(),
        vec![(
            AbilityOrigin::IntrinsicBasicLand(BasicLandType::Forest),
            ManaColor::Green,
        )],
    );
}

#[test]
fn magical_hack_changes_a_land_type_and_its_intrinsic_mana_but_preserves_dryad() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let arbor_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(arbor_id.0, cards::DRYAD_ARBOR, PlayerId::One));
    let hack = card(10_001, cards::MAGICAL_HACK, PlayerId::One);
    game.players[0].hand.push(hack.clone());
    game.players[0].mana_pool.blue = 1;

    let cast = cast_action(hack.id, vec![Target::Permanent(arbor_id)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Forest → Island");

    let arbor = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == arbor_id)
        .unwrap();
    assert_eq!(
        game.effective_subtypes(arbor).as_ref(),
        &["Island", "Dryad"]
    );
    assert_eq!(
        game.mana_ability_activations(arbor)
            .into_iter()
            .map(|activation| (activation.ability, activation.color))
            .collect::<Vec<_>>(),
        vec![(
            AbilityOrigin::IntrinsicBasicLand(BasicLandType::Island),
            ManaColor::Blue,
        )],
    );
}

#[test]
fn magical_hack_can_target_a_nonland_permanent_without_basic_land_type_words() {
    let mut game = ready_game();
    let lotus_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(lotus_id.0, cards::BLACK_LOTUS, PlayerId::Two));
    let hack = card(10_001, cards::MAGICAL_HACK, PlayerId::One);
    game.players[0].hand.push(hack.clone());
    game.players[0].mana_pool.blue = 1;

    let cast = cast_action(hack.id, vec![Target::Permanent(lotus_id)], Vec::new(), 0);
    assert!(
        game.legal_actions(PlayerId::One).contains(&cast),
        "a nonland permanent is a legal target even when it has no words to replace",
    );
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Forest → Island");

    let lotus = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == lotus_id)
        .expect("the unchanged target remains on the battlefield");
    assert_eq!(
        lotus.text_changes,
        vec![BasicLandTypeChange {
            from: BasicLandType::Forest,
            to: BasicLandType::Island,
        }],
    );
    assert_eq!(
        game.permanent_types(lotus),
        Some(CardTypeSet::single(CardType::Artifact)),
    );
    assert!(game.effective_subtypes(lotus).is_empty());
}

#[test]
fn magical_hack_can_change_a_permanent_spell_and_the_change_survives_resolution() {
    let mut game = ready_game();
    let lotus_id = StackObjectId(10_000);
    game.stack
        .push(spell(lotus_id.0, cards::BLACK_LOTUS, PlayerId::One, 0));
    let hack = card(10_001, cards::MAGICAL_HACK, PlayerId::One);
    game.players[0].hand.push(hack.clone());
    game.players[0].mana_pool.blue = 1;

    let cast = cast_action(hack.id, vec![Target::Spell(lotus_id)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Forest → Island");

    assert_eq!(
        game.stack
            .iter()
            .find(|object| object.id == lotus_id)
            .expect("the permanent spell remains on the stack")
            .text_changes,
        vec![BasicLandTypeChange {
            from: BasicLandType::Forest,
            to: BasicLandType::Island,
        }],
    );

    pass_priority_pair(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::BLACK_LOTUS)
            .expect("the permanent spell resolved")
            .text_changes,
        vec![BasicLandTypeChange {
            from: BasicLandType::Forest,
            to: BasicLandType::Island,
        }],
    );
}

#[test]
fn magical_hack_fizzles_without_a_choice_when_its_permanent_target_leaves() {
    let mut game = ready_game();
    let land_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(land_id.0, cards::MOUNTAIN, PlayerId::One));
    let hack = card(10_001, cards::MAGICAL_HACK, PlayerId::One);
    game.players[0].hand.push(hack.clone());
    game.players[0].mana_pool.blue = 1;

    game.apply(
        PlayerId::One,
        cast_action(hack.id, vec![Target::Permanent(land_id)], Vec::new(), 0),
    )
    .unwrap();
    game.destroy_permanent(land_id);
    pass_priority_pair(&mut game);

    assert!(game.pending_decisions.is_empty());
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MAGICAL_HACK),
    );
}

#[test]
fn magical_hack_on_stage_applies_to_land_types_that_stage_later_copies() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let stage_id = CardInstanceId(10_000);
    let arbor_id = CardInstanceId(10_001);
    game.battlefield.extend([
        creature(stage_id.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(arbor_id.0, cards::DRYAD_ARBOR, PlayerId::Two),
    ]);
    let hack = card(10_002, cards::MAGICAL_HACK, PlayerId::One);
    game.players[0].hand.push(hack.clone());
    game.players[0].mana_pool.blue = 1;
    game.apply(
        PlayerId::One,
        cast_action(hack.id, vec![Target::Permanent(stage_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Forest → Island");

    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: stage_id,
            ability: activated_ability_for(&game, stage_id, 0),
            targets: activated_targets(Target::Permanent(arbor_id)),
            cost_object: None,
            x: 0,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    game.battlefield[0].tapped = false;

    let stage = &game.battlefield[0];
    assert_eq!(
        game.effective_subtypes(stage).as_ref(),
        &["Island", "Dryad"]
    );
    assert_eq!(
        game.mana_ability_activations(stage)
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Blue],
    );
}

#[test]
fn magical_hack_does_not_rewrite_land_types_added_by_presence() {
    let mut game = ready_game();
    let land_id = CardInstanceId(10_000);
    let mut land = creature(land_id.0, cards::MOUNTAIN, PlayerId::One);
    land.text_changes.push(BasicLandTypeChange {
        from: BasicLandType::Mountain,
        to: BasicLandType::Island,
    });
    let mut presence = creature(10_001, cards::NYLEAS_PRESENCE, PlayerId::One);
    presence.attached_to = Some(land_id);
    game.battlefield.extend([land, presence]);

    assert_eq!(game.effective_land_types(&game.battlefield[0]), [true; 5]);
    let colors = game
        .mana_ability_activations(&game.battlefield[0])
        .into_iter()
        .map(|activation| activation.color)
        .collect::<Vec<_>>();
    assert_eq!(colors.len(), 5);
    for expected in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ] {
        assert!(colors.contains(&expected));
    }
}

#[test]
fn magical_hack_deduplicates_basic_types_and_intrinsic_mana() {
    let mut game = ready_game();
    let mut taiga = creature(10_000, cards::TAIGA, PlayerId::One);
    taiga.text_changes.push(BasicLandTypeChange {
        from: BasicLandType::Forest,
        to: BasicLandType::Mountain,
    });
    game.battlefield.push(taiga);

    assert_eq!(
        game.effective_subtypes(&game.battlefield[0]).as_ref(),
        &["Mountain"],
    );
    assert_eq!(
        game.mana_ability_activations(&game.battlefield[0])
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Red],
    );
}

#[test]
fn nyleas_presence_attaches_draws_and_adds_all_five_intrinsic_abilities() {
    let mut game = ready_game();
    let land_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(land_id.0, cards::THESPIANS_STAGE, PlayerId::One));
    let presence = card(10_001, cards::NYLEAS_PRESENCE, PlayerId::One);
    game.players[0].hand.push(presence.clone());
    game.players[0].mana_pool.colorless = 1;
    game.players[0].mana_pool.green = 1;
    let library_before = game.players[0].library.len();

    let cast = cast_action(presence.id, vec![Target::Permanent(land_id)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let aura_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::NYLEAS_PRESENCE)
        .expect("Nylea's Presence entered")
        .card
        .id;
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == aura_id)
            .unwrap()
            .attached_to,
        Some(land_id),
    );
    assert_eq!(game.effective_land_types(&game.battlefield[0]), [true; 5]);
    assert_eq!(
        game.mana_ability_activations(&game.battlefield[0])
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![
            ManaColor::Colorless,
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
        ],
    );

    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].library.len(), library_before - 1);

    game.destroy_permanent(aura_id);
    assert_eq!(game.effective_land_types(&game.battlefield[0]), [false; 5]);
    assert_eq!(
        game.mana_ability_activations(&game.battlefield[0])
            .into_iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Colorless],
    );
}

#[test]
fn blood_moon_and_presence_apply_land_type_operations_in_timestamp_order() {
    let target = CardInstanceId(10_001);

    let mut moon_then_presence = ready_game();
    let mut newer_presence = creature(10_002, cards::NYLEAS_PRESENCE, PlayerId::One);
    newer_presence.attached_to = Some(target);
    moon_then_presence.battlefield.extend([
        creature(10_000, cards::BLOOD_MOON, PlayerId::One),
        creature(target.0, cards::THESPIANS_STAGE, PlayerId::One),
        newer_presence,
    ]);
    assert_eq!(
        moon_then_presence.effective_land_types(&moon_then_presence.battlefield[1]),
        [true; 5],
        "a newer additive effect applies after Blood Moon's set effect",
    );

    let mut presence_then_moon = ready_game();
    let mut older_presence = creature(10_000, cards::NYLEAS_PRESENCE, PlayerId::One);
    older_presence.attached_to = Some(target);
    presence_then_moon.battlefield.extend([
        older_presence,
        creature(target.0, cards::THESPIANS_STAGE, PlayerId::One),
        creature(10_002, cards::BLOOD_MOON, PlayerId::One),
    ]);
    assert_eq!(
        presence_then_moon.effective_land_types(&presence_then_moon.battlefield[1]),
        [false, false, false, true, false],
        "a newer Blood Moon set effect overwrites Presence's earlier additions",
    );
}

#[test]
fn an_aura_with_an_illegal_land_target_neither_enters_nor_draws() {
    let mut game = ready_game();
    let land_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(land_id.0, cards::MOUNTAIN, PlayerId::One));
    let presence = card(10_001, cards::NYLEAS_PRESENCE, PlayerId::One);
    game.players[0].hand.push(presence.clone());
    game.players[0].mana_pool.colorless = 1;
    game.players[0].mana_pool.green = 1;
    let library_before = game.players[0].library.len();

    game.apply(
        PlayerId::One,
        cast_action(presence.id, vec![Target::Permanent(land_id)], Vec::new(), 0),
    )
    .unwrap();
    game.destroy_permanent(land_id);
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::NYLEAS_PRESENCE),
    );
    assert_eq!(game.players[0].library.len(), library_before);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::NYLEAS_PRESENCE),
    );
}

#[test]
fn presence_goes_to_the_graveyard_when_its_attached_land_leaves() {
    let mut game = ready_game();
    let land_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(land_id.0, cards::MOUNTAIN, PlayerId::One));
    let presence = card(10_001, cards::NYLEAS_PRESENCE, PlayerId::One);
    game.players[0].hand.push(presence.clone());
    game.players[0].mana_pool.colorless = 1;
    game.players[0].mana_pool.green = 1;
    game.apply(
        PlayerId::One,
        cast_action(presence.id, vec![Target::Permanent(land_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let aura_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::NYLEAS_PRESENCE)
        .unwrap()
        .card
        .id;
    game.destroy_permanent(land_id);
    game.check_state_based_actions();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != aura_id),
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::NYLEAS_PRESENCE),
    );
}
