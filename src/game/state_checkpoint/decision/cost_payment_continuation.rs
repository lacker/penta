fn parse_cost_payment_continuation(
    game: &Game,
    observation: &DecisionObservation,
    payer: PlayerId,
    continuation: &super::model::EffectContinuationSnapshot,
    answers: &[super::model::PaymentAnswerSnapshot],
    chosen: &[u32],
) -> Result<DecisionContinuation, String> {
    use crate::game::cost_payment::PaymentAnswer;
    use super::model::PaymentAnswerSnapshot;
    let continuation = parse_effect_continuation(continuation, game)?;
    let EffectDef::PayOr(definition) = continuation.effect.effect else {
        return Err("cost-payment locator is not a payment procedure".into());
    };
    if !crate::game::cost_payment::uses_cost_payment_window(definition.payment.cost)
        || game.effect_players(definition.payment.payer, &continuation.object, &continuation.context, continuation.effect) != [payer] {
        return Err("cost-payment payer or grammar disagrees with its authored effect".into());
    }
    let window = crate::game::cost_payment::CostPaymentWindow {
        player: payer, definition: continuation.effect, object: continuation.object, context: continuation.context,
        answers: answers.iter().map(|answer| match answer {
            PaymentAnswerSnapshot::Choice(index) => PaymentAnswer::Choice(*index),
            PaymentAnswerSnapshot::Objects(ids) => PaymentAnswer::Objects(ids.iter().copied().map(GameObjectId).collect()),
        }).collect(),
        chosen: chosen.iter().copied().map(GameObjectId).collect(), committing: None,
    };
    let offer = game.cost_payment_offer(&window).ok_or("cost-payment answers are invalid or unpayable")?;
    if observation.cancellable != offer.cancellable {
        return Err("cost-payment cancellation disagrees with its authored offer".into());
    }
    let mut checked = observation.clone();
    checked.cancellable = false;
    validate_authored_decision(&checked, payer, window.object.ability_text().unwrap_or("Pay the cost?"),
        window.visibility(), DecisionPreference::Neutral, offer.count, offer.count, &offer.options, "cost-payment")?;
    Ok(DecisionContinuation::CostPayment(Box::new(window)))
}
