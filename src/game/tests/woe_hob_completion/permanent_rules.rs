use super::*;

#[test]
fn woe_hob_modal_heroes_transform_only_once_for_copied_abilities() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let miles = game
            .put_onto_battlefield(PlayerId::One, cards::MILES_MORALES)
            .unwrap();
        drain_pending(&mut game);
        for color in [ManaColor::Red, ManaColor::Green, ManaColor::White] {
            game.add_unrestricted_mana(PlayerId::One, color, 2);
        }
        let activation = game.legal_actions(PlayerId::One).into_iter().find(|action|
            matches!(action, Action::ActivateAbility { source, .. } if *source == miles)).unwrap();
        game.apply(PlayerId::One, activation).unwrap();
        let mut copied = game.stack.last().unwrap().clone();
        copied.id = game.allocate_object_id();
        copied.card.id = copied.id;
        copied.is_copy = true;
        game.stack.push(copied);
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            drain_pending(state);
            let permanent = state
                .battlefield
                .iter()
                .find(|p| p.card.id == miles)
                .unwrap();
            assert_eq!(permanent.presented, CardPartId(1));
            assert_eq!(permanent.transform_count, 1);
        }
    }
}

#[test]
fn woe_hob_nowhere_to_run_ignores_hexproof_and_suppresses_only_ward() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let stallion = game
            .put_onto_battlefield(PlayerId::Two, cards::COLORSTORM_STALLION)
            .unwrap();
        let permanent = game
            .battlefield
            .iter_mut()
            .find(|p| p.card.id == stallion)
            .unwrap();
        permanent.add_counters(CounterKind::PlusOnePlusOne, 4);
        permanent.temporary_keywords.push(KeywordAbility::Hexproof);
        let nowhere = game
            .put_onto_battlefield(PlayerId::One, cards::NOWHERE_TO_RUN)
            .unwrap();
        drain_pending(&mut game);
        let permanent = game
            .battlefield
            .iter()
            .find(|p| p.card.id == stallion)
            .unwrap();
        assert_eq!(
            game.power(permanent),
            Some(4),
            "the enters trigger bypasses hexproof and ward"
        );
        assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Hexproof));
        let mut ward = false;
        game.for_each_effective_ability(permanent, |ability| {
            ward |= crate::card::AbilityPredicateDef::Is(crate::card::AbilityKindDef::Ward)
                .matches(&ability.ability);
        });
        assert!(
            ward,
            "ward remains an ability despite being unable to trigger"
        );
        assert!(game.permanent_can_be_targeted_by(permanent, PlayerId::One, nowhere, false));
        game.sacrifice_permanent(nowhere);
        let permanent = game
            .battlefield
            .iter()
            .find(|p| p.card.id == stallion)
            .unwrap();
        assert!(!game.permanent_can_be_targeted_by(permanent, PlayerId::One, nowhere, false));
    }
}

#[test]
fn woe_hob_hone_counters_follow_the_equipment_after_dwalin_leaves() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let first = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let second = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let sword = game
            .put_onto_battlefield(PlayerId::One, cards::BONESPLITTER)
            .unwrap();
        assert!(game.try_attach(sword, first));
        let dwalin = game
            .put_onto_battlefield(PlayerId::One, cards::DWALIN_WEAPONMASTER)
            .unwrap();
        drain_pending(&mut game);
        game.sacrifice_permanent(dwalin);
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            let creature = state
                .battlefield
                .iter()
                .find(|p| p.card.id == first)
                .unwrap();
            assert_eq!(
                (state.power(creature), state.toughness(creature)),
                (Some(5), Some(2))
            );
            assert!(state.try_attach(sword, second));
            let creature = state
                .battlefield
                .iter()
                .find(|p| p.card.id == first)
                .unwrap();
            assert_eq!(state.power(creature), Some(2));
            let creature = state
                .battlefield
                .iter()
                .find(|p| p.card.id == second)
                .unwrap();
            assert_eq!(state.power(creature), Some(5));
        }
    }
}

#[test]
fn woe_hob_mind_stone_harness_is_a_persistent_noncopyable_designation() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let stone = game
            .put_onto_battlefield(PlayerId::One, cards::THE_MIND_STONE)
            .unwrap();
        let bear = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        game.step = Step::End;
        game.begin_step_triggers();
        assert!(game.pending_triggers.is_empty());
        assert!(game.stack.is_empty());
        game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 6);
        let activation = game.legal_actions(PlayerId::One).into_iter().find(|action|
            matches!(action, Action::ActivateAbility { source, .. } if *source == stone)).unwrap();
        game.apply(PlayerId::One, activation).unwrap();
        drain_pending(&mut game);
        assert_eq!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == stone)
                .unwrap()
                .designations,
            vec![crate::card::PermanentDesignationDef::Harnessed]
        );
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            state.begin_step_triggers();
            drain_pending(state);
            assert!(!state.battlefield.iter().any(|p| p.card.id == bear));
            assert!(
                state
                    .battlefield
                    .iter()
                    .any(|p| p.card.definition == cards::GRIZZLY_BEARS)
            );
            let exiled = state.exile_permanent_returning_card(stone).unwrap();
            state.return_exiled_card(exiled, ZoneKind::Battlefield, None, None, false, None);
            let returned = state
                .battlefield
                .iter()
                .find(|p| p.card.definition == cards::THE_MIND_STONE)
                .unwrap();
            assert_ne!(returned.card.id, stone);
            assert!(returned.designations.is_empty());
        }
    }
}

#[test]
fn woe_hob_storied_counts_distinct_permanents_and_keeps_the_designation() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let stone = game
            .put_onto_battlefield(PlayerId::One, cards::THE_MIND_STONE)
            .unwrap();
        let fili = game
            .put_onto_battlefield(PlayerId::One, cards::FILI_THE_PATHFINDER)
            .unwrap();
        assert!(
            !game.enduring_story[0],
            "a legendary artifact counts as one permanent"
        );
        drain_pending(&mut game);
        let sword = game
            .put_onto_battlefield(PlayerId::One, cards::BONESPLITTER)
            .unwrap();
        assert!(
            game.enduring_story[0],
            "the third permanent grants the story before priority"
        );
        game.sacrifice_permanent(stone);
        game.sacrifice_permanent(sword);
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            assert!(state.enduring_story[0]);
            assert!(!state.enduring_story[1]);
            let permanent = state
                .battlefield
                .iter()
                .find(|p| p.card.id == fili)
                .unwrap();
            assert_eq!(
                (state.power(permanent), state.toughness(permanent)),
                (Some(3), Some(3))
            );
            let token = state
                .battlefield
                .iter()
                .find(|p| p.card.definition.is_token())
                .unwrap();
            assert_eq!(
                (state.power(token), state.toughness(token)),
                (Some(3), Some(3))
            );
        }
    }
}

#[test]
fn woe_hob_winnowing_uses_each_players_chosen_type_and_sacrifices_together() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let first_bear = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let other_bear = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        game.put_onto_battlefield(PlayerId::One, cards::LLANOWAR_ELVES)
            .unwrap();
        let merfolk = game
            .put_onto_battlefield(PlayerId::Two, cards::MERFOLK_OF_THE_PEARL_TRIDENT)
            .unwrap();
        game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
            .unwrap();
        let changeling = game
            .put_onto_battlefield(PlayerId::Two, cards::CHOMPING_CHANGELING)
            .unwrap();
        drain_pending(&mut game);
        let winnowing = hand(&mut game, cards::WINNOWING);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 6);
        let action = cast_for(&game, PlayerId::One, winnowing).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        drain_pending(&mut game);
        for id in [first_bear, other_bear, merfolk, changeling] {
            assert!(game.battlefield.iter().any(|p| p.card.id == id));
        }
        assert_eq!(game.battlefield.len(), 4);
        assert_eq!(game.permanents_sacrificed_this_turn, [1, 1]);
    }
}

#[test]
fn woe_hob_zack_uses_all_counters_and_last_known_equipment_after_sacrifice() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let zack = game
            .put_onto_battlefield(PlayerId::One, cards::ZACK_FAIR)
            .unwrap();
        let bear = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let equipment = game
            .put_onto_battlefield(PlayerId::One, cards::BONESPLITTER)
            .unwrap();
        assert!(game.try_attach(equipment, zack));
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == zack)
            .unwrap()
            .add_counters(CounterKind::Flying, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        let activation = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a,
            Action::ActivateAbility { source, targets, .. } if *source == zack && targets.iter().any(|selection| selection.targets() == [Target::Permanent(bear)]))).unwrap();
        game.apply(PlayerId::One, activation).unwrap();
        assert!(!game.battlefield.iter().any(|p| p.card.id == zack));
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for (restored, state) in [(false, &mut game), (true, &mut rebuilt)] {
            assert!(
                state
                    .current_or_last_known_counter_inventory(zack)
                    .contains(&(CounterKind::Flying, 1)),
                "Zack counter inventory restored={restored}: {:?}",
                state.current_or_last_known_counter_inventory(zack)
            );
            drain_pending(state);
            let creature = state
                .battlefield
                .iter()
                .find(|p| p.card.id == bear)
                .unwrap();
            assert_eq!(creature.counters(CounterKind::PlusOnePlusOne), 1);
            assert_eq!(creature.counters(CounterKind::Flying), 1);
            assert!(
                state.permanent_has_executable_keyword(creature, KeywordAbility::Indestructible)
            );
            assert_eq!(
                state
                    .battlefield
                    .iter()
                    .find(|p| p.card.id == equipment)
                    .unwrap()
                    .attached_to,
                Some(bear)
            );
        }
    }
}

#[test]
fn woe_hob_earthbend_return_is_independent_of_land_abilities() {
    for prepared in [false, true] {
        for exile in [false, true] {
            let mut game = setup(prepared);
            let forest = game
                .put_onto_battlefield(PlayerId::One, cards::FOREST)
                .unwrap();
            let city = game
                .put_onto_battlefield(PlayerId::One, cards::BA_SING_SE)
                .unwrap();
            assert!(
                !game
                    .battlefield
                    .iter()
                    .find(|p| p.card.id == city)
                    .unwrap()
                    .tapped
            );
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 3);
            let activation = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a,
                Action::ActivateAbility { source, targets, .. } if *source == city
                    && targets.iter().any(|selection| selection.targets() == [Target::Permanent(forest)]))).unwrap();
            game.apply(PlayerId::One, activation).unwrap();
            drain_pending(&mut game);
            let land = game
                .battlefield
                .iter()
                .find(|p| p.card.id == forest)
                .unwrap();
            assert_eq!((game.power(land), game.toughness(land)), (Some(2), Some(2)));
            assert!(game.permanent_has_executable_keyword(land, KeywordAbility::Haste));
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
                attach_constant_resolved_characteristics(
                    state,
                    forest,
                    &[AppliedEffectDef::remove_abilities(
                        crate::card::AbilityPredicateDef::Any,
                    )],
                    ContinuousEffectExpiration::Never,
                );
                if exile {
                    state.exile_permanent(forest);
                } else {
                    state.sacrifice_permanent(forest);
                }
                drain_pending(state);
                let returned = state
                    .battlefield
                    .iter()
                    .find(|p| p.card.definition == cards::FOREST)
                    .unwrap();
                assert_ne!(returned.card.id, forest);
                assert!(returned.tapped);
                assert_eq!(returned.counters(CounterKind::PlusOnePlusOne), 0);
                assert_eq!(state.power(returned), None);
            }
        }
    }
}

#[test]
fn woe_hob_shield_counter_is_intrinsic_and_consumed_once_per_simultaneous_event() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let captain = game
            .put_onto_battlefield(PlayerId::One, cards::CAPTAIN_AMERICA_SUPER_SOLDIER)
            .unwrap();
        let ally = game
            .put_onto_battlefield(PlayerId::One, cards::MILES_MORALES)
            .unwrap();
        drain_pending(&mut game);
        let protected =
            || crate::card::AppliedRuleDef::PlayerRule(crate::card::PlayerRuleDef::Hexproof);
        assert!(game.player_rule_applies(PlayerId::One, protected()));
        assert!(game.permanent_has_executable_keyword(
            game.battlefield.iter().find(|p| p.card.id == ally).unwrap(),
            KeywordAbility::Hexproof
        ));
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == captain)
            .unwrap()
            .add_counters(CounterKind::Shield, 1);
        game.deal_damage_simultaneously(vec![
            DamageAssignment {
                source: None,
                target: Some(Target::Permanent(captain)),
                amount: 3,
                combat: false,
            },
            DamageAssignment {
                source: None,
                target: Some(Target::Permanent(captain)),
                amount: 4,
                combat: false,
            },
        ]);
        let p = game
            .battlefield
            .iter()
            .find(|p| p.card.id == captain)
            .unwrap();
        assert_eq!((p.damage, p.counters(CounterKind::Shield)), (0, 1));
        attach_constant_resolved_characteristics(
            &mut game,
            captain,
            &[AppliedEffectDef::remove_abilities(
                crate::card::AbilityPredicateDef::Any,
            )],
            ContinuousEffectExpiration::Never,
        );
        game.destroy_permanents(&[captain], true);
        assert_eq!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == captain)
                .unwrap()
                .counters(CounterKind::Shield),
            0
        );
        assert!(!game.player_rule_applies(PlayerId::One, protected()));
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == captain)
            .unwrap()
            .add_counters(CounterKind::Shield, 2);
        game.damage_cannot_be_prevented_this_turn = true;
        game.deal_damage_simultaneously(vec![DamageAssignment {
            source: None,
            target: Some(Target::Permanent(captain)),
            amount: 3,
            combat: false,
        }]);
        assert_eq!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == captain)
                .unwrap()
                .counters(CounterKind::Shield),
            1
        );
        game.check_state_based_actions();
        assert!(
            !game.battlefield.iter().any(|p| p.card.id == captain),
            "the remaining shield cannot replace lethal damage destruction"
        );
    }
}

#[test]
fn woe_hob_lightning_doubles_future_damage_to_the_player_and_their_current_permanents() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let lightning = game
            .put_onto_battlefield(PlayerId::One, cards::LIGHTNING_ARMY_OF_ONE)
            .unwrap();
        let bear = game
            .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
        game.deal_damage_simultaneously(vec![DamageAssignment {
            source: Some(lightning),
            target: Some(Target::Player(PlayerId::Two)),
            amount: 3,
            combat: true,
        }]);
        assert_eq!(game.players[1].life, 17);
        game.finish_rules_procedure();
        drain_pending(&mut game);
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            state.deal_damage_simultaneously(vec![DamageAssignment {
                source: None,
                target: Some(Target::Permanent(bear)),
                amount: 1,
                combat: false,
            }]);
            assert_eq!(
                state
                    .battlefield
                    .iter()
                    .find(|p| p.card.id == bear)
                    .unwrap()
                    .damage,
                2
            );
            state
                .battlefield
                .iter_mut()
                .find(|p| p.card.id == bear)
                .unwrap()
                .controller = PlayerId::One;
            state.deal_damage_simultaneously(vec![DamageAssignment {
                source: None,
                target: Some(Target::Permanent(bear)),
                amount: 1,
                combat: false,
            }]);
            assert_eq!(
                state
                    .battlefield
                    .iter()
                    .find(|p| p.card.id == bear)
                    .unwrap()
                    .damage,
                3
            );
            state.commit_next_turn(PlayerId::One, Vec::new());
            state.deal_damage_simultaneously(vec![DamageAssignment {
                source: None,
                target: Some(Target::Player(PlayerId::Two)),
                amount: 1,
                combat: false,
            }]);
            assert_eq!(state.players[1].life, 16);
        }
    }
}

#[test]
fn woe_hob_oko_emblem_retains_the_chosen_type_after_oko_leaves_and_checkpoint_restores() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let oko = game
            .put_onto_battlefield(PlayerId::One, cards::OKO_LORWYN_LIEGE)
            .unwrap();
        let bear = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let elf = game
            .put_onto_battlefield(PlayerId::One, cards::LLANOWAR_ELVES)
            .unwrap();
        game.transform_permanent(oko);
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == oko)
            .unwrap()
            .add_counters(CounterKind::Loyalty, 3);
        let activation = activation_named(&game, oko, "−6");
        game.apply(PlayerId::One, activation).unwrap();
        assert!(!game.battlefield.iter().any(|p| p.card.id == oko));
        pass_priority_pair(&mut game);
        assert!(game.observe(PlayerId::One).decision.is_some());
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            choose_decision_by_label(state, PlayerId::One, "Bear");
            drain_pending(state);
            assert_eq!(
                state.emblems[0].chosen_creature_type.as_deref(),
                Some("Bear")
            );
            assert_eq!(
                state.power(
                    state
                        .battlefield
                        .iter()
                        .find(|p| p.card.id == bear)
                        .unwrap()
                ),
                Some(5)
            );
            assert_eq!(
                state.power(state.battlefield.iter().find(|p| p.card.id == elf).unwrap()),
                Some(1)
            );
            let (wire, hidden) = checkpoint_fixture(state, PlayerId::One);
            let restored = Game::from_observation_checkpoint(
                state.catalog.clone(),
                state.format,
                &wire,
                &hidden,
                0,
            )
            .unwrap();
            assert_eq!(
                restored.power(
                    restored
                        .battlefield
                        .iter()
                        .find(|p| p.card.id == bear)
                        .unwrap()
                ),
                Some(5)
            );
        }
    }
}
