#[test]
fn resolved_continuous_effects_round_trip_order_provenance_expiration_and_frozen_values() {
    let mut game = crate::game::tests::ready_game();
    let target = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SAVANNAH_LIONS)
        .expect("the checkpoint target enters");
    let (source, modify_definition, granted, grant_definition) =
        composite_modify_and_grant(&game.catalog, target);
    let (dynamic_source, dynamic_definition) = dynamic_modify(&game.catalog, target);
    let (subtype_source, subtype_definition, subtype_operation) =
        subtype_change(&game.catalog, target);
    let shared_timestamp = ContinuousEffectTimestamp(90_000);
    let expected = vec![
        ResolvedContinuousEffect {
            definition: modify_definition,
            source,
            timestamp: shared_timestamp,
            component_order: 0,
            expiration: ContinuousEffectExpiration::EndOfTurn,
            kind: ResolvedContinuousEffectKind::PowerToughness(
                ResolvedPowerToughnessOperation::Modify {
                    power: 4,
                    toughness: -3,
                },
            ),
        },
        ResolvedContinuousEffect {
            definition: grant_definition,
            source,
            timestamp: shared_timestamp,
            component_order: 1,
            expiration: ContinuousEffectExpiration::UpkeepOf(PlayerId::Two),
            kind: ResolvedContinuousEffectKind::Abilities(ResolvedAbilityOperation::Add {
                ability: granted,
                grant: GrantId(7),
            }),
        },
        ResolvedContinuousEffect {
            definition: dynamic_definition,
            source: dynamic_source,
            timestamp: ContinuousEffectTimestamp(90_001),
            component_order: 0,
            expiration: ContinuousEffectExpiration::Never,
            kind: ResolvedContinuousEffectKind::PowerToughness(
                ResolvedPowerToughnessOperation::Modify {
                    power: 12,
                    toughness: 9,
                },
            ),
        },
        ResolvedContinuousEffect {
            definition: subtype_definition,
            source: subtype_source,
            timestamp: ContinuousEffectTimestamp(90_002),
            component_order: 0,
            expiration: ContinuousEffectExpiration::EndOfTurn,
            kind: ResolvedContinuousEffectKind::Subtypes(subtype_operation),
        },
    ];
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == target)
        .expect("the target remains on the battlefield")
        .resolved_continuous_effects = expected.clone();

    let (wire, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 60_001);
    let checkpoint_permanent = wire["checkpoint"]["battlefield"]
        .as_array()
        .expect("the checkpoint has a battlefield")
        .iter()
        .find(|permanent| permanent["objectId"] == target.0)
        .expect("the target is checkpointed");
    assert!(checkpoint_permanent.get("powerBonus").is_none());
    assert!(checkpoint_permanent.get("animation").is_none());
    assert_eq!(
        checkpoint_permanent["resolvedContinuousEffects"][0]["timestamp"],
        shared_timestamp.0
    );
    assert_eq!(
        checkpoint_permanent["resolvedContinuousEffects"][1]["componentOrder"],
        1
    );
    assert_eq!(
        rebuilt
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == target)
            .expect("the target reconstructs")
            .resolved_continuous_effects,
        expected,
        "resolution-time values and source provenance must not be recomputed"
    );
}

#[test]
fn resolved_continuous_effect_locator_operation_mismatches_fail_closed() {
    let mut game = crate::game::tests::ready_game();
    let target = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SAVANNAH_LIONS)
        .expect("the checkpoint target enters");
    let (source, definition) = dynamic_modify(&game.catalog, target);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == target)
        .expect("the target remains on the battlefield")
        .resolved_continuous_effects
        .push(ResolvedContinuousEffect {
            definition,
            source,
            timestamp: ContinuousEffectTimestamp(91_000),
            component_order: 0,
            expiration: ContinuousEffectExpiration::Never,
            kind: ResolvedContinuousEffectKind::PowerToughness(
                ResolvedPowerToughnessOperation::Modify {
                    power: 8,
                    toughness: 5,
                },
            ),
        });
    let viewer = PlayerId::One;
    let (mut wire, _) = rebuild_current_checkpoint(&game, viewer, 60_002);
    let permanent = wire["checkpoint"]["battlefield"]
        .as_array_mut()
        .expect("the checkpoint has a battlefield")
        .iter_mut()
        .find(|permanent| permanent["objectId"] == target.0)
        .expect("the target is checkpointed");
    permanent["resolvedContinuousEffects"][0]["operation"] = json!({ "kind": "abilityRemove" });
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        60_003,
    )
    .expect_err("an operation that disagrees with its locator must fail");
    assert!(
        error.contains("does not match its authored locator"),
        "unexpected reconstruction error: {error}"
    );
}

#[test]
fn resolved_continuous_effect_source_splices_fail_closed_on_import_and_export() {
    let mut game = crate::game::tests::ready_game();
    let target = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SAVANNAH_LIONS)
        .expect("the checkpoint target enters");
    let (source, definition) = dynamic_modify(&game.catalog, target);
    let unrelated_source = source_without_applied_effect(&game.catalog, target, definition);
    let effect = ResolvedContinuousEffect {
        definition,
        source,
        timestamp: ContinuousEffectTimestamp(91_001),
        component_order: 0,
        expiration: ContinuousEffectExpiration::EndOfTurn,
        kind: ResolvedContinuousEffectKind::PowerToughness(
            ResolvedPowerToughnessOperation::Modify {
                power: 6,
                toughness: 2,
            },
        ),
    };
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == target)
        .expect("the target remains on the battlefield")
        .resolved_continuous_effects
        .push(effect);

    let viewer = PlayerId::One;
    let (mut wire, rebuilt) = rebuild_current_checkpoint(&game, viewer, 60_004);
    assert_eq!(
        rebuilt
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == target)
            .expect("the honest target reconstructs")
            .resolved_continuous_effects,
        vec![effect],
        "honest source-anchored object rules round trip",
    );
    let permanent = wire["checkpoint"]["battlefield"]
        .as_array_mut()
        .expect("the checkpoint has a battlefield")
        .iter_mut()
        .find(|permanent| permanent["objectId"] == target.0)
        .expect("the target is checkpointed");
    splice_printed_source_ability(
        &mut permanent["resolvedContinuousEffects"][0]["source"]["ability"],
        unrelated_source,
    );
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        60_005,
    )
    .expect_err("an object rule locator cannot be spliced onto another source ability");
    assert!(
        error.contains("locator disagrees with its source ability"),
        "unexpected reconstruction error: {error}",
    );

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == target)
        .expect("the target remains on the battlefield")
        .resolved_continuous_effects[0]
        .source = unrelated_source;
    let checkpoint = game.checkpoint_json(viewer);
    assert_eq!(checkpoint["hasDeferredState"], true);
    let permanent = checkpoint["battlefield"]
        .as_array()
        .expect("the checkpoint has a battlefield")
        .iter()
        .find(|permanent| permanent["objectId"] == target.0)
        .expect("the target is checkpointed");
    assert_eq!(
        permanent["resolvedContinuousEffects"],
        json!([]),
        "an unanchored object rule is omitted rather than attributed catalog-wide",
    );
}

#[test]
fn overlapping_resolved_effect_expirations_round_trip_and_cleanup_independently() {
    let mut game = crate::game::tests::ready_game();
    let target = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SAVANNAH_LIONS)
        .expect("the checkpoint target enters");
    let (source, definition) = dynamic_modify(&game.catalog, target);
    let permanent = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == target)
        .expect("the target remains on the battlefield");
    permanent.resolved_continuous_effects = vec![
        ResolvedContinuousEffect {
            definition,
            source,
            timestamp: ContinuousEffectTimestamp(92_000),
            component_order: 0,
            expiration: ContinuousEffectExpiration::Never,
            kind: ResolvedContinuousEffectKind::PowerToughness(
                ResolvedPowerToughnessOperation::Modify {
                    power: 2,
                    toughness: 2,
                },
            ),
        },
        ResolvedContinuousEffect {
            definition,
            source,
            timestamp: ContinuousEffectTimestamp(92_001),
            component_order: 0,
            expiration: ContinuousEffectExpiration::EndOfTurn,
            kind: ResolvedContinuousEffectKind::PowerToughness(
                ResolvedPowerToughnessOperation::Modify {
                    power: 5,
                    toughness: 5,
                },
            ),
        },
    ];

    let (_, mut rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 60_004);
    rebuilt.finish_cleanup();
    let remaining = &rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == target)
        .expect("the target survives cleanup")
        .resolved_continuous_effects;
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].expiration, ContinuousEffectExpiration::Never);
    assert!(matches!(
        remaining[0].kind,
        ResolvedContinuousEffectKind::PowerToughness(ResolvedPowerToughnessOperation::Modify {
            power: 2,
            toughness: 2
        })
    ));
}

fn unlocated_resolved_effect(object: GameObjectId) -> ResolvedContinuousEffect {
    ResolvedContinuousEffect {
        definition: AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate),
        source: AbilitySourceRef {
            object,
            ability: AbilityOrigin::Printed {
                definition: crate::card::cards::SAVANNAH_LIONS,
                part: CardPartId::PRIMARY,
                ability: AbilityId::PRIMARY,
            },
        },
        timestamp: ContinuousEffectTimestamp(93_000),
        component_order: 0,
        expiration: ContinuousEffectExpiration::Never,
        kind: ResolvedContinuousEffectKind::PowerToughness(
            ResolvedPowerToughnessOperation::Modify {
                power: 1,
                toughness: 1,
            },
        ),
    }
}

#[test]
fn unlocated_battlefield_pending_and_retired_continuous_effects_mark_checkpoints_deferred() {
    let mut battlefield = crate::game::tests::ready_game();
    let object = battlefield
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SAVANNAH_LIONS)
        .expect("the checkpoint target enters");
    battlefield
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == object)
        .expect("the target remains on the battlefield")
        .resolved_continuous_effects
        .push(unlocated_resolved_effect(object));
    assert_eq!(
        battlefield.checkpoint_json(PlayerId::One)["hasDeferredState"],
        true,
        "an unlocated live permanent must fail closed"
    );

    let mut pending = crate::game::tests::ready_game();
    let pending_id = GameObjectId(93_100);
    let mut pending_permanent = crate::game::tests::creature(
        pending_id.0,
        crate::card::cards::SAVANNAH_LIONS,
        PlayerId::One,
    );
    pending_permanent
        .resolved_continuous_effects
        .push(unlocated_resolved_effect(pending_id));
    pending.pending_events.push_back(PendingEvent {
        event: ReplaceableEvent::BattlefieldEntry(PendingBattlefieldEntry {
            permanent: pending_permanent,
            from: ZoneKind::Stack,
            completion: EntryCompletion::None,
            redirected_to: None,
        }),
        applied: Vec::new(),
        effects: Vec::new(),
    });
    assert_eq!(
        pending.checkpoint_json(PlayerId::One)["hasDeferredState"],
        true,
        "an unlocated prospective permanent must fail closed"
    );

    let mut retired = crate::game::tests::ready_game();
    let witness = retired
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SAVANNAH_LIONS)
        .expect("the LKI witness enters");
    let retired_id = GameObjectId(93_200);
    let mut retired_permanent = crate::game::tests::creature(
        retired_id.0,
        crate::card::cards::SAVANNAH_LIONS,
        PlayerId::One,
    );
    retired_permanent
        .resolved_continuous_effects
        .push(unlocated_resolved_effect(retired_id));
    retired.retired_objects.insert(
        retired_id,
        RetiredObject::Permanent {
            permanent: Box::new(retired_permanent),
            power: Some(2),
            toughness: Some(1),
            mana_value: 1,
            keywords: Vec::new(),
        },
    );
    retired
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == witness)
        .expect("the witness remains on the battlefield")
        .damage_sources
        .push(retired_id);
    assert_eq!(
        retired.checkpoint_json(PlayerId::One)["hasDeferredState"],
        true,
        "an unlocated retired permanent required for LKI must fail closed"
    );
}

#[test]
fn pending_and_retired_permanents_round_trip_resolved_continuous_effects() {
    let mut game = crate::game::tests::ready_game();
    let source_object = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SAVANNAH_LIONS)
        .expect("the effect source enters");
    let (source, definition) = dynamic_modify(&game.catalog, source_object);
    let effect = ResolvedContinuousEffect {
        definition,
        source,
        timestamp: ContinuousEffectTimestamp(94_000),
        component_order: 3,
        expiration: ContinuousEffectExpiration::TurnOf {
            player: PlayerId::Two,
            turn: 17,
        },
        kind: ResolvedContinuousEffectKind::PowerToughness(
            ResolvedPowerToughnessOperation::Modify {
                power: -4,
                toughness: 11,
            },
        ),
    };

    let pending_id = GameObjectId(94_100);
    let mut pending_permanent = crate::game::tests::creature(
        pending_id.0,
        crate::card::cards::SAVANNAH_LIONS,
        PlayerId::One,
    );
    pending_permanent.resolved_continuous_effects.push(effect);
    game.pending_events.push_back(PendingEvent {
        event: ReplaceableEvent::BattlefieldEntry(PendingBattlefieldEntry {
            permanent: pending_permanent,
            from: ZoneKind::Stack,
            completion: EntryCompletion::None,
            redirected_to: None,
        }),
        applied: Vec::new(),
        effects: Vec::new(),
    });

    let retired_id = GameObjectId(94_200);
    let mut retired_permanent = crate::game::tests::creature(
        retired_id.0,
        crate::card::cards::SAVANNAH_LIONS,
        PlayerId::One,
    );
    retired_permanent.resolved_continuous_effects.push(effect);
    game.retired_objects.insert(
        retired_id,
        RetiredObject::Permanent {
            permanent: Box::new(retired_permanent),
            power: Some(2),
            toughness: Some(1),
            mana_value: 1,
            keywords: Vec::new(),
        },
    );
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source_object)
        .expect("the LKI witness remains on the battlefield")
        .damage_sources
        .push(retired_id);

    let (_, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 60_005);
    let pending = rebuilt
        .pending_events
        .front()
        .expect("the prospective battlefield entry reconstructs");
    let ReplaceableEvent::BattlefieldEntry(pending_entry) = &pending.event;
    assert_eq!(
        pending_entry.permanent.resolved_continuous_effects,
        vec![effect]
    );

    let RetiredObject::Permanent { permanent, .. } = rebuilt
        .retired_objects
        .get(&retired_id)
        .expect("the referenced retired permanent reconstructs")
    else {
        panic!("the reconstructed retired object remains a permanent");
    };
    assert_eq!(permanent.resolved_continuous_effects, vec![effect]);
}

/// A trigger already on the stack names the object that was on the
/// battlefield, and "return it" has to reach the card that object became.
/// Without the link the checkpoint would restore a game where the return
/// quietly does nothing.
#[test]
fn a_dying_creatures_successor_survives_a_checkpoint() {
    let mut game = crate::game::tests::ready_game();
    let strider = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::MORTUS_STRIDER)
        .expect("the Strider enters");
    game.move_permanents_to_graveyard(&[strider]);
    assert!(
        !game.pending_triggers.is_empty(),
        "its own death trigger is waiting",
    );

    let (wire, rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 60_101);
    assert!(
        wire["checkpoint"]["successors"].is_array(),
        "the link is written down",
    );
    assert_eq!(
        rebuilt.live_object_target(strider),
        game.live_object_target(strider),
        "and the restored game reaches the same card",
    );
    assert!(
        rebuilt.live_object_target(strider).is_some(),
        "which is the card it became, not nothing",
    );
}
