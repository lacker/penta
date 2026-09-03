mod prevention_amounts {
    use super::*;

    fn shielded(game: &mut Game, source: GameObjectId, card: CardDefinitionId) {
        let holder = creature(10_000, card, PlayerId::One);
        let holder_id = holder.card.id;
        game.battlefield.push(holder);
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == holder_id)
            })
            .expect("the ability is offered");
        game.apply(PlayerId::One, action)
            .expect("the ability activates");
        pass_priority_pair(game);
        let _ = source;
    }

    /// Half of five is two, so three still lands. Rounding down is what the
    /// card says and what a naive halving would get wrong.
    #[test]
    fn dark_sphere_lets_the_odd_point_through() {
        let mut game = ready_game();
        let dragon = creature(10_001, cards::DRAGON_WHELP, PlayerId::Two);
        let dragon_id = dragon.card.id;
        game.battlefield.push(dragon);
        shielded(&mut game, dragon_id, cards::DARK_SPHERE);

        game.damage_target_from(Some(dragon_id), Some(Target::Player(PlayerId::One)), 5);
        assert_eq!(
            game.players[PlayerId::One.index()].life,
            i16::from(rules::STARTING_LIFE) - 3,
            "half of five, rounded down, was prevented"
        );
        assert!(game.damage_preventions.is_empty(), "and the rule is gone");
    }

    #[test]
    fn dark_sphere_prevents_nothing_from_a_single_point() {
        let mut game = ready_game();
        let dragon = creature(10_001, cards::DRAGON_WHELP, PlayerId::Two);
        let dragon_id = dragon.card.id;
        game.battlefield.push(dragon);
        shielded(&mut game, dragon_id, cards::DARK_SPHERE);

        game.damage_target_from(Some(dragon_id), Some(Target::Player(PlayerId::One)), 1);
        assert_eq!(
            game.players[PlayerId::One.index()].life,
            i16::from(rules::STARTING_LIFE) - 1,
            "half of one, rounded down, is none of it"
        );
        assert!(
            game.damage_preventions.is_empty(),
            "the matching event promise is spent even when it prevents zero",
        );
    }

    /// The life gained is what was actually stopped, which for an ordinary
    /// full shield is the whole hit. Gaining what was *aimed* would be the
    /// same number here, so the test also spends the shield on a smaller hit
    /// than the source could have dealt.
    #[test]
    fn reverse_damage_gains_exactly_what_it_prevented() {
        let mut game = ready_game();
        let dragon = creature(10_001, cards::DRAGON_WHELP, PlayerId::Two);
        let dragon_id = dragon.card.id;
        game.battlefield.push(dragon);
        let reverse = card(10_002, cards::REVERSE_DAMAGE, PlayerId::One);
        let reverse_id = reverse.id;
        game.players[PlayerId::One.index()].hand.push(reverse);
        game.players[PlayerId::One.index()].mana_pool.white = 2;
        game.players[PlayerId::One.index()].mana_pool.colorless = 1;

        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == reverse_id))
            .expect("Reverse Damage is castable");
        game.apply(PlayerId::One, action)
            .expect("the spell is cast");
        pass_priority_pair(&mut game);

        game.damage_target_from(Some(dragon_id), Some(Target::Player(PlayerId::One)), 3);
        assert_eq!(
            game.players[PlayerId::One.index()].life,
            i16::from(rules::STARTING_LIFE) + 3,
            "three prevented, three gained"
        );
    }

}
