use super::*;

fn staged() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let spell = card(190_000, cards::GORYO_S_VENGEANCE, PlayerId::One);
    let body = card(190_001, cards::ROFELLOS_LLANOWAR_EMISSARY, PlayerId::One);
    let ids = (spell.id, body.id);
    game.players[0].hand.push(spell);
    game.players[0].graveyard.push(body);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    (game, ids.0, ids.1)
}

fn cast(game: &mut Game, spell: GameObjectId, body: GameObjectId) {
    game.apply(
        PlayerId::One,
        cast_action(spell, vec![Target::Card(body)], Vec::new(), 0),
    )
    .unwrap();
    drain_pending(game);
}

fn next_end_step(game: &mut Game) {
    for _ in 0..16 {
        game.advance_step();
        game.finish_rules_procedure();
        if game.step == Step::End {
            return;
        }
        drain_pending(game);
    }
    panic!("the next end step was not reached");
}

#[test]
fn targets_only_legendary_creatures_in_your_graveyard() {
    let (mut game, spell, body) = staged();
    game.players[0].graveyard.extend([
        card(190_002, cards::GRIZZLY_BEARS, PlayerId::One),
        card(190_003, cards::MOX_OPAL, PlayerId::One),
    ]);
    game.players[1]
        .graveyard
        .push(card(190_004, cards::RAGAVAN_NIMBLE_PILFERER, PlayerId::Two));
    let targets: Vec<_> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == spell => {
                Some(choices.iter_targets().copied().collect::<Vec<_>>())
            }
            _ => None,
        })
        .flatten()
        .collect();
    assert_eq!(targets, [Target::Card(body)]);
}

#[test]
fn an_illegal_target_prevents_reanimation_and_the_delayed_trigger() {
    let (mut game, spell, body) = staged();
    game.apply(
        PlayerId::One,
        cast_action(spell, vec![Target::Card(body)], Vec::new(), 0),
    )
    .unwrap();
    game.move_card_from_nonbattlefield_zone(
        body,
        ZoneKind::Graveyard,
        ZoneKind::Hand,
        ZoneMoveCause::Effect {
            controller: PlayerId::Two,
        },
        None,
    );
    drain_pending(&mut game);
    assert!(game.battlefield.is_empty());
    assert!(game.installed_triggers.is_empty());
}

#[test]
fn exile_survives_checkpoint_ability_loss_and_control_change() {
    let (mut game, spell, body) = staged();
    cast(&mut game, spell, body);
    let returned = &game.battlefield[0];
    assert_ne!(returned.card.id, body);
    assert!(game.permanent_has_executable_keyword(returned, KeywordAbility::Haste));
    let (wire, hidden) = checkpoint_fixture(&game, game.priority);
    let mut game = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &hidden,
        190_100,
    )
    .unwrap();
    game.battlefield[0].controller = PlayerId::Two;
    game.battlefield
        .push(creature(190_005, cards::HUMILITY, PlayerId::Two));
    assert!(!game.permanent_has_executable_keyword(&game.battlefield[0], KeywordAbility::Haste));
    next_end_step(&mut game);
    drain_pending(&mut game);
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::ROFELLOS_LLANOWAR_EMISSARY)
    );
    assert!(game.installed_triggers.is_empty());
}

#[test]
fn a_countered_exile_does_not_repeat_and_haste_has_no_end_of_turn_duration() {
    let (mut game, spell, body) = staged();
    cast(&mut game, spell, body);
    next_end_step(&mut game);
    assert_eq!(game.stack.len(), 1);
    let trigger = game.stack[0].id;
    game.counter_spell(trigger);
    next_end_step(&mut game);
    drain_pending(&mut game);
    assert!(game.installed_triggers.is_empty());
    assert_eq!(game.battlefield.len(), 1);
    assert!(game.permanent_has_executable_keyword(&game.battlefield[0], KeywordAbility::Haste));
}

#[test]
fn exile_does_not_follow_a_creature_that_dies_and_returns() {
    for return_again in [false, true] {
        let (mut game, spell, body) = staged();
        cast(&mut game, spell, body);
        let returned = game.battlefield[0].card.id;
        game.move_permanents_to_graveyard(&[returned]);
        if return_again {
            let body = game.players[0].graveyard.pop().unwrap();
            game.put_card_onto_battlefield_from(
                body,
                ZoneKind::Graveyard,
                BattlefieldArrival::under(PlayerId::One),
                None,
            );
        }
        next_end_step(&mut game);
        drain_pending(&mut game);
        assert!(game.players[0].exile.is_empty());
        assert_eq!(game.battlefield.len(), usize::from(return_again));
        if return_again {
            assert!(
                !game.permanent_has_executable_keyword(&game.battlefield[0], KeywordAbility::Haste)
            );
        } else {
            assert!(
                game.players[0]
                    .graveyard
                    .iter()
                    .any(|card| card.definition == cards::ROFELLOS_LLANOWAR_EMISSARY)
            );
        }
    }
}

#[test]
fn casting_during_the_end_step_waits_for_the_following_end_step() {
    let (mut game, spell, body) = staged();
    game.step = Step::End;
    cast(&mut game, spell, body);
    assert_eq!(game.battlefield.len(), 1);
    next_end_step(&mut game);
    assert_eq!(game.active_player, PlayerId::Two);
    drain_pending(&mut game);
    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[0].exile.len(), 1);
}

#[test]
fn splice_pays_its_cost_and_keeps_distinct_targets_and_delayed_bindings() {
    let (mut game, spell, body) = staged();
    let spliced = card(190_002, cards::GORYO_S_VENGEANCE, PlayerId::One);
    let splice_id = spliced.id;
    let second = card(190_003, cards::RAGAVAN_NIMBLE_PILFERER, PlayerId::One);
    let second_id = second.id;
    game.players[0].hand.push(spliced);
    game.players[0].graveyard.push(second);
    let find_splice = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell && choices.spliced() == [splice_id]
                    && choices.iter_targets().copied().collect::<Vec<_>>()
                        == [Target::Card(body), Target::Card(second_id)])
            })
    };
    assert!(find_splice(&game).is_none());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let action = find_splice(&game).expect("{1}{B} plus {2}{B} pays for the spell and splice");
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);
    assert_eq!(game.battlefield.len(), 2);
    assert!(
        game.battlefield.iter().all(
            |permanent| game.permanent_has_executable_keyword(permanent, KeywordAbility::Haste)
        )
    );
    assert!(game.players[0].hand.iter().any(|card| card.id == splice_id));
    assert_eq!(game.installed_triggers.len(), 2);
    next_end_step(&mut game);
    drain_pending(&mut game);
    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[0].exile.len(), 2);
}

#[test]
fn a_redirected_arrival_is_not_exiled_again_at_the_end_step() {
    let (mut game, spell, body) = staged();
    game.battlefield
        .push(creature(190_005, cards::CONTAINMENT_PRIEST, PlayerId::Two));
    cast(&mut game, spell, body);
    assert_eq!(game.players[0].exile.len(), 1);
    let exiled = game.players[0].exile[0].id;
    next_end_step(&mut game);
    drain_pending(&mut game);
    assert_eq!(
        game.players[0].exile[0].id, exiled,
        "a redirected arrival keeps its exile object identity"
    );
}
