use super::composed_mechanic_programs::{staged, start};
use super::*;
use crate::card::{GameActionDef, MechanicId, actions};

const RECLAIM: MechanicId = MechanicId::from_name("test:shared-action");
const REPEATED_COSTS: &[CostDef] = &[
    CostDef::repeated(&[ONE_ACTION.as_cost()], &ValueDef::Constant(2)),
    CostDef::PayLife(1),
];
const PURPOSE: MechanicId = MechanicId::from_name("test:shared-action-payment");
const TWO_CARDS: GameActionDef = actions::choose_exile_from_graveyard(2).named(RECLAIM);
const TWO_CARDS_PAYMENT: EffectDef =
    EffectDef::PayOr(PayOrDef::optional(&[TWO_CARDS.as_cost()], &EffectDef::None).labeled(PURPOSE));
const ONE_ACTION: GameActionDef = actions::choice(&[
    actions::choose_exile_from_graveyard(1),
    actions::choose_sacrifice(1).matching(ObjectPredicateDef::HasType(CardType::Land)),
])
.named(RECLAIM);

fn with_program(effect: EffectDef) -> (Game, GameObjectId) {
    let abilities = Box::leak(Box::new([
        AbilityDef::triggered(
            "Perform the program",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            effect,
        ),
        AbilityDef::triggered(
            "Observe the action",
            TriggerEventDef::MechanicPerformed {
                mechanic: RECLAIM,
                player: PlayerRelation::You,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ]));
    staged(abilities)
}

fn restore(game: &Game) -> Game {
    let (wire, hidden) = checkpoint_fixture(game, PlayerId::One);
    Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 240_000)
        .expect("named action continuation reconstructs from its authored wrapper")
}

fn choose(game: &mut Game, options: Vec<u32>) {
    let decision = game.observe(PlayerId::One).decision.unwrap();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .unwrap();
}

#[test]
fn named_game_action_uses_the_same_program_and_event_through_both_wrappers() {
    for prepared in [false, true] {
        for cost in [false, true] {
            let effect = if cost {
                TWO_CARDS_PAYMENT
            } else {
                TWO_CARDS.as_effect()
            };
            let (mut game, _) = with_program(effect);
            game.set_prepared_engine_enabled(prepared);
            for id in 239_000..239_003 {
                game.players[0]
                    .graveyard
                    .push(card(id, cards::FOREST, PlayerId::One));
            }
            start(&mut game);
            assert_eq!(game.players[0].graveyard.len(), 3);
            game = restore(&game);
            game.set_prepared_engine_enabled(prepared);
            choose(&mut game, if cost { vec![1] } else { vec![0, 1] });
            drain_pending(&mut game);
            assert_eq!(game.players[0].exile.len(), 2);
            assert_eq!(game.players[0].life, 23, "one event for the whole action");
        }
    }
}

#[test]
fn named_game_action_repetitions_can_mix_alternatives_in_a_labeled_bundle() {
    let (mut game, _) = with_program(EffectDef::PayOr(
        PayOrDef::optional(REPEATED_COSTS, &EffectDef::None).labeled(PURPOSE),
    ));
    game.players[0]
        .graveyard
        .push(card(239_010, cards::FOREST, PlayerId::One));
    game.battlefield
        .push(creature(239_011, cards::ISLAND, PlayerId::One));
    start(&mut game);
    game = restore(&game);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    let complete = decision
        .options
        .iter()
        .find(|option| option.id != 0)
        .unwrap()
        .id;
    assert_eq!(game.players[0].life, 20, "selection has no side effects");
    choose(&mut game, vec![complete]);
    drain_pending(&mut game);
    assert_eq!(
        game.players[0].life, 25,
        "pay one life, then observe two occurrences"
    );
    assert_eq!(game.players[0].exile.len(), 1);
    assert_eq!(game.players[0].graveyard.len(), 1);
}

#[test]
fn named_game_action_bundle_reserves_objects_before_spending_anything() {
    let (mut game, _) = with_program(EffectDef::PayOr(PayOrDef::optional(
        REPEATED_COSTS,
        &EffectDef::None,
    )));
    game.players[0]
        .graveyard
        .push(card(239_020, cards::FOREST, PlayerId::One));
    start(&mut game);
    let decision = game.observe(PlayerId::One).decision.unwrap();
    assert_eq!(
        decision.options.len(),
        1,
        "one card cannot pay both action repetitions"
    );
    choose(&mut game, vec![0]);
    drain_pending(&mut game);
    assert_eq!(game.players[0].graveyard.len(), 1);
    assert!(game.players[0].exile.is_empty());
    assert_eq!(game.players[0].life, 20);
}
