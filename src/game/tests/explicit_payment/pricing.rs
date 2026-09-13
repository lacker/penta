use super::*;

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
