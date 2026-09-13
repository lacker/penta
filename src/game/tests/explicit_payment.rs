use super::super::payment::BoundManaPayment;
use super::*;

#[test]
fn explicit_payment_spends_exactly_the_selected_mana_units() {
    let mut game = ready_game();
    game.players[0].mana_pool = ManaPool {
        green: 1,
        blue: 1,
        ..ManaPool::default()
    };
    let obligation = game.mana_payment_obligation(
        PlayerId::One,
        mana_cost!("{1}"),
        0,
        &ManaPaymentPurpose::Other,
    );
    let units = game.payment_mana_units(PlayerId::One);
    let blue = units
        .iter()
        .position(|mana| mana.color == ManaColor::Blue)
        .unwrap();
    let spent = game
        .commit_mana_payment(&obligation, &BoundManaPayment { units: vec![blue] })
        .unwrap();
    assert_eq!(spent, vec![Mana::unrestricted(ManaColor::Blue)]);
    assert_eq!(game.players[0].mana_pool.green, 1);
    assert_eq!(game.players[0].mana_pool.blue, 0);
}

#[test]
fn explicit_payment_rejects_duplicates_shortfalls_and_extra_units_without_mutation() {
    let mut game = ready_game();
    game.players[0].mana_pool.blue = 2;
    let obligation = game.mana_payment_obligation(
        PlayerId::One,
        mana_cost!("{1}"),
        0,
        &ManaPaymentPurpose::Other,
    );
    let before = game.players[0].mana_pool;
    for units in [vec![], vec![0, 0], vec![0, 1], vec![2]] {
        assert!(
            game.commit_mana_payment(&obligation, &BoundManaPayment { units })
                .is_none()
        );
        assert_eq!(game.players[0].mana_pool, before);
        assert!(
            game.players[0].mana.is_empty(),
            "validation does not reconcile the pool"
        );
    }
}

#[test]
fn explicit_payment_can_choose_the_generic_two_brid_branch() {
    let mut game = ready_game();
    game.players[0].mana_pool = ManaPool {
        white: 1,
        colorless: 1,
        ..ManaPool::default()
    };
    let obligation = game.mana_payment_obligation(
        PlayerId::One,
        mana_cost!("{2/W}"),
        0,
        &ManaPaymentPurpose::Other,
    );
    let spent = game
        .commit_mana_payment(&obligation, &BoundManaPayment { units: vec![0, 1] })
        .unwrap();
    assert_eq!(spent.len(), 2);
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

#[test]
fn explicit_payment_checks_restrictions_and_preserves_spend_effects() {
    let mut game = ready_game();
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana = vec![
        Mana {
            color: ManaColor::Blue,
            source: None,
            restrictions: &[ManaRestrictionDef::CastYourCommander],
            spend_effects: &[],
        },
        Mana::unrestricted(ManaColor::Blue),
    ];
    let obligation = game.mana_payment_obligation(
        PlayerId::One,
        mana_cost!("{U}"),
        0,
        &ManaPaymentPurpose::Other,
    );
    assert!(!game.validate_mana_payment(&obligation, &BoundManaPayment { units: vec![0] }));
    assert!(
        game.commit_mana_payment(&obligation, &BoundManaPayment { units: vec![1] })
            .is_some()
    );
    assert_eq!(
        game.players[0].mana[0].restrictions,
        &[ManaRestrictionDef::CastYourCommander]
    );
}

fn choose_operation(game: &mut Game, matches: impl Fn(&Action) -> bool) {
    let index = game
        .manual_payment_actions(PlayerId::One)
        .iter()
        .position(matches)
        .unwrap();
    game.apply(PlayerId::One, Action::BeginPayment).unwrap();
    let decision = game.pending_decisions[0].observation.id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision,
            options: vec![u32::try_from(index).expect("option index fits u32")],
        },
    )
    .unwrap();
    if matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::Payment(super::super::payment::state::PaymentDecision::Funding(_))
    ) {
        let decision = game.pending_decisions[0].observation.id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision,
                options: vec![0],
            },
        )
        .unwrap();
    }
}

fn choose_mana(game: &mut Game, colors: &[ManaColor]) {
    let units = game.payment_mana_units(PlayerId::One);
    let mut selected = Vec::new();
    for color in colors {
        selected.push(
            units
                .iter()
                .enumerate()
                .find(|(index, mana)| mana.color == *color && !selected.contains(index))
                .unwrap()
                .0,
        );
    }
    for unit in selected {
        let decision = game.pending_decisions[0].observation.id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision,
                options: vec![u32::try_from(unit).expect("option index fits u32") + 1],
            },
        )
        .unwrap();
    }
    let decision = game.pending_decisions[0].observation.id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision,
            options: vec![0],
        },
    )
    .unwrap();
}

#[test]
fn explicit_payment_cast_preserves_the_players_choice_when_automatic_also_succeeds() {
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::SOL_RING, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    choose_operation(
        &mut game,
        |a| matches!(a, Action::CastSpell { card, .. } if card.0 == 10_000),
    );
    assert_eq!(game.players[0].mana_pool.total(), 2);
    assert!(game.stack.is_empty());
    choose_mana(&mut game, &[ManaColor::Blue]);
    assert_eq!(game.players[0].mana_pool.blue, 0);
    assert_eq!(game.players[0].mana_pool.green, 1);
    assert_eq!(game.stack.len(), 1);
}

#[test]
fn explicit_payment_ordinary_activation_consumes_the_selected_unit() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::JAYEMDAE_TOME, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 4);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    choose_operation(
        &mut game,
        |a| matches!(a, Action::ActivateAbility { source, .. } if source.0 == 10_000),
    );
    choose_mana(&mut game, &[ManaColor::Blue; 4]);
    assert_eq!(game.players[0].mana_pool.green, 1);
    assert_eq!(game.players[0].mana_pool.blue, 0);
    assert!(game.battlefield[0].tapped);
    assert_eq!(game.stack.len(), 1);
}

#[test]
fn explicit_payment_mana_activation_uses_the_same_selection_and_resolves_immediately() {
    let mut game = ready_game();
    game.battlefield.push(creature(
        10_000,
        cards::IMPLEMENTS_OF_SACRIFICE,
        PlayerId::One,
    ));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    choose_operation(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { source, color: ManaColor::Red, .. } if source.0 == 10_000),
    );
    choose_mana(&mut game, &[ManaColor::Blue]);
    assert_eq!(game.players[0].mana_pool.green, 1);
    assert_eq!(game.players[0].mana_pool.blue, 0);
    assert_eq!(game.players[0].mana_pool.red, 2);
    assert!(game.stack.is_empty());
    assert!(game.battlefield.is_empty());
}

#[test]
fn explicit_payment_cancellation_and_invalid_allocations_do_not_spend_resources() {
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::SOL_RING, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
    choose_operation(
        &mut game,
        |a| matches!(a, Action::CastSpell { card, .. } if card.0 == 10_000),
    );
    let decision = game.pending_decisions[0].observation.id;
    for options in [vec![], vec![0, 0], vec![0, 1], vec![100]] {
        assert!(
            game.apply(PlayerId::One, Action::ChooseDecision { decision, options })
                .is_err()
        );
        assert_eq!(game.players[0].mana_pool.blue, 2);
        assert_eq!(game.players[0].hand.len(), 1);
        assert!(game.stack.is_empty());
    }
    game.apply(PlayerId::One, Action::CancelDecision { decision })
        .unwrap();
    assert_eq!(game.players[0].mana_pool.blue, 2);
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn explicit_payment_prefixes_offer_exactly_the_units_that_can_finish_a_payment() {
    let mut game = ready_game();
    for color in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Green,
        ManaColor::Colorless,
        ManaColor::Colorless,
    ] {
        game.add_unrestricted_mana(PlayerId::One, color, 1);
    }
    for cost in [
        mana_cost!("{1}"),
        mana_cost!("{2/W}"),
        mana_cost!("{W/U}{U}"),
        mana_cost!("{2/W}{1}"),
        mana_cost!("{C}{2}"),
        mana_cost!("{0}"),
    ] {
        let obligation =
            game.mana_payment_obligation(PlayerId::One, cost, 0, &ManaPaymentPurpose::Other);
        let units = |mask: u32| {
            (0..5)
                .filter(|index| mask & (1 << index) != 0)
                .collect::<Vec<_>>()
        };
        for prefix in 0..32 {
            let expected = (0..32).any(|complete| {
                complete & prefix == prefix
                    && game.validate_mana_payment(
                        &obligation,
                        &BoundManaPayment {
                            units: units(complete),
                        },
                    )
            });
            assert_eq!(
                game.mana_selection_can_complete(&obligation, &units(prefix)),
                expected,
                "{cost}, prefix {prefix}"
            );
        }
    }
}

fn fixture_permanent(rules: &CardRules) -> Game {
    let (mut game, id) = super::cost_lists::game_with_cost_rules(rules);
    game.prepared_engine = PreparedEngine::compile(&game.catalog);
    let held = game.players[0].hand.remove(0);
    game.battlefield
        .push(creature(id.0, held.definition, PlayerId::One));
    game
}

#[test]
fn explicit_payment_accepts_a_fixed_hybrid_mana_ability_without_a_planner_bound() {
    static ABILITY: AbilityDef = AbilityDef::activated_mana(
        "{2/W}: Add {R}.",
        &[CostDef::Mana(mana_cost!("{2/W}"))],
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
    );
    let mut game =
        fixture_permanent(&CardRules::new_artifact(mana_cost!("{1}")).with_ability(ABILITY));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    choose_operation(&mut game, |a| {
        matches!(a, Action::ActivateManaAbility { .. })
    });
    choose_mana(&mut game, &[ManaColor::White, ManaColor::Blue]);
    assert_eq!(game.players[0].mana_pool.red, 1);
    assert_eq!(game.players[0].mana_pool.total(), 1);
    assert!(!game.battlefield[0].tapped);
}

#[test]
fn explicit_payment_combines_multiple_mana_nodes_into_one_activation_bill() {
    static COSTS: [CostDef; 2] = [
        CostDef::Mana(mana_cost!("{1}")),
        CostDef::Mana(mana_cost!("{1}")),
    ];
    static MANA: AbilityDef = AbilityDef::activated_mana(
        "{1}, {1}: Add {R}.",
        &COSTS,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
    );
    static STACK: AbilityDef = AbilityDef::activated(
        "{1}, {1}: Gain 1 life.",
        &COSTS,
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    );
    for ability in [MANA, STACK] {
        let mut game =
            fixture_permanent(&CardRules::new_artifact(mana_cost!("{1}")).with_ability(ability));
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
        choose_operation(&mut game, |a| {
            matches!(
                a,
                Action::ActivateManaAbility { .. } | Action::ActivateAbility { .. }
            )
        });
        choose_mana(&mut game, &[ManaColor::Blue; 2]);
        assert_eq!(game.players[0].mana_pool.green, 1);
        assert_eq!(game.players[0].mana_pool.blue, 0);
    }
}

#[test]
fn explicit_payment_pool_derived_production_uses_the_selected_units() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::DOUBLING_CUBE, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 3);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    choose_operation(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { source, .. } if source.0 == 10_000),
    );
    choose_mana(&mut game, &[ManaColor::Blue; 3]);
    assert_eq!(game.players[0].mana_pool.green, 2);
    assert_eq!(game.players[0].mana_pool.blue, 0);
}

#[test]
fn explicit_payment_preview_freezes_a_cast_before_funding_and_defers_state_based_actions() {
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::SOL_RING, PlayerId::One));
    let mut wall = creature(10_001, cards::WORKHORSE, PlayerId::One);
    wall.add_counters(CounterKind::PlusOnePlusOne, 1);
    game.battlefield.push(wall);
    let action = cast_action(GameObjectId(10_000), Vec::new(), Vec::new(), 0);
    let (mut preview, frame) = game.preview_payment(PlayerId::One, &action).unwrap();
    assert!(frame.allows_mana_abilities);
    assert!(
        preview.players[0].hand.is_empty(),
        "the proposed spell has left its source zone"
    );
    assert_eq!(
        game.players[0].hand.len(),
        1,
        "preparing a proposal is read-only"
    );
    assert_eq!(game.battlefield[0].counters(CounterKind::PlusOnePlusOne), 1);
    let mut actions = Vec::new();
    preview.add_payment_mana_actions(PlayerId::One, &mut actions);
    let Action::ActivateManaAbility {
        source,
        ability,
        color,
        counters_removed,
        cost_object,
        combination,
        triggered_mana,
    } = actions.remove(0)
    else {
        unreachable!()
    };
    preview.activate_mana_source(
        PlayerId::One,
        source,
        ability,
        color,
        &ManaActivationChoices::new(counters_removed, cost_object, combination, triggered_mana),
    );
    assert_eq!(
        preview.battlefield.len(),
        1,
        "SBA waits until the enclosing payment completes"
    );
    assert_eq!(
        preview.battlefield[0].counters(CounterKind::PlusOnePlusOne),
        0
    );
    assert!(preview.payment_reservations_hold(PlayerId::One, &frame.reserved));
    assert!(preview.pool_can_pay_obligation(&frame.obligation));
}

#[test]
fn explicit_payment_funding_program_uses_a_zero_toughness_payer_before_state_based_actions() {
    use super::super::payment::funding::{FundingStep, PaymentDraft};
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::FLING, PlayerId::One));
    let mut horse = creature(10_001, cards::WORKHORSE, PlayerId::One);
    horse.add_counters(CounterKind::PlusOnePlusOne, 1);
    game.battlefield.push(horse);
    game.battlefield
        .push(creature(10_002, cards::MOUNTAIN, PlayerId::One));
    let action = cast_action(
        GameObjectId(10_000),
        vec![Target::Player(PlayerId::Two)],
        vec![GameObjectId(10_001)],
        0,
    );
    let mut draft = PaymentDraft {
        player: PlayerId::One,
        action: Box::new(action),
        funding: Vec::new(),
        contributions: Vec::new(),
        announcements: Vec::new(),
        resume: None,
    };
    for source in [10_001, 10_002] {
        let (preview, _) = game.preview_funding(&draft).unwrap();
        let mut actions = Vec::new();
        preview.add_payment_mana_actions(PlayerId::One, &mut actions);
        let action = actions
            .into_iter()
            .find(|a| matches!(a, Action::ActivateManaAbility { source: id, .. } if id.0 == source))
            .unwrap();
        draft.funding.push(FundingStep {
            action,
            mana: None,
            answers: Vec::new(),
        });
    }
    let (preview, _) = game.preview_funding(&draft).unwrap();
    assert_eq!(
        preview.battlefield[0].counters(CounterKind::PlusOnePlusOne),
        0
    );
    assert_eq!(game.battlefield[0].counters(CounterKind::PlusOnePlusOne), 1);
    game.commit_payment_draft(&draft, &BoundManaPayment { units: vec![0, 1] })
        .unwrap();
    assert_eq!(game.stack.len(), 1);
    assert!(game.players[0].hand.is_empty());
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::WORKHORSE)
    );
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

fn begin_unfunded(game: &mut Game, matches: impl Fn(&Action) -> bool) {
    let index = game
        .manual_payment_actions(PlayerId::One)
        .iter()
        .position(matches)
        .unwrap();
    game.apply(PlayerId::One, Action::BeginPayment).unwrap();
    answer_payment(game, u32::try_from(index).expect("option index fits u32"));
}

fn answer_payment(game: &mut Game, option: u32) {
    let pending = &game.pending_decisions[0].observation;
    game.apply(
        pending.player,
        Action::ChooseDecision {
            decision: pending.id,
            options: vec![option],
        },
    )
    .unwrap();
}

fn choose_funding(game: &mut Game, matches: impl Fn(&Action) -> bool) {
    let DecisionContinuation::Payment(super::super::payment::state::PaymentDecision::Funding(
        draft,
    )) = &game.pending_decisions[0].continuation
    else {
        panic!("funding menu")
    };
    let index = game
        .funding_candidates(draft)
        .unwrap()
        .iter()
        .position(matches)
        .unwrap();
    answer_payment(
        game,
        u32::try_from(index).expect("option index fits u32") + 1,
    );
}

#[test]
fn explicit_payment_interactive_funding_works_without_any_automatic_solution() {
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::LIGHTNING_BOLT, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::FOREST, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::FIRE_SPRITES, PlayerId::One));
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::CastSpell { .. }))
    );
    begin_unfunded(&mut game, |a| matches!(a, Action::CastSpell { .. }));
    choose_funding(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { source, .. } if source.0 == 10_001),
    );
    choose_funding(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { source, .. } if source.0 == 10_002),
    );
    answer_payment(&mut game, 1); // Green pays the filter.
    answer_payment(&mut game, 0);
    assert!(game.battlefield.iter().all(|p| !p.tapped));
    assert_eq!(game.players[0].mana_pool.total(), 0);
    answer_payment(&mut game, 0); // Finish funding.
    answer_payment(&mut game, 1); // Select the resulting red unit.
    answer_payment(&mut game, 0);
    assert_eq!(game.stack.len(), 1);
    assert!(game.battlefield.iter().all(|p| p.tapped));
}

#[test]
fn explicit_payment_unfunded_mana_activation_cannot_reenter_itself() {
    let mut game = ready_game();
    game.battlefield.push(creature(
        10_000,
        cards::IMPLEMENTS_OF_SACRIFICE,
        PlayerId::One,
    ));
    game.battlefield
        .push(creature(10_001, cards::FOREST, PlayerId::One));
    begin_unfunded(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { source, color: ManaColor::Red, .. } if source.0 == 10_000),
    );
    choose_funding(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { source, .. } if source.0 == 10_001),
    );
    let DecisionContinuation::Payment(super::super::payment::state::PaymentDecision::Funding(
        draft,
    )) = &game.pending_decisions[0].continuation
    else {
        unreachable!()
    };
    assert!(
        !game
            .funding_candidates(draft)
            .unwrap()
            .iter()
            .any(|a| matches!(a, Action::ActivateManaAbility { source, .. } if source.0 == 10_000))
    );
    answer_payment(&mut game, 0);
    answer_payment(&mut game, 1);
    answer_payment(&mut game, 0);
    assert_eq!(game.players[0].mana_pool.red, 2);
    assert_eq!(game.battlefield.len(), 1);
}

#[test]
fn explicit_payment_funding_preserves_and_commits_replacement_answers() {
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::SOL_RING, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::SKIRK_PROSPECTOR, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::MOGG_FANATIC, PlayerId::One));
    for id in [10_003, 10_004] {
        game.battlefield
            .push(creature(id, cards::REST_IN_PEACE, PlayerId::Two));
    }
    begin_unfunded(&mut game, |a| matches!(a, Action::CastSpell { .. }));
    choose_funding(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { source, cost_object: Some(chosen), .. } if source.0 == 10_001 && chosen.0 == 10_002),
    );
    assert!(
        game.pending_decisions[0]
            .observation
            .prompt
            .contains("replacement effect")
    );
    let answer = game.pending_decisions[0].observation.options[0].id;
    answer_payment(&mut game, answer);
    assert!(
        game.players[0].exile.is_empty(),
        "the proposed answer has not moved the payer"
    );
    answer_payment(&mut game, 0);
    answer_payment(&mut game, 1);
    answer_payment(&mut game, 0);
    assert_eq!(game.stack.len(), 1);
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|c| c.definition == cards::MOGG_FANATIC)
    );
}

#[test]
fn explicit_payment_x_is_not_limited_by_automatic_source_search() {
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::EARTHQUAKE, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::FOREST, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::FIRE_SPRITES, PlayerId::One));
    let mut horse = creature(10_003, cards::WORKHORSE, PlayerId::One);
    horse.add_counters(CounterKind::PlusOnePlusOne, 1);
    game.battlefield.push(horse);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::CastSpell { .. }))
    );
    game.apply(PlayerId::One, Action::BeginPayment).unwrap();
    answer_payment(&mut game, u32::MAX); // Announce X = 1 before choosing funding.
    let index = game
        .manual_payment_actions(PlayerId::One)
        .iter()
        .position(|a| matches!(a, Action::CastSpell { choices, .. } if choices.x() == 1))
        .unwrap();
    answer_payment(
        &mut game,
        u32::try_from(index).expect("option index fits u32"),
    );
    choose_funding(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { source, .. } if source.0 == 10_001),
    );
    choose_funding(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { source, .. } if source.0 == 10_002),
    );
    answer_payment(&mut game, 1);
    answer_payment(&mut game, 0);
    choose_funding(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { source, .. } if source.0 == 10_003),
    );
    answer_payment(&mut game, 0);
    answer_payment(&mut game, 1);
    answer_payment(&mut game, 2);
    answer_payment(&mut game, 0);
    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].x(), 1);
}

#[test]
fn explicit_payment_waits_for_an_opponents_target_before_raising_mana() {
    static ABILITY: AbilityDef = AbilityDef::activated_with_targets(
        "{1}: An opponent chooses a target player. That player gains 1 life.",
        &[CostDef::Mana(mana_cost!("{1}"))],
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any))
                .chosen_by_opponent(),
        ],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
        },
    );
    let mut game =
        fixture_permanent(&CardRules::new_artifact(mana_cost!("{1}")).with_ability(ABILITY));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    begin_unfunded(&mut game, |a| matches!(a, Action::ActivateAbility { .. }));
    assert_eq!(game.pending_decisions[0].observation.player, PlayerId::Two);
    assert_eq!(game.players[0].mana_pool.blue, 1);
    let option = game.pending_decisions[0].observation.options[0].id;
    answer_payment(&mut game, option);
    assert_eq!(game.pending_decisions[0].observation.player, PlayerId::One);
    answer_payment(&mut game, 0);
    answer_payment(&mut game, 1);
    answer_payment(&mut game, 0);
    assert_eq!(game.players[0].mana_pool.blue, 0);
    assert_eq!(game.stack.len(), 1);
}

#[test]
fn explicit_payment_retains_a_cast_offer_when_automatic_funding_fails() {
    let mut game = ready_game();
    game.players[0]
        .library
        .push(card(10_000, cards::LIGHTNING_BOLT, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::FOREST, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::FIRE_SPRITES, PlayerId::One));
    let source = spell(10_003, cards::CHANDRA_TORCH_OF_DEFIANCE, PlayerId::One, 0);
    game.exile_top_and_offer_cast(
        PlayerId::One,
        &source,
        TriggerContext::empty().into(),
        ScopedEffect::primary(EffectDef::None),
    );
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::BeginPayment)
    );
    begin_unfunded(&mut game, |a| matches!(a, Action::CastSpell { .. }));
    choose_funding(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { source, .. } if source.0 == 10_001),
    );
    choose_funding(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { source, .. } if source.0 == 10_002),
    );
    answer_payment(&mut game, 1);
    answer_payment(&mut game, 0);
    answer_payment(&mut game, 0);
    answer_payment(&mut game, 1);
    answer_payment(&mut game, 0);
    assert_eq!(game.stack.len(), 1);
    assert!(game.players[0].exile.is_empty());
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn explicit_payment_mana_abilities_accept_ability_restricted_units() {
    static ABILITY: AbilityDef = AbilityDef::activated_mana(
        "{1}: Add {R}.",
        &[CostDef::Mana(mana_cost!("{1}"))],
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
    );
    let mut game =
        fixture_permanent(&CardRules::new_artifact(mana_cost!("{1}")).with_ability(ABILITY));
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana = vec![Mana {
        color: ManaColor::Blue,
        source: None,
        restrictions: &[ManaRestrictionDef::ActivateAbility(
            ObjectPredicateDef::HasType(CardType::Artifact),
        )],
        spend_effects: &[],
    }];
    choose_operation(&mut game, |a| {
        matches!(a, Action::ActivateManaAbility { .. })
    });
    choose_mana(&mut game, &[ManaColor::Blue]);
    assert_eq!(game.players[0].mana_pool.red, 1);
    assert_eq!(game.players[0].mana_pool.blue, 0);
}

#[path = "explicit_payment/contributions.rs"]
mod contributions;

#[path = "explicit_payment/pricing.rs"]
mod pricing;

#[path = "explicit_payment/resolving.rs"]
mod resolving;
