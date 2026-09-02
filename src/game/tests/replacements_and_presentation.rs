use super::*;

#[test]
fn deterministic_land_entry_replacements_use_object_queries() {
    for (qualifier, expected_tapped) in [
        (None, true),
        (Some((PlayerId::Two, PlayerId::One)), false),
        (Some((PlayerId::One, PlayerId::Two)), true),
    ] {
        let mut game = ready_game();
        game.catalog = crate::card::catalog().unwrap();
        if let Some((owner, controller)) = qualifier {
            game.battlefield.push(Permanent::entering(
                card(9_999, cards::PLAINS, owner),
                CardPartId::PRIMARY,
                controller,
                0,
                0,
            ));
        }
        let retreat = card(10_000, cards::CLIFFTOP_RETREAT, PlayerId::One);
        game.players[0].hand.push(retreat.clone());
        game.apply(
            PlayerId::One,
            Action::PlayLand {
                card: retreat.id,
                option: PlayOptionId::DEFAULT,
            },
        )
        .unwrap();

        let retreat = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::CLIFFTOP_RETREAT)
            .expect("the check land committed");
        assert_eq!(retreat.tapped, expected_tapped);
        assert!(game.pending_decisions.is_empty());
    }

    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    let guildgate = card(10_001, cards::GOLGARI_GUILDGATE, PlayerId::One);
    game.players[0].hand.push(guildgate.clone());
    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: guildgate.id,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::GOLGARI_GUILDGATE)
            .expect("the guildgate committed")
            .tapped
    );
}

#[test]
fn check_land_queries_use_land_types_added_by_static_effects() {
    for with_presence in [false, true] {
        let mut game = ready_game();
        game.catalog = crate::card::catalog().unwrap();
        let land_id = CardInstanceId(9_998);
        game.battlefield
            .push(creature(land_id.0, cards::THESPIANS_STAGE, PlayerId::One));
        if with_presence {
            let mut presence = creature(9_999, cards::NYLEAS_PRESENCE, PlayerId::One);
            presence.attached_to = Some(land_id);
            game.battlefield.push(presence);
        }

        assert_eq!(
            game.effective_land_types(&game.battlefield[0]),
            if with_presence { [true; 5] } else { [false; 5] }
        );
        let retreat = card(10_000, cards::CLIFFTOP_RETREAT, PlayerId::One);
        game.players[0].hand.push(retreat.clone());
        game.apply(
            PlayerId::One,
            Action::PlayLand {
                card: retreat.id,
                option: PlayOptionId::DEFAULT,
            },
        )
        .unwrap();

        assert_eq!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.definition == cards::CLIFFTOP_RETREAT)
                .expect("the check land committed")
                .tapped,
            !with_presence,
            "the condition uses the controlled land's effective basic land types"
        );
    }
}

#[test]
fn an_entering_permanents_own_static_ability_can_grant_its_entry_replacement() {
    let definition_id = CardDefinitionId::new(10_101);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test self-granted entry replacement",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_land(&[]).with_abilities(&TEST_SELF_GRANTED_ENTRY_ABILITY);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let land = card(10_000, definition_id, PlayerId::One);
    game.players[0].hand.push(land.clone());
    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: land.id,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.definition == definition_id)
            .expect("the land committed")
            .tapped,
        "prospective characteristics include the entrant's own static grants"
    );
}

#[test]
fn an_entering_permanents_own_static_land_types_match_external_replacements() {
    let external_id = CardDefinitionId::new(10_101);
    let land_id = CardDefinitionId::new(10_102);
    let mut external = CardDefinition::new(
        external_id,
        "Test Plains entry restriction",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    external.rules = CardRules::new_enchantment(ManaCost::default())
        .with_abilities(&TEST_PLAINS_ENTER_TAPPED_ABILITY);
    synchronize_single_part_definition(&mut external);
    let mut land = CardDefinition::new(
        land_id,
        "Test self-typed land",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    land.rules = CardRules::new_land(&[]).with_abilities(&TEST_SELF_PLAINS_ABILITY);
    synchronize_single_part_definition(&mut land);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([external, land]).unwrap();
    game.battlefield
        .push(creature(9_999, external_id, PlayerId::Two));
    let land = card(10_000, land_id, PlayerId::One);
    game.players[0].hand.push(land.clone());
    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: land.id,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();

    let entered = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == land_id)
        .expect("the land committed");
    assert!(entered.tapped);
    assert_eq!(
        game.effective_land_types(entered),
        [true, false, false, false, false]
    );
}

#[test]
fn an_entering_static_effect_does_not_change_existing_replacement_sources_early() {
    let source_id = CardDefinitionId::new(10_101);
    let mut source = CardDefinition::new(
        source_id,
        "Test nonbasic replacement source",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    source.rules =
        CardRules::new_land(&[]).with_abilities(&TEST_OPPONENT_ENCHANTMENTS_ENTER_TAPPED_ABILITY);
    synchronize_single_part_definition(&mut source);

    let mut game = ready_game();
    let blood_moon = game.catalog.get(cards::BLOOD_MOON).unwrap().clone();
    game.catalog = CardCatalog::new([source, blood_moon]).unwrap();
    game.battlefield
        .push(creature(9_999, source_id, PlayerId::Two));

    game.put_onto_battlefield(PlayerId::One, cards::BLOOD_MOON)
        .expect("Blood Moon is in the focused catalog");

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::BLOOD_MOON)
            .expect("Blood Moon committed")
            .tapped,
        "Blood Moon does not remove an existing nonbasic source's ability before it enters"
    );
}

#[test]
fn a_land_play_option_locks_the_presented_part_on_the_permanent() {
    let definition_id = CardDefinitionId::new(10_100);
    let land_part = CardPartId(1);
    let land_option = PlayOptionId(1);
    let front_rules = CardRules::new_sorcery(ManaCost::new(1, 0));
    let land_rules =
        CardRules::new_land(&[]).with_ability(abilities::enters_tapped("This land enters tapped."));
    let mut definition = CardDefinition::new(
        definition_id,
        "Test modal card",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = front_rules;
    definition.parts = vec![
        CardPart::new(CardPartId::PRIMARY, "Test front", front_rules),
        CardPart::new(land_part, "Test back", land_rules),
    ];
    definition.structure = CardStructure::DoubleFaced {
        front: CardPartId::PRIMARY,
        back: land_part,
        kind: DoubleFacedKind::Modal,
    };
    definition.play_options = vec![
        PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Cast Test front",
            SpellForm::Part(CardPartId::PRIMARY),
            front_rules
                .mana_cost()
                .expect("the front has a printed mana cost"),
            CardEffectStatus::Unsupported,
        ),
        PlayOptionDef::play_land(
            land_option,
            "Play Test back",
            land_part,
            CardEffectStatus::Implemented,
        ),
    ];

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let card = card(10_100, definition_id, PlayerId::One);
    let action = Action::PlayLand {
        card: card.id,
        option: land_option,
    };
    game.players[0].hand.push(card);

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();

    assert_eq!(game.battlefield[0].presented, land_part);
    assert!(game.battlefield[0].tapped);
}

#[test]
fn a_modal_spell_resolves_by_its_locked_part_instead_of_the_canonical_front() {
    let definition_id = CardDefinitionId::new(10_150);
    let creature_part = CardPartId(1);
    let creature_option = PlayOptionId(1);
    let front_rules = CardRules::new_instant(ManaCost::new(1, 1));
    let creature_rules = CardRules::new_creature(ManaCost::new(0, 0), &[], 3, 4)
        .with_abilities(&TEST_FLYING_ABILITY);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test modal spell",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = front_rules;
    definition.parts = vec![
        CardPart::new(CardPartId::PRIMARY, "Test front", front_rules),
        CardPart::new(creature_part, "Test creature back", creature_rules),
    ];
    definition.structure = CardStructure::DoubleFaced {
        front: CardPartId::PRIMARY,
        back: creature_part,
        kind: DoubleFacedKind::Modal,
    };
    definition.play_options = vec![
        PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Cast Test front",
            SpellForm::Part(CardPartId::PRIMARY),
            front_rules
                .mana_cost()
                .expect("the front has a printed mana cost"),
            CardEffectStatus::Unsupported,
        ),
        PlayOptionDef::cast(
            creature_option,
            "Cast Test creature back",
            SpellForm::Part(creature_part),
            creature_rules
                .mana_cost()
                .expect("the modal back has a printed mana cost"),
            CardEffectStatus::Implemented,
        ),
    ];

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let card = card(10_150, definition_id, PlayerId::One);
    let hand_id = card.id;
    game.players[0].hand.push(card);
    let action = Action::CastSpell {
        card: hand_id,
        choices: CastChoices::new(creature_option),
        sacrifices: Vec::new(),
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    let spell_id = game.stack[0].id;
    pass_priority_pair(&mut game);

    let permanent = &game.battlefield[0];
    assert_ne!(permanent.card.id, spell_id);
    assert_eq!(permanent.presented, creature_part);
    assert_eq!(game.power(permanent), Some(3));
    assert_eq!(game.toughness(permanent), Some(4));
    assert!(game.has_flying(permanent));
}

#[test]
fn changing_a_permanents_presented_face_keeps_its_object_identity() {
    let definition_id = CardDefinitionId::new(10_101);
    let back = CardPartId(1);
    let front_rules = CardRules::new_creature(ManaCost::new(2, 0), &[], 2, 2);
    let back_rules = CardRules::new_creature_without_mana_cost(&[], 4, 5)
        .with_abilities(&TEST_FLYING_TRAMPLE_ABILITIES);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test Werewolf",
        CardSet::Innistrad,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = front_rules;
    definition.parts = vec![
        CardPart::new(CardPartId::PRIMARY, "Test Werewolf", front_rules),
        CardPart::new(back, "Test Ravager", back_rules),
    ];
    definition.structure = CardStructure::DoubleFaced {
        front: CardPartId::PRIMARY,
        back,
        kind: DoubleFacedKind::Transforming,
    };
    definition.play_options = vec![PlayOptionDef::cast(
        PlayOptionId::DEFAULT,
        "Cast Test Werewolf",
        SpellForm::Part(CardPartId::PRIMARY),
        front_rules
            .mana_cost()
            .expect("the front has a printed mana cost"),
        CardEffectStatus::Unsupported,
    )];

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let permanent_id = GameObjectId(10_101);
    game.battlefield
        .push(creature(permanent_id.0, definition_id, PlayerId::One));

    let front = &game.observe(PlayerId::One).battlefield[0];
    assert_eq!(front.id, permanent_id);
    assert_eq!(front.characteristics.part(), CardPartId::PRIMARY);
    assert_eq!(
        (front.power, front.toughness, front.flying),
        (Some(2), Some(2), false)
    );

    game.battlefield[0].presented = back;

    let transformed = &game.observe(PlayerId::One).battlefield[0];
    assert_eq!(transformed.id, permanent_id);
    assert_eq!(transformed.characteristics.part(), back);
    assert_eq!(
        (transformed.power, transformed.toughness, transformed.flying),
        (Some(4), Some(5), true),
    );
    assert!(game.has_trample(&game.battlefield[0]));

    game.return_permanent_to_hand(permanent_id);
    let returned_id = game.players[0].hand[0].id;
    assert_ne!(returned_id, permanent_id);
}

#[test]
fn city_in_a_bottle_stops_arabian_nights_cards_being_played() {
    // The prohibition is about where a card was printed, not who holds it, so
    // it binds the Bottle's own controller too -- including a second Bottle.
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::CITY_IN_A_BOTTLE, PlayerId::One));
    game.players[0].hand.extend([
        card(10_001, cards::KIRD_APE, PlayerId::One),
        card(10_002, cards::CITY_IN_A_BOTTLE, PlayerId::One),
        card(10_003, cards::SAVANNAH_LIONS, PlayerId::One),
        card(10_004, cards::CITY_OF_BRASS, PlayerId::One),
        card(10_005, cards::PLAINS, PlayerId::One),
    ]);
    game.players[0].mana_pool = ManaPool {
        white: 3,
        red: 3,
        colorless: 3,
        ..ManaPool::default()
    };

    let playable = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, .. } | Action::PlayLand { card, .. } => Some(card),
            _ => None,
        })
        .collect::<std::collections::HashSet<_>>();

    assert!(
        !playable.contains(&GameObjectId(10_001)),
        "Kird Ape is bottled"
    );
    assert!(
        !playable.contains(&GameObjectId(10_002)),
        "and so is a second City in a Bottle"
    );
    assert!(
        !playable.contains(&GameObjectId(10_004)),
        "the land is bottled too, not only the spells"
    );
    assert!(
        playable.contains(&GameObjectId(10_003)),
        "a card from another expansion is unaffected"
    );
    assert!(playable.contains(&GameObjectId(10_005)));

    assert!(
        game.apply(
            PlayerId::One,
            cast_action(GameObjectId(10_001), Vec::new(), Vec::new(), 0),
        )
        .is_err(),
        "and submitting the cast directly is refused too"
    );

    attach_constant_resolved_characteristics(
        &mut game,
        GameObjectId(10_000),
        &[AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any)],
        ContinuousEffectExpiration::Never,
    );
    let playable = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, .. } | Action::PlayLand { card, .. } => Some(card),
            _ => None,
        })
        .collect::<std::collections::HashSet<_>>();
    assert!(
        playable.contains(&GameObjectId(10_001)),
        "removing the source's static ability removes its live player restriction",
    );
}

#[test]
fn city_in_a_bottle_uses_canonical_origin_even_when_a_reprint_exists() {
    let mut game = ready_game();
    // Kird Ape debuted in Arabian Nights; a later printing does not move it.
    game.catalog = CardCatalog::with_additional_printings(
        game.catalog.definitions().into_iter().cloned(),
        [CardPrinting::new(cards::KIRD_APE, CardSet::Magic2014)],
    )
    .unwrap();
    game.battlefield
        .push(creature(10_000, cards::CITY_IN_A_BOTTLE, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::KIRD_APE, PlayerId::Two));
    // A card from another expansion is untouched, and so is the Bottle.
    game.battlefield
        .push(creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two));

    game.check_state_based_actions();
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.definition)
            .collect::<Vec<_>>(),
        vec![cards::CITY_IN_A_BOTTLE, cards::SAVANNAH_LIONS],
        "only the Arabian Nights card went, and the Bottle spared itself"
    );
    assert_eq!(
        game.players[1]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::KIRD_APE],
        "its controller sacrificed it, so it went to their graveyard"
    );
}

#[test]
fn baseline_and_supported_split_card_play_options_are_offered() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.players[0].hand.extend([
        card(10_000, crate::card::cards::DOOM_BLADE, PlayerId::One),
        card(10_001, crate::card::cards::PITHING_NEEDLE, PlayerId::One),
        card(10_002, crate::card::cards::DOMRI_RADE, PlayerId::One),
        card(10_003, crate::card::cards::LOXODON_SMITER, PlayerId::One),
        card(10_004, crate::card::cards::CLIFFTOP_RETREAT, PlayerId::One),
        card(10_005, crate::card::cards::IZZET_CHARM, PlayerId::One),
        card(10_006, crate::card::cards::TURN_BURN, PlayerId::One),
    ]);
    game.players[0].mana_pool = ManaPool {
        white: 4,
        blue: 4,
        black: 4,
        red: 4,
        green: 4,
        colorless: 4,
    };

    let actions = game.legal_actions(PlayerId::One);
    let cast_cards = actions
        .iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, .. } => Some(*card),
            _ => None,
        })
        .collect::<Vec<_>>();

    // Izzet Charm is complete; its loot mode needs no target and is castable
    // on an empty board.
    // Doom Blade has no creature to target. Turn // Burn is now executable;
    // its separately castable forms contribute two legal play options here.
    assert_eq!(
        cast_cards,
        vec![
            CardInstanceId(10_001),
            CardInstanceId(10_002),
            CardInstanceId(10_003),
            CardInstanceId(10_005),
            CardInstanceId(10_006),
            CardInstanceId(10_006),
        ]
    );
    assert!(actions.contains(&Action::PlayLand {
        card: CardInstanceId(10_004),
        option: PlayOptionId::DEFAULT,
    }));
}
