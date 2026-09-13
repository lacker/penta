use super::*;

fn token(game: &mut Game, controller: PlayerId) -> GameObjectId {
    game.create_token_from(
        controller,
        TokenCharacteristics::creature(&["Soldier"], &[ManaColor::White], 1, 1),
        None,
    )
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game.battlefield.iter().find(|p| p.card.id == id).unwrap();
    (game.power(permanent), game.toughness(permanent))
}

#[test]
fn caretakers_talent_draws_on_entry_once_each_turn_and_per_object() {
    let (mut game, talent) = board(cards::CARETAKER_S_TALENT);
    game.put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .unwrap();
    token(&mut game, PlayerId::Two);
    assert!(game.pending_triggers.is_empty());
    // This entry helper publishes zone-change events without TokensCreated.
    token(&mut game, PlayerId::One);
    token(&mut game, PlayerId::One);
    assert_eq!(game.pending_triggers.len(), 1);
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
    let second = game
        .put_onto_battlefield(PlayerId::One, cards::CARETAKER_S_TALENT)
        .unwrap();
    game.create_token(PlayerId::One, tokens::treasure());
    assert_eq!(game.pending_triggers.len(), 1);
    assert_eq!(game.pending_triggers[0].source.object, second);
    settle(&mut game);
    game.commit_next_turn(PlayerId::Two, Vec::new());
    token(&mut game, PlayerId::One);
    assert_eq!(game.pending_triggers.len(), 2);
    assert!(
        game.pending_triggers
            .iter()
            .any(|t| t.source.object == talent)
    );
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 4);
}

#[test]
fn caretakers_talent_level_two_targets_tokens_and_copies_only_copiable_values() {
    let (mut game, talent) = board(cards::CARETAKER_S_TALENT);
    let original = token(&mut game, PlayerId::One);
    settle(&mut game);
    let permanent = game
        .battlefield
        .iter_mut()
        .find(|p| p.card.id == original)
        .unwrap();
    permanent.add_counters(CounterKind::PlusOnePlusOne, 3);
    permanent.tapped = true;
    game.apply(PlayerId::One, activation(&game, talent).unwrap())
        .unwrap();
    assert!(game.stack.last().unwrap().targets().is_empty());
    pass_priority_pair(&mut game);
    assert_eq!(game.current_or_last_known_class_level(talent), 2);
    settle(&mut game);
    let copy = game
        .battlefield
        .iter()
        .find(|p| p.card.id != original && p.card.definition.is_token())
        .unwrap();
    assert_eq!(stats(&game, original), (Some(4), Some(4)));
    assert_eq!(stats(&game, copy.card.id), (Some(1), Some(1)));
    assert!(!copy.tapped);
    assert_eq!(copy.counters(CounterKind::PlusOnePlusOne), 0);
    assert_eq!(
        game.players[0].hand.len(),
        1,
        "the draw allowance is already spent"
    );
}

#[test]
fn caretakers_talent_can_level_without_a_target_and_copy_noncreature_tokens() {
    let (mut game, talent) = board(cards::CARETAKER_S_TALENT);
    level_up(&mut game, talent);
    assert_eq!(game.current_or_last_known_class_level(talent), 2);
    assert_eq!(game.battlefield.len(), 1);
    let (mut game, talent) = board(cards::CARETAKER_S_TALENT);
    game.create_token(PlayerId::One, tokens::treasure());
    settle(&mut game);
    level_up(&mut game, talent);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        2
    );
}

#[test]
fn caretakers_talent_anthem_tracks_level_controller_and_ability_removal() {
    let (mut game, talent) = board(cards::CARETAKER_S_TALENT);
    let ours = token(&mut game, PlayerId::One);
    let theirs = token(&mut game, PlayerId::Two);
    let nontoken = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .unwrap();
    settle(&mut game);
    assert_eq!(stats(&game, ours), (Some(1), Some(1)));
    game.raise_class_level(talent, 3);
    assert!(
        game.pending_triggers.is_empty(),
        "skipping level 2 does not copy a token"
    );
    assert_eq!(stats(&game, ours), (Some(3), Some(3)));
    assert_eq!(stats(&game, theirs), (Some(1), Some(1)));
    assert_eq!(stats(&game, nontoken), (Some(2), Some(1)));
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == talent)
        .unwrap()
        .controller = PlayerId::Two;
    assert_eq!(stats(&game, ours), (Some(1), Some(1)));
    assert_eq!(stats(&game, theirs), (Some(3), Some(3)));
    attach_constant_resolved_characteristics(
        &mut game,
        talent,
        &[AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any)],
        ContinuousEffectExpiration::Never,
    );
    assert_eq!(stats(&game, theirs), (Some(1), Some(1)));
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == talent)
        .unwrap()
        .resolved_continuous_effects
        .clear();
    assert_eq!(stats(&game, theirs), (Some(3), Some(3)));
    game.destroy_permanent(talent);
    assert_eq!(stats(&game, theirs), (Some(1), Some(1)));
}

#[test]
fn caretakers_talent_checkpoint_and_prepared_reference_anthem_agree() {
    let (mut game, talent) = board(cards::CARETAKER_S_TALENT);
    let soldier = token(&mut game, PlayerId::One);
    settle(&mut game);
    level_up(&mut game, talent);
    level_up(&mut game, talent);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let restored =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
            .unwrap();
    let mut reference = restored.clone();
    reference.set_prepared_engine_enabled(false);
    for state in [&game, &restored, &reference] {
        assert_eq!(stats(state, soldier), (Some(3), Some(3)));
        assert_eq!(state.current_or_last_known_class_level(talent), 3);
    }
    assert_eq!(
        restored.observe(PlayerId::One),
        reference.observe(PlayerId::One)
    );
    assert_eq!(
        restored.legal_actions(PlayerId::One),
        reference.legal_actions(PlayerId::One)
    );
}

#[test]
fn caretakers_talent_token_batch_draws_once_even_with_additional_triggers() {
    let (mut game, _) = board(cards::CARETAKER_S_TALENT);
    game.put_onto_battlefield(PlayerId::One, cards::ELESH_NORN_MOTHER_OF_MACHINES_416)
        .unwrap();
    let spell = game
        .build_zone(PlayerId::One, &[cards::LINGERING_SOULS])
        .unwrap()
        .remove(0);
    let id = spell.id;
    game.players[0].hand.push(spell);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell { card, .. } if *card == id))
        .unwrap();
    game.apply(PlayerId::One, cast).unwrap();
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        2
    );
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn caretakers_talent_entry_suppression_does_not_spend_the_draw_allowance() {
    let (mut game, _) = board(cards::CARETAKER_S_TALENT);
    let orb = game
        .put_onto_battlefield(PlayerId::One, cards::TORPOR_ORB)
        .unwrap();
    token(&mut game, PlayerId::One);
    assert!(game.pending_triggers.is_empty());
    game.destroy_permanent(orb);
    token(&mut game, PlayerId::One);
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn granted_static_power_toughness_uses_the_grant_timestamp() {
    let mut game = ready_game();
    let soldier = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .unwrap();
    attach_constant_resolved_characteristics(
        &mut game,
        soldier,
        &[AppliedEffectDef::set_base_power_toughness(
            ValueDef::Constant(4),
            ValueDef::Constant(4),
        )],
        ContinuousEffectExpiration::Never,
    );
    attach_constant_resolved_characteristics(
        &mut game,
        soldier,
        &[AppliedEffectDef::add_ability(
            &const {
                AbilityDef::static_ability(
                    "This creature has base power and toughness 2/2.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                    },
                )
            },
        )],
        ContinuousEffectExpiration::Never,
    );
    assert_eq!(stats(&game, soldier), (Some(2), Some(2)));
    game.set_prepared_engine_enabled(false);
    assert_eq!(stats(&game, soldier), (Some(2), Some(2)));
}
