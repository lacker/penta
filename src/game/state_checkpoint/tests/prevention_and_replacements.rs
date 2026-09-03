static HALF_DAMAGE_ROUNDED_DOWN: crate::card::HalvedValueDef =
    crate::card::HalvedValueDef::new(ValueDef::DamageEventAmount, crate::card::RoundingDef::Down);

fn keyword_from_ability(ability: &AbilityDef) -> KeywordAbility {
    let crate::card::DeclarativeAbilityDef::Keyword(keyword) = ability.definition else {
        unreachable!("protection constructor always returns a keyword ability")
    };
    keyword
}

#[test]
fn every_runtime_keyword_has_a_stable_checkpoint_round_trip() {
    let mut keywords = vec![
        KeywordAbility::Flying,
        KeywordAbility::Trample,
        KeywordAbility::Haste,
        KeywordAbility::FirstStrike,
        KeywordAbility::DoubleStrike,
        KeywordAbility::Banding,
        KeywordAbility::Vigilance,
        KeywordAbility::Defender,
        KeywordAbility::Deathtouch,
        KeywordAbility::Lifelink,
        KeywordAbility::Reach,
        KeywordAbility::Flash,
        KeywordAbility::Hexproof,
        KeywordAbility::Shroud,
        KeywordAbility::Intimidate,
        KeywordAbility::Undying,
        KeywordAbility::Indestructible,
        KeywordAbility::AttacksEachCombatIfAble,
        KeywordAbility::Compleated,
    ];
    keywords.extend(crate::card::BasicLandType::ALL.map(KeywordAbility::Landwalk));
    keywords.extend(
        [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
            ManaColor::Colorless,
        ]
        .map(|color| {
            keyword_from_ability(&crate::card::abilities::protection_from_color(color))
        }),
    );
    keywords.extend([
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Subtype("Zombie")),
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Subtype("Vampire")),
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Subtype("Werewolf")),
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::Subtype("Vampire"),
            ObjectPredicateDef::Subtype("Werewolf"),
            ObjectPredicateDef::Subtype("Zombie"),
        ])),
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::HasType(
            crate::card::CardType::Creature,
        )),
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::ColorCount(0),
            ObjectPredicateDef::ColorCount(1),
        ]))),
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(crate::card::CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
        ])),
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::HasType(
            crate::card::CardType::Enchantment,
        )),
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::All(&[
            ObjectPredicateDef::Spell,
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(crate::card::CardType::Instant),
                ObjectPredicateDef::HasType(crate::card::CardType::Sorcery),
            ]),
        ])),
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::All(&[
            ObjectPredicateDef::Spell,
            ObjectPredicateDef::Not(&ObjectPredicateDef::ColorCount(0)),
        ])),
        KeywordAbility::ProtectionFrom(&ObjectPredicateDef::ControlledBy(
            PlayerRelation::ChosenPlayer,
        )),
    ]);
    for keyword in keywords {
        assert_eq!(parse_keyword(keyword_snapshot(keyword)), keyword);
    }
}

#[test]
fn checkpoint_redacts_opposing_drawn_card_ids() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let deck = crate::Deck {
        main: vec![crate::card::cards::MOUNTAIN; 60],
        sideboard: Vec::new(),
    };
    let mut game = Game::new(catalog, [deck.clone(), deck], 41).expect("game starts");
    let hidden = game.players[PlayerId::Two.index()].hand[0].id;
    game.drawn_this_turn[PlayerId::Two.index()] = vec![hidden];

    let checkpoint = game.checkpoint_json(PlayerId::One);
    assert_eq!(checkpoint["drawnThisTurn"][1], json!([]));

    let own = game.players[PlayerId::One.index()].hand[0].id;
    game.drawn_this_turn[PlayerId::One.index()] = vec![own];
    let checkpoint = game.checkpoint_json(PlayerId::One);
    assert_eq!(checkpoint["drawnThisTurn"][0], json!([own.0]));
}

#[test]
fn checkpoint_json_is_a_projection_of_one_typed_snapshot_schema() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let deck = crate::Deck {
        main: vec![crate::card::cards::MOUNTAIN; 60],
        sideboard: Vec::new(),
    };
    let game = Game::new(catalog, [deck.clone(), deck], 42).expect("game starts");
    let json = game.checkpoint_json(PlayerId::One);
    let snapshot: GameSnapshot =
        serde_json::from_value(json.clone()).expect("checkpoint matches GameSnapshot");
    assert_eq!(
        serde_json::to_value(snapshot).expect("snapshot serializes"),
        json
    );

    let mut additive = json.clone();
    additive["futureBookkeeping"] = json!({ "ignored": true });
    serde_json::from_value::<GameSnapshot>(additive)
        .expect("typed snapshots ignore unknown additive object members");

    let mut malformed = json;
    malformed
        .as_object_mut()
        .expect("snapshot object")
        .remove("turnsStarted");
    assert!(
        serde_json::from_value::<GameSnapshot>(malformed)
            .expect_err("missing required state must fail")
            .to_string()
            .contains("turnsStarted")
    );
}

#[test]
fn checkpoint_encodes_draw_replacement_and_procedure_state() {
    let mut game = crate::game::tests::ready_game();
    game.pending_procedures
        .push_back(crate::game::PendingProcedure::FinishStepAdvance);
    game.defer_empty_library_loss = true;
    let viewer = game.decision_player().expect("the game awaits an action");
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
    assert_eq!(wire["checkpoint"]["deferEmptyLibraryLoss"], true);
    assert_eq!(
        wire["checkpoint"]["pendingProcedures"][0]["kind"],
        "finishStepAdvance"
    );
    let rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        4_243,
    )
    .expect("procedure state reconstructs");
    assert!(rebuilt.defer_empty_library_loss);
    assert!(matches!(
        rebuilt.pending_procedures.front(),
        Some(crate::game::PendingProcedure::FinishStepAdvance)
    ));
}

#[test]
fn island_sanctuary_draw_choice_and_attack_restriction_survive_checkpoint_round_trip() {
    let mut game = crate::game::tests::ready_game();
    game.put_onto_battlefield(PlayerId::One, crate::card::cards::ISLAND_SANCTUARY)
        .expect("Island Sanctuary enters");
    game.step = Step::Draw;
    game.active_player = PlayerId::One;
    game.players[0].library = vec![crate::game::tests::card(
        77_001,
        crate::card::cards::PLAINS,
        PlayerId::One,
    )];

    assert_eq!(game.draw_card(PlayerId::One), None);
    let checkpoint = game.checkpoint_json(PlayerId::One);
    assert_eq!(
        checkpoint["hasDeferredState"],
        json!(false),
        "{checkpoint:#}"
    );
    let (_, rebuilt_choice) = rebuild_current_checkpoint(&game, PlayerId::One, 4_250);
    let DecisionContinuation::DrawReplacement { replacements, .. } =
        &rebuilt_choice.pending_decisions[0].continuation
    else {
        panic!("draw replacement choice reconstructs as the same continuation");
    };
    assert_eq!(replacements.len(), 1);
    assert!(replacements[0].optional);
    assert!(!replacements[0].installed);

    let decision = game.pending_decisions[0].observation.clone();
    let skip = decision
        .options
        .iter()
        .find(|option| option.ability_text.is_some())
        .expect("the Sanctuary replacement is offered")
        .id;
    game.choose_decision(PlayerId::One, decision.id, &[skip]);
    let (_, rebuilt_restriction) = rebuild_current_checkpoint(&game, PlayerId::One, 4_251);
    assert_eq!(
        rebuilt_restriction.resolved_attack_restrictions,
        game.resolved_attack_restrictions
    );
}

#[test]
fn chains_replacement_and_discard_choices_survive_checkpoint_round_trip() {
    let mut game = crate::game::tests::ready_game();
    game.put_onto_battlefield(
        PlayerId::One,
        crate::card::cards::CHAINS_OF_MEPHISTOPHELES,
    )
    .expect("first Chains enters");
    game.put_onto_battlefield(
        PlayerId::Two,
        crate::card::cards::CHAINS_OF_MEPHISTOPHELES,
    )
    .expect("second Chains enters");
    game.step = Step::PrecombatMain;
    game.players[0].hand = vec![
        crate::game::tests::card(77_010, crate::card::cards::PLAINS, PlayerId::One),
        crate::game::tests::card(77_011, crate::card::cards::MOUNTAIN, PlayerId::One),
    ];
    game.players[0].library = vec![crate::game::tests::card(
        77_012,
        crate::card::cards::FOREST,
        PlayerId::One,
    )];

    assert_eq!(game.draw_card(PlayerId::One), None);
    let (_, rebuilt_choice) = rebuild_current_checkpoint(&game, PlayerId::One, 4_252);
    let DecisionContinuation::DrawReplacement {
        applied,
        replacements,
        ..
    } = &rebuilt_choice.pending_decisions[0].continuation
    else {
        panic!("Chains replacement choice reconstructs");
    };
    assert!(applied.is_empty());
    assert_eq!(replacements.len(), 2);
    assert!(replacements.iter().all(|replacement| matches!(
        replacement.effect.effect,
        crate::card::EffectDef::Discard { .. }
    )));

    let replacement = game.pending_decisions[0].observation.clone();
    game.choose_decision(PlayerId::One, replacement.id, &[1]);
    assert!(matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::DiscardForEffect {
            follow_up: Some(_),
            ..
        }
    ));
    let (_, mut rebuilt_discard) = rebuild_current_checkpoint(&game, PlayerId::One, 4_253);
    let discard = rebuilt_discard.pending_decisions[0].observation.clone();
    rebuilt_discard.choose_decision(PlayerId::One, discard.id, &[discard.options[0].id]);

    assert_eq!(rebuilt_discard.players[0].graveyard.len(), 2);
    assert_eq!(rebuilt_discard.players[0].hand.len(), 1);
    assert!(rebuilt_discard.players[0].library.is_empty());
    assert_eq!(rebuilt_discard.cards_drawn_this_turn[0], 1);
}

#[test]
fn sylvan_library_for_each_payment_resumes_after_checkpoint_round_trip() {
    let mut game = crate::game::tests::ready_game();
    game.turn = 2;
    game.step = Step::Upkeep;
    game.put_onto_battlefield(PlayerId::One, crate::card::cards::SYLVAN_LIBRARY)
        .expect("Sylvan Library enters");
    game.players[0].library = vec![
        crate::game::tests::card(77_010, crate::card::cards::PLAINS, PlayerId::One),
        crate::game::tests::card(77_011, crate::card::cards::MOUNTAIN, PlayerId::One),
        crate::game::tests::card(77_012, crate::card::cards::FOREST, PlayerId::One),
    ];

    game.advance_step();
    let first = game.priority;
    game.apply(first, Action::PassPriority).unwrap();
    game.apply(first.opponent(), Action::PassPriority).unwrap();
    crate::game::tests::pass_until_decision(&mut game);
    let offer = game.pending_decisions[0].observation.clone();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
            options: vec![1],
        },
    )
    .expect("the extra draws are accepted");
    let choice = game.pending_decisions[0].observation.clone();
    assert_eq!(
        choice.order_semantics,
        Some(DecisionOrderSemantics::Resolution)
    );
    let mut chosen = choice
        .options
        .iter()
        .take(2)
        .map(|option| option.id)
        .collect::<Vec<_>>();
    chosen.reverse();
    let expected_order = chosen
        .iter()
        .filter_map(|id| {
            choice
                .options
                .iter()
                .find(|option| option.id == *id)
                .and_then(|option| option.card.map(|(card, _)| Target::Card(card)))
        })
        .collect::<Vec<_>>();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: choice.id,
            options: chosen,
        },
    )
    .expect("two drawn cards are chosen");

    assert!(matches!(
        game.pending_procedures.front(),
        Some(crate::game::PendingProcedure::ForEachInBinding { next: 1, .. })
    ));
    let Some(crate::game::PendingProcedure::ForEachInBinding { context, .. }) =
        game.pending_procedures.front()
    else {
        unreachable!();
    };
    assert_eq!(
        context.object_group(crate::ParentBinding),
        expected_order
    );
    let (_, mut rebuilt) = rebuild_current_checkpoint(&game, PlayerId::One, 4_252);
    assert!(matches!(
        rebuilt.pending_procedures.front(),
        Some(crate::game::PendingProcedure::ForEachInBinding { next: 1, .. })
    ));
    let payment = rebuilt.pending_decisions[0].observation.clone();
    rebuilt
        .apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: payment.id,
                options: vec![0],
            },
        )
        .expect("the reconstructed first payment may be declined");
    assert!(!rebuilt.pending_decisions.is_empty());
}

#[test]
#[allow(clippy::too_many_lines)]
fn resolved_prevention_and_prohibitions_survive_checkpoint_round_trip() {
    let mut game = crate::game::tests::ready_game();
    let source = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::DARK_SPHERE)
        .expect("prevention source enters");
    let prohibited = game
        .put_onto_battlefield(PlayerId::Two, crate::card::cards::ATOG)
        .expect("prohibited creature enters");
    let rule = AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate);
    let rule_locator = ability_locator(&game.catalog, |ability| {
        semantics::applied_effects(ability).contains(&rule)
    })
    .expect("the catalog has a cannot-regenerate rule");
    let rule_source = source_for_locator(source, &rule_locator);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == prohibited)
        .expect("the prohibited creature is present")
        .resolved_continuous_effects
        .push(ResolvedContinuousEffect {
            definition: rule,
            source: rule_source,
            timestamp: ContinuousEffectTimestamp(16),
            component_order: 0,
            expiration: ContinuousEffectExpiration::EndOfTurn,
            kind: ResolvedContinuousEffectKind::Rule(AppliedRuleDef::CannotRegenerate),
        });
    let source_ability = AbilitySourceRef {
        object: source,
        ability: AbilityOrigin::Printed {
            definition: crate::card::cards::DARK_SPHERE,
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        },
    };
    game.damage_preventions = vec![
        ResolvedDamagePrevention {
            source: ResolvedDamageSourceMatcher::Any,
            recipient: ResolvedDamageRecipientMatcher::PlayerAndCreaturesControlledBy(
                PlayerId::One,
            ),
            combat_only: false,
            capacity: ResolvedDamagePreventionCapacity::Unlimited,
            amount: ValueDef::DamageEventAmount,
            gain_life: None,
            source_ability,
            timestamp: ContinuousEffectTimestamp(17),
            expiration: ContinuousEffectExpiration::EndOfTurn,
        },
        ResolvedDamagePrevention {
            source: ResolvedDamageSourceMatcher::Except(source),
            recipient: ResolvedDamageRecipientMatcher::Exact(Target::Player(PlayerId::Two)),
            combat_only: true,
            capacity: ResolvedDamagePreventionCapacity::Events(1),
            amount: ValueDef::Halved(&HALF_DAMAGE_ROUNDED_DOWN),
            gain_life: Some(PlayerId::One),
            source_ability,
            timestamp: ContinuousEffectTimestamp(18),
            expiration: ContinuousEffectExpiration::EndOfTurn,
        },
    ];
    let play_rule = AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
        PlayActionMatcherDef::CastSpell,
        ObjectPredicateDef::Any,
    )));
    let play_locator = ability_locator(&game.catalog, |ability| {
        semantics::applied_effects(ability).contains(&play_rule)
    })
    .expect("the catalog has a play prohibition");
    game.resolved_play_restrictions
        .push(ResolvedPlayRestriction {
            definition: play_rule,
            source: source_for_locator(source, &play_locator),
            affected_player: PlayerId::Two,
            timestamp: ContinuousEffectTimestamp(19),
            component_order: 0,
            expiration: ContinuousEffectExpiration::EndOfTurn,
            restriction: PlayRestrictionDef::new(
                PlayActionMatcherDef::CastSpell,
                ObjectPredicateDef::Any,
            ),
        });

    let viewer = game.decision_player().expect("the game awaits an action");
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    let mut rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        4_244,
    )
    .expect("duration-scoped prevention state reconstructs");
    assert_eq!(rebuilt.damage_preventions, game.damage_preventions);
    assert_eq!(
        rebuilt.resolved_play_restrictions,
        game.resolved_play_restrictions
    );
    let rebuilt_prohibited = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == prohibited)
        .expect("the prohibited creature reconstructs");
    assert!(rebuilt.has_applied_rule(rebuilt_prohibited, AppliedRuleDef::CannotRegenerate,));
    rebuilt.finish_cleanup();
    assert!(rebuilt.damage_preventions.is_empty());
    assert!(rebuilt.resolved_play_restrictions.is_empty());
    let rebuilt_prohibited = rebuilt
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == prohibited)
        .expect("the creature survives cleanup");
    assert!(!rebuilt.has_applied_rule(rebuilt_prohibited, AppliedRuleDef::CannotRegenerate,));
}

#[test]
fn inconsistent_resolved_play_restrictions_fail_checkpoint_export_closed() {
    let mut game = crate::game::tests::ready_game();
    let source = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SAVANNAH_LIONS)
        .expect("the restriction source enters");
    let definition = AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
        PlayActionMatcherDef::CastSpell,
        ObjectPredicateDef::Any,
    )));
    let locator = ability_locator(&game.catalog, |ability| {
        semantics::applied_effects(ability).contains(&definition)
    })
    .expect("the catalog has a play prohibition");
    game.resolved_play_restrictions
        .push(ResolvedPlayRestriction {
            definition,
            source: source_for_locator(source, &locator),
            affected_player: PlayerId::Two,
            timestamp: ContinuousEffectTimestamp(19),
            component_order: 0,
            expiration: ContinuousEffectExpiration::EndOfTurn,
            restriction: PlayRestrictionDef::new(
                PlayActionMatcherDef::PlayLand,
                ObjectPredicateDef::Any,
            ),
        });

    assert_eq!(
        game.checkpoint_json(PlayerId::One)["hasDeferredState"],
        true,
        "a frozen rule that disagrees with its authored definition is not reconstructible",
    );
}

#[test]
fn resolved_play_restriction_source_splices_fail_closed_on_import_and_export() {
    let mut game = crate::game::tests::ready_game();
    let source_object = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SAVANNAH_LIONS)
        .expect("the restriction source enters");
    let restriction = PlayRestrictionDef::new(
        PlayActionMatcherDef::CastSpell,
        ObjectPredicateDef::Any,
    );
    let definition = AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(restriction));
    let locator = ability_locator(&game.catalog, |ability| {
        semantics::applied_effects(ability).contains(&definition)
    })
    .expect("the catalog has a play prohibition");
    let source = source_for_locator(source_object, &locator);
    let unrelated_source = source_without_applied_effect(&game.catalog, source_object, definition);
    let resolved = ResolvedPlayRestriction {
        definition,
        source,
        affected_player: PlayerId::Two,
        timestamp: ContinuousEffectTimestamp(19),
        component_order: 0,
        expiration: ContinuousEffectExpiration::EndOfTurn,
        restriction,
    };
    game.resolved_play_restrictions.push(resolved);

    let viewer = PlayerId::One;
    let (mut wire, rebuilt) = rebuild_current_checkpoint(&game, viewer, 60_006);
    assert_eq!(
        rebuilt.resolved_play_restrictions,
        vec![resolved],
        "honest source-anchored player rules round trip",
    );
    splice_printed_source_ability(
        &mut wire["checkpoint"]["resolvedPlayRestrictions"][0]["source"]["ability"],
        unrelated_source,
    );
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        60_007,
    )
    .expect_err("a player-rule locator cannot be spliced onto another source ability");
    assert!(
        error.contains("locator disagrees with its source ability"),
        "unexpected reconstruction error: {error}",
    );

    game.resolved_play_restrictions[0].source = unrelated_source;
    let checkpoint = game.checkpoint_json(viewer);
    assert_eq!(checkpoint["hasDeferredState"], true);
    assert_eq!(
        checkpoint["resolvedPlayRestrictions"],
        json!([]),
        "an unanchored player rule is omitted rather than attributed catalog-wide",
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn resolved_prevention_retains_controller_lki_and_rejects_spliced_provenance() {
    static MATCHING_PREVENTION_ABILITY: AbilityDef = AbilityDef::activated(
        "Prevent damage from sources you control this turn.",
        &[],
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef {
                source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::ControlledBy(
                    PlayerRelation::You,
                )),
                ..DamageEventMatcherDef::ANY
            }),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    );

    let definition_id = CardDefinitionId::new(10_064);
    let mut definition = CardDefinition::new(
        definition_id,
        "Checkpoint Prevention Source",
        CardSet::Magic2014,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_artifact(crate::ManaCost::new(0, 0))
        .with_ability(MATCHING_PREVENTION_ABILITY);
    let composition = CardComposition::single(definition.name.clone(), definition.rules);
    definition.parts = composition.parts;
    definition.structure = composition.structure;
    definition.play_options = composition.play_options;

    let mut game = crate::game::tests::ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).expect("the synthetic prevention is cataloged");
    let source = game
        .put_onto_battlefield(PlayerId::One, definition_id)
        .expect("the prevention source enters");
    let damage_source = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::SAVANNAH_LIONS)
        .expect("the matching damage source enters");
    let source_ability = AbilitySourceRef {
        object: source,
        ability: AbilityOrigin::Printed {
            definition: definition_id,
            part: CardPartId::PRIMARY,
            ability: AbilityId::PRIMARY,
        },
    };
    game.damage_preventions.push(ResolvedDamagePrevention {
        source: ResolvedDamageSourceMatcher::Matching {
            predicate: ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            relative_to: source,
        },
        recipient: ResolvedDamageRecipientMatcher::Any,
        combat_only: false,
        capacity: ResolvedDamagePreventionCapacity::Unlimited,
        amount: ValueDef::DamageEventAmount,
        gain_life: None,
        source_ability,
        timestamp: ContinuousEffectTimestamp(19),
        expiration: ContinuousEffectExpiration::EndOfTurn,
    });
    game.sacrifice_permanent(source);
    assert!(game.retired_objects.contains_key(&source));

    let (viewer, wire) = checkpoint_wire(&game);
    assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
    assert!(
        wire["checkpoint"]["retiredObjects"]
            .as_array()
            .expect("retired objects are serialized")
            .iter()
            .any(|retired| retired["permanent"]["state"]["objectId"] == source.0)
    );
    let hidden = true_hidden_hypothesis(&game, viewer);

    let mut malformed = wire.clone();
    malformed["checkpoint"]["damagePreventions"][0]["sourceAbility"]["ability"]["definition"] =
        json!(crate::card::cards::LIGHTNING_BOLT.get());
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &malformed,
        &hidden,
        4_245,
    )
    .expect_err("a prevention matcher cannot be spliced onto another source ability");
    assert!(
        error.contains("source ability"),
        "unexpected error: {error}"
    );

    let mut rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 4_246)
            .expect("the retired prevention source reconstructs");
    assert_eq!(rebuilt.controller_of_object(source), Some(PlayerId::One));
    assert_eq!(
        rebuilt.damage_target_from(Some(damage_source), Some(Target::Player(PlayerId::Two)), 3,),
        0,
        "the retained source LKI supplies the relative controller",
    );
    rebuilt.retired_objects.remove(&source);
    assert_eq!(
        rebuilt.damage_target_from(Some(damage_source), Some(Target::Player(PlayerId::Two)), 3,),
        3,
        "without that LKI the same relative predicate no longer matches",
    );
}

#[test]
fn ring_replacement_and_outside_game_choice_reconstruct_and_resume() {
    let mut game = crate::game::tests::ready_game();
    game.players[PlayerId::One.index()].outside_game = game
        .build_zone(PlayerId::One, &[crate::card::cards::SERRA_ANGEL])
        .expect("outside-game card builds");
    let ring = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::RING_OF_MARUF)
        .expect("Ring enters");
    game.players[PlayerId::One.index()].mana_pool.colorless = 5;
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                crate::Action::ActivateAbility { source, .. } if *source == ring
            )
        })
        .expect("Ring activation is legal");
    game.apply(PlayerId::One, activation)
        .expect("Ring activates");
    for _ in 0..2 {
        let priority = game.priority;
        game.apply(priority, crate::Action::PassPriority)
            .expect("priority passes");
    }
    assert_eq!(game.draw_replacements[0].len(), 1);

    let replacement_boundary = checkpoint_wire(&game);
    assert_eq!(
        replacement_boundary.1["checkpoint"]["hasDeferredState"],
        false
    );
    let rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &replacement_boundary.1,
        &true_hidden_hypothesis(&game, replacement_boundary.0),
        4_244,
    )
    .expect("active Ring replacement reconstructs");
    assert_eq!(rebuilt.draw_replacements[0].len(), 1);

    game.draw_cards(PlayerId::One, 2);
    assert!(matches!(
        game.pending_procedures.front(),
        Some(crate::game::PendingProcedure::DrawCards {
            player: PlayerId::One,
            remaining: 1
        })
    ));
    let (viewer, wire) = checkpoint_wire(&game);
    let actions = crate::protocol::protocol_actions(&game.observe(viewer));
    let mut rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        4_245,
    )
    .expect("Ring choice and interrupted draw reconstruct");
    assert_eq!(
        crate::protocol::protocol_actions(&rebuilt.observe(viewer)),
        actions
    );
    let decision = rebuilt
        .observe(viewer)
        .decision
        .expect("Ring choice remains");
    let outside = decision
        .options
        .iter()
        .find(|option| option.zone == crate::game::DecisionZone::OutsideGame)
        .expect("outside-game option is rebound");
    rebuilt
        .apply(
            viewer,
            crate::Action::ChooseDecision {
                decision: decision.id,
                options: vec![outside.id],
            },
        )
        .expect("rebuilt Ring choice resumes");
    assert!(rebuilt.players[0].outside_game.is_empty());
    assert!(
        rebuilt.players[0]
            .hand
            .iter()
            .any(|card| card.definition == crate::card::cards::SERRA_ANGEL)
    );
    assert!(rebuilt.pending_procedures.is_empty());
}
