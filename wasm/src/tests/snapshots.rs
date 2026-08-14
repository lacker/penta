use super::*;

#[test]
fn the_pass_label_stops_where_the_opponents_attack_decision_begins() {
    // The button must not answer a question that belongs to the opponent.
    // Promising "their end step" would predict that they decline to
    // attack; promising "blocks" would predict that they attack. The
    // honest destination is the step where their choice happens.
    //
    // This does not change the label the old opponent-predicting preview
    // produced here -- the human's own auto-pass stopped at the same step
    // either way. It pins the label against a preview that walks past the
    // decision in either direction.
    let mut game =
        WebGame::new("Sligh", "Goblins", "Handcrafted", false, 4_242, None).expect("game starts");
    while game.session.engine_mut().in_pregame() {
        apply_engine_action(game.session.engine_mut(), |action| {
            matches!(action, Action::KeepHand)
        });
    }
    game.session
        .engine_mut()
        .set_hand(game.human, &[])
        .expect("an empty hand is valid");
    let human = game.human;
    // Given to them a turn early, so it is not summoning sick when their
    // combat comes around.
    advance_engine_quietly_until(game.session.engine_mut(), |observation| {
        observation.active_player == human && observation.step == Step::PrecombatMain
    });
    game.session
        .engine_mut()
        .put_onto_battlefield(human.opponent(), penta::card::cards::SAVANNAH_LIONS)
        .expect("the Lions are cataloged");
    advance_engine_quietly_until(game.session.engine_mut(), |observation| {
        observation.active_player == human.opponent() && observation.viewer == human
    });
    assert_eq!(game.session.engine_mut().decision_player(), Some(human));

    assert_eq!(
        game.pass_preview_label().as_deref(),
        Some("Go to their attack"),
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn interwave_snapshot_and_pass_preview_expose_pending_regular_damage() {
    let mut game =
        WebGame::new("Goblins", "Sligh", "Handcrafted", true, 9_394, None).expect("game starts");
    while game.session.engine_mut().in_pregame() {
        apply_engine_action(game.session.engine_mut(), |action| {
            matches!(action, Action::KeepHand)
        });
    }
    game.session
        .engine_mut()
        .set_hand(
            game.human,
            &[
                penta::card::cards::BLACK_LOTUS,
                penta::card::cards::BLACK_KNIGHT,
            ],
        )
        .expect("test hand is cataloged");
    game.session
        .engine_mut()
        .set_hand(game.human.opponent(), &[])
        .expect("an empty hand is valid");

    advance_engine_quietly_until(game.session.engine_mut(), |observation| {
        observation.active_player == game.human && observation.step == Step::PrecombatMain
    });
    let opening = game.session.engine_mut().observe(game.human);
    let lotus_in_hand = opening
        .hand
        .iter()
        .find_map(|(id, definition)| {
            (*definition == penta::card::cards::BLACK_LOTUS).then_some(*id)
        })
        .expect("Black Lotus is in hand");
    let knight = opening
        .hand
        .iter()
        .find_map(|(id, definition)| {
            (*definition == penta::card::cards::BLACK_KNIGHT).then_some(*id)
        })
        .expect("Black Knight is in hand");
    apply_engine_action(
        game.session.engine_mut(),
        |action| matches!(action, Action::CastSpell { card, .. } if *card == lotus_in_hand),
    );
    advance_engine_quietly_until(game.session.engine_mut(), |observation| {
        observation.stack.is_empty() && observation.step == Step::PrecombatMain
    });
    let lotus = game
        .session
        .observe(game.human)
        .battlefield
        .iter()
        .find(|permanent| permanent.definition == penta::card::cards::BLACK_LOTUS)
        .expect("Black Lotus resolved")
        .id;
    apply_engine_action(game.session.engine_mut(), |action| {
        matches!(
            action,
            Action::ActivateManaAbility {
                source,
                color: penta::ManaColor::Black,
                ..
            } if *source == lotus
        )
    });
    apply_engine_action(
        game.session.engine_mut(),
        |action| matches!(action, Action::CastSpell { card, .. } if *card == knight),
    );
    advance_engine_quietly_until(game.session.engine_mut(), |observation| {
        observation.stack.is_empty() && observation.step == Step::PrecombatMain
    });

    advance_engine_quietly_until(game.session.engine_mut(), |observation| {
        observation.active_player == game.human
            && observation.active_turn == 2
            && observation.step == Step::DeclareAttackers
            && observation
                .legal_actions
                .iter()
                .any(|action| matches!(action, Action::DeclareAttacker { .. }))
    });
    let knight = game
        .session
        .observe(game.human)
        .battlefield
        .iter()
        .find(|permanent| permanent.definition == penta::card::cards::BLACK_KNIGHT)
        .expect("Black Knight resolved")
        .id;
    apply_engine_action(
        game.session.engine_mut(),
        |action| matches!(action, Action::DeclareAttacker { attacker, .. } if *attacker == knight),
    );
    apply_engine_action(game.session.engine_mut(), |action| {
        matches!(action, Action::FinishDeclaringAttackers)
    });
    advance_engine_quietly_until(game.session.engine_mut(), |observation| {
        observation.step == Step::DeclareBlockers
            && observation
                .legal_actions
                .iter()
                .any(|action| matches!(action, Action::FinishDeclaringBlockers))
    });
    apply_engine_action(game.session.engine_mut(), |action| {
        matches!(action, Action::FinishDeclaringBlockers)
    });
    advance_engine_quietly_until(game.session.engine_mut(), |observation| {
        observation.regular_combat_damage_pending
    });

    let observation = game.session.engine_mut().observe(game.human);
    assert!(
        !WebGame::attack_awaiting_damage(&observation),
        "the older preview heuristic cannot recognize an already-open damage step",
    );
    assert!(WebGame::combat_damage_awaiting(&observation));
    assert_eq!(
        game.snapshot_value(false)["regularCombatDamagePending"],
        true,
    );
    assert_eq!(game.pass_preview_label().as_deref(), Some("Go to damage"));
}

#[test]
fn missing_card_art_serializes_as_null() {
    assert_eq!(card_art_value(None), Value::Null);
}

#[test]
fn hand_mana_cost_distinguishes_no_cost_from_printed_zero() {
    let catalog = card::catalog().expect("catalog builds");
    let mountain = catalog
        .get(penta::card::cards::MOUNTAIN)
        .expect("Mountain is cataloged");
    let mox = catalog
        .get(penta::card::cards::MOX_RUBY)
        .expect("Mox Ruby is cataloged");

    assert_eq!(hand_mana_cost_value(Some(mountain)), Value::Null);
    let zero = hand_mana_cost_value(Some(mox));
    assert!(zero.is_object());
    assert_eq!(zero["generic"], 0);
    assert_eq!(zero["red"], 0);
}

#[test]
fn visible_cards_include_nested_scryfall_metadata() {
    let game = WebGame::new("Goblins", "Sligh", "Handcrafted", true, 9_394, None).unwrap();
    let snapshot = game.snapshot_value(false);
    let hand = snapshot["human"]["hand"].as_array().unwrap();

    assert_eq!(hand.len(), 7);
    hand.iter().for_each(assert_nested_card_art);
}

#[test]
fn battlefield_and_stack_include_nested_scryfall_metadata() {
    let mut game =
        WebGame::new("Goblins", "Sligh", "Handcrafted", true, 3_756_436_840, None).unwrap();
    act_matching(&mut game, |action| matches!(action, Action::KeepHand));
    game.set_autopass(false).unwrap();
    act_matching(&mut game, |action| {
        matches!(action, Action::CastSpell { .. })
    });

    let stack_snapshot = game.snapshot_value(false);
    let stack = stack_snapshot["stack"].as_array().unwrap();
    assert_eq!(stack.len(), 1);
    assert_eq!(stack[0]["name"], "Black Lotus");
    assert_eq!(stack[0]["counterable"], true);
    assert_nested_card_art(&stack[0]);

    game.set_autopass(true).unwrap();
    let battlefield_snapshot = game.snapshot_value(false);
    let lotus = battlefield_snapshot["battlefield"]
        .as_array()
        .unwrap()
        .iter()
        .find(|card| card["name"] == "Black Lotus")
        .expect("Black Lotus resolved to the battlefield");
    assert_nested_card_art(lotus);
    assert!(lotus.as_object().is_some_and(|card| {
        card.contains_key("chosenCreatureType") && card["chosenCreatureType"].is_null()
    }));
}

fn start_empty_human_main_phase(game: &mut WebGame) {
    while game.session.engine_mut().in_pregame() {
        apply_engine_action(game.session.engine_mut(), |action| {
            matches!(action, Action::KeepHand)
        });
    }
    let human = game.human;
    game.session
        .engine_mut()
        .set_hand(human, &[])
        .expect("an empty hand is valid");
    game.session
        .engine_mut()
        .set_hand(human.opponent(), &[])
        .expect("an empty hand is valid");
    advance_engine_quietly_until(game.session.engine_mut(), |observation| {
        observation.active_player == human && observation.step == Step::PrecombatMain
    });
}

fn put_human_permanent(game: &mut WebGame, definition: CardDefinitionId) -> penta::GameObjectId {
    let human = game.human;
    game.session
        .engine_mut()
        .put_onto_battlefield(human, definition)
        .expect("the test permanent enters the battlefield")
}

#[test]
fn attachments_and_effect_scoped_special_actions_reach_the_browser_snapshot() {
    let mut game = WebGame::new("Goblins", "Sligh", "Handcrafted", true, 19_098, None).unwrap();
    start_empty_human_main_phase(&mut game);

    let licid = put_human_permanent(&mut game, penta::card::cards::QUICKENING_LICID);
    let host = put_human_permanent(&mut game, penta::card::cards::SAVANNAH_LIONS);
    let plains = [
        put_human_permanent(&mut game, penta::card::cards::PLAINS),
        put_human_permanent(&mut game, penta::card::cards::PLAINS),
        put_human_permanent(&mut game, penta::card::cards::PLAINS),
    ];
    let entered_turn = game.session.engine_mut().observe(game.human).active_turn;
    advance_engine_quietly_until(game.session.engine_mut(), |observation| {
        observation.active_player == game.human
            && observation.active_turn > entered_turn
            && observation.step == Step::PrecombatMain
    });

    for source in &plains[..2] {
        apply_engine_action(game.session.engine_mut(), |action| {
            matches!(
                action,
                Action::ActivateManaAbility {
                    source: candidate,
                    color: penta::ManaColor::White,
                    ..
                } if candidate == source
            )
        });
    }
    apply_engine_action(game.session.engine_mut(), |action| {
        matches!(
            action,
            Action::ActivateAbility {
                source,
                targets,
                ..
            } if *source == licid
                && targets.iter().any(|selection| {
                    selection
                        .targets()
                        .contains(&Target::Permanent(host))
                })
        )
    });
    advance_engine_quietly_until(game.session.engine_mut(), |observation| {
        observation.stack.is_empty()
            && observation.active_player == game.human
            && observation.step == Step::PrecombatMain
    });

    let snapshot = game.snapshot_value(false);
    let battlefield = snapshot["battlefield"]
        .as_array()
        .expect("battlefield array");
    let licid_card = battlefield
        .iter()
        .find(|card| card["id"] == licid.0)
        .expect("the Licid is visible");
    assert_eq!(licid_card["attachedTo"], host.0);
    let host_card = battlefield
        .iter()
        .find(|card| card["id"] == host.0)
        .expect("the enchanted creature is visible");
    assert!(host_card["attachedTo"].is_null());

    let action = snapshot["actions"]
        .as_array()
        .expect("actions array")
        .iter()
        .find(|action| action["cardId"] == licid.0 && action["effectId"].is_number())
        .expect("the Licid end action is browser-visible");
    assert_eq!(
        action["label"],
        "Quickening Licid — You may pay {W} to end this effect."
    );
    assert_eq!(action["abilityLabel"], action["label"]);
    assert_eq!(action["ability"]["kind"], "printed");
    assert_eq!(
        action["ability"]["definition"],
        penta::card::cards::QUICKENING_LICID.0
    );
    assert_eq!(action["ability"]["partId"], 0);
    assert_eq!(action["ability"]["abilityId"], 0);
    assert_eq!(action["kind"], "primary");
    assert_eq!(action["paymentAction"], true);
    assert_eq!(action["spellAction"], false);
    assert_eq!(action["manaSourceIds"], json!([plains[2].0]));
}

#[test]
fn standard_visible_cards_include_nested_scryfall_metadata() {
    let game = WebGame::new(
        "Briksza Naya Midrange",
        "Greer G/R Aggro",
        "Handcrafted",
        true,
        2_013,
        Some("isd-rtr-standard".into()),
    )
    .unwrap();
    let snapshot = game.snapshot_value(false);
    let hand = snapshot["human"]["hand"].as_array().unwrap();
    let standard_cards = hand
        .iter()
        .filter(|card| {
            !matches!(
                card["name"].as_str(),
                Some("Plains" | "Island" | "Swamp" | "Mountain" | "Forest")
            )
        })
        .collect::<Vec<_>>();

    assert!(!standard_cards.is_empty());
    for card in standard_cards {
        assert_nested_card_art(card);
    }
}

#[test]
fn shock_land_entry_stays_prospective_until_the_browser_choice_commits_it() {
    let mut game = WebGame::new(
        "Briksza Naya Midrange",
        "Greer G/R Aggro",
        "Handcrafted",
        true,
        2,
        Some("isd-rtr-standard".into()),
    )
    .unwrap();
    act_matching(&mut game, |action| matches!(action, Action::KeepHand));

    let before = game.snapshot_value(false);
    let temple_garden = before["human"]["hand"]
        .as_array()
        .expect("hand array")
        .iter()
        .find(|card| card["name"] == "Temple Garden")
        .expect("the deterministic opening hand contains Temple Garden")["id"]
        .as_u64()
        .and_then(|id| u32::try_from(id).ok())
        .map(CardInstanceId)
        .expect("card ID fits the engine ID type");

    act_matching(
        &mut game,
        |action| matches!(action, Action::PlayLand { card, .. } if *card == temple_garden),
    );

    let prospective = game.snapshot_value(false);
    assert_eq!(prospective["human"]["life"], 20);
    assert_eq!(prospective["decision"]["kind"], "Choice");
    assert_eq!(
        prospective["decision"]["prompt"],
        "Pay 2 life as Temple Garden enters the battlefield?"
    );
    assert_eq!(prospective["decision"]["minimum"], 1);
    assert_eq!(prospective["decision"]["maximum"], 1);
    assert_eq!(prospective["decision"]["cancellable"], false);
    assert_eq!(
        prospective["decision"]["options"],
        json!([
            { "id": 0, "triggerId": null, "label": "Do not pay", "cardId": null, "cardName": null, "members": [], "abilityText": null, "zone": "None" },
            { "id": 1, "triggerId": null, "label": "Pay 2 life", "cardId": null, "cardName": null, "members": [], "abilityText": null, "zone": "None" },
        ])
    );
    assert!(
        prospective["human"]["hand"]
            .as_array()
            .expect("hand array")
            .iter()
            .all(|card| card["id"] != temple_garden.0),
        "the prospective card has left the hand"
    );
    assert!(
        prospective["battlefield"]
            .as_array()
            .expect("battlefield array")
            .iter()
            .all(|card| card["name"] != "Temple Garden"),
        "the prospective card is not a permanent before replacement choices finish"
    );

    let decision = prospective["decision"]["id"]
        .as_u64()
        .and_then(|id| u32::try_from(id).ok())
        .expect("decision ID fits the engine ID type");
    game.choose_decision(decision, "[1]")
        .expect("pay-life choice succeeds");

    let committed = game.snapshot_value(false);
    assert_eq!(committed["human"]["life"], 18);
    assert!(committed["decision"].is_null());
    let temple_garden = committed["battlefield"]
        .as_array()
        .expect("battlefield array")
        .iter()
        .find(|card| card["name"] == "Temple Garden")
        .expect("the chosen entry commits to the battlefield");
    assert_eq!(temple_garden["tapped"], false);
}
