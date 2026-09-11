use super::*;

static MANA_VAULT_CHARACTERISTICS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
];

#[test]
fn stays_tapped_and_can_be_paid_to_untap_at_upkeep() {
    let mut game = ready_game();
    let mut vault = creature(10_000, cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    game.battlefield.push(vault);
    for id in 10_001..10_005 {
        game.battlefield
            .push(creature(id, cards::MOUNTAIN, PlayerId::One));
    }
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();
    assert_eq!(
        game.pending_triggers.len(),
        1,
        "the upkeep ability triggered"
    );
    pass_priority_pair(&mut game);
    pass_until_decision(&mut game);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(
        decision.prompt,
        "At the beginning of your upkeep, you may pay {4}. If you do, untap this artifact."
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![1],
        },
    )
    .unwrap();

    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::MANA_VAULT)
            .unwrap()
            .tapped
    );
}

#[test]
fn can_pay_for_its_upkeep_trigger_while_untapped() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::MANA_VAULT, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::MOUNTAIN, PlayerId::One));
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();
    pass_priority_pair(&mut game);
    pass_until_decision(&mut game);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(
        decision.options.len(),
        2,
        "the Vault and Mountain can pay {{4}}"
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![1],
        },
    )
    .unwrap();

    let vault = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MANA_VAULT)
        .unwrap();
    let mountain = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MOUNTAIN)
        .unwrap();
    assert!(
        !vault.tapped,
        "the Vault taps for mana and the effect untaps it"
    );
    assert!(mountain.tapped, "the fourth mana comes from the Mountain");
}

#[test]
fn multiple_upkeep_choices_do_not_reuse_stale_mana() {
    // Two vaults trigger separately and resolve one at a time. Four Mountains
    // pay for the first; the second must not be offered mana that is gone.
    let mut game = ready_game();
    for id in 10_000..10_002 {
        let mut vault = creature(id, cards::MANA_VAULT, PlayerId::One);
        vault.tapped = true;
        game.battlefield.push(vault);
    }
    for id in 10_002..10_006 {
        game.battlefield
            .push(creature(id, cards::MOUNTAIN, PlayerId::One));
    }
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();
    assert_eq!(game.pending_triggers.len(), 2, "one ability per vault");
    let prompt =
        "At the beginning of your upkeep, you may pay {4}. If you do, untap this artifact.";
    let first = advance_to_prompt(&mut game, PlayerId::One, prompt);
    assert_eq!(first.options.len(), 2, "four Mountains cover the first");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: first.id,
            options: vec![1],
        },
    )
    .unwrap();

    let second = advance_to_prompt(&mut game, PlayerId::One, prompt);
    assert_eq!(
        second.options.len(),
        1,
        "and paying again is not on offer, because the mana is spent"
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: second.id,
            options: vec![0],
        },
    )
    .unwrap();
    drain_pending(&mut game);

    let vaults: Vec<_> = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == cards::MANA_VAULT)
        .map(|permanent| permanent.tapped)
        .collect();
    assert_eq!(vaults, vec![false, true]);
}

#[test]
fn tapped_vault_deals_one_at_the_draw_step() {
    let mut game = ready_game();
    game.turn = 2; // The starting player skips the first turn's entire draw step.
    let mut vault = creature(10_000, cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    game.battlefield.push(vault);
    game.step = Step::Upkeep;

    game.advance_step();
    assert_eq!(game.step, Step::Draw);
    assert_eq!(
        game.players[0].life, 20,
        "the damage waits on the ability rather than landing with the step"
    );

    pass_priority_pair(&mut game);
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, 19);
}

#[test]
fn untapping_in_upkeep_prevents_the_draw_step_damage_trigger() {
    let mut game = ready_game();
    game.turn = 2; // The starting player skips the first turn's entire draw step.
    let mut vault = creature(10_000, cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    game.battlefield.push(vault);
    for id in 10_001..10_005 {
        game.battlefield
            .push(creature(id, cards::MOUNTAIN, PlayerId::One));
    }
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();
    pass_priority_pair(&mut game);
    pass_until_decision(&mut game);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![1],
        },
    )
    .unwrap();
    drain_pending(&mut game);

    game.advance_step();
    assert_eq!(game.step, Step::Draw);
    assert!(
        game.pending_triggers.is_empty() && game.stack.is_empty(),
        "an untapped Vault fails the intervening-if condition as the draw step begins"
    );
    assert_eq!(game.players[0].life, 20, "untapped, so nothing to pay for");
}

#[test]
fn untap_restriction_uses_its_effective_abilities() {
    let mut game = ready_game();
    let mut vault = creature(10_000, cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    game.battlefield.push(vault);

    game.start_next_turn();
    assert!(game.battlefield[0].tapped);

    attach_constant_resolved_characteristics(
        &mut game,
        GameObjectId(10_000),
        &[AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any)],
        ContinuousEffectExpiration::Never,
    );
    game.start_next_turn();
    game.start_next_turn();
    assert!(
        !game.battlefield[0].tapped,
        "removing the static ability lets the ordinary untap procedure untap the Vault"
    );
}

#[test]
fn untap_restriction_wins_over_smokes_choice_procedure() {
    let mut game = ready_game();
    let mut vault = creature(10_000, cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    game.battlefield.push(vault);
    attach_constant_resolved_characteristics(
        &mut game,
        GameObjectId(10_000),
        &MANA_VAULT_CHARACTERISTICS,
        ContinuousEffectExpiration::Never,
    );
    game.battlefield
        .push(creature(10_001, cards::SMOKE, PlayerId::One));

    game.start_next_turn();

    assert!(!game.untap_pending, "the Vault is not an untap choice");
    assert!(game.battlefield[0].tapped);
}

#[test]
fn draw_trigger_checks_tapped_both_times() {
    let mut game = ready_game();
    game.turn = 2; // The starting player skips the first turn's entire draw step.
    let mut vault = creature(10_000, cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    game.battlefield.push(vault);
    game.step = Step::Upkeep;

    game.advance_step();
    assert_eq!(game.pending_triggers.len(), 1);
    game.battlefield[0].tapped = false;
    pass_priority_pair(&mut game);
    drain_pending(&mut game);
    assert_eq!(
        game.players[0].life, 20,
        "untapping before resolution stops the damage"
    );

    game.step = Step::Upkeep;
    game.battlefield[0].tapped = false;
    game.advance_step();
    assert!(game.pending_triggers.is_empty());
    game.battlefield[0].tapped = true;
    assert_eq!(
        game.players[0].life, 20,
        "tapping after the step began cannot create a trigger retroactively"
    );
}

#[test]
fn draw_trigger_uses_last_known_tapped_status() {
    let mut game = ready_game();
    game.turn = 2; // The starting player skips the first turn's entire draw step.
    let mut vault = creature(10_000, cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    let vault_id = vault.card.id;
    game.battlefield.push(vault);
    game.step = Step::Upkeep;

    game.advance_step();
    game.begin_trigger_placement();
    assert_eq!(game.stack.len(), 1);
    game.destroy_permanent(vault_id);
    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[0].life, 19,
        "the condition uses the tapped source's last-known information"
    );
}

/// The clause the card is played for, and the one that turns the other three
/// on: three colourless off one mana, and the tapping is what the draw step
/// then charges a life for.
#[test]
fn tapping_it_makes_three_and_buys_the_draw_step_damage() {
    let mut game = ready_game();
    game.turn = 2; // The starting player skips the first turn's entire draw step.
    let vault = game
        .put_onto_battlefield(PlayerId::One, cards::MANA_VAULT)
        .expect("cataloged");
    drain_pending(&mut game);
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: vault,
            ability: mana_ability_for(&game, vault, ManaColor::Colorless),
            color: ManaColor::Colorless,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .expect("it taps for mana");

    assert_eq!(
        game.players[0].mana_pool.colorless, 3,
        "one mana in, three out",
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == vault)
            .expect("it is still there")
            .tapped,
        "and it is tapped, which is the whole of the drawback",
    );

    game.step = Step::Upkeep;
    game.advance_step();
    assert_eq!(game.step, Step::Draw);
    pass_priority_pair(&mut game);
    drain_pending(&mut game);
    assert_eq!(
        game.players[0].life, 19,
        "the draw step charges a life for leaving it tapped",
    );
}

/// "At the beginning of *your* upkeep": their upkeep is not yours, so the
/// Vault sits there and asks nothing.
#[test]
fn their_upkeep_offers_nothing() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut vault = creature(10_900, cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    game.battlefield.push(vault);
    for id in 10_901..10_905 {
        game.battlefield
            .push(creature(id, cards::MOUNTAIN, PlayerId::One));
    }
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::Upkeep;

    game.handle_upkeep_triggers();

    assert!(
        game.pending_triggers.is_empty(),
        "nothing triggered on their upkeep",
    );
    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "and nobody was asked for four",
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::MANA_VAULT)
            .is_some_and(|permanent| permanent.tapped),
        "the Vault is where it was",
    );
}

/// The same word on the other trigger: a tapped Vault costs its controller
/// nothing at the beginning of the other player's draw step.
#[test]
fn their_draw_step_costs_nothing() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut vault = creature(10_910, cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    game.battlefield.push(vault);
    game.players[0].life = 20;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.step = Step::Draw;

    game.begin_step_triggers();
    drain_pending(&mut game);

    assert_eq!(
        game.players[0].life, 20,
        "their draw step is not yours, however tapped it is",
    );
}

/// "Doesn't untap during your untap step" is a rule about one step, not about
/// the Vault: anything else that untaps it may. A Manifold Key does it for
/// {1} in your main phase, which is six colourless out of one artifact in a
/// turn and the {4} never asked for.
#[test]
fn another_effect_may_untap_it_outside_the_untap_step() {
    let mut game = ready_game();
    game.battlefield.clear();
    let vault = game
        .put_onto_battlefield(PlayerId::One, cards::MANA_VAULT)
        .expect("cataloged");
    let key = game
        .put_onto_battlefield(PlayerId::One, cards::MANIFOLD_KEY)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let tap_for_three = |game: &mut Game| {
        game.apply(
            PlayerId::One,
            Action::ActivateManaAbility {
                source: vault,
                ability: mana_ability_for(game, vault, ManaColor::Colorless),
                color: ManaColor::Colorless,
                counters_removed: None,
                cost_object: None,
                combination: None,
                triggered_mana: None,
            },
        )
        .expect("it taps for mana");
    };

    tap_for_three(&mut game);
    assert_eq!(
        game.players[0].mana_pool.colorless, 3,
        "three from the first"
    );

    let untap =
        game.legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| match action {
                Action::ActivateAbility {
                    source, targets, ..
                } => {
                    *source == key
                        && targets.iter().any(|selection| {
                            selection.targets().iter().any(
                                |target| matches!(target, Target::Permanent(id) if *id == vault),
                            )
                        })
                }
                _ => false,
            })
            .expect("one of the three pays the Key, which names the Vault");
    game.apply(PlayerId::One, untap).expect("it activates");
    pass_priority_pair(&mut game);
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == vault)
            .expect("still there")
            .tapped,
        "the Vault is untapped in the middle of a main phase",
    );

    tap_for_three(&mut game);
    assert_eq!(
        game.players[0].mana_pool.colorless, 5,
        "three, less the one the Key cost, and three again",
    );
}
