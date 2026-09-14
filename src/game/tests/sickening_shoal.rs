use super::*;

fn staged() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].mana_pool = ManaPool::default();
    let shoal = card(194_000, cards::SICKENING_SHOAL, PlayerId::One);
    let source = shoal.id;
    game.players[0].hand.push(shoal);
    let angel = creature(194_001, cards::SERRA_ANGEL, PlayerId::Two);
    let target = angel.card.id;
    game.battlefield.push(angel);
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    (game, source, target)
}

fn offers(game: &Game, source: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == source))
        .collect()
}

#[test]
fn sickening_shoal_exile_cost_announces_exact_mana_value_and_rejects_forged_x() {
    let (mut game, source, target) = staged();
    assert!(offers(&game, source).is_empty(), "cannot exile itself");
    game.players[0]
        .hand
        .push(card(194_002, cards::HYPNOTIC_SPECTER, PlayerId::One));
    game.players[0]
        .hand
        .push(card(194_003, cards::SERRA_ANGEL, PlayerId::One));
    game.players[0]
        .hand
        .push(card(194_004, cards::DARK_RITUAL, PlayerId::One));
    let offered = offers(&game, source);
    let mut xs = offered
        .iter()
        .filter_map(|action| match action {
            Action::CastSpell { choices, .. } => Some(choices.x()),
            _ => None,
        })
        .collect::<Vec<_>>();
    xs.sort_unstable();
    xs.dedup();
    assert_eq!(xs, vec![1, 3], "only black cards supply an exact X");
    let cast = offered
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { choices, .. } if choices.x() == 3))
        .unwrap();
    let mut forged = cast.clone();
    if let Action::CastSpell { choices, .. } = &mut forged {
        *choices = choices.clone().with_x(2);
    }
    assert!(game.apply(PlayerId::One, forged).is_err());
    game.apply(PlayerId::One, cast).unwrap();
    assert_eq!(game.players[0].exile.len(), 1);
    assert_eq!(game.players[0].exile[0].definition, cards::HYPNOTIC_SPECTER);
    assert_eq!(game.stack.last().unwrap().cast.as_ref().unwrap().x, 3);
    pass_priority_pair(&mut game);
    let angel = game
        .battlefield
        .iter()
        .find(|p| p.card.id == target)
        .unwrap();
    assert_eq!(
        (game.power(angel), game.toughness(angel)),
        (Some(1), Some(1))
    );
    let turn = game.turn;
    for _ in 0..60 {
        if game.turn > turn {
            break;
        }
        game.advance_step();
        drain_pending(&mut game);
    }
    let angel = game
        .battlefield
        .iter()
        .find(|p| p.card.id == target)
        .unwrap();
    assert_eq!(
        (game.power(angel), game.toughness(angel)),
        (Some(4), Some(4))
    );
}

#[test]
fn sickening_shoal_normal_mana_payment_does_not_require_a_pitch_card() {
    let (mut game, source, target) = staged();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    let cast = offers(&game, source)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { choices, .. }
            if choices.x() == 4 && choices.costs().alternative().is_none())
        })
        .expect("pay XBB with X=4");
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    assert!(!game.battlefield.iter().any(|p| p.card.id == target));
    assert!(game.players[0].exile.is_empty());
}
