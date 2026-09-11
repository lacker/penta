use super::*;

fn resolve_return(game: &mut Game) {
    game.finish_rules_procedure();
    assert_eq!(game.stack.len(), 1, "the return is a respondable trigger");
    game.apply(game.priority, Action::PassPriority).unwrap();
    game.apply(game.priority, Action::PassPriority).unwrap();
}

#[test]
fn undying_binds_the_dead_card_in_a_simultaneous_batch() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_001, cards::STRANGLEROOT_GEIST, PlayerId::One),
        creature(10_002, cards::DOMRI_RADE, PlayerId::One),
    ]);
    game.move_permanents_to_graveyard(&[GameObjectId(10_001), GameObjectId(10_002)]);
    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[0].graveyard.len(), 2);
    resolve_return(&mut game);
    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(
        game.battlefield[0].card.definition,
        cards::STRANGLEROOT_GEIST
    );
    assert_eq!(game.battlefield[0].counters(CounterKind::PlusOnePlusOne), 1);
    assert_eq!(game.players[0].graveyard[0].definition, cards::DOMRI_RADE);
}

#[test]
fn undying_and_persist_cannot_follow_a_second_zone_change() {
    for keyword in [KeywordAbility::Undying, KeywordAbility::Persist] {
        let mut game = ready_game();
        let mut body = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
        body.temporary_keywords.push(keyword);
        game.battlefield.push(body);
        game.destroy_permanent(GameObjectId(10_001));
        game.finish_rules_procedure();
        assert_eq!(game.stack.len(), 1);
        let dead = game.players[0].graveyard.pop().unwrap();
        let (exiled, _) = game.zone_change_card(dead);
        let (returned, _) = game.zone_change_card(exiled);
        game.players[0].graveyard.push(returned);
        game.apply(game.priority, Action::PassPriority).unwrap();
        game.apply(game.priority, Action::PassPriority).unwrap();
        assert!(game.battlefield.is_empty());
        assert_eq!(game.players[0].graveyard.len(), 1);
    }
}

#[test]
fn undying_and_persist_each_trigger_and_the_first_return_wins() {
    let mut game = ready_game();
    let mut body = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    body.temporary_keywords
        .extend([KeywordAbility::Undying, KeywordAbility::Persist]);
    game.battlefield.push(body);
    game.destroy_permanent(GameObjectId(10_001));
    game.finish_rules_procedure();
    let order = game
        .observe(PlayerId::One)
        .decision
        .expect("two triggers may be ordered");
    assert_eq!(order.options.len(), 2);
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: order.id,
            options: order.options.iter().map(|option| option.id).collect(),
        },
    )
    .unwrap();
    assert_eq!(game.stack.len(), 2);
    pass_until_decision(&mut game);
    assert_eq!(game.battlefield.len(), 1);
    let body = &game.battlefield[0];
    assert_eq!(
        body.counters(CounterKind::PlusOnePlusOne) + body.counters(CounterKind::MinusOneMinusOne),
        1
    );
}

#[test]
fn first_turn_skips_draw_step_for_either_starting_seat() {
    for seat in [PlayerId::One, PlayerId::Two] {
        let mut game = ready_game();
        game.turn = 1;
        game.starting_player = seat;
        game.active_player = seat;
        game.priority = seat;
        game.step = Step::Upkeep;
        let hand = game.players[seat.index()].hand.len();
        game.apply(seat, Action::PassPriority).unwrap();
        game.apply(seat.opponent(), Action::PassPriority).unwrap();
        assert_eq!(game.step, Step::PrecombatMain);
        assert_eq!(game.players[seat.index()].hand.len(), hand);
        game.turn = 2;
        game.step = Step::Upkeep;
        game.apply(seat, Action::PassPriority).unwrap();
        game.apply(seat.opponent(), Action::PassPriority).unwrap();
        assert_eq!(game.step, Step::Draw);
        assert_eq!(game.players[seat.index()].hand.len(), hand + 1);
    }
}

#[test]
fn empty_combat_skips_blockers_and_damage_but_removed_attackers_do_not() {
    for declared in [false, true] {
        let mut game = ready_game();
        game.step = Step::DeclareAttackers;
        if declared {
            let mut attacker = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
            attacker.attacking = true;
            attacker.attack_defender = Some(crate::AttackDefender::Player(PlayerId::Two));
            game.battlefield.push(attacker);
        }
        game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
            .unwrap();
        if declared {
            game.destroy_permanent(GameObjectId(10_001));
        }
        game.apply(PlayerId::One, Action::PassPriority).unwrap();
        game.apply(PlayerId::Two, Action::PassPriority).unwrap();
        assert_eq!(
            game.step,
            if declared {
                Step::DeclareBlockers
            } else {
                Step::EndOfCombat
            }
        );
        if declared {
            game.apply(PlayerId::Two, Action::FinishDeclaringBlockers)
                .unwrap();
            game.apply(game.priority, Action::PassPriority).unwrap();
            game.apply(game.priority, Action::PassPriority).unwrap();
            assert_eq!(game.step, Step::CombatDamage);
        }
    }
}

#[test]
fn public_reveals_survive_later_actions_and_checkpoint_reconstruction() {
    let mut game = ready_game();
    let revealed = game.players[0].library.last().unwrap().clone();
    game.reveal_effect_collection(&[Target::Card(revealed.id)]);
    game.draw_instruction(PlayerId::One, 1);
    game.finish_rules_procedure();
    let expected = vec![(PlayerId::One, revealed.id, revealed.definition)];
    for viewer in [PlayerId::One, PlayerId::Two] {
        assert_eq!(game.observe(viewer).public_reveals, expected);
        let (wire, hidden) = checkpoint_fixture(&game, viewer);
        assert_eq!(
            wire["publicReveals"][0]["definition"],
            serde_json::json!(revealed.definition)
        );
        let rebuilt = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            32,
        )
        .unwrap();
        assert_eq!(rebuilt.observe(viewer).public_reveals, expected);
        assert_ne!(
            game.players[0].hand.last().unwrap().id,
            revealed.id,
            "history does not track hidden moves"
        );
    }
}

#[test]
fn undying_trigger_and_combat_history_survive_checkpoint_reconstruction() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_001, cards::STRANGLEROOT_GEIST, PlayerId::One));
    game.step = Step::DeclareAttackers;
    game.battlefield[0].attacking = true;
    game.battlefield[0].attack_defender = Some(crate::AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    game.destroy_permanent(GameObjectId(10_001));
    game.finish_rules_procedure();
    for viewer in [PlayerId::One, PlayerId::Two] {
        let (wire, hidden) = checkpoint_fixture(&game, viewer);
        let mut rebuilt = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            33,
        )
        .unwrap();
        assert!(rebuilt.combat_had_attackers);
        pass_until_decision(&mut rebuilt);
        assert_eq!(rebuilt.battlefield.len(), 1);
        assert_eq!(
            rebuilt.battlefield[0].counters(CounterKind::PlusOnePlusOne),
            1
        );
    }
}

#[test]
fn entering_attacker_preserves_combat_steps_even_after_it_leaves() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.finish_declaring_attackers();
    let body = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: body,
        from: ZoneKind::Hand,
        completion: EntryCompletion::Attacking {
            defender: crate::AttackDefender::Player(PlayerId::Two),
        },
        redirected_to: None,
    });
    assert!(game.combat_had_attackers);
    let entered = game.battlefield[0].card.id;
    game.destroy_permanent(entered);
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    assert_eq!(game.step, Step::DeclareBlockers);
}
