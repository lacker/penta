//! Discarding chosen cards as casting and activation costs.
//!
//! The card travels with the activation rather than being a mid-payment
//! decision, so the enumerator offers one action per discardable card and an
//! empty hand offers none at all.

use super::*;

#[test]
fn casting_discard_quantity_spends_exactly_that_many_matching_cards() {
    use crate::card::CostQuantityDef;

    for quantity in [CostQuantityDef::Fixed(2), CostQuantityDef::ChosenX] {
        for x in [1, 2] {
            let (mut game, spell) = super::cost_lists::game_with_cost_rules(
                &CardRules::new_sorcery(mana_cost!("{X}{B}")).with_ability(
                    AbilityDef::spell_with_additional_cost(
                        "Discard matching cards as an additional cost.",
                        &[],
                        CostDef::discard(ObjectPredicateDef::HasType(CardType::Land))
                            .with_quantity(quantity),
                        EffectDef::None,
                    ),
                ),
            );
            let lands = [GameObjectId(231_010), GameObjectId(231_011)];
            for id in lands {
                game.players[0]
                    .hand
                    .push(card(id.0, cards::SWAMP, PlayerId::One));
            }
            let nonland = GameObjectId(231_012);
            game.players[0]
                .hand
                .push(card(nonland.0, cards::GRIZZLY_BEARS, PlayerId::One));
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
            let expected = match quantity {
                CostQuantityDef::Fixed(amount) => usize::from(amount),
                _ => usize::from(x),
            };
            let casts = game
                .legal_actions(PlayerId::One)
                .into_iter()
                .filter(|action| {
                    matches!(action, Action::CastSpell { card, choices, .. }
                    if *card == spell && choices.x() == x)
                })
                .collect::<Vec<_>>();
            assert_eq!(casts.len(), if expected == 1 { 2 } else { 1 });
            for action in &casts {
                let Action::CastSpell { sacrifices, .. } = action else {
                    unreachable!()
                };
                assert_eq!(sacrifices.len(), expected);
                assert!(sacrifices.iter().all(|id| lands.contains(id)));
            }
            game.apply(PlayerId::One, casts[0].clone())
                .expect("the matching cards pay the complete cost");
            assert_eq!(game.stack.len(), 1);
            assert_eq!(game.players[0].graveyard.len(), expected);
            assert_eq!(game.players[0].hand.len(), 3 - expected);
            assert!(game.players[0].hand.iter().any(|card| card.id == nonland));
        }
    }
}

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game
}

fn activations(game: &Game, source: GameObjectId) -> Vec<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| {
            matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source)
        })
        .collect()
}

#[test]
fn unsupported_discard_quantities_offer_no_activation() {
    static COSTS: [[CostDef; 1]; 3] = [
        [CostDef::discard(ObjectPredicateDef::Any)
            .with_quantity(crate::card::CostQuantityDef::Fixed(0))],
        [CostDef::discard(ObjectPredicateDef::Any)
            .with_quantity(crate::card::CostQuantityDef::Fixed(2))],
        [CostDef::discard(ObjectPredicateDef::Any)
            .with_quantity(crate::card::CostQuantityDef::ChosenX)],
    ];
    for costs in &COSTS {
        let (mut game, source) = super::cost_lists::game_with_cost_rules(
            &CardRules::new_creature(mana_cost!("{1}"), &["Human"], 1, 1).with_ability(
                AbilityDef::activated(
                    "Discard cards: You gain 1 life.",
                    costs,
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ),
            ),
        );
        let definition = game.players[0].hand.remove(0).definition;
        game.battlefield
            .push(creature(source.0, definition, PlayerId::One));
        for index in 0..3 {
            game.players[0]
                .hand
                .push(card(231_000 + index, cards::SWAMP, PlayerId::One));
        }
        assert!(
            activations(&game, source).is_empty(),
            "unsupported quantities must not be treated as one card: {costs:?}"
        );
    }
}

/// One activation per card in hand, because each is a different cost.
#[test]
fn the_prophet_offers_one_activation_per_discardable_card() {
    let mut game = ready();
    let prophet = creature(10_000, cards::MAD_PROPHET, PlayerId::One);
    let prophet_id = prophet.card.id;
    game.battlefield.push(prophet);

    assert!(
        activations(&game, prophet_id).is_empty(),
        "an empty hand cannot pay",
    );

    for index in 0..3 {
        game.players[PlayerId::One.index()].hand.push(card(
            30_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    assert_eq!(
        activations(&game, prophet_id).len(),
        3,
        "one per card in hand",
    );
}

/// Paying it discards the chosen card and draws, leaving the hand the same
/// size but one card different.
#[test]
fn paying_the_cost_loots() {
    let mut game = ready();
    let prophet = creature(10_000, cards::MAD_PROPHET, PlayerId::One);
    let prophet_id = prophet.card.id;
    game.battlefield.push(prophet);
    game.players[PlayerId::One.index()].hand.push(card(
        30_000,
        cards::GRIZZLY_BEARS,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].library.push(card(
        31_000,
        cards::AIR_ELEMENTAL,
        PlayerId::One,
    ));

    let action = activations(&game, prophet_id)
        .into_iter()
        .next()
        .expect("one card, one activation");
    game.apply(PlayerId::One, action).expect("legal");
    drain_pending(&mut game);

    let hand = &game.players[PlayerId::One.index()].hand;
    assert_eq!(hand.len(), 1, "one out, one in");
    assert_eq!(
        hand[0].definition,
        cards::AIR_ELEMENTAL,
        "the drawn card, not the discarded one",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].graveyard.len(),
        1,
        "and the cost went to the graveyard",
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == prophet_id)
            .expect("still there")
            .tapped,
        "the tap was part of the cost too",
    );
}

/// The Market grants the same ability to a land, which taps for it.
#[test]
fn the_market_lets_a_land_loot() {
    let mut game = ready();
    game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("cataloged");
    let land = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MOUNTAIN)
        .expect("it is there")
        .card
        .id;
    let mut aura = creature(10_000, cards::TIN_STREET_MARKET, PlayerId::One);
    aura.attached_to = Some(land);
    game.battlefield.push(aura);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    assert!(
        activations(&game, land).is_empty(),
        "nothing in hand to pay with",
    );

    game.players[PlayerId::One.index()].hand.push(card(
        30_000,
        cards::GRIZZLY_BEARS,
        PlayerId::One,
    ));
    assert_eq!(
        activations(&game, land).len(),
        1,
        "a card in hand makes the land a looter",
    );
}
