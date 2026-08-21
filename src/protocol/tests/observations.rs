use super::*;

fn test_token() -> crate::TokenCharacteristics {
    static TEST_TOKEN: crate::TokenCharacteristics =
        crate::TokenCharacteristics::artifact_creature(&["Servo"], &[], 1, 1)
            .with_name("Test Servo")
            .with_art(crate::CardArt::new(
                "00000000-0000-0000-0000-000000000001",
                "Test Artist",
            ));
    TEST_TOKEN
}

fn observation_with_printed_and_token_permanents() -> PlayerObservation {
    PlayerObservation {
        viewer: PlayerId::One,
        turn: 1,
        active_turn: 1,
        active_player: PlayerId::One,
        priority: PlayerId::One,
        step: crate::game::Step::CombatDamage,
        regular_combat_damage_pending: true,
        energy_counters: [0, 0],
        poison_counters: [0, 0],
        monarch: None,
        life_totals: [20, 20],
        mana_pools: [crate::ManaPool::default(); 2],
        hand: Vec::new(),
        opponent_hand_size: 0,
        last_seen_hand: None,
        library_sizes: [0, 0],
        revealed_library_top: None,
        graveyards: [Vec::new(), Vec::new()],
        exiles: [Vec::new(), Vec::new()],
        face_down_exile_sizes: [0, 0],
        emblems: Vec::new(),
        battlefield: vec![
            crate::game::PermanentObservation {
                id: GameObjectId(30),
                characteristics: crate::ObjectCharacteristics::card(
                    crate::card::cards::HUNTMASTER_OF_THE_FELLS,
                    crate::CardPartId(1),
                ),
                token: false,
                controller: PlayerId::One,
                face_down: false,
                physical_face: Some(crate::PhysicalFaceObservation {
                    kind: crate::DoubleFacedKind::Transforming,
                    side: crate::PhysicalFaceSide::Back,
                }),
                phased_out: false,
                types: crate::CardTypeSet::single(crate::CardType::Creature),
                chosen_creature_type: Some("Werewolf".into()),
                chosen_card_name: None,
                tapped: false,
                power: Some(4),
                toughness: Some(4),
                damage: 0,
                loyalty: None,
                loyalty_ability_used_this_turn: false,
                attack_defender: None,
                attacking: false,
                blocked_this_combat: false,
                blocking: Vec::new(),
                blocking_this_combat: false,
                attacking_band: None,
                flying: false,
                can_attack: true,
                entered_this_turn: false,
            },
            crate::game::PermanentObservation {
                id: GameObjectId(31),
                characteristics: crate::ObjectCharacteristics::token(
                    test_token(),
                    crate::CardPartId::PRIMARY,
                ),
                token: true,
                controller: PlayerId::One,
                face_down: false,
                physical_face: None,
                phased_out: false,
                types: crate::CardTypeSet::single(crate::CardType::Artifact)
                    .with(crate::CardType::Creature),
                chosen_creature_type: None,
                chosen_card_name: None,
                tapped: false,
                power: Some(1),
                toughness: Some(1),
                damage: 0,
                loyalty: None,
                loyalty_ability_used_this_turn: false,
                attack_defender: None,
                attacking: false,
                blocked_this_combat: false,
                blocking: Vec::new(),
                blocking_this_combat: false,
                attacking_band: None,
                flying: false,
                can_attack: true,
                entered_this_turn: true,
            },
        ],
        stack: Vec::new(),
        decision: None,
        result: None,
        legal_actions: Vec::new(),
        checkpoint: json!({}),
    }
}

#[test]
fn observation_json_carries_interwave_state_and_presented_card_part() {
    let catalog = poc::catalog().expect("catalog builds");
    let observation = observation_with_printed_and_token_permanents();
    let value =
        observation_json_for_format(&catalog, Format::IsdDgmStandard, &observation, false, &[]);
    assert_eq!(value["regularCombatDamagePending"], true);
    assert_eq!(value["battlefield"][0]["objectId"], 30);
    assert_eq!(value["battlefield"][0]["presentedPartId"], 1);
    assert_eq!(
        value["battlefield"][0]["characteristics"]["kind"],
        "printed"
    );
    assert_eq!(value["battlefield"][0]["characteristics"]["partId"], 1);
    assert_eq!(
        value["battlefield"][0]["physicalFace"],
        json!({ "kind": "transforming", "side": "back" }),
    );
    assert_eq!(value["battlefield"][0]["name"], "Ravager of the Fells");
    assert_eq!(value["battlefield"][0]["chosenCreatureType"], "Werewolf");
    let token = &value["battlefield"][1];
    assert_eq!(token["name"], "Test Servo");
    assert_eq!(token["token"], true);
    assert!(token.get("physicalFace").is_none());
    assert!(token.get("definition").is_none());
    assert!(token.get("presentedPartId").is_none());
    assert_eq!(token["characteristics"]["kind"], "token");
    assert_eq!(token["characteristics"]["partId"], 0);
    assert_eq!(token["characteristics"]["name"], "Test Servo");
    assert_eq!(
        token["characteristics"]["art"]["scryfallId"],
        "00000000-0000-0000-0000-000000000001"
    );
    assert_eq!(token["characteristics"]["art"]["artist"], "Test Artist");
    assert_eq!(token["characteristics"]["structure"]["kind"], "single");
    assert_eq!(token["characteristics"]["presentation"]["power"], 1);
    assert_eq!(token["characteristics"]["presentation"]["toughness"], 1);
}

#[test]
fn transforming_token_characteristics_are_inline_and_omit_missing_art() {
    static BACK: crate::TokenPart = crate::TokenPart::new(
        crate::CardPartId(1),
        "Phyrexian",
        crate::CardRules::new_artifact_creature_without_mana_cost(&["Phyrexian"], 0, 0),
    );
    static TOKEN: crate::TokenCharacteristics =
        crate::TokenCharacteristics::artifact(&["Incubator"], &[]).transforming_into(&BACK);
    let characteristics = crate::protocol::json_common::object_characteristics_json(
        crate::ObjectCharacteristics::token(TOKEN, crate::CardPartId(1)),
    );

    assert_eq!(characteristics["kind"], "token");
    assert_eq!(characteristics["partId"], 1);
    assert_eq!(characteristics["name"], "Phyrexian");
    assert!(characteristics.get("art").is_none());
    assert_eq!(
        characteristics["structure"],
        json!({
            "kind": "transformingDoubleFaced",
            "frontPartId": 0,
            "backPartId": 1,
        })
    );
    assert_eq!(characteristics["presentation"]["power"], 0);
    assert_eq!(characteristics["presentation"]["toughness"], 0);
    assert_eq!(
        characteristics["presentation"]["typeLine"],
        "Artifact Creature — Phyrexian"
    );
}

#[test]
fn face_down_characteristics_are_inline_and_have_no_definition() {
    let characteristics = crate::protocol::json_common::object_characteristics_json(
        crate::ObjectCharacteristics::face_down(crate::card::face_down::cloak()),
    );

    assert_eq!(characteristics["kind"], "faceDown");
    assert_eq!(characteristics["name"], "Face-down creature");
    assert!(characteristics.get("definition").is_none());
    assert!(characteristics.get("partId").is_none());
    assert_eq!(characteristics["presentation"]["power"], 2);
    assert_eq!(characteristics["presentation"]["toughness"], 2);
    assert!(
        characteristics["presentation"]["rulesText"]
            .as_str()
            .expect("rules text")
            .contains("Ward {2}")
    );
}

#[test]
fn emblem_characteristics_are_inline_without_catalog_identity() {
    static EMBLEM_ABILITIES: [crate::AbilityDef; 1] = [crate::AbilityDef::not_implemented(
        "Test emblem rule.",
        "protocol fixture",
    )];
    static EMBLEM: crate::EmblemCharacteristics =
        crate::EmblemCharacteristics::new("Test emblem", &EMBLEM_ABILITIES);
    let characteristics = crate::protocol::json_common::object_characteristics_json(
        crate::ObjectCharacteristics::Emblem { emblem: EMBLEM },
    );

    assert_eq!(characteristics["kind"], "emblem");
    assert_eq!(characteristics["name"], "Test emblem");
    assert_eq!(characteristics["presentation"]["kind"], "Emblem");
    assert_eq!(characteristics["presentation"]["typeLine"], "Emblem");
    assert_eq!(
        characteristics["presentation"]["rulesText"],
        "Test emblem rule."
    );
    assert_eq!(
        characteristics["presentation"]["implementationStatus"],
        "metadataOnly"
    );
    assert!(characteristics.get("definition").is_none());
    assert!(characteristics.get("partId").is_none());
    assert!(characteristics.get("art").is_none());
}

#[test]
#[allow(clippy::too_many_lines)]
fn stack_json_uses_game_object_identity_and_preserves_cast_signature() {
    static EMBLEM: crate::EmblemCharacteristics =
        crate::EmblemCharacteristics::new("Test emblem", &[]);

    let catalog = poc::catalog().expect("catalog builds");
    let signature = CastSignature::from_validated_choices(
        SpellForm::Combined(vec![crate::CardPartId(0), crate::CardPartId(1)]),
        structured_choices(),
    );
    let object = StackObservation {
        id: GameObjectId(40),
        kind: StackObjectKind::Spell,
        source: None,
        ability: None,
        ability_text: None,
        characteristics: crate::ObjectCharacteristics::card(
            crate::card::cards::TURN_BURN,
            crate::CardPartId::PRIMARY,
        ),
        controller: PlayerId::One,
        counterable: true,
        targets: signature.iter_targets().copied().collect(),
        chosen_permanents: Vec::new(),
        x: signature.x(),
        signature: Some(signature),
    };
    let value = stack_object_json(&catalog, &object);

    assert_eq!(value["objectId"], 40);
    assert_eq!(value["stackId"], 40);
    assert_eq!(value["counterable"], true);
    assert!(value["sourceObjectId"].is_null());
    assert!(value["source"].is_null());
    assert_eq!(value["signature"]["playOptionId"], 2);
    assert_eq!(value["signature"]["form"]["kind"], "combined");
    assert_eq!(value["signature"]["form"]["partIds"], json!([0, 1]));
    assert_eq!(value["signature"]["modeIds"], json!([3, 1]));
    assert_eq!(value["signature"]["alternativeCostId"], 4);
    assert_eq!(value["signature"]["additionalCostIds"], json!([5]));
    assert_eq!(value["signature"]["x"], 6);
    assert_eq!(value["signature"]["targetSelections"][1]["slotId"], 8);

    let burn_signature = CastSignature::from_validated_choices(
        SpellForm::Part(crate::CardPartId(1)),
        CastChoices::new(crate::PlayOptionId(1)),
    );
    let burn = StackObservation {
        id: GameObjectId(41),
        kind: StackObjectKind::Spell,
        source: None,
        ability: None,
        ability_text: None,
        characteristics: crate::ObjectCharacteristics::card(
            crate::card::cards::TURN_BURN,
            crate::CardPartId(1),
        ),
        controller: PlayerId::One,
        counterable: true,
        targets: Vec::new(),
        chosen_permanents: Vec::new(),
        x: 0,
        signature: Some(burn_signature),
    };
    assert_eq!(stack_object_json(&catalog, &burn)["name"], "Burn");

    let ability = StackObservation {
        id: GameObjectId(42),
        kind: StackObjectKind::ActivatedAbility,
        source: Some(GameObjectId(39)),
        ability: None,
        ability_text: None,
        characteristics: crate::ObjectCharacteristics::card(
            crate::card::cards::MISHRA_S_FACTORY,
            crate::CardPartId::PRIMARY,
        ),
        controller: PlayerId::One,
        counterable: true,
        targets: Vec::new(),
        chosen_permanents: Vec::new(),
        x: 0,
        signature: None,
    };
    let ability_value = stack_object_json(&catalog, &ability);
    assert_eq!(ability_value["objectId"], 42);
    assert_eq!(ability_value["stackId"], 42);
    assert_eq!(ability_value["sourceObjectId"], 39);
    assert_eq!(ability_value["source"], 39);
    assert_ne!(
        ability_value["objectId"], ability_value["sourceObjectId"],
        "the ability and its source are distinct game objects"
    );
    assert!(ability_value["signature"].is_null());

    let trigger = StackObservation {
        id: GameObjectId(43),
        kind: StackObjectKind::TriggeredAbility,
        source: Some(GameObjectId(38)),
        ability: Some(AbilityOrigin::Printed {
            definition: crate::card::cards::ANKH_OF_MISHRA,
            part: crate::CardPartId::PRIMARY,
            ability: crate::AbilityId::PRIMARY,
        }),
        ability_text: Some(
            "Whenever a land enters, Ankh of Mishra deals 2 damage to its controller.".into(),
        ),
        characteristics: crate::ObjectCharacteristics::card(
            crate::card::cards::ANKH_OF_MISHRA,
            crate::CardPartId::PRIMARY,
        ),
        controller: PlayerId::Two,
        counterable: true,
        targets: Vec::new(),
        chosen_permanents: Vec::new(),
        x: 0,
        signature: None,
    };
    let trigger_value = stack_object_json(&catalog, &trigger);
    assert_eq!(trigger_value["kind"], "TriggeredAbility");
    assert_eq!(trigger_value["sourceObjectId"], 38);
    assert_eq!(trigger_value["abilityId"], 0);
    assert_eq!(trigger_value["ability"]["kind"], "printed");
    assert_eq!(
        trigger_value["ability"]["definition"],
        crate::card::cards::ANKH_OF_MISHRA.0
    );
    assert_eq!(
        trigger_value["abilityText"],
        "Whenever a land enters, Ankh of Mishra deals 2 damage to its controller."
    );
    assert_eq!(trigger_value["controller"], "p2");

    let token_trigger = StackObservation {
        id: GameObjectId(44),
        kind: StackObjectKind::TriggeredAbility,
        source: Some(GameObjectId(37)),
        ability: Some(AbilityOrigin::Token {
            part: crate::CardPartId::PRIMARY,
            ability: crate::AbilityId(2),
        }),
        ability_text: Some("Test token trigger".into()),
        characteristics: crate::ObjectCharacteristics::token(
            test_token(),
            crate::CardPartId::PRIMARY,
        ),
        controller: PlayerId::One,
        counterable: true,
        targets: Vec::new(),
        chosen_permanents: Vec::new(),
        x: 0,
        signature: None,
    };
    let token_value = stack_object_json(&catalog, &token_trigger);
    assert_eq!(token_value["name"], "Test Servo");
    assert!(token_value.get("definition").is_none());
    assert!(token_value.get("presentedPartId").is_none());
    assert!(token_value["abilityId"].is_null());
    assert_eq!(token_value["ability"]["kind"], "token");
    assert_eq!(token_value["ability"]["partId"], 0);
    assert_eq!(token_value["ability"]["abilityId"], 2);
    assert_eq!(token_value["characteristics"]["kind"], "token");
    assert_eq!(
        token_value["characteristics"]["art"]["artist"],
        "Test Artist"
    );

    let emblem_trigger = StackObservation {
        id: GameObjectId(45),
        kind: StackObjectKind::TriggeredAbility,
        source: Some(GameObjectId(38)),
        ability: Some(AbilityOrigin::Emblem {
            ability: crate::AbilityId(3),
        }),
        ability_text: Some("Test emblem trigger".into()),
        characteristics: crate::ObjectCharacteristics::Emblem { emblem: EMBLEM },
        controller: PlayerId::One,
        counterable: true,
        targets: Vec::new(),
        chosen_permanents: Vec::new(),
        x: 0,
        signature: None,
    };
    let emblem_value = stack_object_json(&catalog, &emblem_trigger);
    assert_eq!(emblem_value["name"], "Test emblem");
    assert!(emblem_value.get("definition").is_none());
    assert!(emblem_value.get("presentedPartId").is_none());
    assert!(emblem_value["abilityId"].is_null());
    assert_eq!(
        emblem_value["ability"],
        json!({
            "kind": "emblem",
            "abilityId": 3,
        })
    );
    assert_eq!(emblem_value["characteristics"]["kind"], "emblem");
    assert_eq!(
        emblem_value["characteristics"]["presentation"]["typeLine"],
        "Emblem"
    );
}

#[test]
fn decision_json_exposes_trigger_procedure_and_resolution_order_semantics() {
    let catalog = poc::catalog().expect("catalog builds");
    let decision = DecisionObservation {
        id: 7,
        player: PlayerId::One,
        kind: DecisionKind::TriggerOrder,
        order_semantics: Some(DecisionOrderSemantics::Resolution),
        prompt: "Choose the order your triggers resolve".into(),
        visibility: crate::game::DecisionVisibility::Public,
        preference: crate::game::DecisionPreference::Neutral,
        minimum: 2,
        maximum: 2,
        cancellable: false,
        options: vec![
            crate::game::DecisionOption {
                id: 11,
                label: "First Ankh trigger".into(),
                card: Some((
                    GameObjectId(81),
                    crate::ObjectCharacteristics::card(
                        crate::card::cards::ANKH_OF_MISHRA,
                        crate::CardPartId::PRIMARY,
                    ),
                )),
                members: Vec::new(),
                ability_text: Some("First frozen trigger text".into()),
                zone: crate::game::DecisionZone::Battlefield,
            },
            crate::game::DecisionOption {
                id: 12,
                label: "Second Ankh trigger".into(),
                card: Some((
                    GameObjectId(82),
                    crate::ObjectCharacteristics::card(
                        crate::card::cards::ANKH_OF_MISHRA,
                        crate::CardPartId::PRIMARY,
                    ),
                )),
                members: Vec::new(),
                ability_text: Some("Second frozen trigger text".into()),
                zone: crate::game::DecisionZone::Battlefield,
            },
        ],
    };

    let value = decision_json(&catalog, &decision);
    assert_eq!(value["kind"], "TriggerOrder");
    assert_eq!(value["orderSemantics"], "resolution");
    assert_eq!(value["options"][0]["triggerId"], 11);
    assert_eq!(value["options"][0]["card"]["objectId"], 81);
    assert_eq!(
        value["options"][0]["abilityText"],
        "First frozen trigger text"
    );

    let ordinary = DecisionObservation {
        kind: DecisionKind::Choice,
        order_semantics: None,
        ..decision
    };
    assert!(
        decision_json(&catalog, &ordinary)
            .get("orderSemantics")
            .is_none()
    );
}

/// The wire name for a loss on time. A bot switching on `result.reason` is
/// entitled to tell "my opponent gave up" from "I was too slow", so the
/// string is part of the contract rather than an implementation detail.
#[test]
fn a_loss_on_time_is_named_on_the_wire_and_is_not_a_concession() {
    let catalog = poc::catalog().expect("catalog builds");
    let deck = poc::goblins();
    let mut game = Game::new(catalog.clone(), [deck.clone(), deck], 3).expect("game starts");
    game.lose_on_time(PlayerId::Two);

    let observation = game.observe(PlayerId::One);
    let json = observation_json_for_format(
        &catalog,
        Format::OldSchool9394,
        &observation,
        game.in_pregame(),
        &protocol_actions(&observation),
    );
    assert_eq!(json["result"]["winner"], "p1");
    assert_eq!(json["result"]["reason"], "OpponentRanOutOfTime");
}

#[test]
fn decision_json_names_outside_game_card_provenance() {
    let catalog = poc::catalog().expect("catalog builds");
    let decision = DecisionObservation {
        id: 17,
        player: PlayerId::One,
        kind: DecisionKind::Choice,
        order_semantics: None,
        prompt: "Choose a sideboard card".into(),
        visibility: crate::game::DecisionVisibility::Private,
        preference: crate::game::DecisionPreference::Neutral,
        minimum: 1,
        maximum: 1,
        cancellable: false,
        options: vec![crate::game::DecisionOption {
            id: 0,
            label: "Black Lotus".into(),
            card: Some((
                GameObjectId(99),
                crate::ObjectCharacteristics::card(
                    crate::card::cards::BLACK_LOTUS,
                    crate::CardPartId::PRIMARY,
                ),
            )),
            members: Vec::new(),
            ability_text: None,
            zone: crate::game::DecisionZone::OutsideGame,
        }],
    };

    let value = decision_json(&catalog, &decision);
    assert_eq!(value["options"][0]["zone"], "OutsideGame");
}
