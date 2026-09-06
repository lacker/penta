use super::*;

#[test]
fn energy_flux_taxes_every_artifact_and_takes_the_ones_nobody_pays_for() {
    let mut game = ready_game();
    game.turn = 2;
    game.step = Step::Upkeep;
    game.battlefield.push(creature(
        10_000,
        crate::card::cards::ENERGY_FLUX,
        PlayerId::One,
    ));
    // Two of the controller's artifacts, and one the opponent controls: the
    // grant reaches every artifact, but only its controller's upkeep asks.
    game.battlefield
        .push(creature(10_001, cards::SU_CHI, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::MANA_VAULT, PlayerId::One));
    game.battlefield
        .push(creature(10_003, cards::SU_CHI, PlayerId::Two));
    // Enough for exactly one of the two taxes.
    game.players[0].mana_pool.colorless = 2;

    game.handle_upkeep_triggers();
    let mut paid = false;
    for _ in 0..24 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            break;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            // Pay for the first artifact asked about and let the second go.
            // Anything else on the way, such as ordering the two triggers,
            // takes the smallest legal answer.
            let options = if decision.prompt.contains("unless you pay") {
                let pay = !paid && decision.options.iter().any(|option| option.id == 1);
                paid |= pay;
                vec![u32::from(pay)]
            } else {
                decision
                    .options
                    .iter()
                    .map(|option| option.id)
                    .take(decision.minimum)
                    .collect()
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
            continue;
        }
        if game.apply(game.priority, Action::PassPriority).is_err() {
            break;
        }
    }

    assert!(paid, "the tax was offered, not just charged");
    let artifacts = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.id != GameObjectId(10_000))
        .count();
    assert_eq!(artifacts, 2, "the unpaid-for artifact was sacrificed");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(10_003)),
        "the opponent's artifact is not taxed on this player's upkeep"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn cast_validation_rejects_unrecognized_structured_choices() {
    let definition_id = CardDefinitionId::new(10_200);
    let option_id = PlayOptionId(7);
    let implemented_mode = ModeId(0);
    let metadata_mode = ModeId(1);
    let second_implemented_mode = ModeId(2);
    let slot_id = TargetSlotId(0);
    let alternative_id = AlternativeCostId(11);
    let additional_id = AdditionalCostId(13);
    let mut definition = CardDefinition::new(
        definition_id,
        "Structured Bolt",
        CardSet::Alpha,
        crate::card::CardRules::unsupported(),
    );
    definition.rules = CardRules::new_instant(ManaCost::new(0, 1));
    synchronize_single_part_definition(&mut definition);
    let mut option = PlayOptionDef::cast(
        option_id,
        "Cast Structured Bolt",
        SpellForm::Part(CardPartId::PRIMARY),
        ManaCost::new(0, 1),
        CardEffectStatus::Implemented,
    )
    .with_modes(ModeSetDef {
        minimum: 1,
        maximum: 2,
        may_repeat: false,
        modes: vec![
            ModeDef {
                id: implemented_mode,
                label: "Target a player".into(),
                additional_mana_cost: None,
                targets: vec![TargetSlotDef::exactly_one(
                    slot_id,
                    "target player",
                    TargetPredicate::Player,
                )],
                effect_status: CardEffectStatus::Implemented,
            },
            ModeDef {
                id: metadata_mode,
                label: "Not implemented".into(),
                additional_mana_cost: None,
                targets: Vec::new(),
                effect_status: CardEffectStatus::Unsupported,
            },
            ModeDef {
                id: second_implemented_mode,
                label: "Second implemented mode".into(),
                additional_mana_cost: None,
                targets: Vec::new(),
                effect_status: CardEffectStatus::Implemented,
            },
        ],
        conditional_maximum: None,
    });
    option.alternative_costs = vec![AlternativeCostDef {
        id: alternative_id,
        label: "Alternative cost".into(),
        mana_cost: ManaCost::new(1, 0),
    }];
    option.additional_costs = vec![AdditionalCostDef {
        id: additional_id,
        label: "Additional cost".into(),
        mana_cost: Some(ManaCost::new(2, 0)),
        repeatable: false,
    }];
    definition.play_options = vec![option];

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    let card = card(10_200, definition_id, PlayerId::One);
    let card_id = card.id;
    game.players[0].hand.push(card);
    game.players[0].mana_pool.colorless = 20;

    let valid = CastChoices::new(option_id)
        .with_modes(vec![implemented_mode])
        .with_costs(CostConfiguration::new(
            Some(alternative_id),
            vec![additional_id],
        ))
        .with_targets(vec![TargetSelection::single(
            slot_id,
            Target::Player(PlayerId::Two),
        )]);
    let (signature, cost, _) = game
        .validated_cast_signature(PlayerId::One, card_id, &valid, &[])
        .expect("all structured choices are recognized and payable");
    assert_eq!(signature.play_option(), option_id);
    assert_eq!(signature.form(), &SpellForm::Part(CardPartId::PRIMARY));
    assert_eq!(signature.modes(), &[implemented_mode]);
    assert_eq!(signature.costs(), valid.costs());
    assert_eq!(signature.targets(), valid.targets());
    assert_eq!(cost, ManaCost::new(3, 0));

    let canonical_modes = CastChoices::new(option_id)
        .with_modes(vec![implemented_mode, second_implemented_mode])
        .with_costs(CostConfiguration::new(
            Some(alternative_id),
            vec![additional_id],
        ))
        .with_targets(vec![TargetSelection::single(
            slot_id,
            Target::Player(PlayerId::Two),
        )]);
    assert!(
        game.validated_cast_signature(PlayerId::One, card_id, &canonical_modes, &[])
            .is_some(),
        "distinct modes are accepted in positional order",
    );

    let invalid = [
        CastChoices::new(PlayOptionId(99)),
        CastChoices::new(option_id),
        CastChoices::new(option_id).with_modes(vec![metadata_mode]),
        CastChoices::new(option_id).with_modes(vec![implemented_mode, implemented_mode]),
        CastChoices::new(option_id)
            .with_modes(vec![second_implemented_mode, implemented_mode])
            .with_targets(vec![TargetSelection::single(
                slot_id,
                Target::Player(PlayerId::Two),
            )]),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_costs(CostConfiguration::new(
                Some(AlternativeCostId(99)),
                Vec::new(),
            )),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_costs(CostConfiguration::new(None, vec![AdditionalCostId(99)])),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_x(1),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_targets(vec![TargetSelection::single(
                TargetSlotId(99),
                Target::Player(PlayerId::Two),
            )]),
        CastChoices::new(option_id)
            .with_modes(vec![implemented_mode])
            .with_targets(vec![TargetSelection::single(
                slot_id,
                Target::Permanent(GameObjectId(99_999)),
            )]),
    ];
    for choices in invalid {
        assert!(
            game.validated_cast_signature(PlayerId::One, card_id, &choices, &[])
                .is_none(),
            "invalid structured choices were accepted: {choices:?}",
        );
    }
}

#[test]
fn cost_configuration_visitor_preserves_option_order() {
    let definition = CardDefinition::new(
        CardDefinitionId::new(10_201),
        "Ordered Costs",
        CardSet::Alpha,
        crate::card::CardRules::unsupported(),
    );
    let mut option = PlayOptionDef::cast(
        PlayOptionId::DEFAULT,
        "Cast Ordered Costs",
        SpellForm::Part(CardPartId::PRIMARY),
        ManaCost::new(1, 0),
        CardEffectStatus::Implemented,
    );
    let alternatives = [AlternativeCostId(3), AlternativeCostId(7)];
    let additional = [AdditionalCostId(11), AdditionalCostId(13)];
    option.alternative_costs = alternatives
        .into_iter()
        .map(|id| AlternativeCostDef {
            id,
            label: format!("Alternative {}", id.0),
            mana_cost: ManaCost::new(1, 0),
        })
        .collect();
    option.additional_costs = additional
        .into_iter()
        .map(|id| AdditionalCostDef {
            id,
            label: format!("Additional {}", id.0),
            mana_cost: Some(ManaCost::new(1, 0)),
            repeatable: false,
        })
        .collect();

    let game = ready_game();
    let mut actual = Vec::new();
    assert!(
        game.visit_cost_configurations(
            &definition,
            GameObjectId(10_201),
            PlayerId::One,
            &option,
            CastCostContext {
                source_zone: CastSourceZone::Hand,
                offer: None,
            },
            |configuration| {
                actual.push(configuration);
                ControlFlow::Continue(())
            },
        )
        .is_continue()
    );

    let additional_sets = [
        vec![],
        vec![additional[0]],
        vec![additional[1]],
        vec![additional[0], additional[1]],
    ];
    let expected = [None, Some(alternatives[0]), Some(alternatives[1])]
        .into_iter()
        .flat_map(|alternative| {
            additional_sets
                .iter()
                .cloned()
                .map(move |additional| CostConfiguration::new(alternative, additional))
        })
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
    for invalid in [
        vec![additional[1], additional[0]],
        vec![additional[0], additional[0]],
        vec![AdditionalCostId(99)],
    ] {
        assert!(!actual.contains(&CostConfiguration::new(None, invalid)));
    }
}

#[test]
fn generated_mode_selections_are_canonical_combinations() {
    let modes = [ModeId(0), ModeId(1)];
    assert_eq!(
        mode_id_selections(&modes, 2, 2, false),
        vec![vec![ModeId(0), ModeId(1)]],
    );
    assert_eq!(
        mode_id_selections(&modes, 2, 2, true),
        vec![
            vec![ModeId(0), ModeId(0)],
            vec![ModeId(0), ModeId(1)],
            vec![ModeId(1), ModeId(1)],
        ],
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn selected_modal_effects_resolve_distinct_and_deferred_flattened_targets() {
    static FIRST_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Player(PlayerRelation::Opponent),
    )];
    static SECOND_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::AnyTarget,
    )];
    const FIRST: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(1),
    };
    const LOSE_TWO: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(2),
    };
    static SECOND_TRIGGER: AbilityDef = AbilityDef::triggered(
        "At the beginning of the next end step, that player loses 2 life.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        LOSE_TWO,
    );
    const SECOND: EffectDef =
        EffectDef::InstallTrigger(crate::InstalledTriggerDef::once(&SECOND_TRIGGER));
    static MODES: [AbilityDef; 2] = [
        AbilityDef::spell_with_targets("First mode", &FIRST_TARGETS, FIRST),
        AbilityDef::spell_with_targets("Second mode", &SECOND_TARGETS, SECOND),
    ];
    const MODAL: AbilityDef =
        AbilityDef::modal_spell("Choose two.", &MODES).with_mode_selection(2, 2, true);
    let DeclarativeAbilityDef::Spell(spell) = MODAL.definition else {
        panic!("the fixture is a modal spell")
    };

    let distinct = Game::selected_spell_plan(spell, &[ModeId(1), ModeId(0)], &[])
        .expect("the selected modes form a valid plan");
    assert_eq!(distinct.target_defs, [FIRST_TARGETS[0], SECOND_TARGETS[0]],);
    assert_eq!(
        distinct.mode_effects,
        [ScopedEffect::at(FIRST, 0), ScopedEffect::at(SECOND, 1),],
    );

    let repeated = Game::selected_spell_plan(spell, &[ModeId(1), ModeId(1)], &[])
        .expect("a repeatable targeted mode can be selected twice");
    assert_eq!(repeated.target_defs, [SECOND_TARGETS[0], SECOND_TARGETS[0]],);
    assert_eq!(
        repeated.mode_effects,
        [ScopedEffect::at(SECOND, 0), ScopedEffect::at(SECOND, 1),],
    );

    let stack_object = |id: u32,
                        plan: SelectedSpellPlan,
                        modes: Vec<ModeId>,
                        targets: Vec<TargetSelection>| {
        let choices = CastChoices::default()
            .with_modes(modes)
            .with_targets(targets.clone());
        StackObject {
            id: StackObjectId(id),
            kind: StackObjectKind::Spell,
            card: card(id, cards::LIGHTNING_BOLT, PlayerId::One).into(),
            source: None,
            ability: Some(StackAbilityPayload {
                origin: primary_ability(cards::LIGHTNING_BOLT),
                definition: Some(Box::new(MODAL)),
                presentation: ObjectCharacteristics::card(
                    cards::LIGHTNING_BOLT,
                    CardPartId::PRIMARY,
                ),
                text: Some(MODAL.text),
                target_defs: plan.target_defs,
                targets,
                context: TriggerContext::empty().into(),
                resolver: StackAbilityResolver::Declarative(ScopedEffect::primary(EffectDef::None)),
                condition: None,
                mode_effects: plan.mode_effects,
                resolution_destination: None,
                x: 0,
                sacrificed_mana_value: 0,
            }),
            controller: PlayerId::One,
            signature: Some(CastSignature::from_validated_choices(
                SpellForm::Part(CardPartId::PRIMARY),
                choices,
            )),
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast: None,
            face_down: None,
            is_copy: false,
        }
    };

    let mut game = ready_game();
    let distinct = stack_object(
        10_300,
        distinct,
        vec![ModeId(0), ModeId(1)],
        vec![
            TargetSelection::single(TargetSlotId(0), Target::Player(PlayerId::Two)),
            TargetSelection::single(TargetSlotId(1), Target::Player(PlayerId::One)),
        ],
    );
    assert!(game.resolve_stack_ability(&distinct));
    assert_eq!(game.players[0].life, 20, "the deferred mode has not fired");
    assert_eq!(
        game.players[1].life, 19,
        "the first mode used runtime slot 0"
    );
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::End,
        player: PlayerId::One,
    });
    game.finish_rules_procedure();
    drain_pending(&mut game);
    assert_eq!(
        game.players[0].life, 18,
        "the second mode kept runtime slot 1"
    );
    assert_eq!(game.players[1].life, 19);

    let repeated = stack_object(
        10_301,
        repeated,
        vec![ModeId(1), ModeId(1)],
        vec![
            TargetSelection::single(TargetSlotId(0), Target::Player(PlayerId::One)),
            TargetSelection::single(TargetSlotId(1), Target::Player(PlayerId::Two)),
        ],
    );
    assert!(game.resolve_stack_ability(&repeated));
    assert_eq!(game.installed_triggers.len(), 2);
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::End,
        player: PlayerId::One,
    });
    game.finish_rules_procedure();
    drain_pending(&mut game);
    assert_eq!(
        game.players[0].life, 16,
        "the first repeated occurrence used slot 0"
    );
    assert_eq!(
        game.players[1].life, 17,
        "the second repeated occurrence used slot 1"
    );
}

#[test]
fn manual_mode_target_slots_are_rebased_after_selected_modes_are_flattened() {
    let local = |id: ModeId, label: &str| ModeDef {
        id,
        label: label.into(),
        additional_mana_cost: None,
        targets: vec![TargetSlotDef::exactly_one(
            TargetSlotId(0),
            "target player",
            TargetPredicate::Player,
        )],
        effect_status: CardEffectStatus::Implemented,
    };
    let mut option = PlayOptionDef::cast(
        PlayOptionId::DEFAULT,
        "Manual modal spell",
        SpellForm::Part(CardPartId::PRIMARY),
        ManaCost::new(1, 0),
        CardEffectStatus::Implemented,
    );
    option.modes = Some(ModeSetDef {
        minimum: 2,
        maximum: 3,
        may_repeat: true,
        modes: vec![local(ModeId(0), "First"), local(ModeId(1), "Second")],
        conditional_maximum: None,
    });

    let slots = Game::target_slots_for(&option, &[ModeId(0), ModeId(1), ModeId(1)]);
    assert_eq!(
        slots.iter().map(|slot| slot.id).collect::<Vec<_>>(),
        [TargetSlotId(0), TargetSlotId(1), TargetSlotId(2)],
    );
}

#[test]
fn declarative_dual_lands_cast_and_resolve_a_hybrid_creature() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.battlefield.extend([
        creature(10_000, crate::card::cards::CLIFFTOP_RETREAT, PlayerId::One),
        creature(10_001, crate::card::cards::SACRED_FOUNDRY, PlayerId::One),
        creature(10_002, crate::card::cards::SUNPETAL_GROVE, PlayerId::One),
    ]);
    game.players[0].hand.push(card(
        10_003,
        crate::card::cards::BOROS_RECKONER,
        PlayerId::One,
    ));

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell {
                    card: CardInstanceId(10_003),
                    ..
                }
            )
        })
        .expect("three declarative dual lands can pay {R/W}{R/W}{R/W}");
    assert_eq!(game.mana_sources_for_action(PlayerId::One, &cast).len(), 3);

    game.apply(PlayerId::One, cast).unwrap();
    assert!(game.battlefield.iter().all(|permanent| permanent.tapped));
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    pass_priority_pair(&mut game);

    let reckoner = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == crate::card::cards::BOROS_RECKONER)
        .unwrap();
    assert_eq!(game.power(reckoner), Some(3));
    assert_eq!(game.toughness(reckoner), Some(3));
}

#[test]
fn flexible_mana_plan_reserves_the_only_green_source_for_a_multicolor_spell() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.battlefield.extend([
        creature(10_000, crate::card::cards::TEMPLE_GARDEN, PlayerId::One),
        creature(10_001, crate::card::cards::GODLESS_SHRINE, PlayerId::One),
        creature(
            10_002,
            crate::card::cards::ENCROACHING_WASTES,
            PlayerId::One,
        ),
    ]);
    game.players[0].hand.push(card(
        10_003,
        crate::card::cards::LOXODON_SMITER,
        PlayerId::One,
    ));

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell {
                    card: CardInstanceId(10_003),
                    ..
                }
            )
        })
        .expect("Godless Shrine can make white while Temple Garden makes green");
    assert_eq!(
        game.mana_sources_for_action(PlayerId::One, &cast),
        vec![
            CardInstanceId(10_001),
            CardInstanceId(10_000),
            CardInstanceId(10_002),
        ],
    );

    game.apply(PlayerId::One, cast).unwrap();
    assert!(game.battlefield.iter().all(|permanent| permanent.tapped));
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
}

#[test]
fn flash_creatures_keep_their_printed_cast_timing() {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game.step = Step::End;
    game.players[0].mana_pool = ManaPool {
        white: 1,
        colorless: 3,
        ..ManaPool::default()
    };
    game.players[0].hand.extend([
        card(10_000, crate::card::cards::RESTORATION_ANGEL, PlayerId::One),
        card(10_001, crate::card::cards::LOXODON_SMITER, PlayerId::One),
    ]);

    let cast_cards = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, .. } => Some(card),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(cast_cards, vec![CardInstanceId(10_000)]);
}

#[test]
fn city_of_brass_produces_any_color_then_uses_the_stack_for_damage() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::CITY_OF_BRASS, PlayerId::One));

    let ability = mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Blue);
    game.activate_mana_source(
        PlayerId::One,
        CardInstanceId(10_000),
        ability,
        ManaColor::Blue,
        &ManaActivationChoices::default(),
    );

    assert_eq!(game.players[0].mana_pool.blue, 1);
    assert_eq!(game.players[0].life, 20);
    assert!(game.stack.is_empty());
    game.finish_rules_procedure();
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::TriggeredAbility);
    assert_eq!(game.stack[0].source, Some(CardInstanceId(10_000)));

    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].life, 19);
}
