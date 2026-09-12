use super::*;

#[test]
fn etali_can_cast_both_exiled_cards_in_either_order_during_one_resolution() {
    for prepared in [false, true] {
        for reverse in [false, true] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            let etali = game
                .put_onto_battlefield(PlayerId::One, cards::ETALI_PRIMAL_STORM)
                .unwrap();
            game.players[0].library = vec![card(126_010, cards::GRIZZLY_BEARS, PlayerId::One)];
            game.players[1].library = vec![card(126_011, cards::WIND_DRAKE, PlayerId::Two)];
            game.capture_battlefield_triggers(&CommittedTriggerEvent::Attacks {
                object: game.trigger_event_object(permanent(&game, etali)),
                declaration_size: 1,
                attack_number: 1,
                defending_player: PlayerId::Two,
                attacked_a_planeswalker: false,
            });
            game.finish_rules_procedure();
            resolve_to_decision(&mut game);
            let decision = game.pending_decisions[0].observation.clone();
            let mut options: Vec<_> = decision.options.iter().map(|o| o.id).collect();
            if reverse {
                options.reverse();
            }
            let expected: Vec<_> = options
                .iter()
                .map(|id| {
                    game.players
                        .iter()
                        .flat_map(|p| &p.exile)
                        .find(|c| {
                            c.id == decision
                                .options
                                .iter()
                                .find(|o| o.id == *id)
                                .unwrap()
                                .card
                                .as_ref()
                                .unwrap()
                                .0
                        })
                        .unwrap()
                        .definition
                })
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
            for definition in &expected {
                let pending = game
                    .pending_decisions
                    .first()
                    .expect("the next cast is offered before priority returns");
                let DecisionContinuation::MayCastGranted { card: offered, .. } =
                    pending.continuation
                else {
                    panic!("expected an immediate cast offer")
                };
                let action = game
                    .legal_actions(PlayerId::One)
                    .into_iter()
                    .find(|a| matches!(a, Action::CastSpell { card, .. } if *card == offered))
                    .unwrap();
                assert_eq!(
                    game.players
                        .iter()
                        .flat_map(|p| &p.exile)
                        .find(|c| c.id == offered)
                        .unwrap()
                        .definition,
                    *definition
                );
                game.apply(PlayerId::One, action).unwrap();
            }
            assert_eq!(
                game.stack.len(),
                2,
                "both spells must wait until Etali's trigger has finished"
            );
            drain_pending(&mut game);
            for definition in expected {
                assert!(
                    game.battlefield
                        .iter()
                        .any(|p| p.card.definition == definition && p.controller == PlayerId::One)
                );
            }
        }
    }
}

#[test]
fn lively_dirge_enforces_combined_mana_value_before_moving_either_card() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.players[0].graveyard = vec![
            card(126_020, cards::HILL_GIANT, PlayerId::One),
            card(126_021, cards::LLANOWAR_ELVES, PlayerId::One),
            card(126_022, cards::ORNITHOPTER, PlayerId::One),
        ];
        let spell = card(126_023, cards::LIVELY_DIRGE, PlayerId::One);
        game.players[0].hand.push(spell.clone());
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 4);
        let action = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a, Action::CastSpell { card, choices, .. } if *card == spell.id && choices.modes().len() == 1 && choices.modes()[0].index() == 1)).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        resolve_to_decision(&mut game);
        answer_named(&mut game, "Hill Giant");
        let options = &game.pending_decisions[0].observation.options;
        assert!(
            !options.iter().any(|o| o.label.contains("Llanowar Elves")),
            "four plus one exceeds the shared limit"
        );
        assert!(
            game.battlefield.is_empty(),
            "selection must finish before either card moves"
        );
        answer_named(&mut game, "Ornithopter");
        drain_pending(&mut game);
        assert!(
            game.battlefield
                .iter()
                .any(|p| p.card.definition == cards::HILL_GIANT)
        );
        assert!(
            game.battlefield
                .iter()
                .any(|p| p.card.definition == cards::ORNITHOPTER)
        );
        assert!(
            game.players[0]
                .graveyard
                .iter()
                .any(|c| c.definition == cards::LLANOWAR_ELVES)
        );
    }
}

#[test]
fn delivery_moogle_searches_both_zones_for_one_card_and_shuffles_only_when_required() {
    for prepared in [false, true] {
        for both in [false, true] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            game.players[0].graveyard = vec![card(126_030, cards::SOL_RING, PlayerId::One)];
            game.players[0].library = vec![
                card(126_031, cards::LOTUS_PETAL, PlayerId::One),
                card(126_032, cards::FOREST, PlayerId::One),
                card(126_033, cards::ISLAND, PlayerId::One),
            ];
            let mut rng = game.rng.clone();
            let mut library = game.players[0].library.clone();
            if both {
                rng.shuffle(&mut library);
            }
            game.put_onto_battlefield(PlayerId::One, cards::DELIVERY_MOOGLE)
                .unwrap();
            game.finish_rules_procedure();
            resolve_to_decision(&mut game);
            answer_named(
                &mut game,
                if both {
                    "Search both zones"
                } else {
                    "Search your graveyard"
                },
            );
            if both {
                assert_eq!(game.pending_decisions[0].observation.maximum, 1);
                assert!(
                    game.pending_decisions[0]
                        .observation
                        .options
                        .iter()
                        .any(|o| o.label.contains("Lotus Petal"))
                );
            }
            if !game.pending_decisions.is_empty() {
                answer_named(&mut game, "Sol Ring");
            }
            drain_pending(&mut game);
            assert!(
                game.players[0]
                    .hand
                    .iter()
                    .any(|c| c.definition == cards::SOL_RING)
            );
            assert_eq!(game.players[0].library, library);
            assert_eq!(game.rng.clone().next_u64(), rng.next_u64());
        }
    }
}
