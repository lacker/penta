fn parse_authored_pay_or_continuation(
    game: &Game,
    object: &super::super::StackObject,
    context: &super::super::EffectResolutionContext,
    payer: PlayerId,
    cumulative_upkeep_age: Option<u16>,
    scoped: ScopedEffect,
    authored: crate::card::PayOrDef,
) -> Result<
    (
        super::super::ResolvedEffectPayment,
        crate::card::ChoiceVisibilityDef,
        Option<ScopedEffect>,
        Option<ScopedEffect>,
    ),
    String,
> {
    if cumulative_upkeep_age.is_some() {
        return Err("ordinary pay-or checkpoint carries cumulative upkeep state".into());
    }
    // The payer was settled when the decision was queued, and the state it
    // was read from can have moved since: Chain of Vapor asks the controller
    // of a permanent it has already returned to hand. A payer that can no
    // longer be derived is the recorded one, while a payer that derives to
    // somebody else is a disagreement.
    let authored_payment =
        resolved_effect_payment(game, authored.payment, object, context, scoped);
    let payment = match authored_payment {
        Some((expected_payer, payment)) if expected_payer == payer => payment,
        Some(_) => {
            return Err("pay-or payer or payment disagrees with its authored effect".into());
        }
        None => resolved_effect_payment_for_payer(
            game,
            authored.payment,
            object,
            context,
            scoped,
        )
        .ok_or("pay-or authored payment cannot be rebuilt")?,
    };
    Ok((
        payment,
        authored.visibility,
        authored.if_paid.map(|effect| scoped.with_effect(*effect)),
        authored.otherwise.map(|effect| scoped.with_effect(*effect)),
    ))
}
