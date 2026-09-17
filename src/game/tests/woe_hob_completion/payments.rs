use super::*;

#[test]
fn woe_hob_colorstorm_reads_actual_mana_after_taxes_and_discounts() {
    for prepared in [false, true] {
        for taxes in [0, 2] {
            let mut game = setup(prepared);
            game.put_onto_battlefield(PlayerId::One, cards::COLORSTORM_STALLION)
                .unwrap();
            for _ in 0..taxes {
                game.put_onto_battlefield(PlayerId::Two, cards::SPHERE_OF_RESISTANCE)
                    .unwrap();
            }
            let divination = hand(&mut game, cards::DIVINATION);
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 5);
            let action = cast_for(&game, PlayerId::One, divination).unwrap();
            game.apply(PlayerId::One, action).unwrap();
            let cast = game
                .stack
                .iter()
                .find(|object| object.kind == StackObjectKind::Spell)
                .unwrap();
            assert_eq!(cast.cast.as_ref().unwrap().mana_spent, 3 + taxes);
            let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
            let mut rebuilt = Game::from_observation_checkpoint(
                game.catalog.clone(),
                game.format,
                &wire,
                &hidden,
                0,
            )
            .unwrap();
            for state in [&mut game, &mut rebuilt] {
                drain_pending(state);
                assert_eq!(
                    state
                        .battlefield
                        .iter()
                        .filter(|p| p.card.definition.is_token())
                        .count(),
                    usize::from(taxes == 2)
                );
                if taxes == 2 {
                    let token = state
                        .battlefield
                        .iter()
                        .find(|p| p.card.definition.is_token())
                        .unwrap();
                    assert_eq!(
                        state.power(token),
                        Some(3),
                        "temporary pumps are not copied"
                    );
                }
            }
        }
    }
}

#[test]
fn woe_hob_teamwork_offers_legal_supersets_and_respects_negative_power() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let first = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let second = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let negative = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == negative)
            .unwrap()
            .add_counters(CounterKind::power_toughness(-1, 0), 3);
        let divination = hand(&mut game, cards::DIVINATION);
        let nay = hand(&mut game, cards::WE_SAY_THEE_NAY);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 5);
        let cast = cast_for(&game, PlayerId::One, divination).unwrap();
        game.apply(PlayerId::One, cast).unwrap();
        let actions = game.legal_actions(PlayerId::One);
        assert!(actions.iter().any(|action| matches!(action, Action::CastSpell { card, sacrifices, .. }
            if *card == nay && sacrifices.contains(&first) && sacrifices.contains(&second) && sacrifices.len() == 2)));
        assert!(!actions.iter().any(|action| matches!(action, Action::CastSpell { card, sacrifices, .. }
            if *card == nay && sacrifices.contains(&first) && sacrifices.contains(&negative) && sacrifices.len() == 2)));
        let cast = actions
            .into_iter()
            .find(|action| {
                matches!(action, Action::CastSpell { card, sacrifices, .. }
            if *card == nay && sacrifices.len() == 3)
            })
            .expect("all three have total power three");
        game.apply(PlayerId::One, cast).unwrap();
        for id in [first, second, negative] {
            assert!(
                game.battlefield
                    .iter()
                    .find(|p| p.card.id == id)
                    .unwrap()
                    .tapped
            );
        }
        assert_eq!(
            game.stack
                .last()
                .unwrap()
                .cast
                .as_ref()
                .unwrap()
                .additional_costs,
            vec![1]
        );
    }
}

#[test]
fn woe_hob_watch_mana_only_pays_exile_casts_and_survives_reconstruction() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.players[0].library = game
            .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS, cards::FOREST])
            .unwrap();
        let watch = game
            .put_onto_battlefield(PlayerId::One, cards::INTERDIMENSIONAL_WEB_WATCH)
            .unwrap();
        drain_pending(&mut game);
        let held = hand(&mut game, cards::GRIZZLY_BEARS);
        let exiled = game.players[0]
            .exile
            .iter()
            .find(|card| card.definition == cards::GRIZZLY_BEARS)
            .unwrap()
            .id;
        let mana = game.legal_actions(PlayerId::One).into_iter().find(|action| matches!(action,
            Action::ActivateManaAbility { source, combination: Some(split), .. }
                if *source == watch && *split == crate::card::ManaSplit::from_amounts([(ManaColor::Green, 2)]))).unwrap();
        game.apply(PlayerId::One, mana).unwrap();
        assert!(cast_for(&game, PlayerId::One, held).is_none());
        assert!(cast_for(&game, PlayerId::One, exiled).is_some());
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            assert!(cast_for(state, PlayerId::One, held).is_none());
            let action = cast_for(state, PlayerId::One, exiled).unwrap();
            state.apply(PlayerId::One, action).unwrap();
            assert_eq!(state.players[0].mana_pool.total(), 0);
            drain_pending(state);
            assert!(
                state
                    .battlefield
                    .iter()
                    .any(|p| p.card.definition == cards::GRIZZLY_BEARS)
            );
        }
    }
}

#[test]
fn woe_hob_blight_is_an_untargeted_cast_cost_and_pyrrhic_requires_it_for_both_modes() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let payer = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let victim = game
            .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
        let hex = hand(&mut game, cards::REQUITING_HEX);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
        let paid = game.legal_actions(PlayerId::One).into_iter().find(|action| matches!(action,
            Action::CastSpell { card, sacrifices, choices, .. } if *card == hex && sacrifices == &[payer]
                && choices.targets().iter().any(|target| target.targets().contains(&Target::Permanent(victim))))).unwrap();
        game.apply(PlayerId::One, paid).unwrap();
        assert_eq!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == payer)
                .unwrap()
                .counters(CounterKind::MinusOneMinusOne),
            1
        );
        assert_eq!(game.players[0].life, 20);
        drain_pending(&mut game);
        assert_eq!(game.players[0].life, 22);
        assert!(!game.battlefield.iter().any(|p| p.card.id == victim));

        game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
            .unwrap();
        game.put_onto_battlefield(PlayerId::Two, cards::BONESPLITTER)
            .unwrap();
        let strike = hand(&mut game, cards::PYRRHIC_STRIKE);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 3);
        let casts = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter(|action| {
                matches!(action,
            Action::CastSpell { card, .. } if *card == strike)
            })
            .collect::<Vec<_>>();
        assert!(casts.iter().any(|a| matches!(a, Action::CastSpell { choices, sacrifices, .. } if choices.modes().len() == 1 && sacrifices.is_empty())));
        assert!(casts.iter().any(|a| matches!(a, Action::CastSpell { choices, sacrifices, .. } if choices.modes().len() == 2 && sacrifices == &[payer])));
        assert!(casts.iter().all(|a| matches!(a, Action::CastSpell { choices, sacrifices, .. } if (choices.modes().len() == 2) != sacrifices.is_empty())));
        let both = casts
            .into_iter()
            .find(|a| matches!(a, Action::CastSpell { choices, .. } if choices.modes().len() == 2))
            .unwrap();
        game.apply(PlayerId::One, both).unwrap();
        assert!(!game.battlefield.iter().any(|p| p.card.id == payer));
        drain_pending(&mut game);
        assert!(game.battlefield.is_empty());
    }
}

#[test]
fn woe_hob_dyadrine_requires_counters_on_two_distinct_creatures() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.players[0].library = game.build_zone(PlayerId::One, &[cards::FOREST]).unwrap();
        let dyadrine = hand(&mut game, cards::DYADRINE_SYNTHESIS_AMALGAM);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 3);
        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|a| {
                matches!(a,
            Action::CastSpell { card, choices, .. } if *card == dyadrine && choices.x() == 2)
            })
            .unwrap();
        game.apply(PlayerId::One, cast).unwrap();
        drain_pending(&mut game);
        let dyadrine = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::DYADRINE_SYNTHESIS_AMALGAM)
            .unwrap()
            .card
            .id;
        assert_eq!(
            game.current_or_last_known_counters(dyadrine, CounterKind::PlusOnePlusOne),
            4
        );
        let bear = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let attacked =
            game.trigger_event_object(game.battlefield.iter().find(|p| p.card.id == bear).unwrap());
        game.capture_battlefield_triggers(&CommittedTriggerEvent::AttackersDeclared {
            attackers: vec![attacked.clone()],
        });
        game.finish_rules_procedure();
        game.resolve_stack_top();
        let decision = game.observe(PlayerId::One).decision.unwrap();
        assert!(decision.options.iter().all(|o| o.label == "Decline"));
        choose_decision_by_label(&mut game, PlayerId::One, "Decline");
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == bear)
            .unwrap()
            .add_counters(CounterKind::PlusOnePlusOne, 1);
        game.capture_battlefield_triggers(&CommittedTriggerEvent::AttackersDeclared {
            attackers: vec![attacked],
        });
        game.finish_rules_procedure();
        game.resolve_stack_top();
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            let decision = state.observe(PlayerId::One).decision.unwrap();
            let option = decision
                .options
                .iter()
                .find(|o| o.label != "Decline")
                .unwrap()
                .id;
            state
                .apply(
                    PlayerId::One,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options: vec![option],
                    },
                )
                .unwrap();
            drain_pending(state);
            assert_eq!(
                state.current_or_last_known_counters(dyadrine, CounterKind::PlusOnePlusOne),
                3
            );
            assert_eq!(
                state.current_or_last_known_counters(bear, CounterKind::PlusOnePlusOne),
                0
            );
            assert_eq!(state.players[0].hand.len(), 1);
            assert_eq!(state.battlefield.len(), 3);
        }
    }
}

#[test]
fn woe_hob_arachne_card_type_choice_taxes_both_players_and_reconstructs() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.players[1].hand = game
            .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
            .unwrap();
        let arachne = game
            .put_onto_battlefield(PlayerId::One, cards::ARACHNE_PSIONIC_WEAVER)
            .unwrap();
        for _ in 0..4 {
            let Some(decision) = game.observe(PlayerId::One).decision else {
                break;
            };
            if decision.options.iter().any(|o| o.label == "Artifact") {
                assert!(!decision.options.iter().any(|o| o.label == "Creature"));
                choose_decision_by_label(&mut game, PlayerId::One, "Artifact");
                break;
            }
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: decision
                        .options
                        .iter()
                        .take(decision.minimum)
                        .map(|o| o.id)
                        .collect(),
                },
            )
            .unwrap();
        }
        assert_eq!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == arachne)
                .unwrap()
                .chosen_card_type,
            Some(CardType::Artifact)
        );
        let ring = hand(&mut game, cards::SOL_RING);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
        assert!(cast_for(&game, PlayerId::One, ring).is_none());
        let elf = hand(&mut game, cards::LLANOWAR_ELVES);
        assert!(cast_for(&game, PlayerId::One, elf).is_some());
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            assert!(cast_for(state, PlayerId::One, ring).is_none());
            state.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
            assert!(cast_for(state, PlayerId::One, ring).is_some());
            state.active_player = PlayerId::Two;
            state.priority = PlayerId::Two;
            let card = state
                .build_zone(PlayerId::Two, &[cards::SOL_RING])
                .unwrap()
                .remove(0);
            let id = card.id;
            state.players[1].hand.push(card);
            state.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, 1);
            assert!(cast_for(state, PlayerId::Two, id).is_none());
            state.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, 1);
            assert!(cast_for(state, PlayerId::Two, id).is_some());
        }
    }
}

#[test]
fn woe_hob_waterbend_uses_summoning_sick_creatures_but_cannot_pay_unrelated_taxes() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let aang = game
            .put_onto_battlefield(PlayerId::One, cards::AANG_SWIFT_SAVIOR)
            .unwrap();
        drain_pending(&mut game);
        let contributors = (0..9)
            .map(|_| {
                game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let purpose = ManaPaymentPurpose::Ability {
            source: aang,
            waterbend: 8,
            taps_source: false,
            leaves_source: false,
        };
        assert!(game.can_pay_cost_for(PlayerId::One, mana_cost!("{8}"), 0, &purpose));
        assert!(!game.can_pay_cost_for(PlayerId::One, mana_cost!("{9}"), 0, &purpose));
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut game =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == aang))
            .unwrap();
        game.apply(PlayerId::One, action).unwrap();
        drain_pending(&mut game);
        let permanent = game.battlefield.iter().find(|p| p.card.id == aang).unwrap();
        assert_eq!(game.power(permanent), Some(5));
        assert_eq!(
            game.battlefield
                .iter()
                .filter(|p| (p.card.id == aang || contributors.contains(&p.card.id)) && p.tapped)
                .count(),
            8
        );
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn woe_hob_kili_replaces_nonmana_equip_costs_but_keeps_taxes_and_turn_history() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.players[0].library = game
            .build_zone(PlayerId::One, &[cards::FOREST; 10])
            .unwrap();
        let bear = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let kili = game
            .put_onto_battlefield(PlayerId::One, cards::KILI_THE_RESOURCEFUL)
            .unwrap();
        let hauberk = game
            .put_onto_battlefield(PlayerId::One, cards::DEMONMAIL_HAUBERK)
            .unwrap();
        let splitter = game
            .put_onto_battlefield(PlayerId::One, cards::BONESPLITTER)
            .unwrap();
        drain_pending(&mut game);
        assert_eq!(
            game.players[0].hand.len(),
            1,
            "Equipment entries draw only once each turn"
        );
        game.put_onto_battlefield(PlayerId::Two, cards::ANOINTED_PEACEKEEPER)
            .unwrap();
        let decision = game.observe(PlayerId::Two).decision.unwrap();
        let option = decision
            .options
            .iter()
            .find(|option| option.label == "Demonmail Hauberk")
            .unwrap()
            .id;
        game.apply(
            PlayerId::Two,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![option],
            },
        )
        .unwrap();
        drain_pending(&mut game);
        game.priority = PlayerId::One;
        assert!(!game.legal_actions(PlayerId::One).iter().any(|a| matches!(a, Action::ActivateAbilityWithAlternativeCost { source, .. } if *source == hauberk)));
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
        let actions = game.legal_actions(PlayerId::One);
        assert!(
            actions.iter().any(
                |a| matches!(a, Action::ActivateAbility { source, cost_objects, .. }
            if *source == hauberk && !cost_objects.is_empty())
            ),
            "the printed sacrifice cost remains optional"
        );
        let free = actions.into_iter().find(|a| matches!(a, Action::ActivateAbilityWithAlternativeCost { source, targets, cost_objects, .. }
            if *source == hauberk && cost_objects.is_empty() && targets.iter().any(|t| t.targets() == [Target::Permanent(bear)]))).unwrap();
        game.apply(PlayerId::One, free).unwrap();
        assert_eq!(
            game.players[0].mana_pool.total(),
            0,
            "the replacement cost still pays the tax"
        );
        drain_pending(&mut game);
        assert!(game.battlefield.iter().any(|p| p.card.id == kili));
        assert_eq!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == hauberk)
                .unwrap()
                .attached_to,
            Some(bear)
        );
        game.sacrifice_permanent(hauberk);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
        assert!(
            game.legal_actions(PlayerId::One)
                .iter()
                .all(|a| a.alternative_ability_cost().is_none())
        );
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            assert!(
                state
                    .legal_actions(PlayerId::One)
                    .iter()
                    .all(|a| a.alternative_ability_cost().is_none())
            );
            state.commit_next_turn(PlayerId::One, Vec::new());
            state.step = Step::PrecombatMain;
            state.priority = PlayerId::One;
            state.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
            assert!(
                state
                    .legal_actions(PlayerId::One)
                    .iter()
                    .any(|a| matches!(a,
                Action::ActivateAbilityWithAlternativeCost { source, .. } if *source == splitter))
            );
        }
    }
}
