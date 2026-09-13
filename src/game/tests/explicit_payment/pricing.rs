use super::*;

#[test]
fn explicit_payment_prices_the_selected_targets_color_count() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let blade = GameObjectId(10_000);
        let host = GameObjectId(10_001);
        game.battlefield
            .push(creature(blade.0, cards::DRAGONFIRE_BLADE, PlayerId::One));
        game.battlefield.push(token_permanent(
            host.0,
            TokenCharacteristics::creature(
                &["Soldier"],
                &[ManaColor::White, ManaColor::Black],
                2,
                2,
            ),
            PlayerId::One,
        ));
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
        choose_operation(&mut game, |action| {
            matches!(action, Action::ActivateAbility { source, targets, .. }
                if *source == blade && targets.iter().flat_map(TargetSelection::targets)
                    .any(|target| *target == Target::Permanent(host)))
        });
        assert_eq!(game.players[0].mana_pool.total(), 3);
        choose_mana(&mut game, &[ManaColor::Blue; 2]);
        assert_eq!(game.players[0].mana_pool.blue, 0);
        assert_eq!(game.players[0].mana_pool.green, 1);
        assert_eq!(game.battlefield[0].attached_to, None);
        drain_pending(&mut game);
        assert_eq!(game.battlefield[0].attached_to, Some(host));
    }
}

#[test]
fn explicit_payment_mana_pricing_honors_the_cost_modifiers_ability_kind() {
    for (modifier, paid) in [
        (cards::ZIRDA_THE_DAWNWAKER, 2),
        (cards::FORENSIC_GADGETEER, 1),
    ] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_000, cards::CELESTIAL_PRISM, PlayerId::One));
        game.battlefield
            .push(creature(10_001, modifier, PlayerId::One));
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
        choose_operation(
            &mut game,
            |a| matches!(a, Action::ActivateManaAbility { source, color: ManaColor::Red, .. } if source.0 == 10_000),
        );
        choose_mana(&mut game, &vec![ManaColor::Green; paid]);
        assert_eq!(game.players[0].mana_pool.blue, 1);
        assert_eq!(
            game.players[0].mana_pool.green,
            u16::try_from(2 - paid).unwrap()
        );
        assert_eq!(game.players[0].mana_pool.red, 1);
    }
}
