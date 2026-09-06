use super::*;
use crate::CostDef;

fn resolve_upkeep_ability(game: &mut Game) {
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    assert_eq!(
        game.stack.len(),
        1,
        "one cumulative-upkeep ability is waiting"
    );
    game.resolve_stack_top();
}

#[test]
fn cumulative_upkeep_life_is_one_indivisible_scaled_payment() {
    let mut game = ready_game();
    game.step = Step::Upkeep;
    game.players[PlayerId::One.index()].life = 2;
    let gallowbraid = creature(12_000, cards::GALLOWBRAID, PlayerId::One);
    let id = gallowbraid.card.id;
    game.battlefield.push(gallowbraid);

    resolve_upkeep_ability(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Pay 1 life");
    assert_eq!(game.players[PlayerId::One.index()].life, 1);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("the first upkeep was paid")
            .counters(CounterKind::named("age")),
        1,
    );

    resolve_upkeep_ability(&mut game);
    assert!(
        game.pending_decisions.is_empty(),
        "two life cannot be paid at one life"
    );
    assert_eq!(game.players[PlayerId::One.index()].life, 1);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != id),
        "an unaffordable cumulative payment sacrifices the source"
    );
}

#[test]
fn cumulative_upkeep_counter_cost_can_be_paid_or_declined() {
    let mut game = ready_game();
    game.step = Step::Upkeep;
    let aboroth = creature(12_010, cards::ABOROTH, PlayerId::One);
    let id = aboroth.card.id;
    game.battlefield.push(aboroth);

    resolve_upkeep_ability(&mut game);
    choose_decision_by_label(
        &mut game,
        PlayerId::One,
        "Put 1 counter(s) on this permanent",
    );
    let aboroth = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the counter payment kept Aboroth");
    assert_eq!(aboroth.counters(CounterKind::named("age")), 1);
    assert_eq!(aboroth.counters(CounterKind::MinusOneMinusOne), 1);

    resolve_upkeep_ability(&mut game);
    choose_decision_by_label(
        &mut game,
        PlayerId::One,
        "Put 2 counter(s) on this permanent",
    );
    let aboroth = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the second counter payment kept Aboroth");
    assert_eq!(aboroth.counters(CounterKind::named("age")), 2);
    assert_eq!(aboroth.counters(CounterKind::MinusOneMinusOne), 3);

    resolve_upkeep_ability(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Decline");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != id),
        "declining the third three-counter payment sacrifices Aboroth"
    );
}

#[test]
fn cumulative_upkeep_draw_cost_repeats_each_draw() {
    let mut game = ready_game();
    game.step = Step::Upkeep;
    game.battlefield
        .push(creature(12_020, cards::PSYCHIC_VORTEX, PlayerId::One));
    for offset in 0..3 {
        game.players[PlayerId::One.index()].library.push(card(
            12_021 + offset,
            cards::ISLAND,
            PlayerId::One,
        ));
    }

    resolve_upkeep_ability(&mut game);
    assert!(matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::PayOr {
            payment: ResolvedEffectPayment::DrawCards(1),
            cumulative_upkeep_age: Some(1),
            ..
        }
    ));
    choose_decision_by_label(&mut game, PlayerId::One, "Draw 1 card(s)");
    super::delayed_triggers::drain_pending(&mut game);

    resolve_upkeep_ability(&mut game);
    assert!(matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::PayOr {
            payment: ResolvedEffectPayment::DrawCards(2),
            cumulative_upkeep_age: Some(2),
            ..
        }
    ));
    choose_decision_by_label(&mut game, PlayerId::One, "Draw 2 card(s)");
    super::delayed_triggers::drain_pending(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].hand.len(), 3);
}

#[test]
fn unpaid_cumulative_upkeep_is_captured_before_sacrifice() {
    let mut game = ready_game();
    game.step = Step::Upkeep;
    let mut heart = creature(12_030, cards::HEART_OF_BOGARDAN, PlayerId::One);
    let id = heart.card.id;
    heart.set_counters(CounterKind::named("age"), 1);
    game.battlefield.push(heart);
    let opposing = creature(12_031, cards::AIR_ELEMENTAL, PlayerId::Two);
    let opposing_id = opposing.card.id;
    game.battlefield.push(opposing);

    resolve_upkeep_ability(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != id),
        "the unpaid source was sacrificed"
    );
    let [trigger] = game.pending_triggers.as_slice() else {
        panic!("Heart's unpaid cumulative-upkeep trigger was captured");
    };
    assert_eq!(trigger.source.object, id);
    assert_eq!(trigger.context.trigger.event_player, Some(PlayerId::One));
    assert_eq!(trigger.context.trigger.amount, Some(2));

    game.finish_rules_procedure();
    choose_decision_by_label(&mut game, PlayerId::One, "your opponent");
    super::delayed_triggers::drain_pending(&mut game);
    assert_eq!(game.players[PlayerId::Two.index()].life, 18);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == opposing_id)
            .expect("the opposing creature survives two damage")
            .damage,
        2,
    );
}

#[test]
fn inner_sanctum_cumulative_upkeep_card_prevents_damage_to_your_creatures() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(12_040, cards::INNER_SANCTUM, PlayerId::One));
    let protected = creature(12_041, cards::SAVANNAH_LIONS, PlayerId::One);
    let protected_id = protected.card.id;
    game.battlefield.push(protected);
    let opposing = creature(12_042, cards::SAVANNAH_LIONS, PlayerId::Two);
    let opposing_id = opposing.card.id;
    game.battlefield.push(opposing);

    game.damage_target_from_kind(
        Some(opposing_id),
        Some(Target::Permanent(protected_id)),
        2,
        false,
    );
    game.damage_target_from_kind(
        Some(protected_id),
        Some(Target::Permanent(opposing_id)),
        2,
        false,
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == protected_id)
            .unwrap()
            .damage,
        0,
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == opposing_id)
            .unwrap()
            .damage,
        2,
    );
}

#[test]
fn psychic_vortex_cumulative_upkeep_card_sacrifices_a_land_and_discards_its_hand() {
    let mut game = ready_game();
    game.step = Step::End;
    game.battlefield
        .push(creature(12_050, cards::PSYCHIC_VORTEX, PlayerId::One));
    let land = creature(12_051, cards::ISLAND, PlayerId::One);
    let land_id = land.card.id;
    game.battlefield.push(land);
    game.players[PlayerId::One.index()].hand.extend([
        card(12_052, cards::ISLAND, PlayerId::One),
        card(12_053, cards::ISLAND, PlayerId::One),
    ]);

    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::End,
        player: PlayerId::One,
    });
    game.finish_rules_procedure();
    game.resolve_stack_top();
    super::delayed_triggers::drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != land_id)
    );
    assert!(game.players[PlayerId::One.index()].hand.is_empty());
}

#[test]
fn adarkar_unicorn_offers_both_outputs_and_its_mana_only_pays_cumulative_upkeep() {
    let mut game = ready_game();
    let unicorn = creature(12_060, cards::ADARKAR_UNICORN, PlayerId::One);
    let unicorn_id = unicorn.card.id;
    game.battlefield.push(unicorn);

    let actions = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| {
            matches!(
                action,
                Action::ActivateManaAbility { source, .. } if *source == unicorn_id
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        actions.len(),
        2,
        "the printed ability has two exact outputs"
    );
    let two_mana = actions
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility {
                    combination: Some(split),
                    ..
                } if split.total() == 2
            )
        })
        .expect("the colorless-plus-blue output is distinguishable");
    game.apply(PlayerId::One, two_mana).unwrap();

    let player = &game.players[PlayerId::One.index()];
    assert_eq!((player.mana_pool.blue, player.mana_pool.colorless), (1, 1));
    assert!(player.mana.iter().all(|mana| {
        !game.mana_can_pay_for(*mana, &ManaPaymentPurpose::Other)
            && game.mana_can_pay_for(
                *mana,
                &ManaPaymentPurpose::CumulativeUpkeep {
                    source: unicorn_id,
                    snow: false,
                },
            )
    }));
}

#[test]
fn cumulative_upkeep_discard_and_sacrifice_costs_are_atomic_at_age_two() {
    let mut discard_game = ready_game();
    discard_game.step = Step::Upkeep;
    let mut sphinx = creature(12_070, cards::VEXING_SPHINX, PlayerId::One);
    let sphinx_id = sphinx.card.id;
    sphinx.set_counters(CounterKind::named("age"), 1);
    discard_game.battlefield.push(sphinx);
    discard_game.players[PlayerId::One.index()].hand.extend([
        card(12_071, cards::ISLAND, PlayerId::One),
        card(12_072, cards::FOREST, PlayerId::One),
        card(12_073, cards::MOUNTAIN, PlayerId::One),
    ]);

    resolve_upkeep_ability(&mut discard_game);
    let decision = discard_game.observe(PlayerId::One).decision.unwrap();
    assert!(
        decision
            .options
            .iter()
            .skip(1)
            .all(|option| option.members.len() == 2)
    );
    choose_decision_by_label(&mut discard_game, PlayerId::One, "Discard Island, Forest");
    assert_eq!(discard_game.players[PlayerId::One.index()].hand.len(), 1);
    assert!(
        discard_game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == sphinx_id)
    );

    let mut sacrifice_game = ready_game();
    sacrifice_game.step = Step::Upkeep;
    let mut kraken = creature(12_080, cards::POLAR_KRAKEN, PlayerId::One);
    let kraken_id = kraken.card.id;
    kraken.set_counters(CounterKind::named("age"), 1);
    sacrifice_game.battlefield.push(kraken);
    for offset in 0..3 {
        sacrifice_game
            .battlefield
            .push(creature(12_081 + offset, cards::ISLAND, PlayerId::One));
    }

    resolve_upkeep_ability(&mut sacrifice_game);
    let decision = sacrifice_game.observe(PlayerId::One).decision.unwrap();
    assert!(
        decision
            .options
            .iter()
            .skip(1)
            .all(|option| option.members.len() == 2)
    );
    choose_decision_by_label(
        &mut sacrifice_game,
        PlayerId::One,
        "Sacrifice Island, Island",
    );
    assert_eq!(
        sacrifice_game
            .battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::ISLAND)
            .count(),
        1,
    );
    assert!(
        sacrifice_game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == kraken_id)
    );
}

#[test]
fn herald_upkeep_control_lasts_exactly_while_herald_remains() {
    let mut game = ready_game();
    game.step = Step::Upkeep;
    let herald = creature(12_090, cards::HERALD_OF_LESHRAC, PlayerId::One);
    let herald_id = herald.card.id;
    let land = creature(12_091, cards::ISLAND, PlayerId::Two);
    let land_id = land.card.id;
    game.battlefield.extend([herald, land]);

    resolve_upkeep_ability(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Gain control of Island");
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == land_id)
            .unwrap()
            .controller,
        PlayerId::One,
    );

    game.sacrifice_permanents(&[herald_id]);
    game.check_state_based_actions();
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == land_id)
            .unwrap()
            .controller,
        PlayerId::Two,
    );
}

#[test]
fn glacial_chasm_static_prevention_applies_to_its_controller() {
    let mut game = ready_game();
    let chasm = creature(12_100, cards::GLACIAL_CHASM, PlayerId::One);
    let attacker = creature(12_101, cards::SAVANNAH_LIONS, PlayerId::Two);
    let attacker_id = attacker.card.id;
    game.battlefield.extend([chasm, attacker]);

    game.damage_target_from_kind(
        Some(attacker_id),
        Some(Target::Player(PlayerId::One)),
        5,
        true,
    );
    assert_eq!(game.players[PlayerId::One.index()].life, 20);
}

#[test]
fn cumulative_upkeep_can_add_mana_or_benefit_an_opponent() {
    let mut braid_game = ready_game();
    braid_game.step = Step::Upkeep;
    braid_game
        .battlefield
        .push(creature(12_110, cards::BRAID_OF_FIRE, PlayerId::One));
    resolve_upkeep_ability(&mut braid_game);
    choose_decision_by_label(&mut braid_game, PlayerId::One, "Add 1 Red mana");
    assert_eq!(braid_game.players[PlayerId::One.index()].mana_pool.red, 1);

    let mut wall_game = ready_game();
    wall_game.step = Step::Upkeep;
    wall_game
        .battlefield
        .push(creature(12_111, cards::WALL_OF_SHARDS, PlayerId::One));
    resolve_upkeep_ability(&mut wall_game);
    choose_decision_by_label(
        &mut wall_game,
        PlayerId::One,
        "Have an opponent gain 1 life",
    );
    assert_eq!(wall_game.players[PlayerId::Two.index()].life, 21);

    let mut riders_game = ready_game();
    riders_game.step = Step::Upkeep;
    riders_game.battlefield.push(creature(
        12_114,
        cards::VARCHILD_S_WAR_RIDERS,
        PlayerId::One,
    ));
    resolve_upkeep_ability(&mut riders_game);
    choose_decision_by_label(
        &mut riders_game,
        PlayerId::One,
        "Have an opponent create 1 token(s)",
    );
    assert_eq!(
        riders_game
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == PlayerId::Two)
            .count(),
        1,
    );
}

#[test]
fn cumulative_upkeep_snow_mana_requires_a_snow_source() {
    static ABILITIES: [AbilityDef; 1] = [abilities::cumulative_upkeep(CostDef::snow_mana(1))];
    let definition_id = CardDefinitionId::new(120_112);
    let mut definition = CardDefinition::new(
        definition_id,
        "Snow cumulative upkeep fixture",
        CardSet::Coldsnap,
        CardRules::new_enchantment(ManaCost::default()).with_abilities(&ABILITIES),
    );
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    game.step = Step::Upkeep;
    let source = creature(12_112, definition_id, PlayerId::One);
    let snow_land = creature(12_113, cards::SNOW_COVERED_ISLAND, PlayerId::One);
    let snow_land_id = snow_land.card.id;
    game.battlefield.extend([source, snow_land]);

    resolve_upkeep_ability(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Pay 1 snow mana");
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == snow_land_id)
            .expect("the snow land remains")
            .tapped,
        "the payment auto-activates the snow source",
    );
}

#[test]
fn wall_of_shards_cannot_pay_when_the_opponent_cannot_gain_life() {
    let mut game = ready_game();
    game.step = Step::Upkeep;
    game.cannot_gain_life[PlayerId::Two.index()] = true;
    let wall = creature(12_115, cards::WALL_OF_SHARDS, PlayerId::One);
    let wall_id = wall.card.id;
    game.battlefield.push(wall);

    resolve_upkeep_ability(&mut game);
    assert!(game.pending_decisions.is_empty());
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != wall_id),
        "an impossible life-gain payment sacrifices Wall of Shards",
    );
}

#[test]
fn balduvian_fallen_counts_only_black_and_red_mana_spent_on_upkeep() {
    let mut game = ready_game();
    game.step = Step::Upkeep;
    let fallen = creature(12_120, cards::BALDUVIAN_FALLEN, PlayerId::One);
    let fallen_id = fallen.card.id;
    game.battlefield.push(fallen);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);

    resolve_upkeep_ability(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Pay the cost");
    game.finish_rules_procedure();
    assert_eq!(game.stack.len(), 1, "the paid-upkeep trigger was captured");
    game.resolve_stack_top();
    let fallen = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == fallen_id)
        .unwrap();
    assert_eq!(game.power(fallen), Some(4));
}

#[test]
fn karplusan_minotaur_observes_coin_flips_outside_cumulative_upkeep() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let minotaur_definition = game
        .catalog
        .find_by_name("Karplusan Minotaur")
        .expect("Karplusan Minotaur is cataloged");
    let swindler_definition = game
        .catalog
        .find_by_name("Tavern Swindler")
        .expect("Tavern Swindler is cataloged");
    let minotaur = creature(12_125, minotaur_definition, PlayerId::One);
    let minotaur_id = minotaur.card.id;
    let swindler = creature(12_126, swindler_definition, PlayerId::One);
    let swindler_id = swindler.card.id;
    game.battlefield.extend([minotaur, swindler]);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == swindler_id),
        )
        .expect("Tavern Swindler can flip a coin");
    game.apply(PlayerId::One, action).unwrap();
    game.resolve_stack_top();

    assert_eq!(
        game.pending_triggers
            .iter()
            .filter(|trigger| trigger.source.object == minotaur_id)
            .count(),
        1,
        "Karplusan Minotaur observes every coin flip, not only upkeep payments",
    );
}

#[test]
fn thought_lash_exiles_library_cards_as_upkeep_and_activation_costs() {
    let mut game = ready_game();
    game.step = Step::Upkeep;
    let lash = creature(12_130, cards::THOUGHT_LASH, PlayerId::One);
    let lash_id = lash.card.id;
    game.battlefield.push(lash);
    game.players[PlayerId::One.index()].library.extend([
        card(12_131, cards::ISLAND, PlayerId::One),
        card(12_132, cards::FOREST, PlayerId::One),
    ]);

    resolve_upkeep_ability(&mut game);
    choose_decision_by_label(
        &mut game,
        PlayerId::One,
        "Exile the top 1 card(s) of your library",
    );
    assert_eq!(game.players[PlayerId::One.index()].exile.len(), 1);

    game.priority = PlayerId::One;
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == lash_id),
        )
        .expect("the prevention ability is payable with the remaining top card");
    game.apply(PlayerId::One, activation).unwrap();
    assert_eq!(game.players[PlayerId::One.index()].exile.len(), 2);
    game.resolve_stack_top();
    game.damage_target_from_kind(None, Some(Target::Player(PlayerId::One)), 2, false);
    assert_eq!(game.players[PlayerId::One.index()].life, 19);
}
