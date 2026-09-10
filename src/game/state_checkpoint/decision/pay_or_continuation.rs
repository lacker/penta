fn parse_authored_pay_or_continuation(
    game: &Game,
    object: &super::super::StackObject,
    context: &super::super::EffectResolutionContext,
    payer: PlayerId,
    payment_provenance: Option<&String>,
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
    // The payer was settled when the decision was queued, and the state it
    // was read from can have moved since: Chain of Vapor asks the controller
    // of a permanent it has already returned to hand. A payer that can no
    // longer be derived is the recorded one, while a payer that derives to
    // somebody else is a disagreement.
    let (resolved, provenance) = game.resolve_payment_offer(authored, object, context, scoped);
    if provenance.map(|p| p.label.to_string()).as_ref() != payment_provenance {
        return Err("payment provenance disagrees with its authored program".into());
    }
    let payers = game.effect_players(authored.payment.payer, object, context, scoped);
    let authored_payment = match payers.as_slice() {
        [payer] => Some((*payer, resolved.clone())),
        _ => None,
    };
    let payment = match authored_payment {
        Some((expected_payer, payment)) if expected_payer == payer => payment,
        Some(_) => {
            return Err("pay-or payer or payment disagrees with its authored effect".into());
        }
        None => resolved,
    };
    Ok((
        payment,
        authored.visibility,
        authored.if_paid.map(|effect| scoped.with_effect(*effect)),
        authored.otherwise.map(|effect| scoped.with_effect(*effect)),
    ))
}
