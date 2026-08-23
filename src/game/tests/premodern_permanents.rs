use super::*;

#[test]
fn quirion_dryad_grows_when_its_controller_casts_a_nongreen_spell() {
    let mut game = ready_game();
    let dryad = creature(10_000, cards::QUIRION_DRYAD, PlayerId::One);
    let dryad_id = dryad.card.id;
    game.battlefield.push(dryad);
    let incinerate = card(10_001, cards::INCINERATE, PlayerId::One);
    game.players[PlayerId::One.index()]
        .hand
        .push(incinerate.clone());
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.players[PlayerId::One.index()].mana_pool.red = 1;

    game.apply(
        PlayerId::One,
        cast_action(
            incinerate.id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    drain_pending(&mut game);

    let dryad = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == dryad_id)
        .expect("Quirion Dryad remains on the battlefield");
    assert_eq!(dryad.counters(CounterKind::PlusOnePlusOne), 1);
}

#[test]
fn goblin_sharpshooter_pings_a_creature_then_untaps_when_it_dies() {
    let mut game = ready_game();
    let sharpshooter = creature(10_000, cards::GOBLIN_SHARPSHOOTER, PlayerId::One);
    let sharpshooter_id = sharpshooter.card.id;
    let victim = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.extend([sharpshooter, victim]);

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: sharpshooter_id,
            ability: activated_ability_for(&game, sharpshooter_id, 0),
            targets: activated_targets(Target::Permanent(victim_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == sharpshooter_id)
            .expect("Sharpshooter remains")
            .tapped,
        "tapping the Sharpshooter is part of the activation cost"
    );
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != victim_id),
        "the one-toughness creature died"
    );
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == sharpshooter_id)
            .expect("Sharpshooter remains")
            .tapped,
        "the death trigger untapped the Sharpshooter"
    );
}

#[test]
fn goblin_sharpshooter_stays_tapped_during_its_controllers_untap_step() {
    let mut game = ready_game();
    let mut sharpshooter = creature(10_000, cards::GOBLIN_SHARPSHOOTER, PlayerId::One);
    sharpshooter.tapped = true;
    game.battlefield.push(sharpshooter);
    game.active_player = PlayerId::Two;
    game.next_regular_player = PlayerId::One;

    game.start_next_turn();

    assert_eq!(game.active_player, PlayerId::One);
    assert!(game.battlefield[0].tapped);
}

#[test]
fn sylvan_safekeeper_sacrifices_a_land_and_grants_true_shroud() {
    let mut game = ready_game();
    let safekeeper = creature(10_000, cards::SYLVAN_SAFEKEEPER, PlayerId::One);
    let safekeeper_id = safekeeper.card.id;
    let protected = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
    let protected_id = protected.card.id;
    let land = creature(10_002, cards::FOREST, PlayerId::One);
    let land_id = land.card.id;
    game.battlefield.extend([safekeeper, protected, land]);

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: safekeeper_id,
            ability: activated_ability_for(&game, safekeeper_id, 0),
            targets: activated_targets(Target::Permanent(protected_id)),
            cost_objects: vec![land_id],
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != land_id),
        "the land is sacrificed as a cost"
    );
    drain_pending(&mut game);

    let protected = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == protected_id)
        .expect("the protected creature remains");
    assert!(game.permanent_has_executable_keyword(protected, KeywordAbility::Shroud));

    let bolt = card(10_003, cards::LIGHTNING_BOLT, PlayerId::Two);
    let bolt_id = bolt.id;
    game.players[PlayerId::Two.index()].hand.push(bolt);
    assert!(
        !game.permanent_can_be_targeted_by(protected, PlayerId::Two, bolt_id, true),
        "shroud prevents opponents from targeting the creature"
    );
    assert!(
        !game.permanent_can_be_targeted_by(protected, PlayerId::One, bolt_id, true),
        "unlike hexproof, shroud also prevents its controller from targeting it"
    );
}

#[test]
fn claws_of_gix_sacrifices_the_chosen_permanent_before_gaining_life() {
    let mut game = ready_game();
    let claws = creature(10_000, cards::CLAWS_OF_GIX, PlayerId::One);
    let source = claws.card.id;
    let fodder = creature(10_001, cards::MOUNTAIN, PlayerId::One);
    let fodder_id = fodder.card.id;
    game.battlefield.extend([claws, fodder]);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source,
            ability: activated_ability_for(&game, source, 0),
            targets: Vec::new(),
            cost_objects: vec![fodder_id],
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != fodder_id),
        "the permanent is sacrificed as a cost"
    );
    assert_eq!(game.players[PlayerId::One.index()].life, 20);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].life, 21);
}

#[test]
fn warmth_triggers_only_for_an_opponents_red_spell() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::WARMTH, PlayerId::One));
    let incinerate = card(10_001, cards::INCINERATE, PlayerId::Two);
    game.players[PlayerId::Two.index()]
        .hand
        .push(incinerate.clone());
    game.players[PlayerId::Two.index()].mana_pool.colorless = 1;
    game.players[PlayerId::Two.index()].mana_pool.red = 1;
    game.priority = PlayerId::Two;
    game.apply(
        PlayerId::Two,
        cast_action(
            incinerate.id,
            vec![Target::Player(PlayerId::One)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        19,
        "Warmth gained two before Incinerate dealt three"
    );
}

#[test]
fn root_maze_makes_future_artifacts_and_lands_enter_tapped() {
    let mut game = ready_game();
    let root = game
        .put_onto_battlefield(PlayerId::One, cards::ROOT_MAZE)
        .expect("cataloged");
    let artifact = game
        .put_onto_battlefield(PlayerId::Two, cards::DARKSTEEL_INGOT)
        .expect("cataloged");
    let land = game
        .put_onto_battlefield(PlayerId::Two, cards::ISLAND)
        .expect("cataloged");
    let creature = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");

    for object in [artifact, land] {
        assert!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == object)
                .expect("the permanent entered")
                .tapped
        );
    }
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == creature)
            .expect("the creature entered")
            .tapped
    );
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == root)
            .expect("Root Maze was already entering before its effect existed")
            .tapped
    );
}

#[test]
fn phyrexian_arena_draws_then_costs_one_life_each_upkeep() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::PHYREXIAN_ARENA, PlayerId::One));
    game.turn = 2;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 1);
    assert_eq!(game.players[PlayerId::One.index()].life, 19);
}

#[test]
fn tranquil_domain_spares_auras_and_upheaval_returns_every_permanent() {
    let mut game = ready_game();
    let host = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let host_id = host.card.id;
    let mut aura = creature(10_001, cards::VOLCANIC_STRENGTH, PlayerId::One);
    aura.attached_to = Some(host_id);
    let aura_id = aura.card.id;
    let maze = creature(10_002, cards::ROOT_MAZE, PlayerId::Two);
    let maze_id = maze.card.id;
    game.battlefield.extend([host, aura, maze]);
    let domain = card(10_003, cards::TRANQUIL_DOMAIN, PlayerId::One);
    game.players[PlayerId::One.index()]
        .hand
        .push(domain.clone());
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.apply(
        PlayerId::One,
        cast_action(domain.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != maze_id)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == aura_id)
    );

    let opposing_land = creature(10_004, cards::MOUNTAIN, PlayerId::Two);
    game.battlefield.push(opposing_land);
    let upheaval = card(10_005, cards::UPHEAVAL, PlayerId::One);
    game.players[PlayerId::One.index()]
        .hand
        .push(upheaval.clone());
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;
    game.players[PlayerId::One.index()].mana_pool.blue = 2;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(upheaval.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty());
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::SAVANNAH_LIONS)
    );
    assert!(
        game.players[PlayerId::Two.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::MOUNTAIN)
    );
}
