use super::rare_states::assert_reconstructs;

#[test]
fn cost_lists_preserve_the_complete_pending_payment_in_checkpoints() {
    let game = crate::game::tests::cost_lists::mixed_echo_pending_game();
    assert!(matches!(
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation),
        Some(crate::game::DecisionContinuation::PayOr {
            payment: crate::game::ResolvedEffectPayment::All(_),
            ..
        })
    ));
    assert_reconstructs(&game, "a mixed mana and discard Echo payment");
}

#[test]
fn cost_lists_reconstruct_pending_special_action_payments() {
    use crate::game::tests::{card, cost_lists::game_with_cost_rules};
    use crate::{AbilityDef, Action, CardRules, CardType, CostDef, ObjectPredicateDef, PlayerId};
    static COSTS: [CostDef; 2] = [
        CostDef::PayLife(2),
        CostDef::discard(ObjectPredicateDef::HasType(CardType::Land)),
    ];
    static ABILITIES: [AbilityDef; 1] = [crate::card::abilities::suspend(
        "Suspend 2—Pay 2 life, discard a land card.",
        &crate::card::SuspendAbilityDef::fixed(2, &COSTS),
    )];
    let (mut game, id) = game_with_cost_rules(
        &CardRules::new_creature(crate::mana_cost!("{9}"), &["Beast"], 1, 1)
            .with_abilities(&ABILITIES),
    );
    game.players[0]
        .hand
        .push(card(230_101, crate::card::cards::SWAMP, PlayerId::One));
    game.players[0]
        .hand
        .push(card(230_102, crate::card::cards::FOREST, PlayerId::One));
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::Suspend { card, .. } if *card == id))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert_reconstructs(&game, "a pending mixed-cost Suspend special action");
}
