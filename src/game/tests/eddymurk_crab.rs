use super::*;

fn cast_action(game: &Game) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, .. } if *card == CardInstanceId(20_000))
        })
}

#[test]
fn flash_entry_checks_the_turn_and_taps_zero_one_or_two_creatures() {
    for prepared in [false, true] {
        for active in [PlayerId::One, PlayerId::Two] {
            for targets in 0..=2 {
                let mut game = ready_game();
                game.set_prepared_engine_enabled(prepared);
                game.active_player = active;
                game.step = Step::Upkeep;
                game.priority = PlayerId::One;
                game.battlefield.extend([
                    creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One),
                    creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two),
                ]);
                game.players[0]
                    .hand
                    .push(card(20_000, cards::EDDYMURK_CRAB, PlayerId::One));
                game.players[0].mana_pool.blue = 2;
                game.players[0].mana_pool.colorless = 5;

                let action = cast_action(&game).expect("flash permits casting in either upkeep");
                game.apply(PlayerId::One, action).unwrap();
                pass_priority_pair(&mut game);

                let crab = game
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.definition == cards::EDDYMURK_CRAB)
                    .expect("the Crab resolved");
                assert_eq!(crab.tapped, active != PlayerId::One);
                assert_eq!((game.power(crab), game.toughness(crab)), (Some(5), Some(5)));
                let pending = &game.pending_decisions[0];
                let decision = pending.observation.clone();
                assert_eq!(decision.kind, DecisionKind::TriggerPlacement);
                assert_eq!((decision.minimum, decision.maximum), (0, 2));
                let candidates = match &pending.continuation {
                    DecisionContinuation::TriggerPlacement { candidates, .. } => candidates,
                    other => panic!("expected trigger placement, found {other:?}"),
                };
                assert!(
                    candidates.contains(&Target::Permanent(crab.card.id)),
                    "the Crab can target itself"
                );
                let options = (0..targets)
                    .map(|index| {
                        let position = candidates
                            .iter()
                            .position(|candidate| {
                                *candidate == Target::Permanent(GameObjectId(10_000 + index))
                            })
                            .expect("creatures of either controller are legal targets");
                        decision.options[position].id
                    })
                    .collect();
                game.apply(
                    PlayerId::One,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options,
                    },
                )
                .unwrap();
                assert_eq!(
                    game.stack.len(),
                    1,
                    "the tap trigger uses the ordinary stack"
                );
                for id in [GameObjectId(10_000), GameObjectId(10_001)] {
                    assert!(
                        !game
                            .battlefield
                            .iter()
                            .find(|p| p.card.id == id)
                            .unwrap()
                            .tapped
                    );
                }
                pass_priority_pair(&mut game);
                for index in 0..2 {
                    let permanent = game
                        .battlefield
                        .iter()
                        .find(|p| p.card.id == GameObjectId(10_000 + index))
                        .unwrap();
                    assert_eq!(permanent.tapped, index < targets);
                }
            }
        }
    }
}

#[test]
fn graveyard_discount_counts_only_your_instants_and_sorceries_and_preserves_blue() {
    for count in [0_u16, 2, 5, 7] {
        let mut game = ready_game();
        game.players[0]
            .hand
            .push(card(20_000, cards::EDDYMURK_CRAB, PlayerId::One));
        for index in 0..count {
            let definition = if index % 2 == 0 {
                cards::LIGHTNING_BOLT
            } else {
                cards::DIVINATION
            };
            game.players[0].graveyard.push(card(
                30_000 + u32::from(index),
                definition,
                PlayerId::One,
            ));
        }
        game.players[0]
            .graveyard
            .push(card(31_000, cards::GRIZZLY_BEARS, PlayerId::One));
        game.players[1]
            .graveyard
            .push(card(31_001, cards::LIGHTNING_BOLT, PlayerId::Two));
        let generic = 5_u16.saturating_sub(count);
        game.players[0].mana_pool.blue = 1;
        game.players[0].mana_pool.colorless = 10;
        assert!(
            cast_action(&game).is_none(),
            "the discount cannot pay the second blue symbol"
        );
        game.players[0].mana_pool.blue = 2;
        if generic > 0 {
            game.players[0].mana_pool.colorless = generic - 1;
            assert!(
                cast_action(&game).is_none(),
                "unrelated graveyard cards do not reduce the cost"
            );
        }
        game.players[0].mana_pool.colorless = generic;
        let action = cast_action(&game).expect("the exact reduced cost is payable");
        game.apply(PlayerId::One, action).unwrap();
        assert_eq!(game.players[0].mana_pool, ManaPool::default());
    }
}

#[test]
fn uncast_entry_reads_the_prospective_controller_instead_of_the_owner() {
    for active in [PlayerId::One, PlayerId::Two] {
        let mut game = ready_game();
        game.active_player = active;
        let mut crab = creature(20_000, cards::EDDYMURK_CRAB, PlayerId::One);
        crab.controller = PlayerId::Two;
        game.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent: crab,
            from: ZoneKind::Graveyard,
            completion: EntryCompletion::None,
            redirected_to: None,
        });
        game.finish_rules_procedure();
        let crab = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::EDDYMURK_CRAB)
            .expect("the Crab entered without being cast");
        assert_eq!(crab.controller, PlayerId::Two);
        assert_eq!(crab.card.owner, PlayerId::One);
        assert_eq!(crab.tapped, active != PlayerId::Two);
    }
}
