use super::*;
use crate::card::RollDieDef;

fn mana_actions(game: &Game, source: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One).into_iter().filter(|action| {
        matches!(action, Action::ActivateManaAbility { source: id, .. } if *id == source)
    }).collect()
}

fn activate_roll(game: &mut Game, source: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source: id, .. } if *id == source),
        )
        .expect("roll is a stack-using activation");
    game.apply(PlayerId::One, action).unwrap();
}

#[test]
fn mana_artifact_dice_vessel_entry_and_globe_sacrifice_preserve_distinct_pairs() {
    for definition in [cards::FIREMIND_VESSEL, cards::GUILD_GLOBE] {
        let mut game = ready_game();
        let source = game
            .put_onto_battlefield(PlayerId::One, definition)
            .unwrap();
        if definition == cards::FIREMIND_VESSEL {
            assert!(
                game.battlefield
                    .iter()
                    .find(|p| p.card.id == source)
                    .unwrap()
                    .tapped
            );
            assert!(mana_actions(&game, source).is_empty());
            game.battlefield
                .iter_mut()
                .find(|p| p.card.id == source)
                .unwrap()
                .tapped = false;
        } else {
            assert!(game.players[0].hand.is_empty());
            game.finish_rules_procedure();
            assert_eq!(game.stack.len(), 1);
            pass_priority_pair(&mut game);
            assert_eq!(game.players[0].hand.len(), 1);
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
            assert!(
                mana_actions(&game, source).is_empty(),
                "Globe cannot pay for itself with its output"
            );
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        }
        let actions = mana_actions(&game, source);
        assert_eq!(actions.len(), 10);
        for action in actions {
            let mut branch = game.clone();
            branch.apply(PlayerId::One, action).unwrap();
            assert!(branch.stack.is_empty());
            assert_eq!(branch.players[0].mana_pool.total(), 2);
            let mana = &branch.players[0].mana;
            assert_eq!(mana.len(), 2);
            assert_ne!(mana[0].color, mana[1].color);
            assert!(
                mana.iter()
                    .all(|unit| unit.color != ManaColor::Colorless && unit.restrictions.is_empty())
            );
            if definition == cards::GUILD_GLOBE {
                assert!(!branch.battlefield.iter().any(|p| p.card.id == source));
                assert!(
                    branch.players[0]
                        .graveyard
                        .iter()
                        .any(|card| card.definition == definition)
                );
            } else {
                assert!(
                    branch
                        .battlefield
                        .iter()
                        .find(|p| p.card.id == source)
                        .unwrap()
                        .tapped
                );
            }
        }
    }
}

#[test]
fn mana_artifact_dice_pouch_rolls_on_resolution_and_covers_every_face() {
    let mut base = ready_game();
    let pouch = base
        .put_onto_battlefield(PlayerId::One, cards::COMPONENT_POUCH)
        .unwrap();
    let mut seen = [false; 20];
    for seed in 0..200 {
        let mut game = base.clone();
        game.rng = crate::rng::ReplayRng::new(seed);
        let mut expected_rng = game.rng.clone();
        let expected = u16::try_from(expected_rng.index_below(20) + 1).unwrap();
        assert!(mana_actions(&game, pouch).is_empty());
        // Repeated enumeration must not consume randomness or put counters on the source.
        assert!(mana_actions(&game, pouch).is_empty());
        let mut reference = game.clone();
        reference.set_prepared_engine_enabled(false);
        activate_roll(&mut reference, pouch);
        activate_roll(&mut game, pouch);
        assert_eq!(game.stack.len(), 1);
        assert_eq!(
            game.battlefield[0]
                .counters
                .count(CounterKind::named("component")),
            0
        );
        assert!(
            !game
                .events
                .iter()
                .any(|event| matches!(event, GameEvent::DieRolled { .. }))
        );
        pass_priority_pair(&mut game);
        pass_priority_pair(&mut reference);
        assert_eq!(game.events, reference.events);
        assert_eq!(game.battlefield, reference.battlefield);
        assert!(game.stack.is_empty());
        let results = game
            .events
            .iter()
            .filter_map(|event| match event {
                GameEvent::DieRolled {
                    player: PlayerId::One,
                    sides: 20,
                    result,
                } => Some(*result),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(results, [expected]);
        seen[usize::from(expected - 1)] = true;
        assert_eq!(game.rng.next_u64(), expected_rng.next_u64());
        assert_eq!(
            game.battlefield[0]
                .counters
                .count(CounterKind::named("component")),
            if expected <= 9 { 1 } else { 2 }
        );
        let event = game
            .events
            .iter()
            .find(|event| matches!(event, GameEvent::DieRolled { .. }))
            .unwrap();
        let wire = crate::protocol::event_json(&game.catalog, event).unwrap();
        assert_eq!(wire["type"], "DieRolled");
        assert_eq!(wire["result"], expected);
        assert_eq!(wire["sides"], 20);
        if seen.iter().all(|seen| *seen) {
            break;
        }
    }
    assert!(seen.into_iter().all(|seen| seen));
}

#[test]
fn mana_artifact_dice_pouch_counter_cost_and_checkpoint_round_trip() {
    let mut game = ready_game();
    let pouch = game
        .put_onto_battlefield(PlayerId::One, cards::COMPONENT_POUCH)
        .unwrap();
    activate_roll(&mut game, pouch);
    // A pending roll is reconstructed through its catalog ability locator.
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 42)
            .unwrap();
    pass_priority_pair(&mut rebuilt);
    let counters = rebuilt.battlefield[0]
        .counters
        .count(CounterKind::named("component"));
    assert!((1..=2).contains(&counters));
    assert!(
        mana_actions(&rebuilt, pouch).is_empty(),
        "the roll tapped the source"
    );
    rebuilt.battlefield[0].tapped = false;
    rebuilt.battlefield[0].set_counters(CounterKind::named("component"), 1);
    let (wire, hidden) = checkpoint_fixture(&rebuilt, PlayerId::One);
    let game = Game::from_observation_checkpoint(
        rebuilt.catalog.clone(),
        rebuilt.format,
        &wire,
        &hidden,
        42,
    )
    .unwrap();
    let actions = mana_actions(&game, pouch);
    assert_eq!(actions.len(), 10);
    for action in actions {
        let mut branch = game.clone();
        branch.apply(PlayerId::One, action).unwrap();
        assert!(branch.stack.is_empty());
        assert_eq!(branch.players[0].mana_pool.total(), 2);
        assert_ne!(
            branch.players[0].mana[0].color,
            branch.players[0].mana[1].color
        );
        assert_eq!(
            branch.battlefield[0]
                .counters
                .count(CounterKind::named("component")),
            0
        );
        branch.battlefield[0].tapped = false;
        assert!(mana_actions(&branch, pouch).is_empty());
    }
}

#[test]
fn mana_artifact_dice_roll_still_happens_after_source_leaves() {
    let mut game = ready_game();
    let pouch = game
        .put_onto_battlefield(PlayerId::One, cards::COMPONENT_POUCH)
        .unwrap();
    activate_roll(&mut game, pouch);
    game.battlefield.clear();
    let replacement = game
        .put_onto_battlefield(PlayerId::One, cards::COMPONENT_POUCH)
        .unwrap();
    assert_ne!(pouch, replacement);
    pass_priority_pair(&mut game);
    assert!(
        game.events
            .iter()
            .any(|event| matches!(event, GameEvent::DieRolled { sides: 20, .. }))
    );
    assert_eq!(
        game.battlefield[0]
            .counters
            .count(CounterKind::named("component")),
        0
    );
}

#[test]
fn mana_artifact_dice_tables_reject_incomplete_or_ambiguous_ranges() {
    for outcomes in [
        &[][..],
        &[(9, EffectDef::None)][..],
        &[
            (10, EffectDef::None),
            (9, EffectDef::None),
            (20, EffectDef::None),
        ][..],
        &[(21, EffectDef::None)][..],
    ] {
        assert!(std::panic::catch_unwind(|| RollDieDef::new(20, outcomes)).is_err());
    }
    assert!(std::panic::catch_unwind(|| RollDieDef::new(0, &[])).is_err());
}
