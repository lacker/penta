fn parse_special_action_continuation(
    game: &Game,
    observation: &DecisionObservation,
    snapshot: &DecisionContinuationSnapshot,
) -> Result<DecisionContinuation, String> {
    let DecisionContinuationSnapshot::PaySpecialAction {
        player: payer,
        source,
        action,
        payment: recorded,
    } = snapshot
    else {
        return Err("expected a special action payment".into());
    };
    let payer = player(*payer)?;
    let source = GameObjectId(*source);
    let payment = game
        .special_action_payment(payer, source, *action)
        .ok_or("special action cost is absent from its source")?;
    if resolved_effect_payment_snapshot(payment.clone()) != *recorded {
        return Err("special action payment disagrees with its authored cost".into());
    }
    let options = game.special_action_payment_options(payer, source, payment.clone());
    if options.len() < 2 {
        return Err("special action payment should have completed automatically".into());
    }
    validate_authored_decision(
        observation,
        payer,
        "Pay the special action cost",
        crate::DecisionVisibility::Private,
        DecisionPreference::Neutral,
        1,
        1,
        &options,
        "special action payment",
    )?;
    Ok(DecisionContinuation::PaySpecialAction {
        player: payer,
        source,
        action: *action,
        payment,
    })
}
