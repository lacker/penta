fn parse_cumulative_upkeep_continuation(
    game: &Game,
    object: &super::super::StackObject,
    payer: PlayerId,
    recorded_age: Option<u16>,
    scoped: ScopedEffect,
    cost: crate::card::CostDef,
) -> Result<
    (
        super::super::ResolvedEffectPayment,
        crate::card::ChoiceVisibilityDef,
        Option<ScopedEffect>,
        Option<ScopedEffect>,
    ),
    String,
> {
    let source = object
        .source
        .ok_or("cumulative upkeep decision has no source")?;
    let age = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == source)
        .map_or(0, |permanent| {
            permanent.counters(crate::CounterKind::named("age"))
        });
    if recorded_age != Some(age) || payer != object.controller {
        return Err("cumulative upkeep checkpoint disagrees with the live source".into());
    }
    Ok((
        Game::resolved_cumulative_upkeep_payment(cost, source, age),
        crate::card::ChoiceVisibilityDef::Private,
        None,
        Some(scoped.with_effect(EffectDef::Sacrifice {
            object: crate::card::EffectRecipientDef::Source,
        })),
    ))
}
