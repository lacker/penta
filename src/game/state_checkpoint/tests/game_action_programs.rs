use super::super::*;
use super::rare_states::assert_reconstructs;
use super::true_hidden_hypothesis;
use crate::game::tests::composed_mechanic_programs::{staged, start};
use crate::game::tests::game_action_programs::{
    DISCARD_COST, DISCARD_EFFECT, DRAW_THEN_ACTION, add_hand,
};
use crate::game::tests::{card, choose_decision_by_label, creature};

fn restored(game: &Game) -> Game {
    assert_reconstructs(game, "shared action program continuation");
    let observation = game.observe(PlayerId::One);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    let hidden = true_hidden_hypothesis(game, PlayerId::One);
    Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 4242)
        .unwrap()
}

#[test]
fn game_action_programs_restore_cost_and_effect_selections() {
    let (mut effect, _) = staged(&DISCARD_EFFECT);
    add_hand(&mut effect, 4);
    start(&mut effect);
    let mut effect = restored(&effect);
    let decision = effect.observe(PlayerId::One).decision.unwrap();
    effect
        .apply(
            PlayerId::One,
            crate::Action::ChooseDecision {
                decision: decision.id,
                options: decision
                    .options
                    .iter()
                    .take(3)
                    .map(|option| option.id)
                    .collect(),
            },
        )
        .unwrap();
    assert_eq!(effect.players[0].hand.len(), 1);
    assert_eq!(effect.players[0].graveyard.len(), 3);
    let (mut payment, _) = staged(&DISCARD_COST);
    add_hand(&mut payment, 4);
    start(&mut payment);
    let mut payment = restored(&payment);
    choose_decision_by_label(
        &mut payment,
        PlayerId::One,
        "Discard Island, Island, Island",
    );
    assert_eq!(payment.players[0].life, 25);
    assert_eq!(payment.players[0].graveyard.len(), 3);
}

#[test]
fn game_action_programs_restore_committed_payment_during_replacement() {
    let (mut payment, _) = staged(&DRAW_THEN_ACTION);
    payment
        .battlefield
        .push(creature(31_230, crate::card::cards::ISLAND, PlayerId::One));
    payment
        .put_onto_battlefield(PlayerId::One, crate::card::cards::ISLAND_SANCTUARY)
        .unwrap();
    payment.players[0].library = vec![card(31_231, crate::card::cards::PLAINS, PlayerId::One)];
    start(&mut payment);
    payment.step = crate::game::Step::Draw;
    choose_decision_by_label(
        &mut payment,
        PlayerId::One,
        "Draw 1 card(s), Sacrifice Island",
    );
    assert_eq!(payment.players[0].life, 20);
    assert!(
        payment
            .battlefield
            .iter()
            .any(|p| p.card.id == GameObjectId(31_230))
    );
    let mut payment = restored(&payment);
    choose_decision_by_label(&mut payment, PlayerId::One, "Draw the card");
    assert_eq!(payment.players[0].life, 25);
    assert_eq!(payment.players[0].graveyard.len(), 1);
}
