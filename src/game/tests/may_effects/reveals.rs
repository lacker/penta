use super::*;
use crate::card::ObjectCollectionSourceDef;
use crate::card::RevealObjectsDef;

const REVEAL_TWO: EffectDef = EffectDef::RevealObjects(RevealObjectsDef {
    source: ObjectCollectionSourceDef::TopCards {
        player: PlayerRefDef::EffectController,
        count: ValueDef::Constant(2),
    },
    revealed: Some(Binding!("revealed")),
    then: &EffectDef::move_to_zone(
        EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("revealed"))),
        ZoneKind::Hand,
        ZonePlacement::Top,
    ),
});

fn with_library(game: &mut Game, count: usize) {
    game.players[0].library = game
        .build_zone(PlayerId::One, &vec![cards::FOREST; count])
        .unwrap();
}

#[test]
fn may_effects_counted_reveal_requires_every_card_without_revealing_on_offer() {
    for count in 0..=2 {
        let mut game = with_optional(&REVEAL_TWO, 0);
        with_library(&mut game, count);
        start(&mut game);
        assert!(
            !game
                .events
                .iter()
                .any(|event| matches!(event, GameEvent::CardRevealed { .. }))
        );
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
        assert_eq!(game.is_legal_action(PlayerId::One, &accept), count == 2);
        if count == 2 {
            game.apply(PlayerId::One, accept).unwrap();
            assert_eq!(game.players[0].hand.len(), 2);
            assert!(game.players[0].library.is_empty());
        } else {
            assert!(game.apply(PlayerId::One, accept).is_err());
            choose_decision_by_label(&mut game, PlayerId::One, "Decline");
            assert_eq!(game.players[0].library.len(), count);
            assert!(game.players[0].hand.is_empty());
        }
    }
}

#[test]
fn may_effects_mandatory_reveal_is_partial_and_does_not_abort_following_effects() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
        "Reveal the top two cards and put them into your hand. Gain 3 life.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::Sequence(&[
            REVEAL_TWO,
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )];
    for count in 0..=2 {
        let (mut game, _) = staged(&ABILITIES);
        with_library(&mut game, count);
        start(&mut game);
        assert!(game.players[0].library.is_empty());
        assert_eq!(game.players[0].hand.len(), count);
        assert_eq!(game.players[0].life, 23);
        assert_eq!(
            game.events
                .iter()
                .filter(|event| matches!(event, GameEvent::CardRevealed { .. }))
                .count(),
            count
        );
        assert!(game.pending_decisions.is_empty());
        assert!(!game.players[0].tried_to_draw_from_empty_library);
        assert!(game.result.is_none());
    }
}

#[test]
fn may_effects_allow_zero_count_and_empty_collection_reveals() {
    static ZERO: EffectDef = EffectDef::RevealObjects(RevealObjectsDef {
        source: ObjectCollectionSourceDef::TopCards {
            player: PlayerRefDef::EffectController,
            count: ValueDef::Constant(0),
        },
        revealed: None,
        then: &EffectDef::None,
    });
    static ALL: EffectDef = EffectDef::RevealObjects(RevealObjectsDef {
        source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(
            ObjectQueryDef::owned_by(
                ObjectPredicateDef::Any,
                &[ZoneKind::Hand],
                PlayerSetDef::Related(PlayerRelation::You),
            ),
        )),
        revealed: None,
        then: &EffectDef::None,
    });
    for effect in [&ZERO, &ALL] {
        let mut game = with_optional(effect, 0);
        with_library(&mut game, 0);
        start(&mut game);
        choose_decision_by_label(&mut game, PlayerId::One, "Do it");
        assert!(game.pending_decisions.is_empty());
        assert!(game.result.is_none());
    }
}

#[test]
fn may_effects_counted_reveal_reads_bound_counts_through_sequences() {
    static BOUND: EffectDef = EffectDef::BindValue {
        binding: Binding!("count"),
        value: ValueDef::Constant(2),
        effect: &EffectDef::Sequence(&[
            EffectDef::RevealObjects(RevealObjectsDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::BoundValue(Binding!("count")),
                },
                revealed: None,
                then: &EffectDef::None,
            }),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    };
    for count in 1..=2 {
        let mut game = with_optional(&BOUND, 0);
        with_library(&mut game, count);
        start(&mut game);
        let decision = game.observe(PlayerId::One).decision.unwrap();
        assert_eq!(
            decision.options.iter().any(|option| option.id == 1),
            count == 2
        );
        if count == 2 {
            choose_decision_by_label(&mut game, PlayerId::One, "Do it");
            assert_eq!(game.players[0].life, 21);
        }
    }
}

#[test]
fn may_effects_repeat_stops_before_an_incomplete_counted_reveal() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
        "You may reveal two cards and put them into your hand repeatedly.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::Repeat {
            mandatory_first: false,
            player: EffectRecipientDef::Controller,
            effect: &REVEAL_TWO,
        },
    )];
    let (mut game, _) = staged(&ABILITIES);
    with_library(&mut game, 3);
    start(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Do it");
    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(game.players[0].library.len(), 1);
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn may_effects_repeat_keeps_private_ineligibility_private() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::triggered(
        "You may discard three cards repeatedly.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::Repeat {
            mandatory_first: false,
            player: EffectRecipientDef::Controller,
            effect: &DISCARD,
        },
    )];
    let (mut game, _) = staged(&ABILITIES);
    game.players[0].hand.clear();
    start(&mut game);
    assert!(game.observe(PlayerId::Two).decision.is_none());
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert!(!decision.options.iter().any(|option| option.id == 1));
    choose_decision_by_label(&mut game, PlayerId::One, "Decline");
    assert!(game.pending_decisions.is_empty());
}

#[test]
fn may_effects_reveal_binds_cards_from_sets_and_matching_sources() {
    static MOVE_REVEALED: EffectDef = EffectDef::move_to_zone(
        EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("revealed"))),
        ZoneKind::Hand,
        ZonePlacement::Top,
    );
    static FROM_SET: EffectDef = EffectDef::RevealObjects(RevealObjectsDef {
        source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(
            ObjectQueryDef::owned_by(
                ObjectPredicateDef::Any,
                &[ZoneKind::Library],
                PlayerSetDef::Related(PlayerRelation::You),
            ),
        )),
        revealed: Some(Binding!("revealed")),
        then: &MOVE_REVEALED,
    });
    static THROUGH_CREATURE: EffectDef = EffectDef::RevealObjects(RevealObjectsDef {
        source: ObjectCollectionSourceDef::TopCardsThroughFirstMatching {
            player: PlayerRefDef::EffectController,
            object: ObjectPredicateDef::HasType(CardType::Creature),
        },
        revealed: Some(Binding!("revealed")),
        then: &MOVE_REVEALED,
    });
    for (effect, revealed_count) in [(&FROM_SET, 4), (&THROUGH_CREATURE, 2)] {
        for empty in [false, true] {
            let mut game = with_optional(effect, 0);
            let library = if empty {
                vec![]
            } else {
                vec![
                    cards::FOREST,
                    cards::GRIZZLY_BEARS,
                    cards::SERRA_ANGEL,
                    cards::FOREST,
                ]
            };
            game.players[0].library = game.build_zone(PlayerId::One, &library).unwrap();
            let expected = if empty { 0 } else { revealed_count };
            start(&mut game);
            assert!(
                !game
                    .events
                    .iter()
                    .any(|event| matches!(event, GameEvent::CardRevealed { .. }))
            );
            choose_decision_by_label(&mut game, PlayerId::One, "Do it");
            assert_eq!(game.players[0].hand.len(), expected);
            assert_eq!(game.players[0].library.len(), library.len() - expected);
            assert_eq!(
                game.events
                    .iter()
                    .filter(|event| matches!(event, GameEvent::CardRevealed { .. }))
                    .count(),
                expected,
            );
            if !empty && revealed_count == 2 {
                assert!(
                    game.players[0]
                        .hand
                        .iter()
                        .any(|card| card.definition == cards::SERRA_ANGEL)
                );
                assert!(
                    game.players[0]
                        .hand
                        .iter()
                        .all(|card| card.definition != cards::GRIZZLY_BEARS)
                );
            }
            assert!(game.pending_decisions.is_empty());
        }
    }
}
