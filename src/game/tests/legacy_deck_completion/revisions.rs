use super::*;

fn add_test_definition(game: &mut Game, rules: &CardRules) -> CardDefinitionId {
    let id = CardDefinitionId::from_uuid("00000000-0000-0000-0000-00000000cf16");
    let mut definition = CardDefinition::new(
        id,
        "Legacy mechanic regression",
        crate::card::sets::alpha::SET,
        *rules,
    );
    synchronize_single_part_definition(&mut definition);
    let mut definitions: Vec<_> = game.catalog.definitions().into_iter().cloned().collect();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    id
}

#[test]
fn ability_use_is_explicit_nonmana_and_scoped_to_the_ability_and_turn() {
    const RULES: CardRules = CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated(
            "Gain 1 life once each turn.",
            &[],
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::RecordAbilityUse,
            ]),
        )
        .with_activation_condition(&TriggerConditionDef::Not(
            &TriggerConditionDef::SourceAbilityUsedThisTurn,
        )),
        AbilityDef::activated(
            "Gain 2 life once each turn.",
            &[],
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::RecordAbilityUse,
            ]),
        )
        .with_activation_condition(&TriggerConditionDef::Not(
            &TriggerConditionDef::SourceAbilityUsedThisTurn,
        )),
    ]);
    let mut game = ready_game();
    let definition = add_test_definition(&mut game, &RULES);
    let source = put_ready(&mut game, definition);
    activate(&mut game, source);
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, 21);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    game = Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 2)
        .unwrap();
    activate(&mut game, source);
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, 23);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::ActivateAbility { source: s, .. } if *s == source))
    );
    game.start_next_turn();
    assert!(game.abilities_used_this_turn.is_empty());
}

#[test]
fn carpet_adding_zero_does_not_consume_its_use() {
    let mut game = ready_game();
    put_ready(&mut game, cards::CARPET_OF_FLOWERS);
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::PrecombatMain,
        player: PlayerId::One,
    });
    stop_at_decision(&mut game);
    choose_label(&mut game, "your opponent");
    stop_at_decision(&mut game);
    choose_label(&mut game, "Yes");
    drain_pending(&mut game);
    assert_eq!(game.players[0].mana_pool.total(), 0);
    assert!(game.abilities_used_this_turn.is_empty());
    game.put_onto_battlefield(PlayerId::Two, cards::ISLAND)
        .unwrap();
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::PostcombatMain,
        player: PlayerId::One,
    });
    stop_at_decision(&mut game);
    choose_label(&mut game, "your opponent");
    stop_at_decision(&mut game);
    choose_label(&mut game, "Yes");
    choose_label(&mut game, "Green");
    drain_pending(&mut game);
    assert_eq!(game.players[0].mana_pool.green, 1);
    assert_eq!(game.abilities_used_this_turn.len(), 1);
}

#[test]
fn waterbend_contributions_cannot_pay_the_ordinary_generic_cost() {
    let mut game = ready_game();
    let definition = add_test_definition(
        &mut game,
        &CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
            "Waterbend {1}, {2}: You gain 3 life.",
            &[CostDef::Waterbend(1), CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        )),
    );
    let source = put_ready(&mut game, definition);
    put_ready(&mut game, cards::MEMNITE);
    put_ready(&mut game, cards::DARKSTEEL_RELIC);
    let can_activate = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::ActivateAbility { source: s, .. } if *s == source))
    };
    assert!(
        !can_activate(&game),
        "three tappers cannot cover the separate mana cost"
    );
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    assert!(!can_activate(&game));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    assert!(can_activate(&game));
    activate(&mut game, source);
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, 23);
    assert_eq!(game.players[0].mana_pool.total(), 0);
    assert_eq!(game.battlefield.iter().filter(|p| p.tapped).count(), 1);
}

#[test]
fn waterbend_increases_still_require_mana_after_three_contributions() {
    let mut game = ready_game();
    let tax = add_test_definition(
        &mut game,
        &CardRules::new_enchantment(mana_cost!("{1}")).with_ability(AbilityDef::static_ability(
            "Activated abilities cost {2} more.",
            EffectDef::ModifyCost(crate::card::CostModificationDef::AbilityIncrease {
                permanent: ObjectPredicateDef::Any,
                amount: mana_cost!("{2}"),
            }),
        )),
    );
    let vehicle = put_ready(&mut game, cards::INVASION_SUBMERSIBLE);
    drain_pending(&mut game);
    put_ready(&mut game, tax);
    for _ in 0..4 {
        put_ready(&mut game, cards::MEMNITE);
    }
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == vehicle))
    );
    let index = game
        .manual_payment_actions(PlayerId::One)
        .iter()
        .position(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == vehicle))
        .unwrap();
    game.apply(PlayerId::One, Action::BeginPayment).unwrap();
    answer(&mut game, u32::try_from(index).unwrap());
    for _ in 0..3 {
        let DecisionContinuation::Payment(crate::game::payment::state::PaymentDecision::Funding(
            draft,
        )) = &game.pending_decisions[0].continuation
        else {
            panic!("funding")
        };
        let offset = game.funding_candidates(draft).unwrap().len();
        assert!(!game.contribution_candidates(draft).unwrap().is_empty());
        answer(&mut game, u32::try_from(offset + 1).unwrap());
    }
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    game = Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 2)
        .unwrap();
    let DecisionContinuation::Payment(crate::game::payment::state::PaymentDecision::Funding(draft)) =
        &game.pending_decisions[0].continuation
    else {
        panic!("funding")
    };
    assert!(game.contribution_candidates(draft).unwrap().is_empty());
    assert_eq!(
        game.preview_funding(draft)
            .unwrap()
            .1
            .obligation
            .cost
            .generic,
        2
    );
}

fn answer(game: &mut Game, option: u32) {
    let observation = game.pending_decisions[0].observation.clone();
    game.apply(
        observation.player,
        Action::ChooseDecision {
            decision: observation.id,
            options: vec![option],
        },
    )
    .unwrap();
}

#[test]
fn threshold_turns_a_group_off_again_when_the_graveyard_shrinks() {
    let mut game = ready_game();
    let imp = put_ready(&mut game, cards::PUTRID_IMP);
    for count in [6, 7, 6] {
        game.players[0].graveyard = game
            .build_zone(PlayerId::One, &vec![cards::FOREST; count])
            .unwrap();
        let permanent = game.battlefield.iter().find(|p| p.card.id == imp).unwrap();
        assert_eq!(game.power(permanent), Some(if count == 7 { 2 } else { 1 }));
        assert_eq!(
            game.has_applied_rule(permanent, AppliedRuleDef::CANNOT_BLOCK),
            count == 7
        );
    }
}

#[test]
fn ad_nauseam_empty_library_is_not_a_failed_draw() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 5);
    cast(&mut game, cards::AD_NAUSEAM, None);
    game.apply(game.priority, Action::PassPriority).unwrap();
    game.apply(game.priority, Action::PassPriority).unwrap();
    assert!(game.pending_decisions.is_empty());
    assert!(game.stack.is_empty());
    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.players[0].life, 20);
    assert!(!game.players[0].tried_to_draw_from_empty_library);
    assert!(game.result.is_none());
}

#[test]
fn mana_replacement_components_change_type_and_amount_without_land_specific_runtime() {
    const RULES: CardRules = CardRules::new_enchantment(mana_cost!("{1}")).with_ability(
        AbilityDef::defined_replacement(
            "Artifacts tapped for two or more mana produce three green mana instead.",
            crate::card::ReplacementAbilityDef::new().with_event(
                crate::card::ReplacementEventDef::TappedForMana {
                    source_types: CardTypeSet::single(CardType::Artifact),
                    minimum_amount: 2,
                },
            ),
            crate::card::ReplacementEffectDef::Sequence(&[
                crate::card::ReplacementEffectDef::SetManaType(ManaColor::Green),
                crate::card::ReplacementEffectDef::SetEventAmount(3),
            ]),
        ),
    );
    let mut game = ready_game();
    let definition = add_test_definition(&mut game, &RULES);
    put_ready(&mut game, definition);
    let ring = put_ready(&mut game, cards::SOL_RING);
    let permanent = game.battlefield.iter().find(|p| p.card.id == ring).unwrap();
    assert_eq!(
        game.mana_types_permanent_could_produce(permanent, &mut Vec::new()),
        vec![ManaColor::Green]
    );
    assert!(game.can_pay_cost_for(
        PlayerId::One,
        mana_cost!("{G}{G}{G}"),
        0,
        &ManaPaymentPurpose::Other
    ));
    activate_mana(&mut game, ring);
    assert_eq!(game.players[0].mana_pool.green, 3);
    assert!(
        game.players[0]
            .mana
            .iter()
            .all(|mana| mana.source.unwrap().object == ring)
    );
}

#[test]
fn multiple_damping_spheres_do_not_replace_the_one_mana_again() {
    let mut game = ready_game();
    put_ready(&mut game, cards::DAMPING_SPHERE);
    put_ready(&mut game, cards::DAMPING_SPHERE);
    let tomb = put_ready(&mut game, cards::ANCIENT_TOMB);
    activate_mana(&mut game, tomb);
    assert_eq!(game.players[0].mana_pool.colorless, 1);
    assert_eq!(game.players[0].life, 18);
}
