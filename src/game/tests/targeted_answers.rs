use super::*;

#[test]
fn divine_offering_uses_the_destroyed_artifacts_last_known_mana_value() {
    let mut game = ready_game();
    let artifact = creature(10_001, cards::JUGGERNAUT, PlayerId::Two);
    let artifact_id = artifact.card.id;
    game.battlefield.push(artifact);
    assert_eq!(game.permanent_mana_value(&game.battlefield[0]), 4);

    let offering = card(10_002, cards::DIVINE_OFFERING, PlayerId::One);
    game.players[0].hand.push(offering.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 1;
    let cast = cast_action(
        offering.id,
        vec![Target::Permanent(artifact_id)],
        Vec::new(),
        0,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != artifact_id),
        "the artifact was destroyed before its mana value was read",
    );
    assert_eq!(game.players[0].life, 24);
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::JUGGERNAUT)
    );
}

#[test]
fn doom_blade_rejects_black_creatures_and_allows_regeneration() {
    let mut game = ready_game();
    let mut lions = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    lions.regeneration_shields = 1;
    let lions_id = lions.card.id;
    let juzam = creature(10_002, cards::JUZAM_DJINN, PlayerId::Two);
    let juzam_id = juzam.card.id;
    game.battlefield.extend([lions, juzam]);

    let doom_blade = card(10_003, cards::DOOM_BLADE, PlayerId::One);
    game.players[0].hand.push(doom_blade.clone());
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.colorless = 1;
    let hit_lions = cast_action(
        doom_blade.id,
        vec![Target::Permanent(lions_id)],
        Vec::new(),
        0,
    );
    let hit_juzam = cast_action(
        doom_blade.id,
        vec![Target::Permanent(juzam_id)],
        Vec::new(),
        0,
    );
    let legal = game.legal_actions(PlayerId::One);
    assert!(legal.contains(&hit_lions));
    assert!(!legal.contains(&hit_juzam));

    game.apply(PlayerId::One, hit_lions).unwrap();
    pass_priority_pair(&mut game);

    let regenerated = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == lions_id)
        .expect("the regeneration shield replaces destruction");
    assert!(regenerated.tapped);
    assert_eq!(regenerated.regeneration_shields, 0);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.id == juzam_id)
            .count(),
        1,
        "the black creature was never a legal target",
    );
}

#[test]
fn dispel_accepts_and_counters_an_instant_but_rejects_a_sorcery() {
    for (target_definition, is_instant) in
        [(cards::LIGHTNING_BOLT, true), (cards::ARMAGEDDON, false)]
    {
        let mut game = ready_game();
        game.active_player = PlayerId::Two;
        game.priority = PlayerId::Two;
        let target = card(10_001, target_definition, PlayerId::Two);
        game.players[1].hand.push(target.clone());
        game.players[1].mana_pool.white = 1;
        game.players[1].mana_pool.red = 1;
        game.players[1].mana_pool.colorless = 3;
        let target_cast = if is_instant {
            cast_action(
                target.id,
                vec![Target::Player(PlayerId::One)],
                Vec::new(),
                0,
            )
        } else {
            cast_action(target.id, Vec::new(), Vec::new(), 0)
        };
        assert!(game.legal_actions(PlayerId::Two).contains(&target_cast));
        game.apply(PlayerId::Two, target_cast).unwrap();
        let target_stack_id = game.stack.last().unwrap().id;
        game.apply(PlayerId::Two, Action::PassPriority).unwrap();

        let dispel = card(10_002, cards::DISPEL, PlayerId::One);
        game.players[0].hand.push(dispel.clone());
        game.players[0].mana_pool.blue = 1;
        let response = cast_action(
            dispel.id,
            vec![Target::Spell(target_stack_id)],
            Vec::new(),
            0,
        );
        let legal = game.legal_actions(PlayerId::One).contains(&response);
        assert_eq!(
            legal, is_instant,
            "Dispel's target predicate follows the spell's actual type",
        );

        if is_instant {
            game.apply(PlayerId::One, response).unwrap();
            pass_priority_pair(&mut game);
            assert!(game.stack.is_empty());
            assert_eq!(game.players[0].life, 20, "the Bolt was countered");
            assert!(
                game.players[1]
                    .graveyard
                    .iter()
                    .any(|card| card.definition == cards::LIGHTNING_BOLT)
            );
        }
    }
}

#[test]
fn ultimate_price_accepts_exactly_one_color() {
    let mut game = ready_game();
    let mono = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let mono_id = mono.card.id;
    let multicolor = creature(10_002, cards::LOXODON_SMITER, PlayerId::Two);
    let multicolor_id = multicolor.card.id;
    let colorless = creature(10_003, cards::JUGGERNAUT, PlayerId::Two);
    let colorless_id = colorless.card.id;
    game.battlefield.extend([mono, multicolor, colorless]);

    let price = card(10_004, cards::ULTIMATE_PRICE, PlayerId::One);
    game.players[0].hand.push(price.clone());
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.colorless = 1;
    let legal = game.legal_actions(PlayerId::One);
    let targets = |id| cast_action(price.id, vec![Target::Permanent(id)], Vec::new(), 0);
    let hit_mono = targets(mono_id);
    assert!(legal.contains(&hit_mono));
    assert!(!legal.contains(&targets(multicolor_id)));
    assert!(!legal.contains(&targets(colorless_id)));

    game.apply(PlayerId::One, hit_mono).unwrap();
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != mono_id)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == multicolor_id)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == colorless_id)
    );
}

#[test]
fn dissipate_exiles_the_spell_it_counters() {
    let mut game = ready_game();
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[1].hand.push(bolt.clone());
    game.players[1].mana_pool.red = 1;
    let bolt_cast = cast_action(bolt.id, vec![Target::Player(PlayerId::One)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::Two).contains(&bolt_cast));
    game.apply(PlayerId::Two, bolt_cast).unwrap();
    let bolt_stack_id = game.stack.last().unwrap().id;
    game.apply(PlayerId::Two, Action::PassPriority).unwrap();

    let dissipate = card(10_002, cards::DISSIPATE, PlayerId::One);
    game.players[0].hand.push(dissipate.clone());
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.colorless = 1;
    let response = cast_action(
        dissipate.id,
        vec![Target::Spell(bolt_stack_id)],
        Vec::new(),
        0,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&response));
    game.apply(PlayerId::One, response).unwrap();
    pass_priority_pair(&mut game);

    assert!(game.stack.is_empty(), "the spell left the stack");
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .all(|card| card.definition != cards::LIGHTNING_BOLT),
        "a Dissipated spell does not reach the graveyard"
    );
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "it is exiled instead, so it cannot be rebought"
    );
    assert_eq!(game.players[0].life, 20, "the Bolt never resolved");
}

#[test]
fn putrefy_kills_a_creature_or_an_artifact_without_regeneration() {
    let mut game = ready_game();
    let mut shielded_creature = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    shielded_creature.regeneration_shields = 1;
    let creature_id = shielded_creature.card.id;
    let artifact = creature(10_002, cards::BLACK_LOTUS, PlayerId::Two);
    let artifact_id = artifact.card.id;
    let land = creature(10_003, cards::MOUNTAIN, PlayerId::Two);
    let land_id = land.card.id;
    game.battlefield.extend([shielded_creature, artifact, land]);

    let putrefy = card(10_004, cards::PUTREFY, PlayerId::One);
    game.players[0].hand.push(putrefy.clone());
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 1;
    let targets = |id| cast_action(putrefy.id, vec![Target::Permanent(id)], Vec::new(), 0);
    let hit_creature = targets(creature_id);
    let legal = game.legal_actions(PlayerId::One);
    assert!(legal.contains(&hit_creature));
    assert!(legal.contains(&targets(artifact_id)));
    assert!(!legal.contains(&targets(land_id)));

    game.apply(PlayerId::One, hit_creature).unwrap();
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != creature_id),
        "Putrefy's no-regeneration destruction ignores the shield",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == artifact_id),
        "the independently legal artifact target is untouched",
    );
}

#[test]
fn warleaders_helix_burns_and_gains_in_one_resolution() {
    let mut game = ready_game();
    let helix = card(10_001, cards::WARLEADERS_HELIX, PlayerId::One);
    game.players[0].hand.push(helix.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 2;
    let cast = cast_action(helix.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 16, "four damage to the opponent");
    assert_eq!(game.players[0].life, 24, "and four life to you");
}

#[test]
fn warleaders_helix_fizzles_entirely_when_its_creature_target_leaves() {
    let mut game = ready_game();
    let creature = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two);
    let creature_id = creature.card.id;
    game.battlefield.push(creature);

    let helix = card(10_002, cards::WARLEADERS_HELIX, PlayerId::One);
    game.players[0].hand.push(helix.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 2;
    let cast_helix = cast_action(
        helix.id,
        vec![Target::Permanent(creature_id)],
        Vec::new(),
        0,
    );
    assert!(game.legal_actions(PlayerId::One).contains(&cast_helix));
    game.apply(PlayerId::One, cast_helix).unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();

    let swords = card(10_003, cards::SWORDS_TO_PLOWSHARES, PlayerId::Two);
    game.players[1].hand.push(swords.clone());
    game.players[1].mana_pool.white = 1;
    let cast_swords = cast_action(
        swords.id,
        vec![Target::Permanent(creature_id)],
        Vec::new(),
        0,
    );
    assert!(game.legal_actions(PlayerId::Two).contains(&cast_swords));
    game.apply(PlayerId::Two, cast_swords).unwrap();
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != creature_id),
        "the response removed Helix's only target",
    );

    pass_priority_pair(&mut game);

    assert!(game.stack.is_empty());
    assert_eq!(game.players[0].life, 20, "the fizzled Helix gained no life");
    assert!(game.events.iter().any(|event| matches!(
        event,
        GameEvent::SpellFizzled { definition, .. }
            if *definition == cards::WARLEADERS_HELIX
    )));
}
