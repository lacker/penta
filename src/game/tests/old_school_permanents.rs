use super::*;

/// Answers every pending decision until the stack is quiet, taking the named
/// number of options from any prompt that starts with `prompt` and the
/// smallest legal answer everywhere else. Tetravus puts two triggers on the
/// stack at once, so the test cannot assume which one is asked about first.
fn answer_upkeep(game: &mut Game, prompt: &str, take: usize) -> Vec<usize> {
    let mut offered = Vec::new();
    for _ in 0..16 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            break;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let wanted = if decision.prompt.starts_with(prompt) {
                offered.push(decision.options.len());
                take
            } else {
                decision.minimum
            };
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(wanted.max(decision.minimum))
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
    offered
}

#[test]
fn tetravus_trades_counters_for_tetravites_that_remember_which_one_made_them() {
    let mut game = ready_game();
    game.turn = 2;
    game.step = Step::Upkeep;
    let mut tetravus = creature(10_000, cards::TETRAVUS, PlayerId::One);
    tetravus.add_counters(CounterKind::PlusOnePlusOne, 3);
    game.battlefield.push(tetravus);

    game.handle_upkeep_triggers();
    answer_upkeep(&mut game, "Remove any number", 2);

    let tetravus = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000))
        .expect("it is still there");
    assert_eq!(
        tetravus.counters(CounterKind::PlusOnePlusOne),
        1,
        "two of the three counters were traded away"
    );
    assert_eq!(game.power(tetravus), Some(2), "and it shrank with them");

    let tetravites = game
        .battlefield
        .iter()
        .filter(|permanent| is_token_with(permanent, tokens::tetravite()))
        .collect::<Vec<_>>();
    assert_eq!(tetravites.len(), 2, "one Tetravite per counter");
    assert!(
        tetravites
            .iter()
            .all(|token| token.created_by == Some(GameObjectId(10_000))),
        "each one remembers the Tetravus that made it"
    );
    assert!(
        tetravites
            .iter()
            .all(|token| game.permanent_has_executable_keyword(token, KeywordAbility::Flying)),
        "a Tetravite flies"
    );
}

#[test]
fn an_aura_cannot_target_a_tetravite() {
    // "This token can't be enchanted" is a targeting restriction, not
    // something the Aura discovers after it has already arrived and attached.
    let mut game = ready_game();
    game.battlefield
        .push(token_permanent(10_000, tokens::tetravite(), PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
    let aura = card(10_002, cards::VOLCANIC_STRENGTH, PlayerId::One);
    game.players[0].hand.push(aura.clone());
    let bolt = card(10_003, cards::LIGHTNING_BOLT, PlayerId::One);
    game.players[0].hand.push(bolt.clone());
    game.players[0].mana_pool = ManaPool {
        red: 3,
        colorless: 3,
        ..ManaPool::default()
    };

    let targets_of = |game: &Game, spell: GameObjectId| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::CastSpell { card, choices, .. } if card == spell => {
                    choices.iter_targets().copied().next()
                }
                _ => None,
            })
            .collect::<std::collections::HashSet<_>>()
    };

    let aura_targets = targets_of(&game, aura.id);
    assert!(
        !aura_targets.contains(&Target::Permanent(GameObjectId(10_000))),
        "the Tetravite is not a legal Aura target"
    );
    assert!(
        aura_targets.contains(&Target::Permanent(GameObjectId(10_001))),
        "but an ordinary creature still is"
    );
    assert!(
        targets_of(&game, bolt.id).contains(&Target::Permanent(GameObjectId(10_000))),
        "and the restriction is about Auras, not targeting in general"
    );

    assert!(
        game.apply(
            PlayerId::One,
            cast_action(
                aura.id,
                vec![Target::Permanent(GameObjectId(10_000))],
                Vec::new(),
                0,
            ),
        )
        .is_err(),
        "submitting it directly is refused too"
    );
}

#[test]
fn tetravus_takes_back_only_the_tetravites_it_made() {
    let mut game = ready_game();
    game.turn = 2;
    game.step = Step::Upkeep;
    game.battlefield
        .push(creature(10_000, cards::TETRAVUS, PlayerId::One));

    // Two of its own, and one that belongs to a Tetravus that is not here.
    for (id, creator) in [(10_001, 10_000), (10_002, 10_000), (10_003, 10_999)] {
        let mut token = token_permanent(id, tokens::tetravite(), PlayerId::One);
        token.created_by = Some(GameObjectId(creator));
        game.battlefield.push(token);
    }

    game.handle_upkeep_triggers();
    let offered = answer_upkeep(&mut game, "Exile any number", 1);

    assert_eq!(
        offered,
        vec![2],
        "the orphaned Tetravite was never on the menu"
    );
    let tetravus = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000))
        .expect("it is still there");
    assert_eq!(
        tetravus.counters(CounterKind::PlusOnePlusOne),
        1,
        "one Tetravite came home as one counter"
    );
    assert_eq!(game.power(tetravus), Some(2));
    let remaining = game
        .battlefield
        .iter()
        .filter(|permanent| is_token_with(permanent, tokens::tetravite()))
        .map(|permanent| permanent.card.id)
        .collect::<Vec<_>>();
    assert_eq!(
        remaining,
        vec![GameObjectId(10_002), GameObjectId(10_003)],
        "only the one that was exiled left"
    );
}

#[test]
fn an_aura_cannot_stay_on_a_tetravite() {
    let mut game = ready_game();
    let token = token_permanent(10_000, tokens::tetravite(), PlayerId::One);
    let bear = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
    game.battlefield.push(token);
    game.battlefield.push(bear);

    let mut aura = creature(10_002, cards::VOLCANIC_STRENGTH, PlayerId::One);
    aura.attached_to = Some(GameObjectId(10_001));
    let bear = game.battlefield[1].clone();
    assert!(
        game.is_legal_aura_host(&aura, GameObjectId(10_001)),
        "an ordinary creature is a fine host"
    );
    assert_eq!(game.power(&bear), Some(2), "and no Aura is on it yet");
    assert!(
        !game.is_legal_aura_host(&aura, GameObjectId(10_000)),
        "a Tetravite can't be enchanted"
    );
}

#[test]
fn an_assassin_that_connects_ends_the_game_no_matter_the_life_total() {
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut assassin = token_permanent(10_000, assassin_token(), PlayerId::One);
    assassin.attacking = true;
    game.battlefield.push(assassin);
    game.players[1].life = 40;

    game.deal_combat_damage();
    drain_pending(&mut game);

    assert_eq!(
        game.players[1].life, 39,
        "the token still dealt only its one damage"
    );
    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentLostToAnEffect,
        }),
        "and the trigger ended it anyway"
    );
}

#[test]
fn a_blocked_assassin_never_triggers() {
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut assassin = token_permanent(10_000, assassin_token(), PlayerId::One);
    assassin.attacking = true;
    let mut wall = creature(10_001, cards::WALL_OF_STONE, PlayerId::Two);
    wall.blocking = vec![GameObjectId(10_000)];
    game.battlefield.extend([assassin, wall]);

    game.deal_combat_damage();
    drain_pending(&mut game);

    assert_eq!(game.players[1].life, 20, "the wall soaked it up");
    assert_eq!(
        game.result, None,
        "no combat damage reached a player, so nobody lost"
    );
}

#[test]
fn vraska_destroys_a_nonland_permanent_and_ultimates_into_three_assassins() {
    let mut game = ready_game();
    let mut vraska = creature(10_000, cards::VRASKA_THE_UNSEEN, PlayerId::One);
    vraska.set_counters(CounterKind::Loyalty, u16::try_from(7).unwrap_or(0));
    game.battlefield.push(vraska);
    game.battlefield
        .push(creature(10_001, cards::SERRA_ANGEL, PlayerId::Two));
    game.battlefield
        .push(creature(10_002, cards::PLAINS, PlayerId::Two));

    let destroy = Action::ActivateAbility {
        source: GameObjectId(10_000),
        ability: activated_ability_for(&game, GameObjectId(10_000), 1),
        targets: activated_targets(Target::Permanent(GameObjectId(10_001))),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    let at_the_land = Action::ActivateAbility {
        source: GameObjectId(10_000),
        ability: activated_ability_for(&game, GameObjectId(10_000), 1),
        targets: activated_targets(Target::Permanent(GameObjectId(10_002))),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    let actions = game.legal_actions(PlayerId::One);
    assert!(actions.contains(&destroy), "the angel is a legal target");
    assert!(
        !actions.contains(&at_the_land),
        "a land is not a nonland permanent"
    );

    game.apply(PlayerId::One, destroy).unwrap();
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(10_001)),
        "the angel was destroyed"
    );
    let vraska = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000))
        .expect("she paid three of her seven");
    assert_eq!(vraska.counters(CounterKind::Loyalty), 4);
}

#[test]
fn vraskas_ultimate_makes_three_assassins() {
    let mut game = ready_game();
    let mut vraska = creature(10_000, cards::VRASKA_THE_UNSEEN, PlayerId::One);
    vraska.set_counters(CounterKind::Loyalty, u16::try_from(7).unwrap_or(0));
    game.battlefield.push(vraska);

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: GameObjectId(10_000),
            ability: activated_ability_for(&game, GameObjectId(10_000), 2),
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(permanent, assassin_token()))
            .count(),
        3
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(10_000)),
        "paying all seven left her behind"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn jace_lets_an_opponent_split_the_top_three_and_takes_the_pile_he_likes() {
    let mut game = ready_game();
    let mut jace = creature(10_000, cards::JACE_ARCHITECT_OF_THOUGHT, PlayerId::One);
    jace.set_counters(CounterKind::Loyalty, u16::try_from(4).unwrap_or(0));
    game.battlefield.push(jace);
    game.players[0].library.clear();
    game.players[0].hand.clear();
    stack_library(
        &mut game,
        &[
            (10_001, cards::SERRA_ANGEL),
            (10_002, cards::SAVANNAH_LIONS),
            (10_003, cards::LIGHTNING_BOLT),
            (10_004, cards::PLAINS),
        ],
    );

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: GameObjectId(10_000),
            ability: activated_ability_for(&game, GameObjectId(10_000), 1),
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    for _ in 0..4 {
        if game.pending_decisions.is_empty() && game.stack.is_empty() {
            break;
        }
        if game.pending_decisions.is_empty() {
            game.apply(game.priority, Action::PassPriority).unwrap();
            continue;
        }
        break;
    }

    // The opponent separates the three revealed cards: the Angel and Lions
    // against the Bolt.
    let split = game.observe(PlayerId::Two).decision.expect("they split");
    assert_eq!(split.options.len(), 3, "only the top three were revealed");
    let angel = split
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(id, _)| id == GameObjectId(10_001))
        })
        .expect("the angel was revealed")
        .id;
    let lions = split
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(id, _)| id == GameObjectId(10_002))
        })
        .expect("the lions were revealed")
        .id;
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: split.id,
            options: vec![angel, lions],
        },
    )
    .unwrap();

    // Jace's controller takes the one-card pile.
    let choice = game.observe(PlayerId::One).decision.expect("he chooses");
    let bolt = choice
        .options
        .iter()
        .find(|option| option.label.contains("Lightning Bolt"))
        .expect("one pile holds the Bolt")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: choice.id,
            options: vec![bolt],
        },
    )
    .unwrap();

    // Changing zones makes a new object, so these are compared by what the
    // cards are rather than by identity.
    assert_eq!(
        game.players[0]
            .hand
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT],
        "the chosen pile went to hand"
    );
    let mut bottom = game.players[0].library[..2]
        .iter()
        .map(|card| card.definition)
        .collect::<Vec<_>>();
    bottom.sort_unstable();
    let mut expected = vec![cards::SERRA_ANGEL, cards::SAVANNAH_LIONS];
    expected.sort_unstable();
    assert_eq!(bottom, expected, "both losing cards went to the bottom");
    assert_eq!(
        game.players[0].library[2].definition,
        cards::PLAINS,
        "the card outside the split remains above both losing cards"
    );
    let jace = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000))
        .expect("he stayed");
    assert_eq!(jace.counters(CounterKind::Loyalty), 2);
}

#[test]
fn jaces_first_ability_taxes_attackers_until_his_controller_comes_back_around() {
    let mut game = ready_game();
    let mut jace = creature(10_000, cards::JACE_ARCHITECT_OF_THOUGHT, PlayerId::One);
    jace.set_counters(CounterKind::Loyalty, u16::try_from(4).unwrap_or(0));
    game.battlefield.push(jace);

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: GameObjectId(10_000),
            ability: activated_ability_for(&game, GameObjectId(10_000), 0),
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == GameObjectId(10_000))
            .expect("he stayed")
            .counters(CounterKind::Loyalty),
        5
    );

    // The opponent's turn: their attacker is taxed, and Jace's own creature
    // attacking on a later turn is not.
    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::Two);
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.battlefield
        .push(creature(10_001, cards::SERRA_ANGEL, PlayerId::Two));
    game.apply(
        PlayerId::Two,
        Action::DeclareAttacker {
            attacker: GameObjectId(10_001),
            defender: AttackDefender::Player(PlayerId::One),
        },
    )
    .unwrap();
    game.apply(PlayerId::Two, Action::FinishDeclaringAttackers)
        .unwrap();
    drain_pending(&mut game);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_001))
        .expect("still attacking");
    assert_eq!(game.power(angel), Some(3), "the 4/4 attacked into the tax");
    assert_eq!(game.toughness(angel), Some(4), "-1/-0 leaves toughness be");

    // Jace's own next turn takes the listener away.
    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
    assert!(
        game.installed_triggers.is_empty(),
        "his next turn began, so the ability stopped listening"
    );
}

#[test]
fn pendelhaven_only_pumps_something_that_is_still_a_one_one_when_it_resolves() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::PENDELHAVEN, PlayerId::One));
    // A 1/1 and a 2/1: only the first is a legal target.
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
    attach_constant_resolved_characteristics(
        &mut game,
        GameObjectId(10_001),
        &[AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(-1),
            ValueDef::Constant(0),
        )],
        ContinuousEffectExpiration::Never,
    );
    game.battlefield
        .push(creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One));

    let pump = Action::ActivateAbility {
        source: GameObjectId(10_000),
        ability: activated_ability_for(&game, GameObjectId(10_000), 0),
        targets: activated_targets(Target::Permanent(GameObjectId(10_001))),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    let at_the_two_one = Action::ActivateAbility {
        source: GameObjectId(10_000),
        ability: activated_ability_for(&game, GameObjectId(10_000), 0),
        targets: activated_targets(Target::Permanent(GameObjectId(10_002))),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    let actions = game.legal_actions(PlayerId::One);
    assert!(actions.contains(&pump), "the 1/1 is a legal target");
    assert!(
        !actions.contains(&at_the_two_one),
        "a 2/1 is not a 1/1 creature"
    );

    game.apply(PlayerId::One, pump).unwrap();
    // The ability is on the stack. Growing the target before it resolves
    // makes the target illegal, and the whole ability does nothing.
    assert_eq!(game.stack.len(), 1, "it waits on the stack");
    attach_constant_resolved_characteristics(
        &mut game,
        GameObjectId(10_001),
        &[AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(1),
            ValueDef::Constant(0),
        )],
        ContinuousEffectExpiration::Never,
    );
    drain_pending(&mut game);

    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_001))
        .expect("still there");
    assert_eq!(
        (game.power(lions), game.toughness(lions)),
        (Some(2), Some(1)),
        "it stopped being a 1/1 in response, so it got nothing"
    );
}

#[test]
fn pendelhaven_pumps_a_one_one_that_stays_one() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::PENDELHAVEN, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
    attach_constant_resolved_characteristics(
        &mut game,
        GameObjectId(10_001),
        &[AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(-1),
            ValueDef::Constant(0),
        )],
        ContinuousEffectExpiration::Never,
    );

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: GameObjectId(10_000),
            ability: activated_ability_for(&game, GameObjectId(10_000), 0),
            targets: activated_targets(Target::Permanent(GameObjectId(10_001))),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    drain_pending(&mut game);

    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_001))
        .expect("still there");
    assert_eq!(
        (game.power(lions), game.toughness(lions)),
        (Some(2), Some(3))
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == GameObjectId(10_000))
            .expect("still there")
            .tapped,
        "and the land paid for it"
    );
}

#[test]
fn glasses_of_urza_waits_on_the_stack_before_revealing_a_hand() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::GLASSES_OF_URZA, PlayerId::One));
    game.players[1].hand.clear();
    game.players[1]
        .hand
        .push(card(10_001, cards::MOUNTAIN, PlayerId::Two));

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: GameObjectId(10_000),
            ability: activated_ability_for(&game, GameObjectId(10_000), 0),
            targets: activated_targets(Target::Player(PlayerId::Two)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();

    assert_eq!(
        game.stack.len(),
        1,
        "it goes on the stack like anything else"
    );
    assert_eq!(
        game.last_seen_hands[PlayerId::One.index()],
        None,
        "and nothing is seen until it resolves"
    );
    assert!(game.battlefield[0].tapped, "the cost was paid up front");

    drain_pending(&mut game);
    assert_eq!(
        game.last_seen_hands[PlayerId::One.index()],
        Some((PlayerId::Two, vec![(GameObjectId(10_001), cards::MOUNTAIN)])),
    );
}

#[test]
fn dragon_whelp_only_burns_itself_out_on_the_fourth_activation() {
    let mut game = ready_game();
    game.step = Step::PrecombatMain;
    game.battlefield
        .push(creature(10_000, cards::DRAGON_WHELP, PlayerId::One));

    let pump = |game: &Game| Action::ActivateAbility {
        source: GameObjectId(10_000),
        ability: activated_ability_for(game, GameObjectId(10_000), 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    for _ in 0..3 {
        game.players[0].mana_pool.red = 1;
        let action = pump(&game);
        game.apply(PlayerId::One, action).unwrap();
        drain_pending(&mut game);
    }

    let whelp = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000))
        .expect("still here");
    assert_eq!(game.power(whelp), Some(5), "2/3 pumped three times");
    assert!(
        game.installed_triggers.is_empty(),
        "three activations schedule nothing"
    );

    game.players[0].mana_pool.red = 1;
    let action = pump(&game);
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);
    assert_eq!(
        game.installed_triggers.len(),
        1,
        "the fourth one signs its own death warrant"
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(10_000)),
        "but it is still around until the end step"
    );

    game.step = Step::End;
    game.begin_step_triggers();
    drain_pending(&mut game);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(10_000)),
        "the end step collected it"
    );
}

#[test]
fn dragon_whelps_activation_count_resets_with_the_turn() {
    let mut game = ready_game();
    game.step = Step::PrecombatMain;
    game.battlefield
        .push(creature(10_000, cards::DRAGON_WHELP, PlayerId::One));

    for _ in 0..3 {
        game.players[0].mana_pool.red = 1;
        let action = Action::ActivateAbility {
            source: GameObjectId(10_000),
            ability: activated_ability_for(&game, GameObjectId(10_000), 0),
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        };
        game.apply(PlayerId::One, action).unwrap();
        drain_pending(&mut game);
    }

    // Cleanup is where the once-a-turn state goes, the same place the pump
    // itself wears off.
    game.finish_cleanup();
    game.start_next_turn();
    game.step = Step::PrecombatMain;
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();
    game.players[0].mana_pool.red = 1;
    let action = Action::ActivateAbility {
        source: GameObjectId(10_000),
        ability: activated_ability_for(&game, GameObjectId(10_000), 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);

    assert!(
        game.installed_triggers.is_empty(),
        "a new turn makes it the first activation again, not the fourth"
    );
}

#[test]
fn stone_giant_throws_only_what_it_can_lift_and_the_landing_kills_it() {
    let mut game = ready_game();
    game.step = Step::PrecombatMain;
    // The Giant is a 3/4, so it can lift toughness 1 and 2 but not 4.
    game.battlefield
        .push(creature(10_000, cards::STONE_GIANT, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::SERRA_ANGEL, PlayerId::One));
    game.battlefield
        .push(creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two));
    // A creature's tap ability needs it to have been around a turn.
    game.turns_started[PlayerId::One.index()] = 1;

    let throw = |target| Action::ActivateAbility {
        source: GameObjectId(10_000),
        ability: activated_ability_for(&game, GameObjectId(10_000), 0),
        targets: activated_targets(Target::Permanent(target)),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    let actions = game.legal_actions(PlayerId::One);
    assert!(
        actions.contains(&throw(GameObjectId(10_001))),
        "a 2/1 is light enough"
    );
    assert!(
        !actions.contains(&throw(GameObjectId(10_002))),
        "a 4/4 is not: its toughness is not less than the Giant's power"
    );
    assert!(
        !actions.contains(&throw(GameObjectId(10_003))),
        "and it only throws creatures you control"
    );

    game.apply(PlayerId::One, throw(GameObjectId(10_001)))
        .unwrap();
    assert_eq!(game.stack.len(), 1, "it uses the stack now");
    drain_pending(&mut game);

    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_001))
        .expect("airborne, not gone");
    assert!(
        game.permanent_has_executable_keyword(lions, KeywordAbility::Flying),
        "it is in the air"
    );

    let lions = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == GameObjectId(10_001))
        .expect("still the same permanent");
    lions.controller = PlayerId::Two;
    lions.temporary_keywords.push(KeywordAbility::Hexproof);

    game.step = Step::End;
    game.begin_step_triggers();
    drain_pending(&mut game);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(10_001)),
        "the delayed reference is not a new target, so later control and hexproof do not save it"
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(10_002)),
        "nothing else was touched"
    );
}

#[test]
fn maze_of_ith_stops_the_damage_without_calling_off_the_attack() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = true;
    game.priority = PlayerId::Two;
    game.battlefield
        .push(creature(10_000, cards::MAZE_OF_ITH, PlayerId::Two));
    let mut angel = creature(10_001, cards::SERRA_ANGEL, PlayerId::One);
    angel.attacking = true;
    angel.tapped = true;
    game.battlefield.push(angel);
    // A blocker, so there is damage in both directions to prevent.
    let mut lions = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two);
    lions.blocking = vec![GameObjectId(10_001)];
    game.battlefield.push(lions);

    let maze = Action::ActivateAbility {
        source: GameObjectId(10_000),
        ability: activated_ability_for(&game, GameObjectId(10_000), 0),
        targets: activated_targets(Target::Permanent(GameObjectId(10_001))),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    assert!(
        game.legal_actions(PlayerId::Two).contains(&maze),
        "an attacking creature is a legal target"
    );
    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(
            |action| matches!(action, Action::ActivateAbility { targets, .. }
                if targets.iter().any(|selection| selection
                    .targets()
                    .contains(&Target::Permanent(GameObjectId(10_002)))))
        ),
        "a creature that is only blocking is not attacking"
    );
    game.apply(PlayerId::Two, maze).unwrap();
    drain_pending(&mut game);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_001))
        .expect("still there");
    assert!(!angel.tapped, "the Maze untapped it");
    assert!(
        angel.attacking,
        "and left it attacking: the Maze prevents damage, it does not call off the attack"
    );

    game.step = Step::CombatDamage;
    game.deal_combat_damage();

    assert_eq!(game.players[1].life, 20, "no damage got through");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(10_002)),
        "the 4/4 dealt nothing to its blocker"
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == GameObjectId(10_001))
            .expect("still there")
            .damage,
        0,
        "and the blocker dealt nothing back"
    );
}
