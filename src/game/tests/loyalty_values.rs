use super::*;

#[test]
fn loyalty_values_minus_x_announces_pays_and_restores_zero_and_large_x() {
    for prepared in [false, true] {
        for x in [0, 200] {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(prepared);
            let source = game
                .put_onto_battlefield(PlayerId::One, cards::UGIN_THE_SPIRIT_DRAGON)
                .unwrap();
            game.battlefield
                .iter_mut()
                .find(|p| p.card.id == source)
                .unwrap()
                .add_counters(CounterKind::Loyalty, x);
            let actions = game.legal_actions(PlayerId::One).into_iter().filter(|action| matches!(action,
                Action::ActivateAbility { source: id, ability: AbilityOrigin::Printed { ability: AbilityId(1), .. }, .. }
                if *id == source)).collect::<Vec<_>>();
            assert_eq!(actions.len(), usize::from(x + 8));
            let action = actions
                .into_iter()
                .find(|action| {
                    matches!(action,
                Action::ActivateAbility { x: chosen, .. } if *chosen == x)
                })
                .unwrap();
            let mut invalid = action.clone();
            if let Action::ActivateAbility { x: chosen, .. } = &mut invalid {
                *chosen = x + 8;
            }
            assert!(game.apply(PlayerId::One, invalid).is_err());
            assert_eq!(
                game.battlefield
                    .iter()
                    .find(|p| p.card.id == source)
                    .unwrap()
                    .counters(CounterKind::Loyalty),
                x + 7
            );
            game.apply(PlayerId::One, action).unwrap();
            assert_eq!(
                game.battlefield
                    .iter()
                    .find(|p| p.card.id == source)
                    .unwrap()
                    .counters(CounterKind::Loyalty),
                7
            );
            assert_eq!(game.stack.last().unwrap().x(), x);
            let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
            let mut game = Game::from_observation_checkpoint(
                game.catalog.clone(),
                game.format,
                &wire,
                &hidden,
                42,
            )
            .unwrap();
            assert_eq!(game.stack.last().unwrap().x(), x);
            game.resolve_stack_top();
            assert!(
                !game
                    .legal_actions(PlayerId::One)
                    .iter()
                    .any(|action| matches!(action,
                Action::ActivateAbility { source: id, .. } if *id == source)),
                "even −0 consumes the loyalty activation"
            );
        }
    }
}

#[test]
fn loyalty_values_minus_x_requires_the_loyalty_timing_window() {
    let mut game = ready_game();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::UGIN_THE_SPIRIT_DRAGON)
        .unwrap();
    game.step = Step::Upkeep;
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action,
        Action::ActivateAbility { source: id, .. } if *id == source))
    );
}

#[test]
fn loyalty_values_minus_x_combines_with_a_mana_x_cost() {
    const ABILITIES: &[AbilityDef] = &[AbilityDef::activated(
        "{X}, −X: You gain X life.",
        &[
            CostDef::Mana(mana_cost!("{X}")),
            CostDef::Loyalty(ValueDef::Negate(&ValueDef::ChosenX)),
        ],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::ChosenX,
        },
    )];
    let mut game = ready_game();
    let mut permanent = token_permanent(
        990_001,
        crate::card::TokenCharacteristics::creature(&[], &[], 2, 2).with_abilities(ABILITIES),
        PlayerId::One,
    );
    permanent.add_counters(CounterKind::Loyalty, 5);
    let source = permanent.card.id;
    game.battlefield.push(permanent);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    let actions = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| {
            matches!(action,
        Action::ActivateAbility { source: id, .. } if *id == source)
        })
        .collect::<Vec<_>>();
    assert_eq!(actions.len(), 4, "both resources constrain X");
    let action = actions
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { x: 3, .. }))
        .unwrap();
    let before = game.players[0].life;
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
    assert_eq!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == source)
            .unwrap()
            .counters(CounterKind::Loyalty),
        2
    );
    game.resolve_stack_top();
    assert_eq!(game.players[0].life, before + 3);
}
