use super::*;

#[test]
fn urgent_exorcism_takes_spirits_and_enchantments_but_nothing_else() {
    // The predicate is a subtype or a card type, so a plain creature is out
    // of reach while a Spirit creature is not.
    for (target_definition, legal) in [
        (cards::STRANGLEROOT_GEIST, true),
        (cards::ENERGY_FLUX, true),
        (cards::SAVANNAH_LIONS, false),
        (cards::BLACK_VISE, false),
    ] {
        let mut game = ready_game();
        let target = creature(10_000, target_definition, PlayerId::Two);
        let target_id = target.card.id;
        game.battlefield.push(target);
        let spell = card(10_001, cards::URGENT_EXORCISM, PlayerId::One);
        game.players[0].hand.push(spell.clone());
        game.players[0].mana_pool.white = 1;
        game.players[0].mana_pool.colorless = 1;

        let action = cast_action(spell.id, vec![Target::Permanent(target_id)], Vec::new(), 0);
        assert_eq!(
            game.legal_actions(PlayerId::One).contains(&action),
            legal,
            "{target_definition:?} should be {}",
            if legal { "targetable" } else { "out of reach" }
        );
    }
}

#[test]
fn ray_of_revelation_destroys_an_enchantment() {
    let mut game = ready_game();
    let target = creature(10_000, cards::ENERGY_FLUX, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.push(target);
    let spell = card(10_001, cards::RAY_OF_REVELATION, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(spell.id, vec![Target::Permanent(target_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty());
}

#[test]
fn mizzium_mortars_cannot_be_aimed_at_your_own_creature() {
    for (controller, legal) in [(PlayerId::Two, true), (PlayerId::One, false)] {
        let mut game = ready_game();
        let target = creature(10_000, cards::SERRA_ANGEL, controller);
        let target_id = target.card.id;
        game.battlefield.push(target);
        let spell = card(10_001, cards::MIZZIUM_MORTARS, PlayerId::One);
        game.players[0].hand.push(spell.clone());
        game.players[0].mana_pool.red = 1;
        game.players[0].mana_pool.colorless = 1;

        let action = cast_action(spell.id, vec![Target::Permanent(target_id)], Vec::new(), 0);
        assert_eq!(
            game.legal_actions(PlayerId::One).contains(&action),
            legal,
            "a creature controlled by {controller} should be {}",
            if legal { "targetable" } else { "out of reach" }
        );
        if !legal {
            continue;
        }
        game.apply(PlayerId::One, action).unwrap();
        pass_priority_pair(&mut game);
        // Serra Angel is 4/4, so four damage is exactly lethal.
        assert!(game.battlefield.is_empty());
    }
}

#[test]
fn thragtusk_gains_five_life_when_it_enters() {
    let mut game = ready_game();
    let tusk = card(10_001, cards::THRAGTUSK, PlayerId::One);
    game.players[0].hand.push(tusk.clone());
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 4;

    game.apply(
        PlayerId::One,
        cast_action(tusk.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    // The trigger is a stack object now, so it needs its own resolution.
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].life, 25);
    assert_eq!(game.battlefield.len(), 1);
}

#[test]
fn think_twice_draws_a_card() {
    let mut game = ready_game();
    let spell = card(10_001, cards::THINK_TWICE, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;
    let before = game.players[0].library.len();

    game.apply(
        PlayerId::One,
        cast_action(spell.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].library.len(), before - 1);
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn blasphemous_act_burns_down_both_sides() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::JUZAM_DJINN, PlayerId::Two));
    // A land is not a creature and must survive.
    game.battlefield
        .push(creature(10_002, cards::MOUNTAIN, PlayerId::Two));
    let spell = card(10_003, cards::BLASPHEMOUS_ACT, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 8;

    game.apply(
        PlayerId::One,
        cast_action(spell.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.battlefield.len(), 1, "only the land is left");
    assert_eq!(game.battlefield[0].card.definition, cards::MOUNTAIN);
}

#[test]
fn obzedat_drains_the_opponent_when_it_enters() {
    let mut game = ready_game();
    let obzedat = card(10_001, cards::OBZEDAT_GHOST_COUNCIL, PlayerId::One);
    game.players[0].hand.push(obzedat.clone());
    game.players[0].mana_pool.white = 2;
    game.players[0].mana_pool.black = 2;
    game.players[0].mana_pool.colorless = 1;

    // The creature spell itself takes no targets; the entry trigger picks its
    // own when it goes on the stack.
    game.apply(
        PlayerId::One,
        cast_action(obzedat.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    if let Some(decision) = game.observe(PlayerId::One).decision {
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![decision.options[0].id],
            },
        )
        .unwrap();
    }
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 18);
    assert_eq!(game.players[0].life, 22);
}

#[test]
fn tragic_slip_shrinks_a_creature_and_kills_a_small_one() {
    // Savannah Lions is 2/1, so -1/-1 is lethal; Serra Angel is 4/4 and lives.
    for (definition, survives) in [(cards::SAVANNAH_LIONS, false), (cards::SERRA_ANGEL, true)] {
        let mut game = ready_game();
        let target = creature(10_000, definition, PlayerId::Two);
        let target_id = target.card.id;
        game.battlefield.push(target);
        let spell = card(10_001, cards::TRAGIC_SLIP, PlayerId::One);
        game.players[0].hand.push(spell.clone());
        game.players[0].mana_pool.black = 1;

        game.apply(
            PlayerId::One,
            cast_action(spell.id, vec![Target::Permanent(target_id)], Vec::new(), 0),
        )
        .unwrap();
        pass_priority_pair(&mut game);

        if survives {
            let permanent = game.battlefield.first().expect("the angel survives");
            assert_eq!(game.power(permanent), Some(3));
            assert_eq!(game.toughness(permanent), Some(3));
        } else {
            assert!(game.battlefield.is_empty(), "{definition:?} should die");
        }
    }
}

#[test]
fn quicken_draws_alongside_its_flash_grant() {
    let mut game = ready_game();
    let spell = card(10_001, cards::QUICKEN, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.blue = 1;
    let before = game.players[0].library.len();

    game.apply(
        PlayerId::One,
        cast_action(spell.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].library.len(), before - 1);
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn mutilate_scales_with_the_swamps_you_control() {
    for (swamps, angel_survives) in [(1, true), (4, false)] {
        let mut game = ready_game();
        for index in 0..swamps {
            game.battlefield
                .push(creature(11_000 + index, cards::SWAMP, PlayerId::One));
        }
        // Serra Angel is 4/4, so it dies only once four Swamps are out.
        game.battlefield
            .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::Two));
        let spell = card(10_001, cards::MUTILATE, PlayerId::One);
        game.players[0].hand.push(spell.clone());
        game.players[0].mana_pool.black = 2;
        game.players[0].mana_pool.colorless = 2;

        game.apply(
            PlayerId::One,
            cast_action(spell.id, Vec::new(), Vec::new(), 0),
        )
        .unwrap();
        pass_priority_pair(&mut game);

        let angel = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::SERRA_ANGEL);
        if angel_survives {
            let angel = angel.expect("one Swamp is only -1/-1");
            assert_eq!(game.power(angel), Some(3));
            assert_eq!(game.toughness(angel), Some(3));
        } else {
            assert!(angel.is_none(), "four Swamps is -4/-4 and lethal");
        }
    }
}

#[test]
fn abrupt_decay_only_reaches_cheap_nonland_permanents() {
    // Savannah Lions is {W}, Serra Angel is {3}{W}{W}, and a land is a land.
    for (definition, legal) in [
        (cards::SAVANNAH_LIONS, true),
        (cards::BLACK_VISE, true),
        (cards::SERRA_ANGEL, false),
        (cards::MOUNTAIN, false),
    ] {
        let mut game = ready_game();
        let target = creature(10_000, definition, PlayerId::Two);
        let target_id = target.card.id;
        game.battlefield.push(target);
        let spell = card(10_001, cards::ABRUPT_DECAY, PlayerId::One);
        game.players[0].hand.push(spell.clone());
        game.players[0].mana_pool.black = 1;
        game.players[0].mana_pool.green = 1;

        let action = cast_action(spell.id, vec![Target::Permanent(target_id)], Vec::new(), 0);
        assert_eq!(
            game.legal_actions(PlayerId::One).contains(&action),
            legal,
            "{definition:?} should be {}",
            if legal { "targetable" } else { "out of reach" }
        );
        if !legal {
            continue;
        }
        game.apply(PlayerId::One, action).unwrap();
        pass_priority_pair(&mut game);
        assert!(game.battlefield.is_empty(), "{definition:?} is destroyed");
    }
}

#[test]
fn abrupt_decay_says_on_the_card_that_it_cannot_be_countered() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SAVANNAH_LIONS, PlayerId::Two));
    let decay = card(10_001, cards::ABRUPT_DECAY, PlayerId::One);
    let counterspell = card(10_002, cards::COUNTERSPELL, PlayerId::Two);
    game.players[0].hand.push(decay.clone());
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.green = 1;
    game.players[1].hand.push(counterspell.clone());
    game.players[1].mana_pool.blue = 2;

    game.apply(
        PlayerId::One,
        cast_action(
            decay.id,
            vec![Target::Permanent(CardInstanceId(10_000))],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let decay_on_stack = game.stack[0].id;
    game.apply(
        PlayerId::Two,
        cast_action(
            counterspell.id,
            vec![Target::Spell(decay_on_stack)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield.is_empty(),
        "the Decay resolved despite the Counterspell"
    );
}

#[test]
fn unburial_rites_reanimates_from_your_own_graveyard() {
    let mut game = ready_game();
    game.players[0]
        .graveyard
        .push(card(10_000, cards::SERRA_ANGEL, PlayerId::One));
    // The opponent's graveyard is out of reach.
    game.players[1]
        .graveyard
        .push(card(10_002, cards::JUZAM_DJINN, PlayerId::Two));
    let rites = card(10_001, cards::UNBURIAL_RITES, PlayerId::One);
    game.players[0].hand.push(rites.clone());
    // {4}{B} to cast; the flashback cost is the white half.
    game.players[0].mana_pool.black = 1;
    game.players[0].mana_pool.colorless = 4;

    let theirs = cast_action(
        rites.id,
        vec![Target::Card(CardInstanceId(10_002))],
        Vec::new(),
        0,
    );
    assert!(
        !game.legal_actions(PlayerId::One).contains(&theirs),
        "their graveyard is not yours"
    );

    game.apply(
        PlayerId::One,
        cast_action(
            rites.id,
            vec![Target::Card(CardInstanceId(10_000))],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(game.battlefield[0].card.definition, cards::SERRA_ANGEL);
    assert_eq!(game.battlefield[0].controller, PlayerId::One);
    assert!(
        !game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "the angel left the graveyard, though the Rites itself arrives there"
    );
}

#[test]
fn oblivion_ring_exiles_another_nonland_permanent() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::Two));
    // A land is not a legal target, and neither is the Ring itself.
    game.battlefield
        .push(creature(10_002, cards::MOUNTAIN, PlayerId::Two));
    let ring = card(10_001, cards::OBLIVION_RING, PlayerId::One);
    game.players[0].hand.push(ring.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 2;

    game.apply(
        PlayerId::One,
        cast_action(ring.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the entry trigger asks for its target");
    let offered: Vec<_> = decision
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect();
    assert_eq!(
        offered,
        vec![cards::SERRA_ANGEL],
        "neither the land nor the Ring itself is offered"
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[0].id],
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].exile[0].definition, cards::SERRA_ANGEL);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::SERRA_ANGEL)
    );
}

#[test]
fn war_priest_of_thune_may_decline_to_destroy() {
    for destroy in [true, false] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_000, cards::ENERGY_FLUX, PlayerId::Two));
        let priest = card(10_001, cards::WAR_PRIEST_OF_THUNE, PlayerId::One);
        game.players[0].hand.push(priest.clone());
        game.players[0].mana_pool.white = 1;
        game.players[0].mana_pool.colorless = 1;

        game.apply(
            PlayerId::One,
            cast_action(priest.id, Vec::new(), Vec::new(), 0),
        )
        .unwrap();
        pass_priority_pair(&mut game);

        let decision = game
            .observe(PlayerId::One)
            .decision
            .expect("the trigger asks about its optional target");
        assert_eq!(decision.minimum, 0, "you may, so declining is an answer");
        let options = if destroy {
            vec![decision.options[0].id]
        } else {
            Vec::new()
        };
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options,
            },
        )
        .unwrap();
        pass_priority_pair(&mut game);

        let flux_alive = game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::ENERGY_FLUX);
        assert_eq!(flux_alive, !destroy);
    }
}

#[test]
fn war_priest_of_thune_arrives_even_with_no_enchantment_to_destroy() {
    let mut game = ready_game();
    let priest = card(10_001, cards::WAR_PRIEST_OF_THUNE, PlayerId::One);
    game.players[0].hand.push(priest.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(priest.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "nothing to destroy, so nothing to ask"
    );
    assert_eq!(game.battlefield.len(), 1);
}

#[test]
fn rest_in_peace_exiles_both_graveyards_as_it_enters() {
    let mut game = ready_game();
    game.players[0]
        .graveyard
        .push(card(10_000, cards::SAVANNAH_LIONS, PlayerId::One));
    game.players[1]
        .graveyard
        .push(card(10_002, cards::JUZAM_DJINN, PlayerId::Two));
    let rip = card(10_001, cards::REST_IN_PEACE, PlayerId::One);
    game.players[0].hand.push(rip.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(rip.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);

    assert!(game.players[0].graveyard.is_empty());
    assert!(game.players[1].graveyard.is_empty());
    assert_eq!(game.players[0].exile[0].definition, cards::SAVANNAH_LIONS);
    assert_eq!(game.players[1].exile[0].definition, cards::JUZAM_DJINN);
}

#[test]
fn counterflux_counters_theirs_and_survives_theirs() {
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    let flux = card(10_001, cards::COUNTERFLUX, PlayerId::One);
    let counterspell = card(10_002, cards::COUNTERSPELL, PlayerId::Two);
    game.players[1].hand.push(bolt.clone());
    game.players[1].mana_pool.red = 1;
    game.players[1].mana_pool.blue = 2;
    game.players[1].hand.push(counterspell.clone());
    game.players[0].hand.push(flux.clone());
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.red = 1;
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
        cast_action(flux.id, vec![Target::Spell(bolt_on_stack)], Vec::new(), 0),
    )
    .unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let flux_on_stack = game.stack[1].id;
    game.apply(
        PlayerId::Two,
        cast_action(
            counterspell.id,
            vec![Target::Spell(flux_on_stack)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    for _ in 0..3 {
        pass_priority_pair(&mut game);
    }

    assert!(game.stack.is_empty());
    assert_eq!(game.players[0].life, 20, "the Bolt never resolved");
    assert_eq!(
        game.players[1].graveyard[0].definition,
        cards::COUNTERSPELL,
        "their Counterspell resolved and did nothing"
    );
}

#[test]
fn flinthoof_boar_can_buy_haste_the_turn_it_arrives() {
    let mut game = ready_game();
    let mut boar = creature(10_000, cards::FLINTHOOF_BOAR, PlayerId::One);
    // Summoning sick: it entered on the turn now in progress.
    boar.entered_controller_turn = game.turns_started[0];
    game.battlefield.push(boar);
    game.players[0].mana_pool.red = 1;
    let boar_id = CardInstanceId(10_000);

    assert!(
        !game.can_attack(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == boar_id)
                .unwrap()
        ),
        "summoning sick before the ability"
    );

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == boar_id),
        )
        .expect("the haste ability is activatable");
    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    let boar = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == boar_id)
        .expect("the boar is still there");
    assert!(game.can_attack(boar), "it bought haste until end of turn");
}

#[test]
fn arbor_elf_untaps_a_forest_but_not_a_mountain() {
    let mut game = ready_game();
    let mut elf = creature(10_000, cards::ARBOR_ELF, PlayerId::One);
    elf.entered_controller_turn = game.turns_started[0] - 1;
    game.battlefield.push(elf);
    let mut forest = creature(10_001, cards::FOREST, PlayerId::One);
    forest.tapped = true;
    game.battlefield.push(forest);
    let mut mountain = creature(10_002, cards::MOUNTAIN, PlayerId::One);
    mountain.tapped = true;
    game.battlefield.push(mountain);

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == CardInstanceId(10_000))
        })
        .expect("the elf can untap something");
    let Action::ActivateAbility { targets, .. } = &activate else {
        unreachable!("the action just matched")
    };
    assert_eq!(
        targets
            .iter()
            .flat_map(TargetSelection::targets)
            .copied()
            .collect::<Vec<_>>(),
        vec![Target::Permanent(CardInstanceId(10_001))],
        "only the Forest is a legal target"
    );

    game.apply(PlayerId::One, activate).unwrap();
    pass_priority_pair(&mut game);

    let tapped = |game: &Game, id: u32| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == CardInstanceId(id))
            .expect("still on the battlefield")
            .tapped
    };
    assert!(!tapped(&game, 10_001), "the Forest untapped");
    assert!(tapped(&game, 10_002), "the Mountain did not");
}

#[test]
fn unflinching_courage_pumps_what_it_enchants() {
    let mut game = ready_game();
    let angel = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    // A second creature must not be affected.
    game.battlefield
        .push(creature(10_003, cards::SAVANNAH_LIONS, PlayerId::One));
    let aura = card(10_001, cards::UNFLINCHING_COURAGE, PlayerId::One);
    game.players[0].hand.push(aura.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(aura.id, vec![Target::Permanent(angel_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == angel_id)
        .expect("the angel is enchanted, not gone");
    assert_eq!(game.power(angel), Some(6), "4/4 plus 2/2");
    assert_eq!(game.toughness(angel), Some(6));

    let lions = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS)
        .expect("the lions are still there");
    assert_eq!(
        game.power(lions),
        Some(2),
        "the other creature is untouched"
    );
}

#[test]
fn an_aura_falls_off_when_its_host_dies() {
    let mut game = ready_game();
    let lions = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let lions_id = lions.card.id;
    game.battlefield.push(lions);
    let aura = card(10_001, cards::UNFLINCHING_COURAGE, PlayerId::One);
    game.players[0].hand.push(aura.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(aura.id, vec![Target::Permanent(lions_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.battlefield.len(), 2, "creature and aura");

    game.destroy_permanent_without_regeneration(lions_id);
    game.check_state_based_actions();

    assert!(
        game.battlefield.is_empty(),
        "the aura followed its host off the battlefield"
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::UNFLINCHING_COURAGE),
        "and went to its owner's graveyard"
    );
}

#[test]
fn underworld_connections_lends_its_land_a_draw_ability() {
    let mut game = ready_game();
    let mut swamp = creature(10_000, cards::SWAMP, PlayerId::One);
    swamp.entered_controller_turn = game.turns_started[0] - 1;
    let swamp_id = swamp.card.id;
    game.battlefield.push(swamp);
    let aura = card(10_001, cards::UNDERWORLD_CONNECTIONS, PlayerId::One);
    game.players[0].hand.push(aura.clone());
    game.players[0].mana_pool.black = 2;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(aura.id, vec![Target::Permanent(swamp_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let library_before = game.players[0].library.len();
    // The Aura became a new object as it left the stack, so its permanent id
    // is not the card id it was cast from.
    let aura_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::UNDERWORLD_CONNECTIONS)
        .expect("the aura is on the battlefield")
        .card
        .id;
    // The Swamp still has its own mana ability, so pick the granted one by
    // its origin rather than by guessing at the order.
    let draw = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, ability, .. }
                    if *source == swamp_id
                        && matches!(ability, AbilityOrigin::Granted { source: granter, .. }
                            if *granter == aura_id)
            )
        })
        .expect("the aura granted the land an activated ability");
    game.apply(PlayerId::One, draw).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].library.len(), library_before - 1);
    assert_eq!(game.players[0].life, 19, "one life paid");
}

#[test]
fn thragtusk_leaves_a_beast_behind() {
    let mut game = ready_game();
    let tusk = card(10_001, cards::THRAGTUSK, PlayerId::One);
    game.players[0].hand.push(tusk.clone());
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 4;

    game.apply(
        PlayerId::One,
        cast_action(tusk.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].life, 25, "the entry trigger gained 5");

    let tusk_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::THRAGTUSK)
        .expect("the tusk is on the battlefield")
        .card
        .id;
    game.destroy_permanent_without_regeneration(tusk_id);
    game.check_state_based_actions();
    // Placing a captured trigger on the stack, and resolving it, happen as the
    // game processes actions, so the test has to keep playing rather than only
    // poking the engine.
    for _ in 0..12 {
        if game.battlefield.iter().any(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Beast"], &[ManaColor::Green], 3, 3),
            )
        }) {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    let beast = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Beast"], &[ManaColor::Green], 3, 3),
            )
        })
        .expect("a Beast token replaced it");
    assert_eq!(game.power(beast), Some(3));
    assert_eq!(game.toughness(beast), Some(3));
    assert_eq!(beast.controller, PlayerId::One);
}

#[test]
fn a_token_ceases_to_exist_rather_than_reaching_a_graveyard() {
    let mut game = ready_game();
    game.create_token(
        PlayerId::One,
        tokens::creature(&["Beast"], &[ManaColor::Green], 3, 3),
    );
    let token_id = game.battlefield[0].card.id;
    assert!(game.players[0].graveyard.is_empty());

    game.destroy_permanent_without_regeneration(token_id);
    game.check_state_based_actions();

    assert!(
        game.battlefield.is_empty(),
        "the token left the battlefield"
    );
    assert!(
        game.players[0].graveyard.is_empty(),
        "and ceased to exist rather than landing in a graveyard"
    );
}

#[test]
fn a_token_is_never_deck_legal() {
    let catalog = poc::catalog().expect("catalog builds");
    assert!(
        catalog
            .find_by_name(&tokens::creature(&["Beast"], &[ManaColor::Green], 3, 3).name())
            .is_none(),
        "a token is absent from the card catalog rather than merely format-illegal",
    );
}
#[test]
fn put_onto_battlefield_reaches_a_board_state_directly() {
    let mut game = ready_game();
    let id = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("Serra Angel is in the catalog");

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield");
    assert_eq!(angel.controller, PlayerId::Two);
    assert_eq!(game.power(angel), Some(4));
    assert!(!angel.tapped);

    assert_eq!(
        game.put_onto_battlefield(PlayerId::One, CardDefinitionId::new(60_000)),
        Err(ZoneError::UnknownCard(CardDefinitionId::new(60_000))),
        "an unknown definition is refused rather than guessed at"
    );
}

/// Feldon's Cane sweeps a whole graveyard into its library. The query
/// vocabulary already reached card zones; only the audit line said otherwise.
mod graveyard_sweep {
    use super::*;

    #[test]
    fn feldons_cane_returns_your_graveyard_and_leaves_the_opponents_alone() {
        let mut game = ready_game();
        game.turns_started[PlayerId::One.index()] = 1;
        let cane = creature(10_000, cards::FELDONS_CANE, PlayerId::One);
        let cane_id = cane.card.id;
        game.battlefield.push(cane);
        for id in [10_001, 10_002, 10_003] {
            game.players[PlayerId::One.index()].graveyard.push(card(
                id,
                cards::SAVANNAH_LIONS,
                PlayerId::One,
            ));
        }
        game.players[PlayerId::Two.index()].graveyard.push(card(
            10_004,
            cards::SEDGE_TROLL,
            PlayerId::Two,
        ));
        let library_before = game.players[PlayerId::One.index()].library.len();

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == cane_id)
            })
            .expect("the Cane offers its ability");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        drain_pending(&mut game);

        assert!(
            game.players[PlayerId::One.index()].graveyard.is_empty(),
            "your graveyard went to your library"
        );
        assert_eq!(
            game.players[PlayerId::One.index()].library.len(),
            library_before + 3,
        );
        assert_eq!(
            game.players[PlayerId::Two.index()].graveyard.len(),
            1,
            "and the opponent's graveyard is untouched"
        );
        assert!(
            !game
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == cane_id),
            "the Cane exiled itself paying for it"
        );
    }

    #[test]
    fn feldons_cane_reports_complete_coverage() {
        let catalog = poc::catalog().expect("catalog builds");
        let card = catalog
            .get(cards::FELDONS_CANE)
            .expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            crate::ImplementationStatus::Complete,
        );
    }
}
