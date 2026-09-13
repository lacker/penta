use super::*;

fn choose_effect_payment(game: &mut Game) {
    game.apply(PlayerId::One, Action::BeginPayment).unwrap();
    let pending = game.pending_decisions[0].observation.clone();
    let option = pending
        .options
        .iter()
        .find(|option| option.label.starts_with("Choose mana:"))
        .unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: pending.id,
            options: vec![option.id],
        },
    )
    .unwrap();
}

#[test]
fn explicit_payment_during_resolution_can_use_a_filter_the_automatic_planner_cannot_chain() {
    static COSTS: [CostDef; 1] = [CostDef::Mana(mana_cost!("{R}"))];
    static PAID: EffectDef = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    };
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::FOREST, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::FIRE_SPRITES, PlayerId::One));
    let source = spell(10_002, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::PayOr(PayOrDef::optional(&COSTS, &PAID))),
        &source,
        TriggerContext::empty(),
    );
    assert_eq!(
        game.pending_decisions[0].observation.options.len(),
        1,
        "automatic planning cannot fund red through the filter"
    );
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .contains(&Action::PassPriority)
    );
    let forest = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateManaAbility { source, .. } if source.0 == 10_000))
        .unwrap();
    game.apply(PlayerId::One, forest).unwrap();
    // The costed mana ability itself permits exact payment in this same window.
    choose_operation(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { source, .. } if source.0 == 10_001),
    );
    choose_mana(&mut game, &[ManaColor::Green]);
    assert_eq!(game.pending_decisions[0].observation.options.len(), 2);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .contains(&Action::PassPriority)
    );
    choose_effect_payment(&mut game);
    choose_mana(&mut game, &[ManaColor::Red]);
    assert_eq!(game.players[0].life, 23);
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

#[test]
fn explicit_payment_preserves_nonmana_choices_in_a_resolving_cost_bundle() {
    static COSTS: [CostDef; 3] = [
        CostDef::Mana(mana_cost!("{1}")),
        CostDef::PayLife(2),
        CostDef::DiscardCards(1),
    ];
    static PAID: EffectDef = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(5),
    };
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::SWAMP, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    let source = spell(10_001, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::PayOr(PayOrDef::optional(&COSTS, &PAID))),
        &source,
        TriggerContext::empty(),
    );
    choose_effect_payment(&mut game);
    choose_mana(&mut game, &[ManaColor::Blue]);
    assert_eq!(game.players[0].life, 23);
    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.players[0].mana_pool.green, 1);
}

#[test]
fn explicit_payment_selects_units_for_each_differently_restricted_mana_obligation() {
    static COSTS: [CostDef; 2] = [
        CostDef::Mana(mana_cost!("{1}")),
        CostDef::repeated(&[CostDef::SnowMana(1)], &ValueDef::Constant(1)),
    ];
    static PAID: EffectDef = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    };
    let mut game = ready_game();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.battlefield
        .push(creature(10_000, cards::SNOW_COVERED_ISLAND, PlayerId::One));
    let land = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateManaAbility { .. }))
        .unwrap();
    game.apply(PlayerId::One, land).unwrap();
    let source = spell(10_001, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::PayOr(PayOrDef::optional(&COSTS, &PAID))),
        &source,
        TriggerContext::empty(),
    );
    choose_effect_payment(&mut game);
    answer_payment(&mut game, 1); // Ordinary red pays the generic bill.
    answer_payment(&mut game, 0);
    assert_eq!(
        game.players[0].mana_pool.total(),
        2,
        "earlier bills remain proposals"
    );
    answer_payment(&mut game, 1); // Snow blue in the projected remaining pool.
    answer_payment(&mut game, 0);
    assert_eq!(game.players[0].mana_pool.total(), 0);
    assert_eq!(game.players[0].life, 23);
}

#[test]
fn explicit_payment_resolving_payment_works_when_automatic_spends_the_needed_snow_unit() {
    static COSTS: [CostDef; 2] = [
        CostDef::Mana(mana_cost!("{1}")),
        CostDef::repeated(&[CostDef::SnowMana(1)], &ValueDef::Constant(1)),
    ];
    static PAID: EffectDef = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    };
    let mut game = ready_game();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.battlefield
        .push(creature(10_000, cards::SNOW_COVERED_PLAINS, PlayerId::One));
    let land = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateManaAbility { .. }))
        .unwrap();
    game.apply(PlayerId::One, land).unwrap();
    let source = spell(10_001, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::PayOr(PayOrDef::optional(&COSTS, &PAID))),
        &source,
        TriggerContext::empty(),
    );
    assert_eq!(
        game.pending_decisions[0].observation.options.len(),
        1,
        "automatic allocation consumes white before blue and cannot finish the snow bill"
    );
    choose_effect_payment(&mut game);
    answer_payment(&mut game, 1); // Ordinary blue pays the generic bill.
    answer_payment(&mut game, 0);
    assert_eq!(
        game.players[0].mana_pool.total(),
        2,
        "earlier bills remain proposals"
    );
    answer_payment(&mut game, 1); // Snow white in the projected remaining pool.
    answer_payment(&mut game, 0);
    assert_eq!(game.players[0].mana_pool.total(), 0);
    assert_eq!(game.players[0].life, 23);
}
