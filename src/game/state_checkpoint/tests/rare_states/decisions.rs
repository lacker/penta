/// Liliana's +1 asks both players at once. Between the two answers the engine
/// holds one seat's committed discard while the other is still choosing, which
/// is a choice the waiting seat must not be able to read out of its own
/// checkpoint -- and which the host must still be able to hand back.
#[test]
fn a_multi_player_discard_reconstructs_while_one_choice_is_still_hidden() {
    let mut game = staged_modern_game();
    let walker_id = GameObjectId(10_000);
    let mut walker = creature(
        walker_id.0,
        crate::card::cards::LILIANA_OF_THE_VEIL,
        PlayerId::One,
    );
    walker.set_counters(CounterKind::Loyalty, 3);
    game.battlefield.push(walker);
    for (player, base) in [(PlayerId::One, 11_000), (PlayerId::Two, 12_000)] {
        for offset in 0..2 {
            let card = card(base + offset, crate::card::cards::MOUNTAIN, player);
            game.players[player.index()].hand.push(card);
        }
    }

    game.apply(
        PlayerId::One,
        loyalty_action(
            walker_id,
            crate::card::cards::LILIANA_OF_THE_VEIL,
            0,
            Vec::new(),
        ),
    )
    .expect("Liliana's plus ability activates");
    resolve_top_of_stack(&mut game);

    assert_eq!(
        game.decision_player(),
        Some(PlayerId::One),
        "the active player discards first"
    );
    assert_reconstructs(&game, "a multi-player discard before any choice");

    answer_with_first_option(&mut game);
    assert_eq!(
        game.decision_player(),
        Some(PlayerId::Two),
        "the opposing seat still owes a discard"
    );
    let chosen = matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::DiscardForEffect { ref chosen, .. }
            if chosen.iter().any(|(player, cards)| *player == PlayerId::One && !cards.is_empty())
    );
    assert!(chosen, "the first seat's discard must already be recorded");
    assert_reconstructs(&game, "a multi-player discard holding a hidden choice");
}

/// Liliana's ultimate is the only card-owned pile program in the catalog and
/// it never fires in sampled play. Its two continuations carry a callback,
/// which the snapshot must address by registry key rather than by pointer.
#[test]
fn a_card_owned_pile_program_reconstructs_at_both_of_its_boundaries() {
    let mut game = staged_modern_game();
    let walker_id = GameObjectId(10_000);
    let mut walker = creature(
        walker_id.0,
        crate::card::cards::LILIANA_OF_THE_VEIL,
        PlayerId::One,
    );
    walker.set_counters(CounterKind::Loyalty, 6);
    game.battlefield.push(walker);
    for offset in 0..3 {
        game.battlefield.push(creature(
            12_000 + offset,
            crate::card::cards::WALKING_CORPSE,
            PlayerId::Two,
        ));
    }

    game.apply(
        PlayerId::One,
        loyalty_action(
            walker_id,
            crate::card::cards::LILIANA_OF_THE_VEIL,
            2,
            vec![TargetSelection::single(
                TargetSlotId(0),
                Target::Player(PlayerId::Two),
            )],
        ),
    )
    .expect("Liliana's ultimate activates");
    resolve_top_of_stack(&mut game);

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::SeparateIntoPiles { .. })
        ),
        "the ultimate must be waiting on a pile split, not {:?}",
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation)
    );
    assert_reconstructs(&game, "a card-owned pile split");

    answer_with_first_option(&mut game);
    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::ChoosePile { .. })
        ),
        "splitting must lead to the pile choice"
    );
    assert_reconstructs(&game, "a card-owned pile choice");
}

/// An installed trigger belongs to no object: its source has already resolved,
/// and it watches the game until a named player's next turn. Sampled play
/// rarely leaves one at a decision boundary, so cover it explicitly.
#[test]
fn an_effect_installed_trigger_reconstructs() {
    let mut game = staged_modern_game();
    let walker_id = GameObjectId(10_000);
    let mut walker = creature(
        walker_id.0,
        crate::card::cards::JACE_ARCHITECT_OF_THOUGHT,
        PlayerId::One,
    );
    walker.set_counters(CounterKind::Loyalty, 4);
    game.battlefield.push(walker);

    game.apply(
        PlayerId::One,
        loyalty_action(
            walker_id,
            crate::card::cards::JACE_ARCHITECT_OF_THOUGHT,
            0,
            Vec::new(),
        ),
    )
    .expect("Jace's plus ability activates");
    resolve_top_of_stack(&mut game);

    assert_eq!(
        game.installed_triggers.len(),
        1,
        "the resolved ability must leave an installed trigger behind"
    );
    assert_reconstructs(&game, "an installed trigger watching the game");
}

/// The opposing seat splits public cards that remain in the effect controller's
/// library. Both seats can reconstruct either decision, so checkpoint import
/// must use the cards' actual origin rather than assuming that the deciding or
/// observing seat owns the library. Each reconstructed decision is answered to
/// prove that its typed ids still address live cards, not merely that its
/// observation can be rendered.
#[test]
#[allow(clippy::too_many_lines)]
fn a_generic_pile_split_reconstructs_and_resumes_for_both_seats() {
    let mut game = staged_modern_game();
    let walker_id = GameObjectId(10_000);
    let mut walker = creature(
        walker_id.0,
        crate::card::cards::JACE_ARCHITECT_OF_THOUGHT,
        PlayerId::One,
    );
    walker.set_counters(CounterKind::Loyalty, 4);
    game.battlefield.push(walker);

    game.apply(
        PlayerId::One,
        loyalty_action(
            walker_id,
            crate::card::cards::JACE_ARCHITECT_OF_THOUGHT,
            1,
            Vec::new(),
        ),
    )
    .expect("Jace's minus ability activates");
    resolve_top_of_stack(&mut game);

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::SplitForEffect { .. })
        ),
        "the ability must be waiting on a generic pile split, not {:?}",
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation)
    );
    assert_reconstructs(&game, "a generic pile split");

    let split_observation = game.observe(PlayerId::Two);
    let split_actions = crate::protocol::protocol_actions(&split_observation);
    let split_wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &split_observation,
        game.in_pregame(),
        &split_actions,
    );
    assert!(
        split_wire["checkpoint"]["decisionState"]["continuation"]
            .get("items")
            .is_none(),
        "split items are reconstructed from the authored outer effect",
    );
    let canonical_items = match &game.pending_decisions[0].continuation {
        DecisionContinuation::SplitForEffect { items, .. } => items.clone(),
        _ => unreachable!(),
    };
    let outside = game.players[PlayerId::One.index()]
        .library
        .iter()
        .enumerate()
        .find(|(_, card)| !canonical_items.contains(&Target::Card(card.id)))
        .expect("the library has a card outside Jace's top three");
    let mut spliced_split = split_wire.clone();
    let old_object = spliced_split["decision"]["options"][0]["card"]["objectId"]
        .as_u64()
        .expect("Jace's split option is a card");
    let outside_name = game
        .catalog
        .get(outside.1.definition)
        .expect("the outside card is cataloged")
        .name
        .clone();
    spliced_split["decision"]["options"][0]["label"] = Value::String(outside_name);
    spliced_split["decision"]["options"][0]["card"]["objectId"] = Value::from(outside.1.id.0);
    spliced_split["decision"]["options"][0]["card"]["definition"] =
        Value::from(outside.1.definition.get());
    spliced_split["decision"]["options"][0]["card"]["characteristics"]["definition"] =
        Value::from(outside.1.definition.get());
    spliced_split["checkpoint"]["decisionState"]["options"][0]["card"]["objectId"] =
        Value::from(outside.1.id.0);
    spliced_split["checkpoint"]["decisionState"]["options"][0]["card"]["characteristics"]
        ["definition"] = Value::from(outside.1.definition.get());
    let origin = spliced_split["checkpoint"]["decisionState"]["cardOrigins"]
        .as_array_mut()
        .expect("split origins are an array")
        .iter_mut()
        .find(|origin| origin["objectId"].as_u64() == Some(old_object))
        .expect("the replaced split option has an origin");
    origin["objectId"] = Value::from(outside.1.id.0);
    origin["index"] = Value::from(outside.0);
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &spliced_split,
        &true_hidden_hypothesis(&game, PlayerId::Two),
        4_249,
    )
    .expect_err("Jace's split cannot substitute a card outside the authored top three");
    assert!(
        error.contains("pile split decision options disagree"),
        "unexpected split error: {error}",
    );

    for (viewer, seed) in [(PlayerId::One, 4_250), (PlayerId::Two, 4_251)] {
        let mut rebuilt = rebuild_from_truth_for_viewer(&game, viewer, seed);
        let items = match &rebuilt.pending_decisions[0].continuation {
            DecisionContinuation::SplitForEffect { items, .. } => items,
            other => panic!("split reconstruction changed continuation: {other:?}"),
        };
        assert!(
            items.iter().all(|target| matches!(target, Target::Card(id)
                if rebuilt.players[PlayerId::One.index()]
                    .library
                    .iter()
                    .any(|card| card.id == *id))),
            "{viewer:?} reconstruction must bind every item to player one's library",
        );
        answer_with_first_option(&mut rebuilt);
        assert!(matches!(
            rebuilt.pending_decisions[0].continuation,
            DecisionContinuation::ChoosePileForEffect { .. }
        ));
    }

    answer_with_first_option(&mut game);
    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::ChoosePileForEffect { .. })
        ),
        "splitting must lead to the pile choice"
    );
    assert_reconstructs(&game, "a generic pile choice");

    let pile_observation = game.observe(PlayerId::One);
    let pile_actions = crate::protocol::protocol_actions(&pile_observation);
    let pile_wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &pile_observation,
        game.in_pregame(),
        &pile_actions,
    );
    let mut duplicated = pile_wire.clone();
    let first = duplicated["checkpoint"]["decisionState"]["continuation"]["first"][0].clone();
    duplicated["checkpoint"]["decisionState"]["continuation"]["second"][0] = first;
    let first_member = duplicated["decision"]["options"][0]["members"][0].clone();
    duplicated["decision"]["options"][1]["members"][0] = first_member;
    let first_member = duplicated["checkpoint"]["decisionState"]["options"][0]["members"][0]
        .clone();
    duplicated["checkpoint"]["decisionState"]["options"][1]["members"][0] = first_member;
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &duplicated,
        &true_hidden_hypothesis(&game, PlayerId::One),
        4_250,
    )
    .expect_err("a pile choice cannot duplicate one item and omit another");
    assert!(
        error.contains("exact disjoint partition"),
        "unexpected pile error: {error}",
    );

    for (viewer, seed) in [(PlayerId::One, 4_252), (PlayerId::Two, 4_253)] {
        let mut rebuilt = rebuild_from_truth_for_viewer(&game, viewer, seed);
        let (first, second) = match &rebuilt.pending_decisions[0].continuation {
            DecisionContinuation::ChoosePileForEffect { first, second, .. } => (first, second),
            other => panic!("pile-choice reconstruction changed continuation: {other:?}"),
        };
        assert!(
            first
                .iter()
                .chain(second)
                .all(|target| matches!(target, Target::Card(id)
                if rebuilt.players[PlayerId::One.index()]
                    .library
                    .iter()
                    .any(|card| card.id == *id))),
            "{viewer:?} reconstruction must bind every pile member to player one's library",
        );
        answer_with_first_option(&mut rebuilt);
        assert!(
            !matches!(
                rebuilt
                    .pending_decisions
                    .first()
                    .map(|pending| &pending.continuation),
                Some(DecisionContinuation::ChoosePileForEffect { .. })
            ),
            "the reconstructed pile choice must resume its nested effect",
        );
    }
}

/// Copy Artifact chooses what to be as it enters, so the seat sees a
/// replacement decision whose options are permanents, and then a permanent
/// whose characteristics come from a frozen copy rather than its own card.
#[test]
fn an_entering_copy_reconstructs_while_choosing_and_after_it_has_copied() {
    let mut game = staged_game();
    game.battlefield.push(creature(
        12_000,
        crate::card::cards::ORNITHOPTER,
        PlayerId::Two,
    ));
    let copy = card(11_000, crate::card::cards::COPY_ARTIFACT, PlayerId::One);
    let copy_id = copy.id;
    game.players[PlayerId::One.index()].hand.push(copy);
    fill_mana(&mut game, PlayerId::One, 4);

    game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: copy_id,
            choices: crate::CastChoices::default(),
            sacrifices: Vec::new(),
        },
    )
    .expect("Copy Artifact is castable");
    resolve_top_of_stack(&mut game);

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::BattlefieldEntryCopy { .. })
        ),
        "entering must ask what to copy, not {:?}",
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation)
    );
    assert_reconstructs(&game, "a permanent choosing what to enter as");

    answer_with_option_naming(&mut game, GameObjectId(12_000));
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.copy_effect.is_some()),
        "the choice must leave a copied permanent behind"
    );
    assert_reconstructs(&game, "a permanent that entered as a copy");
}

/// A permanent that named a card as it entered carries that name for the rest
/// of the game. The name is free text rather than a catalog id, so it is the
/// one piece of permanent state a locator cannot address.
#[test]
fn a_permanent_that_named_a_card_reconstructs_while_naming_and_after() {
    let mut game = staged_modern_game();
    let needle = card(11_000, crate::card::cards::PITHING_NEEDLE, PlayerId::One);
    let needle_id = needle.id;
    game.players[PlayerId::One.index()].hand.push(needle);
    fill_mana(&mut game, PlayerId::One, 4);

    game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: needle_id,
            choices: crate::CastChoices::default(),
            sacrifices: Vec::new(),
        },
    )
    .expect("Pithing Needle is castable");
    resolve_top_of_stack(&mut game);

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::BattlefieldEntryScalarChoice {
                choice: crate::card::BattlefieldEntryScalarChoiceDef {
                    destination: crate::card::BattlefieldEntryChoiceDestinationDef::CardName,
                    ..
                },
                ..
            })
        ),
        "entering must ask for a card name, not {:?}",
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation)
    );
    assert_reconstructs(&game, "a permanent choosing a card name");

    let viewer = PlayerId::One;
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let mut wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    assert!(
        wire["checkpoint"]["decisionState"]["continuation"]
            .get("destination")
            .is_none(),
        "the chosen-value destination comes from the authored replacement locator",
    );
    let mut wrong_kind = wire.clone();
    wrong_kind["decision"]["kind"] = Value::String("TriggerOrder".into());
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wrong_kind,
        &true_hidden_hypothesis(&game, viewer),
        4_243,
    )
    .expect_err("a scalar entry choice rejects another decision procedure's kind");
    assert!(
        error.contains("decision kind disagrees"),
        "unexpected error: {error}"
    );
    let hidden = true_hidden_hypothesis(&game, viewer);
    let mut spliced_destination = wire.clone();
    spliced_destination["checkpoint"]["decisionState"]["continuation"]["effect"]["ability"]["definition"] =
        Value::from(crate::card::cards::CAVERN_OF_SOULS.get());
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &spliced_destination,
        &hidden,
        4_243,
    )
    .expect_err("Cavern's creature-type destination cannot be spliced onto Pithing Needle");
    assert!(
        error.contains("locator disagrees with its replacement source"),
        "unexpected reconstruction error: {error}",
    );

    wire["decision"]["options"][0]["label"] = Value::String("Tampered Name".into());
    wire["checkpoint"]["decisionState"]["continuation"]["choices"][0] =
        Value::String("Tampered Name".into());
    let error =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 4_243)
            .expect_err("matching edited labels cannot replace the authored card-name vocabulary");
    assert!(
        error.contains("vocabulary disagrees"),
        "unexpected reconstruction error: {error}",
    );

    answer_with_first_option(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.chosen_card_name.is_some()),
        "the choice must leave a named permanent behind"
    );
    assert_reconstructs(&game, "a permanent holding a chosen card name");
}

/// A chosen player shares the scalar entry continuation but lands in typed
/// permanent state rather than free-form text.
#[test]
fn a_permanent_that_chose_a_player_reconstructs_while_choosing_and_after() {
    let mut game = staged_modern_game();
    game.put_onto_battlefield(
        PlayerId::One,
        crate::card::cards::TRUE_NAME_NEMESIS,
    )
    .expect("True-Name Nemesis is cataloged");

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::BattlefieldEntryScalarChoice {
                choice: crate::card::BattlefieldEntryScalarChoiceDef {
                    destination: crate::card::BattlefieldEntryChoiceDestinationDef::Player,
                    ..
                },
                ..
            })
        ),
        "entering must ask for a player through the scalar choice continuation",
    );
    assert_reconstructs(&game, "a permanent choosing a player");

    answer_with_first_option(&mut game);
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.card.definition == crate::card::cards::TRUE_NAME_NEMESIS
            && permanent.chosen_player == Some(PlayerId::One)
    }));
    assert_reconstructs(&game, "a permanent holding a chosen player");
}

/// Creature types use the same scalar entry-choice procedure as card names,
/// while recording the answer in a distinct typed destination.
#[test]
fn a_permanent_that_named_a_creature_type_reconstructs_while_naming_and_after() {
    let mut game = staged_modern_game();
    let cavern = card(11_100, crate::card::cards::CAVERN_OF_SOULS, PlayerId::One);
    let cavern_id = cavern.id;
    game.players[PlayerId::One.index()].hand.push(cavern);

    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: cavern_id,
            option: crate::PlayOptionId::DEFAULT,
        },
    )
    .expect("Cavern of Souls is playable");

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::BattlefieldEntryScalarChoice {
                choice: crate::card::BattlefieldEntryScalarChoiceDef {
                    destination: crate::card::BattlefieldEntryChoiceDestinationDef::CreatureType,
                    ..
                },
                ..
            })
        ),
        "entering must ask for a creature type through the scalar choice continuation"
    );
    assert_reconstructs(&game, "a permanent choosing a creature type");

    answer_with_first_option(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.chosen_creature_type.is_some()),
        "the choice must leave a permanent holding the chosen creature type"
    );
    assert_reconstructs(&game, "a permanent holding a chosen creature type");
}

#[test]
#[allow(clippy::too_many_lines)]
fn an_optional_entry_replacement_reconstructs_resumes_and_rejects_splices() {
    let mut game = staged_modern_game();
    let cackler = card(11_150, crate::card::cards::RAKDOS_CACKLER, PlayerId::One);
    let cackler_id = cackler.id;
    game.players[PlayerId::One.index()].hand.push(cackler);
    fill_mana(&mut game, PlayerId::One, 1);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == cackler_id))
        .expect("Rakdos Cackler is castable");
    game.apply(PlayerId::One, action)
        .expect("Rakdos Cackler is cast");
    resolve_top_of_stack(&mut game);

    let (context, effect) = match game
        .pending_decisions
        .first()
        .map(|pending| &pending.continuation)
    {
        Some(DecisionContinuation::BattlefieldEntryOptional { context, effect }) => {
            (*context, *effect)
        }
        other => panic!("unleash must suspend as an optional entry replacement: {other:?}"),
    };
    assert!(matches!(
        effect,
        crate::card::ReplacementEffectDef::ModifyBattlefieldEntry(
            crate::card::BattlefieldEntryModificationDef::AddCounters {
                kind: CounterKind::PlusOnePlusOne,
                amount: 1,
            }
        )
    ));
    let pending_event = game
        .pending_events
        .front()
        .expect("the prospective entry remains suspended");
    assert_eq!(
        pending_event
            .applied
            .iter()
            .filter(|source| **source == context.source)
            .count(),
        1,
        "the optional source is recorded exactly once before asking",
    );
    assert!(
        pending_event.effects.is_empty(),
        "acceptance, not suspension, queues the authored operation",
    );
    assert_reconstructs(&game, "an optional unleash entry replacement");

    let observation = game.observe(PlayerId::One);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    let hidden = true_hidden_hypothesis(&game, PlayerId::One);
    for (label, mut edited) in [
        ("effect source", wire.clone()),
        ("applied source", wire.clone()),
        ("decision options", wire.clone()),
    ] {
        if label == "effect source" {
            edited["checkpoint"]["decisionState"]["continuation"]["effect"]["ability"]["definition"] =
                Value::from(crate::card::cards::FESTERHIDE_BOAR.get());
        } else if label == "applied source" {
            edited["checkpoint"]["pendingEvents"][0]["applied"] = serde_json::json!([]);
        } else {
            edited["decision"]["options"][1]["label"] = Value::String("Tampered".into());
        }
        let error = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &edited,
            &hidden,
            4_244,
        )
        .unwrap_err();
        assert!(
            error.contains("locator disagrees")
                || error.contains("context disagrees")
                || error.contains("options disagree"),
            "unexpected {label} error: {error}",
        );
    }

    let mut accepted = rebuild_from_truth(&game, 4_245);
    answer_with_label(&mut accepted, "Accept");
    resolve_top_of_stack(&mut accepted);
    let accepted = accepted
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == crate::card::cards::RAKDOS_CACKLER)
        .expect("the accepted Cackler enters");
    assert_eq!(accepted.counters(CounterKind::PlusOnePlusOne), 1);

    let mut declined = rebuild_from_truth(&game, 4_246);
    answer_with_label(&mut declined, "Decline");
    resolve_top_of_stack(&mut declined);
    let declined = declined
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == crate::card::cards::RAKDOS_CACKLER)
        .expect("the declined Cackler enters");
    assert_eq!(declined.counters(CounterKind::PlusOnePlusOne), 0);
}
