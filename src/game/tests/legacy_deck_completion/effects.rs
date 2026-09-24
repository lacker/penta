use super::*;

#[test]
fn ad_nauseam_stops_after_the_last_card_including_after_checkpoint_restore() {
    for library in [
        vec![cards::GRIZZLY_BEARS],
        vec![cards::FOREST, cards::GRIZZLY_BEARS, cards::SERRA_ANGEL],
    ] {
        for starting_life in [7, 20] {
            let mut game = ready_game();
            game.players[0].life = starting_life;
            game.players[0].library = game.build_zone(PlayerId::One, &library).unwrap();
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 5);
            cast(&mut game, cards::AD_NAUSEAM, None);
            game.apply(game.priority, Action::PassPriority).unwrap();
            game.apply(game.priority, Action::PassPriority).unwrap();
            for remaining in (1..library.len()).rev() {
                assert_eq!(game.players[0].library.len(), remaining);
                assert!(
                    game.result.is_none(),
                    "life loss waits for resolution to end"
                );
                let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
                game = Game::from_observation_checkpoint(
                    game.catalog.clone(),
                    game.format,
                    &wire,
                    &hidden,
                    2,
                )
                .unwrap();
                choose_label(&mut game, "Yes");
            }
            assert!(game.players[0].library.is_empty());
            assert_eq!(game.players[0].hand.len(), library.len());
            assert_eq!(
                game.players[0].life,
                starting_life - if library.len() == 1 { 2 } else { 7 }
            );
            assert!(
                game.pending_decisions.is_empty(),
                "no empty-library repeat offer"
            );
            assert!(game.stack.is_empty());
            assert!(!game.players[0].tried_to_draw_from_empty_library);
            if game.players[0].life == 0 {
                assert!(matches!(
                    game.result,
                    Some(GameResult::Winner {
                        winner: PlayerId::Two,
                        ..
                    })
                ));
            } else {
                assert!(game.result.is_none());
            }
        }
    }
}

#[test]
fn ad_nauseam_repeats_after_each_card_and_stops_without_an_extra_reveal() {
    let mut game = ready_game();
    game.players[0].library = game
        .build_zone(
            PlayerId::One,
            &[cards::FOREST, cards::GRIZZLY_BEARS, cards::SERRA_ANGEL],
        )
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 5);
    cast(&mut game, cards::AD_NAUSEAM, None);
    stop_at_decision(&mut game);
    assert_eq!(game.players[0].life, 15);
    assert_eq!(game.players[0].hand[0].definition, cards::SERRA_ANGEL);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    game = Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 2)
        .unwrap();
    choose_label(&mut game, "Yes");
    assert_eq!(game.players[0].life, 13);
    assert_eq!(game.players[0].hand.len(), 2);
    choose_label(&mut game, "No");
    drain_pending(&mut game);
    assert_eq!(game.players[0].library.len(), 1);
}

#[test]
fn putrid_imp_discard_flying_and_threshold_are_independent() {
    let mut game = ready_game();
    let imp = put_ready(&mut game, cards::PUTRID_IMP);
    game.players[0].graveyard = game.build_zone(PlayerId::One, &[cards::FOREST; 6]).unwrap();
    game.players[0].hand = game.build_zone(PlayerId::One, &[cards::SWAMP]).unwrap();
    activate(&mut game, imp);
    drain_pending(&mut game);
    let permanent = game.battlefield.iter().find(|p| p.card.id == imp).unwrap();
    assert_eq!(game.power(permanent), Some(2));
    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Flying));
    assert_eq!(game.players[0].graveyard.len(), 7);
    assert!(game.players[0].hand.is_empty());
}

#[test]
fn debt_to_the_deathless_drains_twice_chosen_x() {
    let mut game = ready_game();
    game.players[0].hand = game
        .build_zone(PlayerId::One, &[cards::DEBT_TO_THE_DEATHLESS])
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 5);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell {choices, ..} if choices.x() == 3))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, 26);
    assert_eq!(game.players[1].life, 14);
}

#[test]
fn lavinia_rejects_noncreatures_above_land_count_and_counters_free_creatures() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::Two, cards::LAVINIA_AZORIUS_RENEGADE)
        .unwrap();
    game.players[0].hand = game
        .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT, cards::MEMNITE])
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 10);
    let actions = game.legal_actions(PlayerId::One);
    let bolt = game.players[0].hand[0].id;
    assert!(
        !actions
            .iter()
            .any(|a| matches!(a, Action::CastSpell {card, ..} if *card == bolt))
    );
    let free = actions
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell { .. }))
        .unwrap();
    game.apply(PlayerId::One, free).unwrap();
    drain_pending(&mut game);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::MEMNITE)
    );
}

#[test]
fn beseech_without_bargain_keeps_the_searched_card_hidden_then_puts_it_in_hand() {
    let mut game = ready_game();
    game.players[0].library = game
        .build_zone(PlayerId::One, &[cards::SERRA_ANGEL])
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 4);
    cast(&mut game, cards::BESEECH_THE_MIRROR, None);
    drain_pending(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].hand[0].definition, cards::SERRA_ANGEL);
    assert!(game.players[0].exile.is_empty());
}

#[test]
fn ba_sing_se_entry_and_earthbend_two() {
    let mut game = ready_game();
    let first = put_ready(&mut game, cards::BA_SING_SE);
    assert!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == first)
            .unwrap()
            .tapped
    );
    let forest = put_ready(&mut game, cards::FOREST);
    let second = put_ready(&mut game, cards::BA_SING_SE);
    assert!(
        !game
            .battlefield
            .iter()
            .find(|p| p.card.id == second)
            .unwrap()
            .tapped
    );
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 3);
    let action = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a, Action::ActivateAbility {source, targets, ..} if *source == second && targets.iter().any(|t| t.targets().contains(&Target::Permanent(forest))))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|p| p.card.id == forest)
        .unwrap();
    assert_eq!(game.power(permanent), Some(2));
    assert_eq!(permanent.counters(CounterKind::PlusOnePlusOne), 2);
}

#[test]
fn cloak_returns_a_hand_card_immediately_and_keeps_its_duration_in_checkpoints() {
    let mut game = ready_game();
    game.players[1].hand = game
        .build_zone(PlayerId::Two, &[cards::GRIZZLY_BEARS])
        .unwrap();
    let cloak = put_ready(&mut game, cards::CLOAK_AND_DAGGER_ENTWINED);
    drain_pending(&mut game);
    assert!(game.players[1].hand.is_empty());
    assert_eq!(game.players[1].exile.len(), 1);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut restored =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 2)
            .unwrap();
    restored.move_permanents_to_zone(&[cloak], ZoneKind::Graveyard, ZonePlacement::Top);
    assert_eq!(restored.players[1].hand.len(), 1);
    assert!(restored.players[1].exile.is_empty());
    assert!(
        restored.pending_triggers.is_empty(),
        "the return is not a triggered ability"
    );
}

#[test]
fn cloak_does_not_exile_when_it_left_before_its_enter_trigger_resolved() {
    let mut game = ready_game();
    game.players[1].hand = game
        .build_zone(PlayerId::Two, &[cards::GRIZZLY_BEARS])
        .unwrap();
    let cloak = put_ready(&mut game, cards::CLOAK_AND_DAGGER_ENTWINED);
    game.move_permanents_to_zone(&[cloak], ZoneKind::Graveyard, ZonePlacement::Top);
    drain_pending(&mut game);
    assert_eq!(game.players[1].hand.len(), 1);
    assert!(game.players[1].exile.is_empty());
}

#[test]
fn ghost_vacuum_returns_only_linked_creatures_with_their_entry_characteristics() {
    let mut game = ready_game();
    let vacuum = put_ready(&mut game, cards::GHOST_VACUUM);
    game.players[1].graveyard = game
        .build_zone(PlayerId::Two, &[cards::GRIZZLY_BEARS, cards::FOREST])
        .unwrap();
    for _ in 0..2 {
        activate(&mut game, vacuum);
        drain_pending(&mut game);
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == vacuum)
            .unwrap()
            .tapped = false;
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 6);
    activate(&mut game, vacuum);
    drain_pending(&mut game);
    let bear = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::GRIZZLY_BEARS)
        .unwrap();
    assert_eq!(bear.controller, PlayerId::One);
    assert_eq!(bear.card.owner, PlayerId::Two);
    assert_eq!(game.power(bear), Some(1));
    assert!(
        game.effective_subtypes(bear)
            .contains(crate::card::Subtype::Spirit)
    );
    assert!(
        game.effective_subtypes(bear)
            .contains(crate::card::Subtype::Bear)
    );
    assert_eq!(bear.counters(CounterKind::Flying), 1);
    assert_eq!(game.players[1].exile.len(), 1);
    assert_eq!(game.players[1].exile[0].definition, cards::FOREST);
}

#[test]
fn lazotep_quarry_replaces_creature_types_and_copies_the_exiled_card() {
    let mut game = ready_game();
    let quarry = put_ready(&mut game, cards::LAZOTEP_QUARRY);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS])
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateAbility {source, x: 2, ..} if *source == quarry))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);
    let zombie = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert_eq!(game.power(zombie), Some(4));
    assert!(
        game.effective_subtypes(zombie)
            .contains(crate::card::Subtype::Zombie)
    );
    assert!(
        !game
            .effective_subtypes(zombie)
            .contains(crate::card::Subtype::Bear)
    );
    assert_eq!(game.players[0].exile.len(), 1);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::LAZOTEP_QUARRY)
    );
}

#[test]
fn raph_and_mikey_put_the_first_creature_into_combat_without_casting_it() {
    let mut game = ready_game();
    let turtles = put_ready(&mut game, cards::RAPH_MIKEY_TROUBLEMAKERS);
    game.players[0].library = game
        .build_zone(
            PlayerId::One,
            &[
                cards::SERRA_ANGEL,
                cards::GRIZZLY_BEARS,
                cards::FOREST,
                cards::ISLAND,
            ],
        )
        .unwrap();
    game.step = Step::DeclareAttackers;
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: turtles,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .unwrap();
    game.finish_declaring_attackers();
    drain_pending(&mut game);
    let bear = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::GRIZZLY_BEARS)
        .unwrap();
    assert!(bear.tapped && bear.attacking);
    assert_eq!(
        bear.attack_defender,
        Some(AttackDefender::Player(PlayerId::Two))
    );
    assert_eq!(
        game.players[0].library.last().unwrap().definition,
        cards::SERRA_ANGEL
    );
}

#[test]
fn carpet_declining_first_main_preserves_second_main_but_adding_mana_consumes_it() {
    let mut game = ready_game();
    let carpet = put_ready(&mut game, cards::CARPET_OF_FLOWERS);
    for _ in 0..2 {
        game.put_onto_battlefield(PlayerId::Two, cards::ISLAND)
            .unwrap();
    }
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::PrecombatMain,
        player: PlayerId::One,
    });
    stop_at_decision(&mut game);
    choose_label(&mut game, "your opponent");
    stop_at_decision(&mut game);
    choose_label(&mut game, "No");
    drain_pending(&mut game);
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
    assert_eq!(game.players[0].mana_pool.green, 2);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut restored =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 2)
            .unwrap();
    restored.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::PostcombatMain,
        player: PlayerId::One,
    });
    assert!(
        !restored
            .pending_triggers
            .iter()
            .any(|t| t.source.object == carpet)
    );
}

#[test]
fn veil_protects_player_and_permanents_and_draws_after_opponents_blue_spell() {
    let mut game = ready_game();
    let bear = put_ready(&mut game, cards::GRIZZLY_BEARS);
    game.players[1].hand = game
        .build_zone(PlayerId::Two, &[cards::BRAINSTORM])
        .unwrap();
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 1);
    game.priority = PlayerId::Two;
    let brainstorm = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell { .. }))
        .unwrap();
    game.apply(PlayerId::Two, brainstorm).unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    cast(&mut game, cards::VEIL_OF_SUMMER, None);
    // Resolve just Veil, before Brainstorm's decisions.
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    assert_eq!(game.players[0].hand.len(), 1);
    game.players[1].hand = game
        .build_zone(PlayerId::Two, &[cards::UNSUMMON, cards::DURESS])
        .unwrap();
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 1);
    game.priority = PlayerId::Two;
    assert!(!game.legal_actions(PlayerId::Two).iter().any(|a| matches!(a, Action::CastSpell {choices, ..} if choices.targets().iter().any(|t| t.targets().contains(&Target::Permanent(bear))))));
}

#[test]
fn bargained_beseech_checks_the_selected_adventure_spell_and_never_offers_lands() {
    for (found, primary_allowed, offered) in [
        (cards::BRAMBLE_FAMILIAR, true, true),
        (cards::VIRTUE_OF_LOYALTY, false, true),
        (cards::FOREST, true, false),
        (cards::SERRA_ANGEL, true, false),
    ] {
        let mut game = ready_game();
        put_ready(&mut game, cards::DARKSTEEL_RELIC);
        game.players[0].library = game.build_zone(PlayerId::One, &[found]).unwrap();
        game.players[0].hand = game
            .build_zone(PlayerId::One, &[cards::BESEECH_THE_MIRROR])
            .unwrap();
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 4);
        let cast = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a, Action::CastSpell { choices, .. } if !choices.costs().additional().is_empty())).unwrap();
        game.apply(PlayerId::One, cast).unwrap();
        super::super::delayed_triggers::drain_to_decision(&mut game);
        if !offered {
            drain_pending(&mut game);
            assert_eq!(game.players[0].hand[0].definition, found);
            assert!(game.players[0].exile.is_empty());
            continue;
        }
        assert!(
            !game.pending_decisions.is_empty(),
            "missing offer for {found:?}"
        );
        let casts: Vec<_> = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter(|a| matches!(a, Action::CastSpell { .. }))
            .collect();
        assert!(!casts.is_empty());
        for action in &casts {
            let Action::CastSpell { choices, .. } = action else {
                unreachable!()
            };
            let option = game
                .catalog
                .get(found)
                .unwrap()
                .play_option(choices.play_option())
                .unwrap();
            assert_eq!(
                option.form == crate::card::SpellForm::Part(crate::CardPartId::PRIMARY),
                primary_allowed
            );
        }
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut restored =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 2)
                .unwrap();
        assert_eq!(
            game.legal_actions(PlayerId::One),
            restored.legal_actions(PlayerId::One)
        );
        restored.apply(PlayerId::One, casts[0].clone()).unwrap();
        assert!(
            restored.players[0]
                .hand
                .iter()
                .all(|c| c.definition != found)
        );
        assert!(restored.stack.iter().any(|s| s.card.definition == found));
    }
}

#[test]
fn jegantha_companion_compares_exact_symbols_on_each_starting_card() {
    let catalog = poc::catalog().unwrap();
    for (definition, allowed) in [
        (cards::LIGHTNING_BOLT, true),
        (cards::FIREBALL, true),
        (cards::SERRA_ANGEL, false),
        (cards::JEGANTHA_THE_WELLSPRING, true),
        (cards::RAPH_MIKEY_TROUBLEMAKERS, false),
    ] {
        let deck = crate::Deck {
            main: vec![definition],
            sideboard: vec![cards::JEGANTHA_THE_WELLSPRING],
            commanders: vec![],
        };
        assert_eq!(
            deck.validate_companion(
                &catalog,
                cards::JEGANTHA_THE_WELLSPRING,
                crate::Format::VintageCube
            )
            .is_ok(),
            allowed
        );
    }
}
