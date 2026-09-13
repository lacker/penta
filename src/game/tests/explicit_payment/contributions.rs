use super::*;

fn choose_contribution(
    game: &mut Game,
    source: u32,
    symbol: crate::game::payment::contributions::PaymentSymbol,
) {
    let DecisionContinuation::Payment(crate::game::payment::state::PaymentDecision::Funding(draft)) =
        &game.pending_decisions[0].continuation
    else {
        panic!("funding menu")
    };
    let offset = game.funding_candidates(draft).unwrap().len();
    let index = game
        .contribution_candidates(draft)
        .unwrap()
        .iter()
        .position(|c| c.source.0 == source && c.symbol == symbol)
        .unwrap();
    answer_payment(game, u32::try_from(offset + index + 1).unwrap());
}

#[test]
fn explicit_payment_convoke_selects_the_symbol_and_keeps_unspent_mana() {
    use crate::game::payment::contributions::PaymentSymbol;
    static ABILITIES: [AbilityDef; 2] = [
        abilities::convoke(),
        AbilityDef::spell(
            "Draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ];
    let (mut game, _) = super::cost_lists::game_with_cost_rules(
        &CardRules::new_instant(mana_cost!("{W/U}{W/B}")).with_abilities(&ABILITIES),
    );
    game.prepared_engine = PreparedEngine::compile(&game.catalog);
    game.battlefield
        .push(creature(90_001, cards::SAVANNAH_LIONS, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    begin_unfunded(&mut game, |a| matches!(a, Action::CastSpell { .. }));
    choose_contribution(
        &mut game,
        90_001,
        PaymentSymbol::Flexible(crate::card::FlexibleManaSymbol::WhiteBlack),
    );
    assert!(!game.battlefield[0].tapped);
    answer_payment(&mut game, 0);
    choose_mana(&mut game, &[ManaColor::Blue]);
    assert!(game.battlefield[0].tapped);
    assert_eq!(game.players[0].mana_pool.white, 1);
    assert_eq!(game.stack.len(), 1);
}

#[test]
fn explicit_payment_improvise_and_delve_pay_generic_without_making_mana() {
    use crate::game::payment::contributions::PaymentSymbol;
    static ABILITIES: [AbilityDef; 3] = [
        abilities::improvise(),
        abilities::delve(),
        AbilityDef::spell(
            "Draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ];
    let (mut game, _) = super::cost_lists::game_with_cost_rules(
        &CardRules::new_instant(mana_cost!("{2}{C}")).with_abilities(&ABILITIES),
    );
    game.prepared_engine = PreparedEngine::compile(&game.catalog);
    game.battlefield
        .push(creature(90_001, cards::SOL_RING, PlayerId::One));
    game.players[0]
        .graveyard
        .push(card(90_002, cards::ISLAND, PlayerId::One));
    begin_unfunded(&mut game, |a| matches!(a, Action::CastSpell { .. }));
    choose_contribution(&mut game, 90_001, PaymentSymbol::Generic);
    choose_contribution(&mut game, 90_002, PaymentSymbol::Generic);
    assert!(
        !game.pending_decisions[0]
            .observation
            .options
            .iter()
            .any(|o| o.id == 0),
        "direct contributions cannot pay true colorless"
    );
    let decision = game.pending_decisions[0].observation.id;
    game.apply(PlayerId::One, Action::CancelDecision { decision })
        .unwrap();
    assert!(!game.battlefield[0].tapped);
    assert_eq!(game.players[0].graveyard.len(), 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    begin_unfunded(&mut game, |a| matches!(a, Action::CastSpell { .. }));
    choose_contribution(&mut game, 90_001, PaymentSymbol::Generic);
    choose_contribution(&mut game, 90_002, PaymentSymbol::Generic);
    answer_payment(&mut game, 0);
    choose_mana(&mut game, &[ManaColor::Colorless]);
    assert!(game.battlefield[0].tapped);
    assert!(game.players[0].graveyard.is_empty());
    assert_eq!(
        game.stack[0]
            .cast
            .as_ref()
            .unwrap()
            .exiled_payment_cards
            .len(),
        1
    );
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

#[test]
fn explicit_payment_non_tapping_mana_ability_can_then_convoke() {
    use crate::game::payment::contributions::PaymentSymbol;
    let mut game = ready_game();
    game.players[0]
        .hand
        .push(card(10_000, cards::SPROUT_SWARM, PlayerId::One));
    for (id, definition) in [
        (10_001, cards::SKIRK_PROSPECTOR),
        (10_002, cards::MOGG_FANATIC),
        (10_003, cards::GRIZZLY_BEARS),
    ] {
        game.battlefield
            .push(creature(id, definition, PlayerId::One));
    }
    begin_unfunded(
        &mut game,
        |a| matches!(a, Action::CastSpell { choices, .. } if choices.costs().additional().is_empty()),
    );
    choose_funding(
        &mut game,
        |a| matches!(a, Action::ActivateManaAbility { cost_object: Some(id), .. } if id.0 == 10_002),
    );
    choose_contribution(&mut game, 10_001, PaymentSymbol::Generic);
    choose_contribution(&mut game, 10_003, PaymentSymbol::Color(ManaColor::Green));
    answer_payment(&mut game, 0);
    choose_mana(&mut game, &[]);
    assert!(game.battlefield.iter().all(|p| p.tapped));
    assert_eq!(game.players[0].mana_pool.red, 1);
    assert_eq!(game.players[0].graveyard.len(), 1);
}
