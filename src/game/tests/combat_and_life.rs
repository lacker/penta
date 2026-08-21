use super::*;

#[test]
fn a_first_striker_kills_a_smaller_blocker_before_it_can_answer() {
    let mut game = ready_game();
    // Black Knight is a 2/2 first striker; Savannah Lions is a 2/1, so
    // without an earlier damage step both would die together.
    let mut attacker = creature(10_000, cards::BLACK_KNIGHT, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    let mut blocker = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    blocker.blocking = vec![attacker_id];
    let blocker_id = blocker.card.id;
    game.battlefield = vec![attacker, blocker];

    game.step = Step::DeclareBlockers;
    game.advance_step();
    assert!(game.regular_combat_damage_pending());
    pass_priority_pair(&mut game);

    let survivor = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker_id)
        .expect("the first striker survives");
    assert_eq!(survivor.damage, 0, "the blocker never got to swing back");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == blocker_id),
        "the blocker died in the first-strike step",
    );
}

#[test]
fn boros_charm_double_strike_hits_an_unblocked_player_twice() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield = vec![attacker];
    let charm = card(10_001, cards::BOROS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.white = 1;
    let life_before = game.players[1].life;

    game.apply(
        PlayerId::One,
        cast_mode(charm.id, ModeId(2), vec![Target::Permanent(attacker_id)]),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    game.step = Step::DeclareBlockers;
    game.advance_step();
    assert_eq!(
        game.players[1].life,
        life_before - 2,
        "double strike deals once before the inter-wave priority window",
    );
    assert!(game.regular_combat_damage_pending());

    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[1].life,
        life_before - 4,
        "a 2/1 double striker deals two damage twice"
    );
}

#[test]
fn archangel_of_thune_grows_the_team_on_its_own_lifelink_damage() {
    let mut game = ready_game();
    let mut angel = creature(10_000, cards::ARCHANGEL_OF_THUNE, PlayerId::One);
    angel.attacking = true;
    game.battlefield = vec![angel];
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SERRA_ANGEL, PlayerId::Two));

    game.deal_combat_damage();
    for _ in 0..8 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let counters = |id: u32| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == CardInstanceId(id))
            .map(|permanent| permanent.counters[CounterKind::PlusOnePlusOne.index()])
    };
    assert_eq!(counters(10_000), Some(1), "the Angel counts itself");
    assert_eq!(counters(10_001), Some(1));
    assert_eq!(counters(10_002), Some(0), "not the opponent's creature");
    // Lifelink gained 3, and the trigger is one counter per gain rather than
    // one per point of life.
    assert_eq!(game.players[0].life, 23);
}

#[test]
fn rhox_faithmender_doubles_your_life_gain_but_not_your_opponent_s() {
    for (gainer, expected) in [(PlayerId::One, 8), (PlayerId::Two, 4)] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_000, cards::RHOX_FAITHMENDER, PlayerId::One));
        let before = game.players[gainer.index()].life;

        game.gain_life(gainer, 4);

        assert_eq!(
            game.players[gainer.index()].life - before,
            expected,
            "life gained by {gainer}",
        );
    }
}

#[test]
fn two_faithmenders_multiply_together_rather_than_adding() {
    let mut game = ready_game();
    for id in [10_000, 10_001] {
        game.battlefield
            .push(creature(id, cards::RHOX_FAITHMENDER, PlayerId::One));
    }
    let before = game.players[0].life;

    game.gain_life(PlayerId::One, 3);

    assert_eq!(game.players[0].life - before, 12);
}

#[test]
fn think_twice_can_be_flashed_back_once_and_then_is_gone() {
    let mut game = ready_game();
    game.players[0]
        .graveyard
        .push(card(10_000, cards::THINK_TWICE, PlayerId::One));
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;
    let hand_before = game.players[0].hand.len();

    let flashback = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == CardInstanceId(10_000)))
        .expect("a card in the graveyard offers its flashback option");
    game.apply(PlayerId::One, flashback).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].hand.len(), hand_before + 1, "it drew");
    assert!(
        game.players[0].graveyard.is_empty(),
        "a flashback spell does not return to the graveyard"
    );
    assert_eq!(game.players[0].exile.len(), 1);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { .. })),
        "an exiled card cannot be flashed back again",
    );
}

#[test]
fn a_card_in_hand_is_not_offered_its_flashback_cost() {
    let mut game = ready_game();
    let think_twice = card(10_000, cards::THINK_TWICE, PlayerId::One);
    game.players[0].hand.push(think_twice);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;

    let options = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { choices, .. } => Some(choices.play_option()),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(options, vec![PlayOptionId::DEFAULT]);
}

#[test]
fn flinthoof_boar_grows_for_a_mountain_you_control_and_only_once() {
    let mut game = ready_game();
    let boar = creature(10_000, cards::FLINTHOOF_BOAR, PlayerId::One);
    let boar_id = boar.card.id;
    game.battlefield.push(boar);
    // The opponent's Mountain is not one you control.
    game.battlefield
        .push(creature(10_001, cards::MOUNTAIN, PlayerId::Two));

    let stats = |game: &Game| {
        let boar = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == boar_id)
            .expect("still there");
        (game.power(boar), game.toughness(boar))
    };
    assert_eq!(stats(&game), (Some(2), Some(2)), "printed 2/2");

    game.battlefield
        .push(creature(10_002, cards::MOUNTAIN, PlayerId::One));
    assert_eq!(stats(&game), (Some(3), Some(3)));

    game.battlefield[2].text_changes.push(BasicLandTypeChange {
        from: BasicLandType::Mountain,
        to: BasicLandType::Island,
    });
    assert_eq!(
        stats(&game),
        (Some(2), Some(2)),
        "the condition reads the land's effective subtype",
    );

    game.battlefield
        .push(creature(10_003, cards::MOUNTAIN, PlayerId::One));
    assert_eq!(stats(&game), (Some(3), Some(3)));

    // "As long as you control a Mountain" is a condition, so a second one
    // adds nothing.
    game.battlefield
        .push(creature(10_004, cards::MOUNTAIN, PlayerId::One));
    assert_eq!(stats(&game), (Some(3), Some(3)));
}

#[test]
fn a_wall_may_block_but_never_attacks_and_never_stops_a_juggernaut() {
    let mut game = ready_game();
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    let juggernaut = creature(10_000, cards::JUGGERNAUT, PlayerId::One);
    let juggernaut_id = juggernaut.card.id;
    game.battlefield.push(juggernaut);
    let wall = creature(10_001, cards::WALL_OF_STONE, PlayerId::Two);
    let wall_id = wall.card.id;
    game.battlefield.push(wall);
    let lions = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two);
    let lions_id = lions.card.id;
    game.battlefield.push(lions);

    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(
            |action| matches!(action, Action::DeclareAttacker { attacker, defender: AttackDefender::Player(PlayerId::Two) } if *attacker == wall_id)
        ),
        "defender keeps the Wall home",
    );

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: juggernaut_id,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .unwrap();
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .unwrap();
    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;
    let blocks = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker { blocker, .. } => Some(blocker),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        blocks,
        vec![lions_id],
        "the Wall cannot block a Juggernaut, but the Lions can"
    );
}

#[test]
fn boros_reckoner_returns_the_damage_it_took_to_a_target_of_its_choice() {
    let mut game = ready_game();
    let reckoner = creature(10_000, cards::BOROS_RECKONER, PlayerId::One);
    game.battlefield.push(reckoner);
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[1].hand.push(bolt.clone());
    game.players[1].mana_pool.red = 1;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::PrecombatMain;
    let life_before = game.players[1].life;

    game.apply(
        PlayerId::Two,
        cast_action(
            bolt.id,
            vec![Target::Permanent(CardInstanceId(10_000))],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    for _ in 0..12 {
        if game.players[1].life <= life_before - 3 {
            break;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            // Aim the trigger at the player who threw the Bolt rather than
            // taking whichever option happens to come first.
            let options = decision
                .options
                .iter()
                .find(|option| option.label == "your opponent")
                .map_or_else(
                    || {
                        decision
                            .options
                            .iter()
                            .take(decision.minimum.max(1))
                            .map(|option| option.id)
                            .collect::<Vec<_>>()
                    },
                    |option| vec![option.id],
                );
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

    // Three damage in, three damage back out at the player who threw it.
    assert_eq!(game.players[1].life, life_before - 3);
}

#[test]
fn burning_earth_burns_only_the_nonbasic_taps() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::BURNING_EARTH)
        .expect("cataloged");
    let foundry = game
        .put_onto_battlefield(PlayerId::Two, cards::SACRED_FOUNDRY)
        .expect("cataloged");
    let entry = game
        .observe(PlayerId::Two)
        .decision
        .expect("Sacred Foundry applies its entry replacement during setup");
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: entry.id,
            options: vec![1],
        },
    )
    .unwrap();
    let mountain = game
        .put_onto_battlefield(PlayerId::Two, cards::MOUNTAIN)
        .expect("cataloged");
    let life_before = game.players[1].life;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::PrecombatMain;

    let tap_for_red = |game: &Game, source: GameObjectId| {
        game.legal_actions(PlayerId::Two)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateManaAbility { source: id, color, .. }
                    if *id == source && *color == ManaColor::Red)
            })
            .expect("the land taps for red")
    };

    let action = tap_for_red(&game, mountain);
    game.apply(PlayerId::Two, action).unwrap();
    assert_eq!(
        game.players[1].life, life_before,
        "a basic Mountain is not a nonbasic land"
    );

    let action = tap_for_red(&game, foundry);
    game.apply(PlayerId::Two, action).unwrap();
    for _ in 0..8 {
        if game.players[1].life < life_before {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    assert_eq!(game.players[1].life, life_before - 1);
}

#[test]
fn celestial_flare_only_takes_a_creature_that_is_in_combat() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    attacker.attacking = true;
    game.battlefield.push(attacker);
    // Sitting at home, so the Flare cannot reach it.
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two));
    let flare = card(10_002, cards::CELESTIAL_FLARE, PlayerId::One);
    game.players[0].hand.push(flare.clone());
    game.players[0].mana_pool.white = 2;

    game.apply(
        PlayerId::One,
        cast_action(flare.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    // One candidate means no decision: the Angel simply goes.
    assert!(game.pending_decisions.is_empty());
    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>(),
        vec![CardInstanceId(10_001)],
        "the attacker was sacrificed and the untapped Lions stayed"
    );
}

#[test]
fn celestial_flare_lets_the_targeted_player_pick_which_attacker_dies() {
    let mut game = ready_game();
    for id in [10_000, 10_001] {
        let mut attacker = creature(id, cards::SAVANNAH_LIONS, PlayerId::Two);
        attacker.attacking = true;
        game.battlefield.push(attacker);
    }
    let flare = card(10_002, cards::CELESTIAL_FLARE, PlayerId::One);
    game.players[0].hand.push(flare.clone());
    game.players[0].mana_pool.white = 2;

    game.apply(
        PlayerId::One,
        cast_action(flare.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::Two)
        .decision
        .expect("the sacrifice is the targeted player's choice");
    assert_eq!(decision.player, PlayerId::Two);
    let keep = decision
        .options
        .iter()
        .find(|option| {
            option.card
                == Some((
                    CardInstanceId(10_001),
                    ObjectCharacteristics::card(cards::SAVANNAH_LIONS, CardPartId::PRIMARY),
                ))
        })
        .expect("both attackers are offered");
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![keep.id],
        },
    )
    .unwrap();

    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>(),
        vec![CardInstanceId(10_000)],
        "the one they chose is the one that died"
    );
}

#[test]
fn thundermaw_hellkite_only_shocks_the_fliers_across_the_table() {
    let mut game = ready_game();
    // A flier they control, a ground creature they control, and a flier of
    // your own: only the first is named.
    game.battlefield
        .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::Two));
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two));
    game.battlefield
        .push(creature(10_002, cards::SERRA_ANGEL, PlayerId::One));

    game.put_onto_battlefield(PlayerId::One, cards::THUNDERMAW_HELLKITE)
        .expect("cataloged");
    for _ in 0..8 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let state = |id: u32| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == CardInstanceId(id))
            .map(|permanent| (permanent.damage, permanent.tapped))
    };
    assert_eq!(state(10_000), Some((1, true)), "their flier");
    assert_eq!(state(10_001), Some((0, false)), "their ground creature");
    assert_eq!(state(10_002), Some((0, false)), "your own flier");
}

#[test]
fn azorius_charm_puts_an_attacker_back_on_top_of_its_library() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    attacker.attacking = true;
    game.battlefield.push(attacker);
    let charm = card(10_001, cards::AZORIUS_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.blue = 1;
    let library_before = game.players[1].library.len();

    game.apply(
        PlayerId::One,
        cast_mode(
            charm.id,
            ModeId(2),
            vec![Target::Permanent(CardInstanceId(10_000))],
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty(), "the attacker left combat");
    assert_eq!(game.players[1].library.len(), library_before + 1);
    assert_eq!(
        game.players[1].library.last().map(|card| card.definition),
        Some(cards::SERRA_ANGEL),
        "on top, not shuffled in",
    );
}

#[test]
fn an_order_can_buy_first_strike_and_win_a_trade_it_would_have_lost() {
    let mut game = ready_game();
    let mut order = creature(10_000, cards::ORDER_OF_THE_EBON_HAND, PlayerId::One);
    order.attacking = true;
    let order_id = order.card.id;
    game.battlefield.push(order);
    // Another 2/1: without first strike the two would kill each other.
    let mut blocker = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    blocker.blocking = vec![order_id];
    game.battlefield.push(blocker);
    game.players[0].mana_pool.black = 1;
    let first_strike = activated_ability_for(&game, order_id, 0);

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    ability,
                    ..
                } if *source == order_id && *ability == first_strike
            )
        })
        .expect("the first-strike ability is activatable");
    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    game.step = Step::DeclareBlockers;
    game.advance_step();
    assert_eq!(game.step, Step::CombatDamage);
    assert!(
        game.regular_combat_damage_pending(),
        "the bought first strike creates an inter-wave priority window",
    );
    pass_priority_pair(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>(),
        vec![order_id],
        "the Order struck first and took nothing back"
    );
}

#[test]
fn syncopate_exiles_the_spell_when_its_controller_will_not_pay() {
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[1].hand.push(bolt.clone());
    game.players[1].mana_pool.red = 1;
    let syncopate = card(10_001, cards::SYNCOPATE, PlayerId::One);
    game.players[0].hand.push(syncopate.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::PrecombatMain;

    game.apply(
        PlayerId::Two,
        cast_action(bolt.id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
    )
    .unwrap();
    let spell = game.stack.last().expect("the Bolt is on the stack").id;
    // Enough to pay, so the choice is real rather than a formality.
    game.players[1].mana_pool.colorless = 2;
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    game.apply(
        PlayerId::One,
        cast_action(syncopate.id, vec![Target::Spell(spell)], Vec::new(), 2),
    )
    .unwrap();
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let decision = game
        .observe(PlayerId::Two)
        .decision
        .expect("the Bolt's controller is asked, not Syncopate's");
    assert_eq!(decision.player, PlayerId::Two);
    let decline = decision
        .options
        .iter()
        .find(|option| option.label == "Decline")
        .expect("declining is always available");
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decline.id],
        },
    )
    .unwrap();

    assert_eq!(game.players[0].life, 20, "the Bolt never resolved");
    assert!(game.players[1].graveyard.is_empty(), "exiled, not buried");
    assert_eq!(
        game.players[1]
            .exile
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT]
    );
}

#[test]
fn izzet_charm_lets_a_paying_controller_keep_the_spell() {
    let mut game = ready_game();
    let ritual = card(10_000, cards::DARK_RITUAL, PlayerId::Two);
    game.players[1].hand.push(ritual.clone());
    game.players[1].mana_pool.black = 1;
    let charm = card(10_001, cards::IZZET_CHARM, PlayerId::One);
    game.players[0].hand.push(charm.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.red = 1;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::PrecombatMain;

    game.apply(
        PlayerId::Two,
        cast_action(ritual.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    let spell = game.stack.last().expect("the Ritual is on the stack").id;
    game.players[1].mana_pool.colorless = 2;
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    game.apply(
        PlayerId::One,
        cast_mode(charm.id, ModeId(0), vec![Target::Spell(spell)]),
    )
    .unwrap();
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }

    let decision = game.observe(PlayerId::Two).decision.expect("a real choice");
    let pay = decision
        .options
        .iter()
        .find(|option| option.label == "Pay the cost")
        .expect("they can afford it");
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![pay.id],
        },
    )
    .unwrap();

    assert!(
        game.stack.iter().any(|object| object.id == spell),
        "paying keeps the spell on the stack"
    );
    assert_eq!(game.players[1].mana_pool.colorless, 0, "the two was spent");
}

#[test]
fn tragic_slip_is_a_minus_one_until_something_dies() {
    for morbid in [false, true] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::Two));
        if morbid {
            game.battlefield
                .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
            game.destroy_permanent(CardInstanceId(10_001));
        }
        let slip = card(10_002, cards::TRAGIC_SLIP, PlayerId::One);
        game.players[0].hand.push(slip.clone());
        game.players[0].mana_pool.black = 1;

        game.apply(
            PlayerId::One,
            cast_action(
                slip.id,
                vec![Target::Permanent(CardInstanceId(10_000))],
                Vec::new(),
                0,
            ),
        )
        .unwrap();
        pass_priority_pair(&mut game);

        let angel = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == CardInstanceId(10_000));
        if morbid {
            assert!(angel.is_none(), "a 4/4 does not survive -13/-13");
        } else {
            let angel = angel.expect("a 4/4 shrugs off -1/-1");
            assert_eq!(
                (game.power(angel), game.toughness(angel)),
                (Some(3), Some(3))
            );
        }
    }
}

#[test]
fn ratchet_bomb_sweeps_the_mana_value_it_ticked_up_to() {
    let mut game = ready_game();
    let bomb = game
        .put_onto_battlefield(PlayerId::One, cards::RATCHET_BOMB)
        .expect("cataloged");
    // Savannah Lions costs one, Serra Angel five, and a land is spared
    // whatever the count.
    game.put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::MOUNTAIN)
        .expect("cataloged");

    let activate = |game: &Game, index: usize| {
        let mut actions = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::ActivateAbility {
                    source,
                    ability: AbilityOrigin::Printed { ability, .. },
                    ..
                } if source == bomb => Some((
                    ability,
                    Action::ActivateAbility {
                        source,
                        ability: AbilityOrigin::Printed {
                            definition: cards::RATCHET_BOMB,
                            part: CardPartId::PRIMARY,
                            ability,
                        },
                        targets: Vec::new(),
                        cost_objects: Vec::new(),
                        x: 0,
                        modes: Vec::new(),
                    },
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        actions.sort_by_key(|(ability, _)| *ability);
        actions.get(index).map(|(_, action)| action.clone())
    };

    // Tick to one charge counter.
    let tick = activate(&game, 0).expect("the charge ability is activatable");
    game.apply(PlayerId::One, tick).unwrap();
    pass_priority_pair(&mut game);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bomb)
        .expect("still there")
        .tapped = false;

    let detonate = activate(&game, 1).expect("the sweep ability is activatable");
    game.apply(PlayerId::One, detonate).unwrap();
    pass_priority_pair(&mut game);

    let left = game
        .battlefield
        .iter()
        .map(|permanent| permanent.card.definition)
        .collect::<Vec<_>>();
    assert_eq!(
        left,
        vec![cards::SERRA_ANGEL, cards::MOUNTAIN],
        "only the one-drop matched the single charge counter"
    );
}
