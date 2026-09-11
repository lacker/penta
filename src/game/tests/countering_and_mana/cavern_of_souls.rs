use super::*;
use crate::card::sets;

#[test]
fn naming_choices_show_opponents_only_a_pending_notice_and_the_final_name() {
    for (definition, answer) in [
        (cards::CAVERN_OF_SOULS, "Angel"),
        (cards::PITHING_NEEDLE, "Black Lotus"),
    ] {
        let mut opponent_views = Vec::new();
        for hidden_card in [cards::RESTORATION_ANGEL, cards::THRAGTUSK] {
            let mut game = ready_game();
            game.players[0]
                .hand
                .push(card(19_088, hidden_card, PlayerId::One));
            game.put_onto_battlefield(PlayerId::One, definition)
                .expect("naming permanent is cataloged");
            let chooser = game.observe(PlayerId::One);
            let decision = chooser.decision.expect("the chooser receives a menu");
            let option = decision
                .options
                .iter()
                .find(|option| option.label == answer)
                .expect("the requested answer is offered")
                .id;
            let opponent = game.observe(PlayerId::Two);
            let notice = opponent
                .decision
                .as_ref()
                .expect("the opponent sees a pending choice");
            assert_eq!(notice.player, PlayerId::One);
            assert_eq!(notice.prompt, decision.prompt);
            assert!(
                notice.options.is_empty(),
                "opponents must not receive the selection menu"
            );
            assert!(
                opponent
                    .legal_actions
                    .iter()
                    .all(|action| matches!(action, Action::Concede))
            );
            assert!(
                game.apply(
                    PlayerId::Two,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options: vec![option],
                    }
                )
                .is_err(),
                "the opponent cannot answer the notice"
            );
            assert!(opponent.checkpoint["decisionState"].is_null());
            assert_eq!(opponent.checkpoint["hasDeferredState"], true);
            opponent_views.push(crate::protocol::observation_json_for_format(
                &game.catalog,
                game.format,
                &opponent,
                game.in_pregame(),
                &[],
            ));
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![option],
                },
            )
            .expect("the chooser can submit the full menu's option ID");
            let after = game.observe(PlayerId::Two);
            assert!(after.decision.is_none());
            let permanent = after
                .battlefield
                .iter()
                .find(|permanent| permanent.characteristics.card_definition() == Some(definition))
                .expect("the permanent enters after the choice");
            assert_eq!(
                permanent
                    .chosen_creature_type
                    .as_deref()
                    .or(permanent.chosen_card_name.as_deref()),
                Some(answer),
                "the selected value becomes public",
            );
        }
        assert_eq!(
            opponent_views[0], opponent_views[1],
            "changing the hidden hand must not change the pending notice or checkpoint"
        );
    }
}

fn acceptance_play_cavern_choosing(game: &mut Game, creature_type: &str) -> GameObjectId {
    let cavern = card(19_000, cards::CAVERN_OF_SOULS, PlayerId::One);
    game.players[0].hand.push(cavern.clone());
    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: cavern.id,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::CAVERN_OF_SOULS),
        "an as-enters choice finishes before Cavern is on the battlefield",
    );
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("Cavern asks for a creature type");
    let option = decision
        .options
        .iter()
        .find(|option| option.label == creature_type)
        .unwrap_or_else(|| panic!("{creature_type} is an available creature type"))
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .unwrap();
    let permanent = game
        .observe(PlayerId::One)
        .battlefield
        .into_iter()
        .find(|candidate| {
            candidate.characteristics.card_definition() == Some(cards::CAVERN_OF_SOULS)
        })
        .expect("Cavern entered after its creature type was chosen");
    assert_eq!(
        permanent.chosen_creature_type.as_deref(),
        Some(creature_type)
    );
    permanent.id
}

#[test]
fn cavern_of_souls_requires_and_records_a_creature_type_choice() {
    let mut game = ready_game();
    let cavern = card(19_080, cards::CAVERN_OF_SOULS, PlayerId::One);
    game.players[0].hand.push(cavern.clone());
    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: cavern.id,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the entry replacement creates a mandatory choice");
    assert_eq!(decision.prompt, "Choose a creature type");
    assert!(
        decision
            .options
            .iter()
            .any(|option| option.label == "Angel")
    );
    assert!(
        decision
            .options
            .iter()
            .any(|option| option.label == "Time Lord"),
        "Cavern can name a legal type absent from the loaded card catalog",
    );
    assert!(
        decision
            .options
            .iter()
            .all(|option| option.label != "Artifact")
    );
    assert!(
        game.observe(PlayerId::One).battlefield.is_empty(),
        "the replacement choice is made before Cavern enters",
    );
    let angel = decision
        .options
        .iter()
        .find(|option| option.label == "Angel")
        .unwrap()
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![angel],
        },
    )
    .unwrap();
    let observed = game.observe(PlayerId::Two);
    let cavern = observed
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.characteristics.card_definition() == Some(cards::CAVERN_OF_SOULS)
        })
        .expect("Cavern enters after the choice");
    assert_eq!(cavern.chosen_creature_type.as_deref(), Some("Angel"));
}

#[test]
fn cavern_choices_ignore_noncreature_subtypes_on_creature_cards() {
    let definition_id = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000004a8d");
    let mut equipment_creature = CardDefinition::new(
        definition_id,
        "Test equipment creature",
        sets::magic_2014::SET,
        crate::card::CardRules::unsupported(),
    );
    equipment_creature.rules =
        CardRules::new_artifact_creature(ManaCost::new(1, 0), &["Equipment", "Rabbit"], 1, 1);
    synchronize_single_part_definition(&mut equipment_creature);

    let mut game = ready_game();
    let cavern_definition = game
        .catalog
        .get(cards::CAVERN_OF_SOULS)
        .expect("the acceptance catalog contains Cavern")
        .clone();
    game.catalog = CardCatalog::new([cavern_definition, equipment_creature]).unwrap();
    let cavern = card(19_086, cards::CAVERN_OF_SOULS, PlayerId::One);
    game.players[0]
        .hand
        .extend([cavern.clone(), card(19_087, definition_id, PlayerId::One)]);

    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: cavern.id,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("Cavern asks for a creature type");
    assert!(
        decision
            .options
            .iter()
            .any(|option| option.label == "Rabbit")
    );
    assert!(
        decision
            .options
            .iter()
            .all(|option| option.label != "Equipment"),
        "artifact subtypes do not become legal creature-type choices",
    );
}

#[test]
fn cavern_colored_mana_cannot_pay_for_a_nonmatching_creature() {
    let mut game = ready_game();
    let creature = card(19_090, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    game.players[0].hand.push(creature.clone());
    let cavern = acceptance_play_cavern_choosing(&mut game, "Angel");
    let ability = mana_ability_for(&game, cavern, ManaColor::White);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: cavern,
            ability,
            color: ManaColor::White,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .unwrap();

    assert!(
        game.legal_actions(PlayerId::One).iter().all(
            |action| !matches!(action, Action::CastSpell { card, .. } if *card == creature.id)
        )
    );
    assert!(
        game.apply(
            PlayerId::One,
            cast_action(creature.id, Vec::new(), Vec::new(), 0),
        )
        .is_err(),
        "restricted mana cannot be forced through validation",
    );
}

#[test]
fn cavern_mana_spent_on_a_matching_creature_makes_it_uncounterable() {
    let mut game = ready_game();
    let creature = card(19_100, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    let counterspell = card(19_101, cards::COUNTERSPELL, PlayerId::Two);
    game.players[0].hand.push(creature.clone());
    game.players[1].hand.push(counterspell.clone());
    let cavern = acceptance_play_cavern_choosing(&mut game, "Soldier");
    let ability = mana_ability_for(&game, cavern, ManaColor::White);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: cavern,
            ability,
            color: ManaColor::White,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .unwrap();
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    let action = acceptance_cast_action_for_card(&game, PlayerId::One, creature.id);
    game.apply(PlayerId::One, action).unwrap();
    assert!(
        game.stack
            .last()
            .unwrap()
            .applied_effects
            .iter()
            .any(|effect| {
                effect.effect == AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered)
                    && effect.source.is_some_and(|source| source.object == cavern)
            })
    );
    acceptance_attempt_counterspell(&mut game, counterspell.id);
    assert_eq!(game.stack.len(), 1, "the Cavern-paid creature remains");
    pass_priority_pair(&mut game);
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.card.definition == cards::ICATIAN_JAVELINEERS
            && permanent.controller == PlayerId::One
    }));
}

#[test]
fn generic_payment_prefers_eligible_cavern_mana_with_a_spell_rider() {
    let mut game = ready_game();
    let angel = card(19_105, cards::RESTORATION_ANGEL, PlayerId::One);
    game.players[0].hand.push(angel.clone());
    let cavern = acceptance_play_cavern_choosing(&mut game, "Angel");
    let ability = mana_ability_for(&game, cavern, ManaColor::Blue);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: cavern,
            ability,
            color: ManaColor::Blue,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);

    let action = acceptance_cast_action_for_card(&game, PlayerId::One, angel.id);
    game.apply(PlayerId::One, action).unwrap();

    let spell = game
        .stack
        .last()
        .expect("Restoration Angel is on the stack");
    assert!(spell.applied_effects.iter().any(|effect| {
        effect.effect == AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered)
            && effect.source.is_some_and(|source| source.object == cavern)
    }));
    assert_eq!(game.players[0].mana_pool.blue, 0);
    assert_eq!(game.players[0].mana_pool.colorless, 1);
}

#[test]
fn cavern_mana_keeps_its_chosen_type_and_rider_after_cavern_leaves() {
    let mut game = ready_game();
    let creature = card(19_105, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    game.players[0].hand.push(creature.clone());
    let cavern = acceptance_play_cavern_choosing(&mut game, "Soldier");
    let ability = mana_ability_for(&game, cavern, ManaColor::White);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: cavern,
            ability,
            color: ManaColor::White,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .unwrap();
    game.destroy_permanent_without_regeneration(cavern);

    let action = acceptance_cast_action_for_card(&game, PlayerId::One, creature.id);
    game.apply(PlayerId::One, action).unwrap();

    assert!(
        game.stack
            .last()
            .unwrap()
            .applied_effects
            .iter()
            .any(|effect| {
                effect.effect == AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered)
                    && effect.source.is_some_and(|source| source.object == cavern)
            })
    );
}

#[test]
fn automatic_payment_uses_cavern_when_its_rider_benefits_the_spell() {
    let mut game = ready_game();
    let goblin = card(19_106, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    let counterspell = card(19_107, cards::COUNTERSPELL, PlayerId::Two);
    game.players[0].hand.push(goblin.clone());
    game.players[1].hand.push(counterspell.clone());
    let cavern = acceptance_play_cavern_choosing(&mut game, "Goblin");
    game.battlefield
        .push(creature(19_108, cards::MOUNTAIN, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    let action = acceptance_cast_action_for_card(&game, PlayerId::One, goblin.id);
    game.apply(PlayerId::One, action).unwrap();

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == cavern)
            .is_some_and(|permanent| permanent.tapped),
        "the automatic payment plan taps Cavern for the beneficial rider",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| {
                permanent.card.definition == cards::MOUNTAIN && permanent.tapped
            })
            .count(),
        0,
    );
    assert!(
        game.stack
            .last()
            .unwrap()
            .applied_effects
            .iter()
            .any(|effect| {
                effect.effect == AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered)
                    && effect.source.is_some_and(|source| source.object == cavern)
            })
    );

    acceptance_attempt_counterspell(&mut game, counterspell.id);
    assert_eq!(game.stack.len(), 1, "the Cavern-paid Goblin remains");
}

#[test]
fn cavern_does_not_protect_a_creature_paid_for_with_other_mana() {
    for chosen_type in ["Angel", "Soldier"] {
        let mut game = ready_game();
        let creature = card(19_110, cards::ICATIAN_JAVELINEERS, PlayerId::One);
        let counterspell = card(19_111, cards::COUNTERSPELL, PlayerId::Two);
        game.players[0].hand.push(creature.clone());
        game.players[1].hand.push(counterspell.clone());
        let _cavern = acceptance_play_cavern_choosing(&mut game, chosen_type);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
        game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

        let action = acceptance_cast_action_for_card(&game, PlayerId::One, creature.id);
        game.apply(PlayerId::One, action).unwrap();
        assert!(game.stack.last().unwrap().applied_effects.is_empty());
        acceptance_attempt_counterspell(&mut game, counterspell.id);
        assert!(game.stack.is_empty(), "Counterspell counters the creature");
        assert!(
            game.battlefield
                .iter()
                .all(|permanent| { permanent.card.definition != cards::ICATIAN_JAVELINEERS })
        );
    }
}

#[test]
fn caverns_colorless_mana_is_unrestricted_and_has_no_countering_rider() {
    let mut game = ready_game();
    let sol_ring = card(19_120, cards::SOL_RING, PlayerId::One);
    let counterspell = card(19_121, cards::COUNTERSPELL, PlayerId::Two);
    game.players[0].hand.push(sol_ring.clone());
    game.players[1].hand.push(counterspell.clone());
    let cavern = acceptance_play_cavern_choosing(&mut game, "Angel");
    let ability = mana_ability_for(&game, cavern, ManaColor::Colorless);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: cavern,
            ability,
            color: ManaColor::Colorless,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .unwrap();
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    let action = acceptance_cast_action_for_card(&game, PlayerId::One, sol_ring.id);
    game.apply(PlayerId::One, action).unwrap();
    assert!(game.stack.last().unwrap().applied_effects.is_empty());
    acceptance_attempt_counterspell(&mut game, counterspell.id);
    assert!(game.stack.is_empty());
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SOL_RING),
    );
}

fn cavern_colored_mana(game: &mut Game, cavern: GameObjectId) -> crate::game::Mana {
    let ability = mana_ability_for(game, cavern, ManaColor::White);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: cavern,
            ability,
            color: ManaColor::White,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .unwrap();
    *game.players[0].mana.iter().last().unwrap()
}

fn cavern_payment(definition: CardDefinitionId) -> ManaPaymentPurpose {
    ManaPaymentPurpose::Spell {
        object: GameObjectId(19_200),
        definition,
        controller: PlayerId::One,
        form: SpellForm::Part(CardPartId::PRIMARY),
        reserved_life_payment: 0,
    }
}

#[test]
fn cavern_bound_subtype_restrictions_use_each_producers_labeled_choice() {
    let mut game = ready_game();
    let soldier_cavern = acceptance_play_cavern_choosing(&mut game, "Soldier");
    let soldier_mana = cavern_colored_mana(&mut game, soldier_cavern);
    let angel_cavern = game
        .put_onto_battlefield(PlayerId::One, cards::CAVERN_OF_SOULS)
        .unwrap();
    choose_decision_by_label(&mut game, PlayerId::One, "Angel");
    let angel_mana = cavern_colored_mana(&mut game, angel_cavern);
    let soldier = cavern_payment(cards::ICATIAN_JAVELINEERS);
    let angel = cavern_payment(cards::SERRA_ANGEL);
    assert!(game.mana_can_pay_for(soldier_mana, &soldier));
    assert!(!game.mana_can_pay_for(soldier_mana, &angel));
    assert!(game.mana_can_pay_for(angel_mana, &angel));
    assert!(!game.mana_can_pay_for(angel_mana, &soldier));
    assert!(!game.mana_can_pay_for(soldier_mana, &cavern_payment(cards::SWORDS_TO_PLOWSHARES)));
    assert!(!game.mana_can_pay_for(soldier_mana, &ManaPaymentPurpose::Other));

    let (mut object, _) = game.payment_object(&soldier).unwrap();
    let bound =
        ObjectPredicateDef::Subtype(crate::SubtypeDef::Binding(Binding!("cavern_creature_type")));
    assert!(game.trigger_object_matches(bound, &object, soldier_cavern, true));
    assert!(!game.trigger_object_matches(bound, &object, angel_cavern, true));
    assert!(!game.trigger_object_matches(
        ObjectPredicateDef::Subtype(crate::SubtypeDef::Binding(Binding!("pithing_needle_name"))),
        &object,
        soldier_cavern,
        true,
    ));
    assert!(game.trigger_object_matches(
        ObjectPredicateDef::Subtype(crate::SubtypeDef::Literal("Soldier")),
        &object,
        angel_cavern,
        true,
    ));
    object.types = crate::card::CardTypeSet::EMPTY;
    assert!(
        game.trigger_object_matches(bound, &object, soldier_cavern, true),
        "the subtype predicate itself does not imply creature"
    );

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == soldier_cavern)
        .unwrap()
        .chosen_creature_type_binding = None;
    assert!(
        !game.mana_can_pay_for(soldier_mana, &soldier),
        "an unnamed choice cannot satisfy an explicit binding"
    );
}

#[test]
fn cavern_bound_mana_round_trips_before_and_after_its_source_returns() {
    let mut game = ready_game();
    let creature = card(19_201, cards::ICATIAN_JAVELINEERS, PlayerId::One);
    game.players[0].hand.push(creature.clone());
    let cavern = acceptance_play_cavern_choosing(&mut game, "Soldier");
    let mana = cavern_colored_mana(&mut game, cavern);

    for returned in [false, true] {
        if returned {
            game.return_permanent_to_hand(cavern);
            let index = game.players[0]
                .hand
                .iter()
                .position(|card| card.definition == cards::CAVERN_OF_SOULS)
                .unwrap();
            let returning = game.players[0].hand.remove(index);
            game.put_card_onto_battlefield_from(
                returning,
                ZoneKind::Hand,
                crate::game::BattlefieldArrival::under(PlayerId::One),
                None,
            );
            choose_decision_by_label(&mut game, PlayerId::One, "Angel");
            let replacement = game
                .battlefield
                .iter()
                .find(|permanent| permanent.card.definition == cards::CAVERN_OF_SOULS)
                .unwrap();
            assert_ne!(replacement.card.id, cavern);
            assert_eq!(replacement.chosen_creature_type.as_deref(), Some("Angel"));
        }
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            42,
        )
        .expect("bound mana and its live or retired source reconstruct");
        let rebuilt_mana = *rebuilt.players[0].mana.first().unwrap();
        assert_eq!(rebuilt_mana, mana);
        assert_eq!(
            rebuilt.source_subtype(
                crate::SubtypeDef::Binding(Binding!("cavern_creature_type")),
                cavern
            ),
            Some("Soldier"),
            "the producing incarnation retains its binding (returned={returned})",
        );
        assert!(
            rebuilt.mana_can_pay_for(rebuilt_mana, &cavern_payment(cards::ICATIAN_JAVELINEERS))
        );
        assert!(!rebuilt.mana_can_pay_for(rebuilt_mana, &cavern_payment(cards::SERRA_ANGEL)));
        let action = acceptance_cast_action_for_card(&rebuilt, PlayerId::One, creature.id);
        rebuilt.apply(PlayerId::One, action).unwrap();
        assert!(
            rebuilt
                .stack
                .last()
                .unwrap()
                .applied_effects
                .iter()
                .any(|effect| {
                    effect.effect == AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered)
                        && effect.source.is_some_and(|source| source.object == cavern)
                })
        );
    }
}
