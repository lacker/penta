use super::*;

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
