use super::*;

// Each constructor accepts the same cost expression, while supplying its own
// mandatory cost. Exercise empty, nonmana, and mixed payments through activation.
macro_rules! mechanics {
    ($costs:expr) => {
        [
            (abilities::cycling!("Cycling", $costs), ZoneKind::Graveyard),
            (
                abilities::typecycling!(
                    "Basic landcycling",
                    $costs,
                    ObjectPredicateDef::HasType(CardType::Land),
                ),
                ZoneKind::Graveyard,
            ),
            (
                abilities::bloodrush!($costs, "Bloodrush", &[], EffectDef::None),
                ZoneKind::Graveyard,
            ),
            (abilities::scavenge!($costs, "Scavenge"), ZoneKind::Exile),
            (
                abilities::eternalize!("Eternalize", $costs),
                ZoneKind::Exile,
            ),
            (abilities::ninjutsu!("Ninjutsu", $costs), ZoneKind::Hand),
        ]
    };
}

fn staged(ability: &AbilityDef, destination: ZoneKind) -> (Game, GameObjectId, GameObjectId) {
    let (mut game, source) = cost_lists::game_with_cost_rules(
        &CardRules::new_creature(mana_cost!("{3}"), &["Test"], 2, 2).with_ability(*ability),
    );
    if destination == ZoneKind::Exile {
        let card = game.players[0].hand.pop().unwrap();
        game.players[0].graveyard.push(card);
    }
    let mut attacker = creature(230_110, cards::GRIZZLY_BEARS, PlayerId::One);
    let attacker_id = attacker.card.id;
    if destination == ZoneKind::Hand {
        game.step = Step::DeclareBlockers;
        game.attackers_declared = true;
        game.blockers_declared = true;
        attacker.attacking = true;
        attacker.tapped = true;
        attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    }
    game.battlefield.push(attacker);
    (game, source, attacker_id)
}

fn activation(game: &Game, source: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source))
}

fn assert_payment(cases: &[(AbilityDef, ZoneKind)], mana: u16, life: i16) {
    for (ability, destination) in cases {
        let destination = *destination;
        let (mut game, source, attacker) = staged(ability, destination);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, mana);
        let action = activation(&game, source).expect(ability.text);
        game.apply(PlayerId::One, action).unwrap();

        assert_eq!(
            game.players[0].mana_pool,
            ManaPool::default(),
            "{}",
            ability.text
        );
        assert_eq!(game.players[0].life, 20 - life, "{}", ability.text);
        assert_eq!(game.stack.len(), 1, "costs are paid before resolution");
        let zone = match destination {
            ZoneKind::Hand => &game.players[0].hand,
            ZoneKind::Graveyard => &game.players[0].graveyard,
            ZoneKind::Exile => &game.players[0].exile,
            _ => unreachable!(),
        };
        assert!(
            zone.iter().any(|card| card.definition
                == CardDefinitionId::from_uuid("00000000-0000-0000-0000-0000000186a1")),
            "{}",
            ability.text
        );
        if destination == ZoneKind::Hand {
            assert!(
                game.players[0]
                    .hand
                    .iter()
                    .any(|card| card.definition == cards::GRIZZLY_BEARS)
            );
            assert!(game.battlefield.iter().all(|card| card.card.id != attacker));
        }
        assert!(
            activation(&game, source).is_none(),
            "the required object was spent"
        );
    }
}

#[test]
fn empty_mechanic_costs_still_pay_the_intrinsic_cost() {
    assert_payment(&mechanics!(crate::NO_COSTS), 0, 0);
}

#[test]
fn nonmana_mechanic_costs_are_paid_alongside_the_intrinsic_cost() {
    const COSTS: &[CostDef] = &[CostDef::PayLife(2)];
    assert_payment(&mechanics!(COSTS), 0, 2);
}

#[test]
fn mixed_mechanic_costs_are_paid_alongside_the_intrinsic_cost() {
    const COSTS: &[CostDef] = &[CostDef::Mana(mana_cost!("{1}")), CostDef::PayLife(2)];
    assert_payment(&mechanics!(COSTS), 1, 2);
}

#[test]
fn unaffordable_variable_costs_do_not_spend_intrinsic_cost_objects() {
    const COSTS: &[CostDef] = &[CostDef::Mana(mana_cost!("{1}")), CostDef::PayLife(2)];
    for (ability, destination) in mechanics!(COSTS) {
        let (mut game, source, _) = staged(&ability, destination);
        game.players[0].life = 1;
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        assert!(activation(&game, source).is_none(), "{}", ability.text);
        assert_eq!(game.players[0].mana_pool.colorless, 1);
        assert!(game.players[0].exile.is_empty());
        assert_eq!(game.battlefield.len(), 1);
        let source_zone = if destination == ZoneKind::Exile {
            &game.players[0].graveyard
        } else {
            &game.players[0].hand
        };
        assert!(source_zone.iter().any(|card| card.definition
            == CardDefinitionId::from_uuid("00000000-0000-0000-0000-0000000186a1")));
    }
}

#[test]
fn multiple_mana_and_life_components_are_combined_before_pricing() {
    const COSTS: &[CostDef] = &[
        CostDef::Mana(mana_cost!("{1}")),
        CostDef::PayLife(2),
        CostDef::Mana(mana_cost!("{2}")),
        CostDef::PayLife(3),
    ];
    let discounted = mechanics!(COSTS).map(|(ability, zone)| {
        (
            ability.with_activation_cost_reduction(ValueDef::Constant(1), 0),
            zone,
        )
    });
    assert_payment(&discounted, 2, 5);
}

#[test]
fn mana_abilities_cannot_spend_life_reserved_for_the_mechanic_cost() {
    const COSTS: &[CostDef] = &[CostDef::Mana(mana_cost!("{1}")), CostDef::PayLife(2)];
    for (ability, destination) in [
        (abilities::cycling!("Cycling", COSTS), ZoneKind::Graveyard),
        (abilities::eternalize!("Eternalize", COSTS), ZoneKind::Exile),
    ] {
        for life in [2, 4] {
            let (mut game, source, _) = staged(&ability, destination);
            resolve_channel(&mut game);
            game.players[0].life = life;
            let action = activation(&game, source);
            if life == 2 {
                assert!(
                    action.is_none(),
                    "Channel cannot spend the life the ability owes"
                );
                assert_eq!(game.players[0].life, 2);
            } else {
                game.apply(PlayerId::One, action.expect(ability.text))
                    .unwrap();
                assert_eq!(
                    game.players[0].life, 1,
                    "two life for the mechanic, one for mana"
                );
                assert_eq!(game.players[0].mana_pool, ManaPool::default());
                assert_eq!(game.stack.len(), 1);
            }
        }
    }
}
