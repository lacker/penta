use super::composed_mechanic_programs::{staged, start};
use super::game_action_programs::add_hand;
use super::*;
use crate::card::{DiscardSelectionDef, actions};

const DISCARD: EffectDef = EffectDef::Discard {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::Constant(3),
    selection: DiscardSelectionDef::RecipientChooses,
    then: None,
};
const ACTION_DISCARD: EffectDef = actions::choose_discard(3).as_effect();
const EXACT_DISCARD: EffectDef = EffectDef::ChooseExact(crate::card::ChooseExactDef {
    binding: Binding!("discarded"),
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
        ObjectPredicateDef::Any,
        &[ZoneKind::Hand],
        PlayerSetDef::Related(PlayerRelation::You),
    )),
    exclude: None,
    amount: ValueDef::Sum(&crate::card::SumValueDef {
        left: ValueDef::Constant(1),
        right: ValueDef::Constant(2),
    }),
    visibility: ChoiceVisibilityDef::Private,
    then: &EffectDef::discard_cards(EffectRecipientDef::objects(ObjectSetDef::Binding(
        Binding!("discarded"),
    ))),
});

fn with_optional(effect: &'static EffectDef, hand: u32) -> Game {
    let abilities = Box::leak(Box::new([AbilityDef::triggered(
        "You may perform the action.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect,
        },
    )]));
    let (mut game, _) = staged(abilities);
    add_hand(&mut game, hand);
    game
}

#[test]
fn may_effects_require_the_full_discard_and_preserve_the_decline_after_restore() {
    for effect in [&DISCARD, &ACTION_DISCARD, &EXACT_DISCARD] {
        for hand in 0..=3 {
            let mut game = with_optional(effect, hand);
            start(&mut game);
            let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
            let mut game = Game::from_observation_checkpoint(
                game.catalog.clone(),
                game.format,
                &wire,
                &hidden,
                31_400,
            )
            .unwrap();
            let decision = game.observe(PlayerId::One).decision.unwrap();
            let accept = Action::ChooseDecision {
                decision: decision.id,
                options: vec![1],
            };
            assert_eq!(game.is_legal_action(PlayerId::One, &accept), hand == 3);
            if hand < 3 {
                assert!(game.apply(PlayerId::One, accept).is_err());
                assert_eq!(game.players[0].hand.len(), hand as usize);
                assert!(game.players[0].graveyard.is_empty());
                choose_decision_by_label(&mut game, PlayerId::One, "Decline");
            } else {
                game.apply(PlayerId::One, accept).unwrap();
                assert!(game.players[0].hand.is_empty());
                assert_eq!(game.players[0].graveyard.len(), 3);
            }
        }
    }
}

#[test]
fn may_effects_require_a_complete_action_alternative() {
    static CHOICE: EffectDef = actions::choice(&[
        actions::choose_exile_from_graveyard(2),
        actions::choose_sacrifice(1).matching(ObjectPredicateDef::HasType(CardType::Land)),
    ])
    .as_effect();
    for graveyard in 0..=2 {
        let mut game = with_optional(&CHOICE, 0);
        game.players[0].graveyard = (0..graveyard)
            .map(|offset| card(31_410 + offset, cards::ISLAND, PlayerId::One))
            .collect();
        start(&mut game);
        let decision = game.observe(PlayerId::One).decision.unwrap();
        assert_eq!(
            decision.options.iter().any(|option| option.id == 1),
            graveyard == 2
        );
    }
}

#[test]
fn may_effects_allow_drawing_from_an_empty_library() {
    static DRAW: EffectDef = EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };
    let mut game = with_optional(&DRAW, 0);
    game.players[0].library.clear();
    start(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Do it");
    assert!(matches!(
        game.result,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            ..
        })
    ));
}

#[test]
fn may_effects_check_selections_after_prior_instructions() {
    static DRAW_THEN_DISCARD: EffectDef = EffectDef::Sequence(&[
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
        DISCARD,
    ]);
    let mut game = with_optional(&DRAW_THEN_DISCARD, 2);
    start(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Do it");
    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.players[0].graveyard.len(), 3);
}

#[test]
fn may_effects_keep_filtered_hand_eligibility_private() {
    static DISCARD_CREATURES: EffectDef = EffectDef::Discard {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
        selection: DiscardSelectionDef::RandomMatching(&ObjectPredicateDef::HasType(
            CardType::Creature,
        )),
        then: None,
    };
    for creatures in 0..=2 {
        let mut game = with_optional(&DISCARD_CREATURES, 3);
        game.players[0].hand.extend(
            (0..creatures).map(|offset| card(31_420 + offset, cards::GRIZZLY_BEARS, PlayerId::One)),
        );
        start(&mut game);
        let decision = game.observe(PlayerId::One).decision.unwrap();
        assert_eq!(
            decision.options.iter().any(|option| option.id == 1),
            creatures == 2
        );
        assert!(game.observe(PlayerId::Two).decision.is_none());
        if creatures == 2 {
            choose_decision_by_label(&mut game, PlayerId::One, "Do it");
            assert_eq!(game.players[0].hand.len(), 3);
            assert!(
                game.players[0]
                    .graveyard
                    .iter()
                    .all(|card| card.definition == cards::GRIZZLY_BEARS)
            );
        }
    }
}

#[test]
fn may_effects_allow_a_zero_card_selection() {
    static ZERO: EffectDef = actions::choose_discard(0).as_effect();
    let mut game = with_optional(&ZERO, 0);
    start(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Do it");
    assert!(game.pending_decisions.is_empty());
    assert!(game.players[0].graveyard.is_empty());
}
