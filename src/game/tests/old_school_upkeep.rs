use super::*;

#[test]
fn artifact_entry_replacements_apply_during_spell_resolution() {
    for (definition, mana) in [(cards::TIME_VAULT, 2), (cards::NEVINYRRALS_DISK, 4)] {
        let mut game = ready_game();
        let artifact = card(10_000, definition, PlayerId::One);
        let hand_id = artifact.id;
        game.players[0].hand.push(artifact);
        game.players[0].mana_pool.colorless = mana;

        game.apply(
            PlayerId::One,
            cast_action(hand_id, Vec::new(), Vec::new(), 0),
        )
        .unwrap();
        assert!(
            game.battlefield
                .iter()
                .all(|permanent| permanent.card.definition != definition),
            "a spell is not yet a prospective battlefield entry"
        );
        pass_priority_pair(&mut game);

        let entered = game
            .battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == definition)
            .collect::<Vec<_>>();
        assert_eq!(entered.len(), 1);
        assert!(entered[0].tapped);
        assert_ne!(entered[0].card.id, hand_id);
        assert!(game.pending_decisions.is_empty());
        assert!(game.stack.is_empty());
    }
}

#[test]
fn blind_obedience_competes_with_a_permanents_own_entry_replacement() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_001, cards::BLIND_OBEDIENCE, PlayerId::Two));
    let vault = card(10_000, cards::TIME_VAULT, PlayerId::One);
    game.players[0].hand.push(vault.clone());
    game.players[0].mana_pool.colorless = 2;

    game.apply(
        PlayerId::One,
        cast_action(vault.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let order = game
        .observe(PlayerId::One)
        .decision
        .expect("the entering permanent's controller orders both replacements");
    assert_eq!(order.kind, DecisionKind::Choice);
    assert_eq!(order.options.len(), 2);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::TIME_VAULT)
    );
    let blind_obedience = order
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(_, definition)| definition == cards::BLIND_OBEDIENCE)
        })
        .expect("Blind Obedience supplies one applicable replacement")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: order.id,
            options: vec![blind_obedience],
        },
    )
    .unwrap();

    let entered = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::TIME_VAULT)
        .expect("re-evaluation applies Time Vault's remaining replacement and commits");
    assert!(entered.tapped);
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn time_vault_currently_untaps_by_banking_a_skip_for_a_later_turn() {
    // This pins what the engine does, which is not what the card says. The
    // replacement is worded against the turn that is beginning: skipping it
    // is the cost of untapping. Here the offer arrives during the untap step,
    // so that turn is already under way and is played out in full, and the
    // skip is spent on the controller's next turn instead. Fixing it means
    // moving the choice ahead of untap and ending the turn on acceptance,
    // which is turn-flow work rather than a card-local change.
    let mut game = ready_game();
    let mut vault = creature(10_000, cards::TIME_VAULT, PlayerId::Two);
    vault.tapped = true;
    game.battlefield.push(vault);

    game.start_next_turn();
    let decision = game.observe(PlayerId::Two).decision.unwrap();
    let untap = Action::ChooseDecision {
        decision: decision.id,
        options: vec![1],
    };
    game.apply(PlayerId::Two, untap).unwrap();
    assert!(!game.battlefield[0].tapped);

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
}

#[test]
fn sylvan_library_triggers_onto_the_stack_and_may_be_declined() {
    let mut game = ready_game();
    game.turn = 2;
    game.step = Step::Upkeep;
    game.battlefield
        .push(creature(10_000, cards::SYLVAN_LIBRARY, PlayerId::One));
    game.players[0].library = vec![
        card(10_001, cards::PLAINS, PlayerId::One),
        card(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
        card(10_003, cards::SWORDS_TO_PLOWSHARES, PlayerId::One),
    ];

    game.advance_step();
    assert_eq!(
        game.players[0].hand.len(),
        1,
        "the draw step draws one; the extras wait on the ability"
    );
    assert_eq!(game.pending_triggers.len(), 1, "the ability triggered");

    pass_priority_pair(&mut game);
    assert_eq!(game.stack.len(), 1, "and it went on the stack");
    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "so the opponent had a window before any of it happened"
    );

    pass_until_decision(&mut game);
    let offer = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(offer.prompt, "Draw two additional cards?");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
            options: vec![0],
        },
    )
    .unwrap();

    assert_eq!(game.players[0].hand.len(), 1, "declining draws nothing");
    assert_eq!(game.players[0].life, 20, "and costs nothing");
    assert!(game.observe(PlayerId::One).decision.is_none());
}

#[test]
fn sylvan_library_may_draw_from_an_empty_library_before_state_based_actions() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.queue_sylvan_offer(PlayerId::One);

    let offer = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(offer.prompt, "Draw two additional cards?");
    game.choose_decision(PlayerId::One, offer.id, &[1]);

    assert_eq!(
        game.result, None,
        "choosing the draw only records the failed attempts during resolution"
    );
    game.finish_rules_procedure();
    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            reason: WinReason::OpponentTriedToDrawFromEmptyLibrary,
        })
    );
}

#[test]
fn sylvan_library_pays_life_or_puts_each_chosen_card_back() {
    let mut game = ready_game();
    game.turn = 2;
    game.step = Step::Upkeep;
    game.battlefield
        .push(creature(10_000, cards::SYLVAN_LIBRARY, PlayerId::One));
    game.players[0].library = vec![
        card(10_001, cards::PLAINS, PlayerId::One),
        card(10_002, cards::SAVANNAH_LIONS, PlayerId::One),
        card(10_003, cards::SWORDS_TO_PLOWSHARES, PlayerId::One),
    ];

    game.advance_step();
    pass_priority_pair(&mut game);
    pass_until_decision(&mut game);
    let offer = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
            options: vec![1],
        },
    )
    .unwrap();
    assert_eq!(game.players[0].hand.len(), 3, "one drawn plus two more");

    for mode in [1, 0] {
        let selection = game.observe(PlayerId::One).decision.unwrap();
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: selection.id,
                options: vec![selection.options[0].id],
            },
        )
        .unwrap();
        let decision = game.observe(PlayerId::One).decision.unwrap();
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![mode],
            },
        )
        .unwrap();
    }

    assert_eq!(game.players[0].life, 16, "four life for the one kept");
    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(
        game.players[0].library.len(),
        1,
        "the other went back on top"
    );
}

#[test]
fn juggernaut_must_attack_if_able() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    let juggernaut = creature(10_000, cards::JUGGERNAUT, PlayerId::One);
    let juggernaut_id = juggernaut.card.id;
    game.battlefield.push(juggernaut);

    let actions = game.legal_actions(PlayerId::One);
    assert!(!actions.contains(&Action::FinishDeclaringAttackers));
    assert!(actions.contains(&Action::DeclareAttacker {
        attacker: juggernaut_id,
        defender: AttackDefender::Player(PlayerId::Two),
    }));

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: juggernaut_id,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .unwrap();
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::FinishDeclaringAttackers)
    );
}

#[test]
fn triskelion_enters_with_counters_and_spends_one_to_deal_damage() {
    let mut game = ready_game();
    let triskelion = card(10_000, cards::TRISKELION, PlayerId::One);
    let triskelion_id = triskelion.id;
    game.players[0].hand.push(triskelion);
    game.players[0].mana_pool.colorless = 6;

    game.apply(
        PlayerId::One,
        cast_action(triskelion_id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::TRISKELION)
        .unwrap();
    let permanent_id = permanent.card.id;
    assert_eq!(game.power(permanent), Some(4));
    assert_eq!(game.toughness(permanent), Some(4));

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: permanent_id,
            ability: activated_ability_for(&game, permanent_id, 0),
            targets: activated_targets(Target::Player(PlayerId::Two)),
            cost_object: None,
            x: 0,
        },
    )
    .unwrap();
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == permanent_id)
        .unwrap();
    assert_eq!(game.power(permanent), Some(3));
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].life, 19);
}

#[test]
fn triskelion_cannot_activate_without_a_plus_one_counter() {
    let mut game = ready_game();
    let triskelion = creature(10_000, cards::TRISKELION, PlayerId::One);
    let source = triskelion.card.id;
    game.battlefield.push(triskelion);

    assert!(!game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: candidate, .. } if *candidate == source)
    ));

    game.battlefield[0].counters[CounterKind::PlusOnePlusOne.index()] = 1;
    assert!(game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: candidate, .. } if *candidate == source)
    ));
}

#[test]
fn tundras_pay_counterspells_double_blue_cost() {
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    let counterspell = card(10_001, cards::COUNTERSPELL, PlayerId::One);
    game.players[1].hand.push(bolt.clone());
    game.players[1].mana_pool.red = 1;
    game.players[0].hand.push(counterspell.clone());
    game.battlefield
        .push(creature(10_002, cards::TUNDRA, PlayerId::One));
    game.battlefield
        .push(creature(10_003, cards::TUNDRA, PlayerId::One));
    game.priority = PlayerId::Two;

    game.apply(
        PlayerId::Two,
        cast_action(bolt.id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
    )
    .unwrap();
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    let bolt_on_stack = game.stack[0].id;
    game.apply(
        PlayerId::One,
        cast_action(
            counterspell.id,
            vec![Target::Spell(bolt_on_stack)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.stack.is_empty());
    assert_eq!(game.players[0].life, 20);
    assert_eq!(game.players[0].graveyard[0].definition, cards::COUNTERSPELL);
    assert_eq!(
        game.players[1].graveyard[0].definition,
        cards::LIGHTNING_BOLT
    );
}

#[test]
fn counterspell_removes_an_older_spell_without_disturbing_an_intervening_spell() {
    let mut game = ready_game();
    let older_bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    let intervening_bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::Two);
    let counterspell = card(10_002, cards::COUNTERSPELL, PlayerId::One);
    game.players[1]
        .hand
        .extend([older_bolt.clone(), intervening_bolt.clone()]);
    game.players[1].mana_pool.red = 2;
    game.players[0].hand.push(counterspell.clone());
    game.players[0].mana_pool.blue = 2;
    game.priority = PlayerId::Two;

    game.apply(
        PlayerId::Two,
        cast_action(
            older_bolt.id,
            vec![Target::Player(PlayerId::One)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    let older_stack_id = game.stack[0].id;
    game.apply(
        PlayerId::Two,
        cast_action(
            intervening_bolt.id,
            vec![Target::Player(PlayerId::One)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    let intervening_stack_id = game.stack[1].id;
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    game.apply(
        PlayerId::One,
        cast_action(
            counterspell.id,
            vec![Target::Spell(older_stack_id)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();

    pass_priority_pair(&mut game);

    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].id, intervening_stack_id);
    assert_eq!(game.players[0].life, 20);
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the targeted older spell was countered",
    );

    pass_priority_pair(&mut game);
    assert!(game.stack.is_empty());
    assert_eq!(game.players[0].life, 17);
}

#[test]
fn swords_exiles_a_creature_and_grants_life_equal_to_power() {
    let mut game = ready_game();
    let boar = creature(10_000, cards::FLINTHOOF_BOAR, PlayerId::Two);
    let boar_id = boar.card.id;
    game.battlefield
        .extend([boar, creature(10_001, cards::MOUNTAIN, PlayerId::Two)]);
    let boar = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == boar_id)
        .expect("Flinthoof Boar is on the battlefield");
    assert_eq!(game.power(boar), Some(3), "the Mountain's bonus applies");

    let swords = card(10_002, cards::SWORDS_TO_PLOWSHARES, PlayerId::One);
    game.players[0].hand.push(swords.clone());
    game.players[0].mana_pool.white = 1;
    let cast = cast_action(swords.id, vec![Target::Permanent(boar_id)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));

    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != boar_id)
    );
    assert_eq!(game.players[1].life, 23);
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == cards::FLINTHOOF_BOAR)
    );
}

#[test]
fn swords_cannot_target_order_of_the_ebon_hand() {
    let mut game = ready_game();
    let order = creature(10_000, cards::ORDER_OF_THE_EBON_HAND, PlayerId::Two);
    let order_id = order.card.id;
    game.battlefield.push(order);
    let swords = card(10_001, cards::SWORDS_TO_PLOWSHARES, PlayerId::One);
    game.players[0].hand.push(swords.clone());
    game.players[0].mana_pool.white = 1;

    let swords_action = cast_action(swords.id, vec![Target::Permanent(order_id)], Vec::new(), 0);
    assert!(!game.legal_actions(PlayerId::One).contains(&swords_action));
}

#[test]
fn order_of_leitbur_can_gain_first_strike() {
    let mut game = ready_game();
    let order = creature(10_000, cards::ORDER_OF_LEITBUR, PlayerId::One);
    let order_id = order.card.id;
    game.battlefield.push(order);
    game.players[0].mana_pool.white = 1;
    let activation = Action::ActivateAbility {
        source: order_id,
        ability: activated_ability_for(&game, order_id, 0),
        targets: Vec::new(),
        cost_object: None,
        x: 0,
    };

    assert!(game.legal_actions(PlayerId::One).contains(&activation));
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);

    let order = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == order_id)
        .unwrap();
    assert!(
        game.permanent_has_executable_keyword(order, KeywordAbility::FirstStrike),
        "the resolved declarative activation grants executable first strike",
    );
}

#[test]
fn protection_from_white_prevents_white_blockers() {
    let mut game = ready_game();
    let mut order = creature(10_000, cards::ORDER_OF_THE_EBON_HAND, PlayerId::One);
    order.attacking = true;
    let lion = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    game.battlefield = vec![order, lion];
    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::One;
    game.attackers_declared = true;
    game.blockers_declared = false;

    assert!(
        !game
            .legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker: CardInstanceId(10_001),
                attacker: CardInstanceId(10_000),
            })
    );
}

#[test]
fn protection_does_not_prevent_a_protected_creature_from_blocking() {
    let mut game = ready_game();
    let mut lion = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    lion.attacking = true;
    let knight = creature(10_001, cards::BLACK_KNIGHT, PlayerId::Two);
    game.battlefield = vec![lion, knight];
    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::One;
    game.attackers_declared = true;
    game.blockers_declared = false;

    assert!(
        game.legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker: CardInstanceId(10_001),
                attacker: CardInstanceId(10_000),
            })
    );
}

#[test]
fn protection_prevents_damage_from_a_source_of_the_protected_color() {
    let mut game = ready_game();
    let lion = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let knight = creature(10_001, cards::BLACK_KNIGHT, PlayerId::Two);
    let lion_id = lion.card.id;
    let knight_id = knight.card.id;
    game.battlefield = vec![lion, knight];

    game.damage_target_from(Some(lion_id), Some(Target::Permanent(knight_id)), 2);

    let knight = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == knight_id)
        .expect("protection keeps Black Knight on the battlefield");
    assert_eq!(knight.damage, 0);
}

#[test]
fn vampire_nighthawk_deathtouch_and_lifelink_are_executable_keyword_abilities() {
    let mut game = ready_game();
    game.players[0].life = 10;
    let nighthawk = creature(10_000, cards::VAMPIRE_NIGHTHAWK, PlayerId::One);
    let nighthawk_id = nighthawk.card.id;
    let angel = creature(10_001, cards::SERRA_ANGEL, PlayerId::Two);
    let angel_id = angel.card.id;
    game.battlefield = vec![nighthawk, angel];

    game.damage_target_from(Some(nighthawk_id), Some(Target::Permanent(angel_id)), 1);
    game.check_state_based_actions();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != angel_id),
        "one point from a source with deathtouch is lethal",
    );
    assert_eq!(game.players[0].life, 11);

    game.damage_target_from(Some(nighthawk_id), Some(Target::Player(PlayerId::Two)), 2);
    assert_eq!(game.players[0].life, 13);
    assert_eq!(game.players[1].life, 18);
}

#[test]
fn ancestral_recall_draws_three_and_time_walk_queues_an_extra_turn() {
    let mut game = ready_game();
    let ancestral = card(10_000, cards::ANCESTRAL_RECALL, PlayerId::One);
    game.players[0].hand.push(ancestral.clone());
    game.players[0].mana_pool.blue = 1;
    let hand_before = game.players[0].hand.len();
    game.apply(
        PlayerId::One,
        cast_action(
            ancestral.id,
            vec![Target::Player(PlayerId::One)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), hand_before - 1 + 3);

    let time_walk = card(10_001, cards::TIME_WALK, PlayerId::One);
    game.players[0].hand.push(time_walk.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        cast_action(time_walk.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
    assert_eq!(game.observe(PlayerId::One).active_turn, 2);
}

#[test]
fn serra_angel_attacks_without_tapping() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    let serra = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    let serra_id = serra.card.id;
    game.battlefield.push(serra);

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: serra_id,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .unwrap();

    let serra = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == serra_id)
        .unwrap();
    assert!(serra.attacking);
    assert!(!serra.tapped);
}

#[test]
fn hellrider_burns_once_per_attacker_including_itself() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    let hellrider = creature(10_000, cards::HELLRIDER, PlayerId::One);
    let hellrider_id = hellrider.card.id;
    game.battlefield.push(hellrider);
    let lions = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
    let lions_id = lions.card.id;
    game.battlefield.push(lions);
    // A creature the opponent controls is not "a creature you control", and
    // it is not attacking anyway.
    game.battlefield
        .push(creature(10_002, cards::SERRA_ANGEL, PlayerId::Two));
    let life_before = game.players[1].life;

    for attacker in [hellrider_id, lions_id] {
        game.apply(
            PlayerId::One,
            Action::DeclareAttacker {
                attacker,
                defender: AttackDefender::Player(PlayerId::Two),
            },
        )
        .unwrap();
    }
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .unwrap();
    // Drain the whole batch rather than stopping at the expected life total,
    // so a third trigger would show up as too much damage.
    for _ in 0..12 {
        if game.stack.is_empty() && game.pending_decisions.is_empty() {
            break;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .take(decision.minimum.max(1))
                .map(|option| option.id)
                .collect::<Vec<_>>();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    assert_eq!(
        game.players[1].life,
        life_before - 2,
        "one trigger per attacking creature"
    );
}

#[test]
fn ivory_tower_and_jayemdae_tome_provide_control_card_advantage() {
    let mut game = ready_game();
    game.players[0].life = 10;
    for id in 10_000..10_006 {
        game.players[0]
            .hand
            .push(card(id, cards::MOUNTAIN, PlayerId::One));
    }
    game.battlefield
        .push(creature(10_010, cards::IVORY_TOWER, PlayerId::One));
    let tome = creature(10_011, cards::JAYEMDAE_TOME, PlayerId::One);
    let tome_id = tome.card.id;
    game.battlefield.push(tome);
    game.players[0].mana_pool.colorless = 4;

    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    assert_eq!(game.players[0].life, 10);
    assert_eq!(game.stack.len(), 1);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].life, 12);
    let hand_before = game.players[0].hand.len();
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: tome_id,
            ability: activated_ability_for(&game, tome_id, 0),
            targets: Vec::new(),
            cost_object: None,
            x: 0,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), hand_before + 1);
}

#[test]
fn library_of_alexandria_draw_activation_keeps_its_printed_ability_id() {
    let mut game = ready_game();
    for id in 10_000..10_007 {
        game.players[0]
            .hand
            .push(card(id, cards::MOUNTAIN, PlayerId::One));
    }
    let library = creature(10_010, cards::LIBRARY_OF_ALEXANDRIA, PlayerId::One);
    let library_id = library.card.id;
    game.battlefield.push(library);

    let expected_origin = AbilityOrigin::Printed {
        definition: cards::LIBRARY_OF_ALEXANDRIA,
        part: CardPartId::PRIMARY,
        ability: AbilityId(1),
    };
    let activation = Action::ActivateAbility {
        source: library_id,
        ability: expected_origin,
        targets: Vec::new(),
        cost_object: None,
        x: 0,
    };

    assert_eq!(activated_ability_for(&game, library_id, 0), expected_origin);
    assert!(game.legal_actions(PlayerId::One).contains(&activation));
    game.apply(PlayerId::One, activation).unwrap();
    assert_eq!(game.stack[0].ability_origin(), Some(expected_origin));

    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), 8);
}
