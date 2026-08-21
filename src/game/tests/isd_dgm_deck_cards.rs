use super::*;

fn isd_dgm_game() -> Game {
    let mut game = ready_game();
    game.catalog = crate::card::catalog().unwrap();
    game
}

#[test]
fn griselbrand_pays_seven_life_then_draws_seven_cards() {
    let mut game = isd_dgm_game();
    let griselbrand = creature(20_000, cards::GRISELBRAND, PlayerId::One);
    let source = griselbrand.card.id;
    game.battlefield.push(griselbrand);
    game.players[0].library = (0..7)
        .map(|offset| card(20_100 + offset, cards::MOUNTAIN, PlayerId::One))
        .collect();

    let activation = Action::ActivateAbility {
        source,
        ability: activated_ability_for(&game, source, 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    assert!(game.legal_actions(PlayerId::One).contains(&activation));

    game.apply(PlayerId::One, activation).unwrap();
    assert_eq!(game.players[0].life, 13, "life is paid as a cost");
    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.stack.len(), 1);

    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].hand.len(), 7);
    assert!(game.players[0].library.is_empty());
    assert_eq!(game.result, None);
}

#[test]
fn planar_cleansing_destroys_nonlands_but_leaves_both_players_lands() {
    let mut game = isd_dgm_game();
    let cleansing = card(21_000, cards::PLANAR_CLEANSING, PlayerId::One);
    let mountain = creature(21_001, cards::MOUNTAIN, PlayerId::One);
    let lantern = creature(21_002, cards::CHROMATIC_LANTERN, PlayerId::One);
    let forest = creature(21_003, cards::FOREST, PlayerId::Two);
    let griselbrand = creature(21_004, cards::GRISELBRAND, PlayerId::Two);
    let mountain_id = mountain.card.id;
    let forest_id = forest.card.id;
    game.battlefield
        .extend([mountain, lantern, forest, griselbrand]);
    game.players[0].hand.push(cleansing.clone());
    game.players[0].mana_pool.white = 3;
    game.players[0].mana_pool.colorless = 3;

    let cast = cast_action(cleansing.id, Vec::new(), Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>(),
        vec![mountain_id, forest_id]
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::PLANAR_CLEANSING)
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::CHROMATIC_LANTERN)
    );
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::GRISELBRAND)
    );
}

#[test]
fn door_to_nothingness_makes_its_target_lose() {
    let mut game = isd_dgm_game();
    let door = creature(22_000, cards::DOOR_TO_NOTHINGNESS, PlayerId::One);
    let source = door.card.id;
    game.battlefield.push(door);
    game.players[0].mana_pool = ManaPool {
        white: 2,
        blue: 2,
        black: 2,
        red: 2,
        green: 2,
        colorless: 0,
    };

    let activation = Action::ActivateAbility {
        source,
        ability: activated_ability_for(&game, source, 0),
        targets: activated_targets(Target::Player(PlayerId::Two)),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    assert!(game.legal_actions(PlayerId::One).contains(&activation));

    game.apply(PlayerId::One, activation).unwrap();
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != source),
        "the Door was sacrificed as an activation cost"
    );

    pass_priority_pair(&mut game);

    assert_eq!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentLostToAnEffect,
        })
    );
}

#[test]
fn chromatic_lantern_grants_colored_mana_and_gilded_lotus_adds_three() {
    let mut game = isd_dgm_game();
    let lantern = creature(23_000, cards::CHROMATIC_LANTERN, PlayerId::One);
    let mountain = creature(23_001, cards::MOUNTAIN, PlayerId::One);
    let forest = creature(23_002, cards::FOREST, PlayerId::Two);
    let lantern_id = lantern.card.id;
    let mountain_id = mountain.card.id;
    let forest_id = forest.card.id;
    game.battlefield.extend([lantern, mountain, forest]);

    let mountain_blue = Action::ActivateManaAbility {
        source: mountain_id,
        ability: mana_ability_for(&game, mountain_id, ManaColor::Blue),
        color: ManaColor::Blue,
        counters_removed: None,
        cost_object: None,
        combination: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&mountain_blue));
    game.apply(PlayerId::One, mountain_blue).unwrap();
    assert_eq!(game.players[0].mana_pool.blue, 1);

    let lantern_black = Action::ActivateManaAbility {
        source: lantern_id,
        ability: mana_ability_for(&game, lantern_id, ManaColor::Black),
        color: ManaColor::Black,
        counters_removed: None,
        cost_object: None,
        combination: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&lantern_black));
    game.apply(PlayerId::One, lantern_black).unwrap();
    assert_eq!(game.players[0].mana_pool.black, 1);
    assert!(game.stack.is_empty(), "mana abilities resolve immediately");

    let opponent_forest = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == forest_id)
        .unwrap();
    assert_eq!(
        game.mana_ability_activations(opponent_forest)
            .iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Green],
        "the Lantern does not grant an ability to an opponent's land"
    );

    let mut lotus_game = isd_dgm_game();
    let lotus = creature(23_100, cards::GILDED_LOTUS, PlayerId::One);
    let lotus_id = lotus.card.id;
    lotus_game.battlefield.push(lotus);
    let add_red = Action::ActivateManaAbility {
        source: lotus_id,
        ability: mana_ability_for(&lotus_game, lotus_id, ManaColor::Red),
        color: ManaColor::Red,
        counters_removed: None,
        cost_object: None,
        combination: None,
    };
    assert!(lotus_game.legal_actions(PlayerId::One).contains(&add_red));
    lotus_game.apply(PlayerId::One, add_red).unwrap();
    assert_eq!(lotus_game.players[0].mana_pool.red, 3);
    assert!(lotus_game.battlefield[0].tapped);
    assert!(lotus_game.stack.is_empty());
}

#[test]
fn garruk_offers_only_supported_modes_and_makes_one_wurm_per_controlled_land() {
    let mut game = isd_dgm_game();
    let garruk = game
        .put_onto_battlefield(PlayerId::One, cards::GARRUK_PRIMAL_HUNTER)
        .expect("Garruk is cataloged");
    for land in [cards::FOREST, cards::MOUNTAIN, cards::PLAINS] {
        game.put_onto_battlefield(PlayerId::One, land)
            .expect("the controlled land is cataloged");
    }
    game.put_onto_battlefield(PlayerId::Two, cards::ISLAND)
        .expect("the opponent's land is cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == garruk)
        .expect("Garruk is on the battlefield")
        .set_counters(CounterKind::Loyalty, 6);
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let actions = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == garruk),
        )
        .collect::<Vec<_>>();
    let offered = |id| {
        actions.iter().any(|action| {
            matches!(action, Action::ActivateAbility {
                ability: AbilityOrigin::Printed { ability, .. },
                ..
            } if *ability == id)
        })
    };
    assert!(offered(AbilityId::PRIMARY), "the supported +1 is offered");
    assert!(offered(AbilityId(1)), "the minus three is offered too");
    assert!(offered(AbilityId(2)), "the supported minus six is offered");

    let ultimate = actions
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility {
                ability: AbilityOrigin::Printed { ability, .. },
                ..
            } if *ability == AbilityId(2))
        })
        .unwrap();
    game.apply(PlayerId::One, ultimate).unwrap();
    pass_priority_pair(&mut game);

    let wurms = game
        .battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Wurm"], &[ManaColor::Green], 6, 6),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(wurms.len(), 3, "the opponent's land is not counted");
    assert!(wurms.iter().all(|wurm| {
        wurm.controller == PlayerId::One
            && game.creature_stats(wurm)
                == Some(crate::CreatureStats {
                    power: 6,
                    toughness: 6,
                })
    }));
}

#[test]
fn increasing_ambition_searches_for_exactly_one_card_from_hand() {
    let mut game = isd_dgm_game();
    game.players[0].library.clear();
    game.players[0].library.extend([
        card(24_101, cards::MOUNTAIN, PlayerId::One),
        card(24_102, cards::FOREST, PlayerId::One),
        card(24_103, cards::PLAINS, PlayerId::One),
        card(24_104, cards::ISLAND, PlayerId::One),
        card(24_105, cards::SWAMP, PlayerId::One),
        card(24_106, cards::LIGHTNING_BOLT, PlayerId::One),
        card(24_107, cards::SERRA_ANGEL, PlayerId::One),
        card(24_108, cards::BLACK_LOTUS, PlayerId::One),
    ]);
    let remainder_before = game.players[0]
        .library
        .iter()
        .filter(|card| card.id != GameObjectId(24_102))
        .map(|card| card.id)
        .collect::<Vec<_>>();
    let ambition = card(24_100, cards::INCREASING_AMBITION, PlayerId::One);
    game.players[0].hand.push(ambition.clone());
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.colorless = 4;

    let cast = cast_action(ambition.id, Vec::new(), Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(decision.visibility, DecisionVisibility::Private);
    assert_eq!((decision.minimum, decision.maximum), (1, 1));
    assert!(game.observe(PlayerId::Two).decision.is_none());
    let forest = decision
        .options
        .iter()
        .find(|option| {
            option.card
                == Some((
                    GameObjectId(24_102),
                    ObjectCharacteristics::card(cards::FOREST, CardPartId::PRIMARY),
                ))
        })
        .expect("the unrestricted search offers Forest")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![forest],
        },
    )
    .unwrap();

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::FOREST)
    );
    let remainder_after = game.players[0]
        .library
        .iter()
        .map(|card| card.id)
        .collect::<Vec<_>>();
    assert_eq!(remainder_after.len(), remainder_before.len());
    assert_ne!(remainder_after, remainder_before, "the library shuffled");
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::INCREASING_AMBITION)
    );
}

#[test]
fn increasing_ambition_stays_hand_only_when_flashback_is_granted() {
    let mut game = isd_dgm_game();
    let ambition = card(24_200, cards::INCREASING_AMBITION, PlayerId::One);
    game.players[0].graveyard.push(ambition.clone());
    game.players[0].mana_pool = ManaPool {
        white: 10,
        blue: 10,
        black: 10,
        red: 10,
        green: 10,
        colorless: 10,
    };
    game.temporary_ability_grants.push(TemporaryAbilityGrant {
        object: ambition.id,
        ability: CARD_COST_FLASHBACK,
    });

    assert!(game.legal_actions(PlayerId::One).into_iter().all(|action| {
        !matches!(action, Action::CastSpell { card, .. } if card == ambition.id)
    }));
    let definition = game.catalog.get(cards::INCREASING_AMBITION).unwrap();
    assert_eq!(
        definition.play_options[0].restriction,
        PlayRestriction::FromHandOnly
    );
}

#[test]
fn temporal_mastery_schedules_an_extra_turn_and_exiles_itself() {
    let mut game = isd_dgm_game();
    let mastery = card(24_300, cards::TEMPORAL_MASTERY, PlayerId::One);
    game.players[0].hand.push(mastery.clone());
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.colorless = 5;

    let cast = cast_action(mastery.id, Vec::new(), Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    assert!(
        game.extra_turns.is_empty(),
        "the spell still uses the stack"
    );
    pass_priority_pair(&mut game);

    assert_eq!(game.extra_turns, vec![PlayerId::One]);
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::TEMPORAL_MASTERY),
        "the post-resolution destination is exile"
    );
}

#[test]
fn supported_searches_and_metadata_only_spells_have_the_right_cast_availability() {
    let mut game = isd_dgm_game();
    game.players[0].hand.extend([
        card(24_000, cards::FOG, PlayerId::One),
        card(24_001, cards::WORLDFIRE, PlayerId::One),
        card(24_002, cards::FARSEEK, PlayerId::One),
        card(24_003, cards::RANGERS_PATH, PlayerId::One),
    ]);
    game.players[0].mana_pool = ManaPool {
        white: 10,
        blue: 10,
        black: 10,
        red: 10,
        green: 10,
        colorless: 10,
    };

    let cast_cards = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, .. } => Some(card),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert!(
        cast_cards.contains(&CardInstanceId(24_000)),
        "Fog is now executable"
    );
    assert!(!cast_cards.contains(&CardInstanceId(24_001)));
    assert!(
        cast_cards.contains(&CardInstanceId(24_002)),
        "Farseek uses the shared tapped search"
    );
    assert!(
        cast_cards.contains(&CardInstanceId(24_003)),
        "Ranger's Path uses the shared multi-card tapped search"
    );
}

#[test]
fn rangers_path_puts_two_forest_cards_onto_the_battlefield_tapped() {
    let mut game = isd_dgm_game();
    let path = card(24_400, cards::RANGERS_PATH, PlayerId::One);
    game.players[0].hand.push(path.clone());
    game.players[0].library.extend([
        card(24_401, cards::FOREST, PlayerId::One),
        card(24_402, cards::FOREST, PlayerId::One),
        card(24_403, cards::MOUNTAIN, PlayerId::One),
    ]);
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 3;

    game.apply(
        PlayerId::One,
        cast_action(path.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!((decision.minimum, decision.maximum), (0, 2));
    let forests = decision
        .options
        .iter()
        .filter(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(cards::FOREST)
            })
        })
        .map(|option| option.id)
        .collect::<Vec<_>>();
    assert_eq!(forests.len(), 2);
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: forests,
        },
    )
    .unwrap();

    let forests = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == cards::FOREST)
        .collect::<Vec<_>>();
    assert_eq!(forests.len(), 2);
    assert!(forests.iter().all(|forest| forest.tapped));
}
