use super::*;

#[test]
fn observation_json_carries_interwave_state_and_presented_card_part() {
    let catalog = poc::catalog().expect("catalog builds");
    let observation = PlayerObservation {
        viewer: PlayerId::One,
        turn: 1,
        active_turn: 1,
        active_player: PlayerId::One,
        priority: PlayerId::One,
        step: crate::game::Step::CombatDamage,
        regular_combat_damage_pending: true,
        life_totals: [20, 20],
        mana_pools: [crate::ManaPool::default(); 2],
        hand: Vec::new(),
        opponent_hand_size: 0,
        last_seen_hand: None,
        library_sizes: [0, 0],
        graveyards: [Vec::new(), Vec::new()],
        exiles: [Vec::new(), Vec::new()],
        emblems: Vec::new(),
        battlefield: vec![crate::game::PermanentObservation {
            id: GameObjectId(30),
            definition: crate::card::cards::HUNTMASTER_OF_THE_FELLS,
            presented: crate::CardPartId(1),
            controller: PlayerId::One,
            attached_to: Some(GameObjectId(29)),
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
            blocking: None,
            flying: false,
            can_attack: true,
            entered_this_turn: false,
        }],
        stack: Vec::new(),
        decision: None,
        result: None,
        legal_actions: Vec::new(),
        checkpoint: json!({}),
    };

    let value =
        observation_json_for_format(&catalog, Format::IsdRtrStandard, &observation, false, &[]);
    assert_eq!(value["regularCombatDamagePending"], true);
    assert_eq!(value["battlefield"][0]["objectId"], 30);
    assert_eq!(value["battlefield"][0]["presentedPartId"], 1);
    assert_eq!(value["battlefield"][0]["name"], "Ravager of the Fells");
    assert_eq!(value["battlefield"][0]["attachedTo"], 29);
    assert_eq!(value["battlefield"][0]["chosenCreatureType"], "Werewolf");
    assert_eq!(
        card_part_name(
            &catalog,
            crate::card::cards::HUNTMASTER_OF_THE_FELLS,
            crate::CardPartId(99),
        ),
        "Huntmaster of the Fells"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn stack_json_uses_game_object_identity_and_preserves_cast_signature() {
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
        definition: crate::card::cards::TURN_BURN,
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
        definition: crate::card::cards::TURN_BURN,
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
        definition: crate::card::cards::MISHRA_S_FACTORY,
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
        definition: crate::card::cards::ANKH_OF_MISHRA,
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
                card: Some((GameObjectId(81), crate::card::cards::ANKH_OF_MISHRA)),
                members: Vec::new(),
                ability_text: Some("First frozen trigger text".into()),
                zone: crate::game::DecisionZone::Battlefield,
            },
            crate::game::DecisionOption {
                id: 12,
                label: "Second Ankh trigger".into(),
                card: Some((GameObjectId(82), crate::card::cards::ANKH_OF_MISHRA)),
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
            card: Some((GameObjectId(99), crate::card::cards::BLACK_LOTUS)),
            members: Vec::new(),
            ability_text: None,
            zone: crate::game::DecisionZone::OutsideGame,
        }],
    };

    let value = decision_json(&catalog, &decision);
    assert_eq!(value["options"][0]["zone"], "OutsideGame");
}
