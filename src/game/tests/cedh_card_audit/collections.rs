use super::*;

#[test]
fn amphibian_downpour_storm_copy_enters_as_an_aura_token_with_its_new_target() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.record_spell_cast(PlayerId::One, GameObjectId(125_000));
        game.battlefield
            .push(creature(125_001, cards::GRIZZLY_BEARS, PlayerId::Two));
        game.battlefield
            .push(creature(125_002, cards::SERRA_ANGEL, PlayerId::Two));
        let aura = card(125_003, cards::AMPHIBIAN_DOWNPOUR_51, PlayerId::One);
        game.players[0].hand.push(aura.clone());
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 3);
        game.apply(
            PlayerId::One,
            cast_action(
                aura.id,
                vec![Target::Permanent(GameObjectId(125_001))],
                vec![],
                0,
            ),
        )
        .unwrap();
        resolve_to_decision(&mut game);
        let decision = game.pending_decisions[0].observation.clone();
        let index = match &game.pending_decisions[0].continuation {
            DecisionContinuation::CopyStackObject { target_lists, .. } => target_lists
                .iter()
                .position(|targets| {
                    flatten_target_selections(targets) == [Target::Permanent(GameObjectId(125_002))]
                })
                .unwrap(),
            other => panic!("expected a copy retarget decision, got {other:?}"),
        };
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![decision.options[index].id],
            },
        )
        .unwrap();
        drain_pending(&mut game);
        for id in [125_001, 125_002] {
            let p = permanent(&game, GameObjectId(id));
            assert_eq!(game.power(p), Some(1));
            assert!(game.object_subtypes(p.card.id).contains(&"Frog"));
            assert!(!game.permanent_has_executable_keyword(p, KeywordAbility::Flying));
        }
        assert!(
            game.battlefield
                .iter()
                .any(|p| p.card.definition.is_token()
                    && p.attached_to == Some(GameObjectId(125_002)))
        );
    }
}

#[test]
fn bonehoard_dracosaur_produces_once_per_card_type_and_grants_play_permissions() {
    for prepared in [false, true] {
        for mixed in [false, true] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            game.battlefield.push(creature(
                125_010,
                cards::BONEHOARD_DRACOSAUR_134,
                PlayerId::One,
            ));
            game.players[0].library = vec![
                card(125_011, cards::FOREST, PlayerId::One),
                card(
                    125_012,
                    if mixed {
                        cards::GRIZZLY_BEARS
                    } else {
                        cards::PLAINS
                    },
                    PlayerId::One,
                ),
            ];
            game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerId::One,
            });
            game.finish_rules_procedure();
            drain_pending(&mut game);
            assert_eq!(game.players[0].exile.len(), 2);
            assert_eq!(
                game.battlefield
                    .iter()
                    .filter(|p| p.card.definition.is_token()
                        && game.object_subtypes(p.card.id).contains(&"Dinosaur"))
                    .count(),
                1
            );
            assert_eq!(
                game.battlefield
                    .iter()
                    .filter(|p| game.object_subtypes(p.card.id).contains(&"Treasure"))
                    .count(),
                usize::from(mixed)
            );
            assert!(
                game.legal_actions(PlayerId::One)
                    .iter()
                    .any(|a| matches!(a, Action::PlayLand { .. }))
            );
        }
    }
}

#[test]
fn obstinate_gargoyle_distinguishes_controlled_auras_from_any_equipment() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let id = GameObjectId(125_020);
        game.battlefield
            .push(creature(id.0, cards::OBSTINATE_GARGOYLE_195, PlayerId::One));
        let mut aura = creature(125_021, cards::GIANT_STRENGTH, PlayerId::Two);
        aura.attached_to = Some(id);
        game.battlefield.push(aura);
        assert!(
            !game.permanent_has_executable_keyword(permanent(&game, id), KeywordAbility::Flying)
        );
        game.battlefield[1].controller = PlayerId::One;
        assert!(
            game.permanent_has_executable_keyword(permanent(&game, id), KeywordAbility::Flying)
        );
        game.battlefield.pop();
        let mut equipment = creature(125_022, cards::BONESPLITTER, PlayerId::Two);
        equipment.attached_to = Some(id);
        game.battlefield.push(equipment);
        assert!(
            game.permanent_has_executable_keyword(permanent(&game, id), KeywordAbility::Flying)
        );
        game.battlefield.pop();
        game.battlefield[0].add_counters(CounterKind::named("charge"), 1);
        assert!(
            game.permanent_has_executable_keyword(permanent(&game, id), KeywordAbility::Flying)
        );
    }
}

#[test]
fn realmbreaker_borrows_a_milled_land_tapped_and_exiles_every_later_departure() {
    for prepared in [false, true] {
        for destination in [
            ZoneKind::Hand,
            ZoneKind::Library,
            ZoneKind::Graveyard,
            ZoneKind::Command,
        ] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            game.battlefield.push(creature(
                125_030,
                cards::REALMBREAKER_THE_INVASION_TREE_374,
                PlayerId::One,
            ));
            game.players[1].library = vec![card(125_031, cards::FOREST, PlayerId::Two)];
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
            let action = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == GameObjectId(125_030))).unwrap();
            game.apply(PlayerId::One, action).unwrap();
            resolve_to_decision(&mut game);
            if !game.pending_decisions.is_empty() {
                answer_named(&mut game, "Forest");
            }
            drain_pending(&mut game);
            let land = game
                .battlefield
                .iter()
                .find(|p| p.card.definition == cards::FOREST)
                .unwrap();
            let id = land.card.id;
            assert_eq!(land.controller, PlayerId::One);
            assert_eq!(land.card.owner, PlayerId::Two);
            assert!(land.tapped);
            game.move_permanents_to_zone(&[id], destination, ZonePlacement::Top);
            drain_pending(&mut game);
            assert!(game.battlefield.iter().all(|p| p.card.id != id));
            assert_eq!(
                game.players[1].exile.len(),
                1,
                "departure to {destination:?}"
            );
            assert!(game.players[1].graveyard.is_empty());
            assert!(game.players[1].command.is_empty());
        }
    }
}

#[test]
fn savage_order_shuffles_after_a_failed_search() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.battlefield
            .push(creature(125_040, cards::SERRA_ANGEL, PlayerId::One));
        game.players[0].library = vec![
            card(125_041, cards::FOREST, PlayerId::One),
            card(125_042, cards::ISLAND, PlayerId::One),
        ];
        let order = card(125_043, cards::SAVAGE_ORDER_32, PlayerId::One);
        game.players[0].hand.push(order.clone());
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 4);
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|a| matches!(a, Action::CastSpell { card, .. } if *card == order.id))
            .unwrap();
        game.apply(PlayerId::One, action).unwrap();
        let mut expected_rng = game.rng.clone();
        let mut shuffled = game.players[0].library.clone();
        expected_rng.shuffle(&mut shuffled);
        drain_pending(&mut game);
        assert_eq!(game.players[0].library, shuffled);
        assert_eq!(game.rng.clone().next_u64(), expected_rng.next_u64());
        assert!(game.battlefield.is_empty());
    }
}

#[test]
fn kediss_triggers_for_an_opponent_but_has_no_other_opponent_to_damage() {
    for prepared in [false, true] {
        let mut game = commander_fixture(vec![cards::GRIZZLY_BEARS]);
        game.set_prepared_engine_enabled(prepared);
        let commander = game.players[0].command[0].id;
        game.move_target_to_zone(
            Target::Card(commander),
            ZoneKind::Battlefield,
            ZoneMoveCause::Rules,
            Some(BattlefieldArrival::under(PlayerId::One)),
            ZonePlacement::Top,
        );
        drain_pending(&mut game);
        let commander = game.battlefield[0].card.id;
        game.battlefield.push(creature(
            125_050,
            cards::KEDISS_EMBERCLAW_FAMILIAR_573,
            PlayerId::One,
        ));
        game.deal_combat_damage_to_player(commander, PlayerId::One, 1);
        game.finish_rules_procedure();
        assert!(game.stack.is_empty() && game.pending_triggers.is_empty());
        game.deal_combat_damage_to_player(commander, PlayerId::Two, 2);
        game.finish_rules_procedure();
        assert_eq!(game.stack.len() + game.pending_triggers.len(), 1);
        drain_pending(&mut game);
        assert_eq!(game.players[0].life, 39);
        assert_eq!(game.players[1].life, 38);
    }
}
