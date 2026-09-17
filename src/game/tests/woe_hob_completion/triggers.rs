use super::*;

#[test]
fn woe_hob_wan_shi_tong_remembers_x_after_leaving_before_its_draw_trigger() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.players[0].library = game.build_zone(PlayerId::One, &[cards::FOREST; 8]).unwrap();
        let wan = hand(&mut game, cards::WAN_SHI_TONG_LIBRARIAN);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 7);
        let cast = game.legal_actions(PlayerId::One).into_iter().find(|action|
            matches!(action, Action::CastSpell { card, choices, .. } if *card == wan && choices.x() == 5)).unwrap();
        game.apply(PlayerId::One, cast).unwrap();
        game.pass_priority(PlayerId::One);
        game.pass_priority(PlayerId::Two);
        let permanent = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::WAN_SHI_TONG_LIBRARIAN)
            .unwrap();
        assert_eq!(permanent.counters(CounterKind::PlusOnePlusOne), 0);
        let source = permanent.card.id;
        game.sacrifice_permanent(source);
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            drain_pending(state);
            assert_eq!(state.players[0].hand.len(), 2);
        }
    }
}

#[test]
fn woe_hob_vision_records_modes_when_chosen_even_if_the_trigger_is_countered() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.put_onto_battlefield(PlayerId::One, cards::THE_VISION)
            .unwrap();
        game.players[0].library = game
            .build_zone(PlayerId::One, &[cards::FOREST, cards::ISLAND])
            .unwrap();
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
        for n in 0..4 {
            let ring = hand(&mut game, cards::SOL_RING);
            let action = cast_for(&game, PlayerId::One, ring).unwrap();
            game.apply(PlayerId::One, action).unwrap();
            if n < 2 {
                let decision = game.observe(PlayerId::One).decision.unwrap();
                assert_eq!(decision.options.len(), 3 - n);
                assert!(
                    n == 0
                        || decision
                            .options
                            .iter()
                            .all(|o| !o.label.starts_with("Solar Beam"))
                );
                let label = if n == 0 {
                    "Solar Beam"
                } else {
                    "Density Control"
                };
                let choice = decision
                    .options
                    .iter()
                    .find(|o| o.label.starts_with(label))
                    .unwrap()
                    .id;
                game.apply(
                    PlayerId::One,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options: vec![choice],
                    },
                )
                .unwrap();
                let trigger = game.stack.last().unwrap().id;
                game.counter_spell(trigger);
            }
            if n == 3 {
                assert_eq!(
                    game.stack.len(),
                    1,
                    "no trigger can stay on the stack without an available mode"
                );
            }
            drain_pending(&mut game);
            let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
            game = Game::from_observation_checkpoint(
                game.catalog.clone(),
                game.format,
                &wire,
                &hidden,
                0,
            )
            .unwrap();
        }
        assert_eq!(game.players[0].hand.len(), 1);
    }
}

#[test]
fn woe_hob_esper_origins_flashback_returns_transformed_with_finality_and_first_chapter() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        game.players[0].graveyard = game
            .build_zone(PlayerId::One, &[cards::ESPER_ORIGINS])
            .unwrap();
        game.players[0].library = game
            .build_zone(
                PlayerId::One,
                &[cards::FOREST, cards::ISLAND, cards::PLAINS],
            )
            .unwrap();
        let esper = game.players[0].graveyard[0].id;
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 4);
        let action = cast_for(&game, PlayerId::One, esper).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        drain_pending(&mut game);
        let permanent = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::ESPER_ORIGINS)
            .unwrap();
        assert_eq!(game.power(permanent), Some(4));
        assert_eq!(permanent.counters(CounterKind::Finality), 1);
        assert_eq!(permanent.counters(CounterKind::Lore), 1);
        assert_eq!(game.players[0].hand.len(), 1);
        assert_eq!(game.players[0].life, 22);
        let id = permanent.card.id;
        game.sacrifice_permanent(id);
        assert!(
            game.players[0]
                .exile
                .iter()
                .any(|card| card.definition == cards::ESPER_ORIGINS)
        );
        assert!(
            !game.players[0]
                .graveyard
                .iter()
                .any(|card| card.definition == cards::ESPER_ORIGINS)
        );
    }
}

#[test]
fn woe_hob_sneak_has_its_own_timing_and_returns_to_the_same_planeswalker_defender() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let attacker = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let planeswalker = game
            .put_onto_battlefield(PlayerId::Two, cards::JACE_THE_MIND_SCULPTOR)
            .unwrap();
        let elektra = hand(&mut game, cards::ELEKTRA_DAUGHTER_OF_THE_HAND);
        let p = game
            .battlefield
            .iter_mut()
            .find(|p| p.card.id == attacker)
            .unwrap();
        p.attacking = true;
        p.attack_defender = Some(AttackDefender::Planeswalker(planeswalker));
        game.step = Step::DeclareBlockers;
        game.blockers_declared = false;
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 3);
        assert!(cast_for(&game, PlayerId::One, elektra).is_none());
        game.blockers_declared = true;
        let action = cast_for(&game, PlayerId::One, elektra).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        assert!(
            game.players[0]
                .hand
                .iter()
                .any(|c| c.definition == cards::GRIZZLY_BEARS)
        );
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        game =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        drain_pending(&mut game);
        let ninja = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::ELEKTRA_DAUGHTER_OF_THE_HAND)
            .unwrap();
        assert!(ninja.tapped && ninja.attacking);
        assert_eq!(
            ninja.attack_defender,
            Some(AttackDefender::Planeswalker(planeswalker))
        );
    }
}

#[test]
fn woe_hob_celestial_reunion_announces_the_type_and_beholds_mixed_zones() {
    for prepared in [false, true] {
        for chosen in [None, Some("Elf"), Some("Druid")] {
            let mut game = setup(prepared);
            let elf = game
                .put_onto_battlefield(PlayerId::One, cards::LLANOWAR_ELVES)
                .unwrap();
            let revealed = hand(&mut game, cards::LLANOWAR_ELVES);
            let reunion = hand(&mut game, cards::CELESTIAL_REUNION);
            game.players[0].library = game
                .build_zone(PlayerId::One, &[cards::ELVISH_VISIONARY, cards::FOREST])
                .unwrap();
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 3);
            let action = game.legal_actions(PlayerId::One).into_iter().find(|action| matches!(action,
                Action::CastSpell { card, choices, sacrifices } if *card == reunion && choices.x() == 2
                    && choices.costs().chosen_creature_type().map(crate::card::Subtype::name) == chosen
                    && if chosen.is_some() { sacrifices == &[elf, revealed] } else { sacrifices.is_empty() }))
                .expect("every shared type is a distinct announced choice");
            game.apply(PlayerId::One, action).unwrap();
            assert!(game.players[0].hand.iter().any(|card| card.id == revealed));
            assert!(
                game.battlefield
                    .iter()
                    .any(|p| p.card.id == elf && !p.tapped)
            );
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
                pass_priority_pair(state);
                choose_decision_by_label(state, PlayerId::One, "Elvish Visionary");
                drain_pending(state);
                let on_battlefield = state
                    .battlefield
                    .iter()
                    .any(|p| p.card.definition == cards::ELVISH_VISIONARY);
                assert_eq!(
                    on_battlefield,
                    chosen == Some("Elf"),
                    "chosen type: {chosen:?}"
                );
                if !on_battlefield {
                    assert!(
                        state.players[0]
                            .hand
                            .iter()
                            .any(|c| c.definition == cards::ELVISH_VISIONARY)
                    );
                }
            }
        }
    }
}

#[test]
fn woe_hob_earthbender_ascension_reflexive_condition_is_checked_twice() {
    for remove_before_resolution in [false, true] {
        let mut game = setup(true);
        let bear = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let ascension = game
            .put_onto_battlefield(PlayerId::One, cards::EARTHBENDER_ASCENSION)
            .unwrap();
        drain_pending(&mut game);
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == ascension)
            .unwrap()
            .set_counters(CounterKind::named("quest"), 3);
        game.put_onto_battlefield(PlayerId::One, cards::FOREST)
            .unwrap();
        game.finish_rules_procedure();
        pass_priority_pair(&mut game);
        assert_eq!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == ascension)
                .unwrap()
                .counters(CounterKind::named("quest")),
            4
        );
        if let Some(decision) = game.observe(PlayerId::One).decision {
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![decision.options[0].id],
                },
            )
            .unwrap();
        }
        assert_eq!(
            game.stack.len(),
            1,
            "threshold schedules a separate reflexive trigger"
        );
        if remove_before_resolution {
            game.remove_counters_from_object(
                Target::Permanent(ascension),
                CounterKind::named("quest"),
                1,
            );
        }
        drain_pending(&mut game);
        let bear = game.battlefield.iter().find(|p| p.card.id == bear).unwrap();
        assert_eq!(
            bear.counters(CounterKind::PlusOnePlusOne),
            u16::from(!remove_before_resolution)
        );
        assert_eq!(
            game.permanent_has_executable_keyword(bear, KeywordAbility::Trample),
            !remove_before_resolution
        );
    }
}

#[test]
fn woe_hob_first_spell_discounts_read_history_before_the_sources_entered() {
    for prepared in [false, true] {
        for cast_before_sources in [false, true] {
            let mut game = setup(prepared);
            if cast_before_sources {
                let thor = hand(&mut game, cards::THOR_GOD_OF_THUNDER);
                game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 5);
                let action = cast_for(&game, PlayerId::One, thor).unwrap();
                game.apply(PlayerId::One, action).unwrap();
                drain_pending(&mut game);
            }
            game.put_onto_battlefield(PlayerId::One, cards::SERAH_FARRON)
                .unwrap();
            game.put_onto_battlefield(PlayerId::One, cards::MOMO_FRIENDLY_FLIER)
                .unwrap();
            drain_pending(&mut game);
            let thor = hand(&mut game, cards::THOR_GOD_OF_THUNDER);
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);
            assert_eq!(
                cast_for(&game, PlayerId::One, thor).is_some(),
                !cast_before_sources
            );
            if cast_before_sources {
                game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
            }
            let action = cast_for(&game, PlayerId::One, thor).unwrap();
            game.apply(PlayerId::One, action).unwrap();
            assert_eq!(game.players[0].mana_pool.total(), 0);
        }
    }
}

#[test]
fn woe_hob_sunderflock_uses_the_greatest_elemental_mana_value_and_cast_entry() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let elemental = game
            .put_onto_battlefield(PlayerId::One, cards::HEARTH_ELEMENTAL)
            .unwrap();
        let other = game
            .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
        let flock = hand(&mut game, cards::SUNDERFLOCK);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
        assert!(cast_for(&game, PlayerId::One, flock).is_none());
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
        let action = cast_for(&game, PlayerId::One, flock).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        drain_pending(&mut game);
        assert!(game.battlefield.iter().any(|p| p.card.id == elemental));
        assert!(!game.battlefield.iter().any(|p| p.card.id == other));
        assert!(
            game.players[1]
                .hand
                .iter()
                .any(|c| c.definition == cards::GRIZZLY_BEARS)
        );
    }
}
