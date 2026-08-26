//! Card-level coverage for copy effects whose exceptions add characteristics
//! or abilities, and for copies made from nonbattlefield cards.

use super::*;

fn cast_as_copy(
    game: &mut Game,
    serial: u32,
    definition: CardDefinitionId,
    copied: GameObjectId,
) -> GameObjectId {
    let copy = card(serial, definition, PlayerId::One);
    let old_id = copy.id;
    game.players[0].hand.push(copy);
    game.players[0].mana_pool.white = 10;
    game.players[0].mana_pool.blue = 10;
    game.players[0].mana_pool.black = 10;
    game.players[0].mana_pool.red = 10;
    game.players[0].mana_pool.green = 10;
    game.players[0].mana_pool.colorless = 10;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == old_id))
        .expect("the copy creature is castable");
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the entering copy effect asks for a creature");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(id, _)| id == copied))
        .expect("the intended creature is a copy option")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .unwrap();

    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
        .expect("the copy creature entered")
        .card
        .id
}

#[test]
fn back_from_the_brink_prices_each_card_then_copies_its_exiled_successor() {
    let mut game = ready_game();
    let brink = CardInstanceId(160_000);
    let angel = card(160_001, cards::SERRA_ANGEL, PlayerId::One);
    let gargantuan = card(160_002, cards::QUICKSILVER_GARGANTUAN, PlayerId::One);
    let angel_id = angel.id;
    let gargantuan_id = gargantuan.id;
    game.battlefield
        .push(creature(brink.0, cards::BACK_FROM_THE_BRINK, PlayerId::One));
    game.players[0].graveyard.extend([angel, gargantuan]);
    game.players[0].mana_pool.white = 2;
    game.players[0].mana_pool.colorless = 3;

    let ability = activated_ability_for(&game, brink, 0);
    let affordable = Action::ActivateAbility {
        source: brink,
        ability,
        targets: Vec::new(),
        cost_objects: vec![angel_id],
        mana_payment: None,
        x: 0,
        modes: Vec::new(),
    };
    let unaffordable = Action::ActivateAbility {
        source: brink,
        ability,
        targets: Vec::new(),
        cost_objects: vec![gargantuan_id],
        mana_payment: None,
        x: 0,
        modes: Vec::new(),
    };
    let actions = game.legal_actions(PlayerId::One);
    assert!(actions.contains(&affordable));
    assert!(!actions.contains(&unaffordable));

    game.apply(PlayerId::One, affordable).unwrap();
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .all(|card| card.id != angel_id)
    );
    let exiled = game.players[0]
        .exile
        .iter()
        .find(|card| card.definition == cards::SERRA_ANGEL)
        .expect("the selected creature paid the exile cost")
        .id;
    assert_ne!(exiled, angel_id, "the zone change creates a successor");
    assert_eq!(game.players[0].mana_pool.white, 0);
    assert_eq!(game.players[0].mana_pool.colorless, 0);

    pass_priority_pair(&mut game);
    let token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition.is_token())
        .expect("resolution created a token copy");
    assert_eq!(
        Game::effective_rules_source(token),
        ObjectCharacteristics::card(cards::SERRA_ANGEL, CardPartId::PRIMARY),
    );
    assert_eq!(
        (game.power(token), game.toughness(token)),
        (Some(4), Some(4))
    );
    assert!(game.has_flying(token));
}

#[test]
fn cackling_counterpart_creates_a_token_copy_of_its_target() {
    let mut game = ready_game();
    let angel = CardInstanceId(160_010);
    let spell = card(160_011, cards::CACKLING_COUNTERPART, PlayerId::One);
    game.battlefield
        .push(creature(angel.0, cards::SERRA_ANGEL, PlayerId::One));
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.colorless = 1;

    let legal = game.legal_actions(PlayerId::One);
    let cast = legal
        .iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell.id))
        .cloned()
        .unwrap_or_else(|| panic!("Cackling Counterpart is castable: {legal:#?}"));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    let token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition.is_token())
        .expect("the spell created its copy token");
    assert_eq!(
        (game.power(token), game.toughness(token)),
        (Some(4), Some(4))
    );
    assert!(game.has_flying(token));
}

#[test]
fn entry_copy_exceptions_compose_power_color_and_triggered_abilities() {
    let target = CardInstanceId(160_020);

    let mut gargantuan_game = ready_game();
    gargantuan_game
        .battlefield
        .push(creature(target.0, cards::SERRA_ANGEL, PlayerId::Two));
    let gargantuan = cast_as_copy(
        &mut gargantuan_game,
        160_021,
        cards::QUICKSILVER_GARGANTUAN,
        target,
    );
    let gargantuan = gargantuan_game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == gargantuan)
        .unwrap();
    assert_eq!(
        (
            gargantuan_game.power(gargantuan),
            gargantuan_game.toughness(gargantuan)
        ),
        (Some(7), Some(7)),
    );
    assert!(gargantuan_game.has_flying(gargantuan));

    let mut vesuvan_game = ready_game();
    vesuvan_game
        .battlefield
        .push(creature(target.0, cards::SERRA_ANGEL, PlayerId::Two));
    let vesuvan = cast_as_copy(
        &mut vesuvan_game,
        160_022,
        cards::VESUVAN_DOPPELGANGER,
        target,
    );
    let vesuvan = vesuvan_game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == vesuvan)
        .unwrap();
    let vesuvan_rules = vesuvan_game.effective_rules(vesuvan).unwrap();
    assert_eq!(
        vesuvan_game.effective_colors(vesuvan, &vesuvan_rules),
        [false, true, false, false, false],
    );
    assert!(
        vesuvan_game
            .effective_abilities(vesuvan)
            .iter()
            .any(|ability| {
                matches!(
                    ability.ability.definition,
                    DeclarativeAbilityDef::Triggered(_)
                ) && ability
                    .ability
                    .text
                    .starts_with("At the beginning of your upkeep")
            })
    );
}

#[test]
fn evil_twin_adds_an_arbitrary_activated_ability_to_the_copy() {
    let target = CardInstanceId(160_020);
    let mut twin_game = ready_game();
    let victim = CardInstanceId(160_024);
    twin_game.battlefield.extend([
        creature(target.0, cards::SERRA_ANGEL, PlayerId::Two),
        creature(victim.0, cards::SERRA_ANGEL, PlayerId::Two),
    ]);
    let twin = cast_as_copy(&mut twin_game, 160_025, cards::EVIL_TWIN, target);
    twin_game.turns_started[PlayerId::One.index()] = 1;
    twin_game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == twin)
        .unwrap()
        .entered_controller_turn = 0;
    let destroy_twin = Action::ActivateAbility {
        source: twin,
        ability: activated_ability_for(&twin_game, twin, 0),
        targets: activated_targets(Target::Permanent(victim)),
        cost_objects: Vec::new(),
        mana_payment: None,
        x: 0,
        modes: Vec::new(),
    };
    assert!(
        twin_game
            .legal_actions(PlayerId::One)
            .contains(&destroy_twin)
    );
    twin_game.apply(PlayerId::One, destroy_twin).unwrap();
    pass_priority_pair(&mut twin_game);
    assert!(
        twin_game
            .battlefield
            .iter()
            .all(|permanent| permanent.card.id != victim),
        "the arbitrary added activated ability resolves",
    );
}

#[test]
fn progenitor_mimic_adds_an_upkeep_copy_trigger() {
    let target = CardInstanceId(160_020);
    let mut mimic_game = ready_game();
    mimic_game
        .battlefield
        .push(creature(target.0, cards::SERRA_ANGEL, PlayerId::Two));
    cast_as_copy(&mut mimic_game, 160_026, cards::PROGENITOR_MIMIC, target);
    mimic_game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerId::One,
    });
    drain_pending(&mut mimic_game);
    assert_eq!(
        mimic_game
            .battlefield
            .iter()
            .filter(|permanent| permanent.card.definition.is_token())
            .count(),
        1,
        "the added upkeep ability makes one copy token",
    );
}

#[test]
fn lithoform_engine_copies_a_permanent_spell_into_a_token() {
    let mut game = ready_game();
    let engine = CardInstanceId(160_030);
    let angel = card(160_031, cards::SERRA_ANGEL, PlayerId::One);
    game.battlefield
        .push(creature(engine.0, cards::LITHOFORM_ENGINE, PlayerId::One));
    game.players[0].hand.push(angel.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 9);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == angel.id))
        .expect("Serra Angel is castable");
    game.apply(PlayerId::One, cast).unwrap();
    let original = game.stack.last().expect("the Angel is on the stack").id;
    let copy = Action::ActivateAbility {
        source: engine,
        ability: activated_ability_for(&game, engine, 2),
        targets: activated_targets(Target::Spell(original)),
        cost_objects: Vec::new(),
        mana_payment: None,
        x: 0,
        modes: Vec::new(),
    };
    assert!(game.legal_actions(PlayerId::One).contains(&copy));
    game.apply(PlayerId::One, copy).unwrap();

    pass_priority_pair(&mut game);
    assert!(
        game.stack
            .iter()
            .any(|object| object.is_copy && object.card.definition == cards::SERRA_ANGEL),
        "the Engine puts a copy of the permanent spell on the stack",
    );
    pass_priority_pair(&mut game);

    let token = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition.is_token())
        .expect("the copied permanent spell resolves as a token");
    assert_eq!(
        Game::effective_rules_source(token),
        ObjectCharacteristics::card(cards::SERRA_ANGEL, CardPartId::PRIMARY),
    );
    assert!(game.has_flying(token));

    pass_priority_pair(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| {
                Game::effective_rules_source(permanent)
                    == ObjectCharacteristics::card(cards::SERRA_ANGEL, CardPartId::PRIMARY)
            })
            .count(),
        2,
        "the physical original resolves separately",
    );
}

#[test]
fn lithoform_engine_copies_an_ability_and_may_retarget_it() {
    let mut game = ready_game();
    let engine = CardInstanceId(160_040);
    let sorcerer = CardInstanceId(160_041);
    let first = CardInstanceId(160_042);
    let second = CardInstanceId(160_043);
    game.turns_started[PlayerId::One.index()] = 1;
    game.battlefield.extend([
        creature(engine.0, cards::LITHOFORM_ENGINE, PlayerId::One),
        creature(sorcerer.0, cards::PRODIGAL_SORCERER, PlayerId::One),
        creature(first.0, cards::SAVANNAH_LIONS, PlayerId::Two),
        creature(second.0, cards::SAVANNAH_LIONS, PlayerId::Two),
    ]);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    let ping = Action::ActivateAbility {
        source: sorcerer,
        ability: activated_ability_for(&game, sorcerer, 0),
        targets: activated_targets(Target::Permanent(first)),
        cost_objects: Vec::new(),
        mana_payment: None,
        x: 0,
        modes: Vec::new(),
    };
    game.apply(PlayerId::One, ping).unwrap();
    let original = game.stack.last().expect("the ping is on the stack").id;
    let copy = Action::ActivateAbility {
        source: engine,
        ability: activated_ability_for(&game, engine, 0),
        targets: activated_targets(Target::Spell(original)),
        cost_objects: Vec::new(),
        mana_payment: None,
        x: 0,
        modes: Vec::new(),
    };
    assert!(game.legal_actions(PlayerId::One).contains(&copy));
    game.apply(PlayerId::One, copy).unwrap();
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the copied ability may choose a new target");
    let option = match &game
        .pending_decisions
        .first()
        .expect("the retarget decision is pending")
        .continuation
    {
        DecisionContinuation::CopyStackObject { target_lists, .. } => target_lists
            .iter()
            .position(|targets| flatten_target_selections(targets) == [Target::Permanent(second)])
            .and_then(|index| u32::try_from(index).ok())
            .expect("the other creature is a retarget option"),
        continuation => panic!("unexpected retarget continuation: {continuation:?}"),
    };
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .unwrap();

    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != first && permanent.card.id != second),
        "the original and copied abilities damage their separate targets",
    );
}
