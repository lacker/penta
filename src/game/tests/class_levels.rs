//! Class bars are ordinary stack activations and live, noncopiable ability grants.
use super::*;

fn board(definition: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.players[0].library = game
        .build_zone(PlayerId::One, &[cards::ISLAND; 12])
        .unwrap();
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, 20);
    }
    let id = game
        .put_onto_battlefield(PlayerId::One, definition)
        .unwrap();
    (game, id)
}

fn activation(game: &Game, id: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == id))
}

fn settle(game: &mut Game) {
    for _ in 0..128 {
        if let Some(pending) = game.pending_decisions.first() {
            let d = pending.observation.clone();
            let options = d
                .options
                .iter()
                .take(d.minimum.max(1).min(d.maximum))
                .map(|o| o.id)
                .collect();
            game.apply(
                d.player,
                Action::ChooseDecision {
                    decision: d.id,
                    options,
                },
            )
            .unwrap();
        } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        } else {
            game.apply(game.priority, Action::PassPriority).unwrap();
        }
    }
    panic!("Class resolution did not settle");
}

fn level_up(game: &mut Game, id: GameObjectId) {
    game.apply(
        PlayerId::One,
        activation(game, id).expect("next Class bar is legal"),
    )
    .unwrap();
    settle(game);
}

#[test]
fn class_levels_require_previous_level_sorcery_timing_and_stack_resolution() {
    let (mut game, id) = board(cards::WIZARD_CLASS);
    let action = activation(&game, id).unwrap();
    assert_eq!(game.current_or_last_known_class_level(id), 1);
    game.step = Step::Upkeep;
    assert!(activation(&game, id).is_none());
    game.step = Step::PrecombatMain;
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.current_or_last_known_class_level(id), 1);
    assert!(activation(&game, id).is_none(), "the stack is not empty");
    let stack_id = game.stack.last().unwrap().id;
    game.counter_spell(stack_id);
    assert_eq!(game.current_or_last_known_class_level(id), 1);
    level_up(&mut game, id);
    assert_eq!(game.current_or_last_known_class_level(id), 2);
    assert_eq!(
        game.players[0].hand.len(),
        2,
        "newly granted level trigger fires"
    );
    level_up(&mut game, id);
    assert_eq!(game.current_or_last_known_class_level(id), 3);
    assert!(activation(&game, id).is_none());
    assert!(game.battlefield[0].counters.iter().next().is_none());
}

#[test]
fn class_levels_do_not_use_counters_or_emit_intermediate_level_events() {
    let (mut game, id) = board(cards::WIZARD_CLASS);
    game.battlefield[0].add_counters(CounterKind::named("level"), 8);
    assert_eq!(game.current_or_last_known_class_level(id), 1);
    assert!(activation(&game, id).is_some());
    game.raise_class_level(id, 3);
    assert!(
        game.pending_triggers.is_empty(),
        "level 2 was never reached"
    );
    assert_eq!(game.battlefield[0].counters(CounterKind::named("level")), 8);
    game.battlefield[0].counters.clear();
    assert_eq!(game.current_or_last_known_class_level(id), 3);
    // A copied activation can resolve later and set a lower level.
    game.raise_class_level(id, 2);
    assert_eq!(game.pending_triggers.len(), 1);
    game.raise_class_level(id, 2);
    assert_eq!(
        game.pending_triggers.len(),
        1,
        "no change means no level event"
    );
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 2);
}

#[test]
fn class_levels_survive_copy_changes_control_and_checkpoint_but_not_blink() {
    let (mut game, id) = board(cards::WIZARD_CLASS);
    level_up(&mut game, id);
    game.battlefield[0].copy_effect = Some(copied_characteristics(cards::ISLAND));
    game.battlefield[0].controller = PlayerId::Two;
    assert_eq!(game.current_or_last_known_class_level(id), 2);
    game.battlefield[0].copy_effect = Some(copied_characteristics(cards::WIZARD_CLASS));
    game.battlefield[0].controller = PlayerId::One;
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut restored =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
            .unwrap();
    assert_eq!(
        game.legal_actions(PlayerId::One),
        restored.legal_actions(PlayerId::One)
    );
    assert_eq!(restored.current_or_last_known_class_level(id), 2);
    assert_eq!(
        restored.observe(PlayerId::One).battlefield[0].class_level,
        Some(2)
    );
    restored.phase_out(id);
    let (wire, hidden) = checkpoint_fixture(&restored, PlayerId::One);
    restored = Game::from_observation_checkpoint(
        restored.catalog.clone(),
        restored.format,
        &wire,
        &hidden,
        0,
    )
    .unwrap();
    restored.phase_in_for(PlayerId::One);
    assert_eq!(restored.current_or_last_known_class_level(id), 2);
    let mut copier = creature(90_000, cards::COPY_ARTIFACT, PlayerId::One);
    copier.copy_effect = restored.copiable_values_of(Target::Permanent(id));
    restored.battlefield.push(copier);
    assert_eq!(
        restored.current_or_last_known_class_level(GameObjectId(90_000)),
        1
    );
    let exiled = restored.exile_permanent_returning_card(id).unwrap();
    let returned = restored
        .return_exiled_card(exiled, ZoneKind::Battlefield, None, None, false, None)
        .unwrap();
    assert_ne!(id, returned);
    assert_eq!(restored.current_or_last_known_class_level(returned), 1);
    restored.raise_class_level(id, 3);
    assert_eq!(
        restored.current_or_last_known_class_level(returned),
        1,
        "a retired source cannot level the new object"
    );
}

#[test]
fn class_level_grants_vanish_with_abilities_and_return_at_the_retained_level() {
    let (mut game, id) = board(cards::WIZARD_CLASS);
    game.raise_class_level(id, 3);
    assert!(
        game.effective_abilities(&game.battlefield[0])
            .iter()
            .any(|a| a.ability.text.starts_with("Whenever you draw"))
    );
    attach_constant_resolved_characteristics(
        &mut game,
        id,
        &[AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any)],
        ContinuousEffectExpiration::Never,
    );
    assert!(game.effective_abilities(&game.battlefield[0]).is_empty());
    assert_eq!(game.current_or_last_known_class_level(id), 3);
    game.battlefield[0].resolved_continuous_effects.clear();
    assert!(
        game.effective_abilities(&game.battlefield[0])
            .iter()
            .any(|a| a.ability.text.starts_with("Whenever you draw"))
    );
}

#[test]
fn class_level_trigger_checkpoint_and_prepared_reference_paths_agree() {
    let (mut game, id) = board(cards::WIZARD_CLASS);
    game.apply(PlayerId::One, activation(&game, id).unwrap())
        .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.current_or_last_known_class_level(id), 2);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut restored =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
            .unwrap();
    let mut reference = restored.clone();
    reference.set_prepared_engine_enabled(false);
    settle(&mut game);
    settle(&mut restored);
    settle(&mut reference);
    // Hidden-zone reconstruction supplies identities anew; compare the drawn
    // definitions, while the cloned prepared/reference states agree exactly.
    let hand_definitions = |game: &Game| {
        game.players[0]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>()
    };
    assert_eq!(hand_definitions(&game), hand_definitions(&restored));
    assert_eq!(restored.players[0].hand.len(), 2);
    assert_eq!(restored.current_or_last_known_class_level(id), 2);
    assert_eq!(restored.players, reference.players);
    assert_eq!(restored.battlefield, reference.battlefield);
    assert_eq!(
        restored.legal_actions(PlayerId::One),
        reference.legal_actions(PlayerId::One)
    );
}

#[test]
fn stormchaser_level_trigger_targets_after_level_resolution_and_survives_departure() {
    let (mut game, id) = board(cards::STORMCHASERS_TALENT);
    settle(&mut game);
    let graveyard = game
        .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT])
        .unwrap();
    game.players[0].graveyard.extend(graveyard);
    game.apply(PlayerId::One, activation(&game, id).unwrap())
        .unwrap();
    assert!(
        game.stack.last().unwrap().targets().is_empty(),
        "a Class bar does not target"
    );
    pass_priority_pair(&mut game);
    assert_eq!(game.current_or_last_known_class_level(id), 2);
    game.destroy_permanent(id);
    settle(&mut game);
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|c| c.definition == cards::LIGHTNING_BOLT),
        "the granted trigger remains independent of its departed source"
    );
}

#[test]
fn bandit_class_discard_choice_is_opponents_and_upkeep_condition_is_rechecked() {
    for nonland in [false, true] {
        let (mut game, id) = board(cards::BANDIT_S_TALENT);
        game.players[1].hand = game
            .build_zone(
                PlayerId::Two,
                &[
                    if nonland {
                        cards::LIGHTNING_BOLT
                    } else {
                        cards::ISLAND
                    },
                    cards::ISLAND,
                    cards::ISLAND,
                ],
            )
            .unwrap();
        // Select payment when available, and the first legal card choices.
        for _ in 0..64 {
            if let Some(pending) = game.pending_decisions.first() {
                let d = pending.observation.clone();
                assert_eq!(d.player, PlayerId::Two);
                let options = if matches!(pending.continuation, DecisionContinuation::PayOr { .. })
                {
                    vec![d.options.last().unwrap().id]
                } else {
                    d.options.iter().take(d.minimum).map(|o| o.id).collect()
                };
                game.apply(
                    d.player,
                    Action::ChooseDecision {
                        decision: d.id,
                        options,
                    },
                )
                .unwrap();
            } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
                break;
            } else {
                game.apply(game.priority, Action::PassPriority).unwrap();
            }
        }
        assert_eq!(game.players[1].hand.len(), if nonland { 2 } else { 1 });
        game.raise_class_level(id, 2);
        game.players[1].hand.clear();
        game.active_player = PlayerId::Two;
        game.priority = PlayerId::Two;
        game.step = Step::Upkeep;
        game.begin_step_triggers();
        game.players[1].hand = game.build_zone(PlayerId::Two, &[cards::ISLAND; 2]).unwrap();
        settle(&mut game);
        assert_eq!(
            game.players[1].life, 20,
            "intervening-if condition rechecks on resolution"
        );
    }
}

#[test]
fn scavenger_level_three_can_return_a_creature_just_sacrificed_with_finality() {
    let (mut game, id) = board(cards::SCAVENGER_S_TALENT);
    for _ in 0..3 {
        game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
            .unwrap();
    }
    game.raise_class_level(id, 3);
    game.step = Step::End;
    game.begin_step_triggers();
    for _ in 0..128 {
        if let Some(pending) = game.pending_decisions.first() {
            let d = pending.observation.clone();
            let options = if matches!(pending.continuation, DecisionContinuation::PayOr { .. }) {
                vec![d.options.last().unwrap().id]
            } else {
                d.options
                    .iter()
                    .take(d.minimum.max(1).min(d.maximum))
                    .map(|o| o.id)
                    .collect()
            };
            game.apply(
                d.player,
                Action::ChooseDecision {
                    decision: d.id,
                    options,
                },
            )
            .unwrap();
        } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        } else {
            game.apply(game.priority, Action::PassPriority).unwrap();
        }
    }
    let returned = game
        .battlefield
        .iter()
        .filter(|p| p.card.definition == cards::SAVANNAH_LIONS)
        .collect::<Vec<_>>();
    assert_eq!(
        returned.len(),
        1,
        "the reanimation choice happens after all three sacrifices"
    );
    assert_eq!(returned[0].counters(CounterKind::Finality), 1);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.token_characteristics.is_some())
            .count(),
        1,
        "one simultaneous death batch produces one Food"
    );
    assert!(
        game.battlefield.iter().any(|p| p.card.id == id),
        "the Class cannot pay its own other-permanents cost"
    );
}

#[test]
fn cleric_class_reads_returned_creatures_battlefield_toughness_before_life_trigger() {
    let (mut game, id) = board(cards::CLERIC_CLASS);
    game.put_onto_battlefield(PlayerId::One, cards::GLORIOUS_ANTHEM)
        .unwrap();
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::SAVANNAH_LIONS])
        .unwrap();
    level_up(&mut game, id);
    level_up(&mut game, id);
    assert_eq!(
        game.players[0].life, 23,
        "two toughness on the battlefield plus Cleric's replacement"
    );
    let lions = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::SAVANNAH_LIONS)
        .unwrap();
    assert_eq!(
        lions.counters(CounterKind::PlusOnePlusOne),
        1,
        "the level-2 ability remains granted at level 3"
    );
}

mod caretakers_talent;
