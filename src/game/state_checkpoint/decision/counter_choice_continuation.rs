// Reconstructing resolution-time color and counter choices.

fn parse_choose_color_continuation(
    continuation: &EffectContinuationSnapshot,
    targets: &[TargetSnapshot],
    game: &Game,
) -> Result<DecisionContinuation, String> {
    let followup = parse_effect_continuation(continuation, game)?;
    // The operation and duration live on the effect itself, which the
    // locator already found; storing them again would create two authorities.
    let EffectDef::ChooseColor {
        operation,
        duration,
        ..
    } = followup.effect.effect
    else {
        return Err("a color choice located a different effect".to_owned());
    };
    Ok(DecisionContinuation::ChooseColor {
        object: followup.object,
        context: followup.context,
        scoped: followup.effect,
        targets: targets.iter().copied().map(parse_target).collect(),
        operation,
        duration,
    })
}

fn parse_choose_counter_continuation(
    continuation: &EffectContinuationSnapshot,
    target: TargetSnapshot,
    kinds: &[CounterKindSnapshot],
    game: &Game,
) -> Result<DecisionContinuation, String> {
    let followup = parse_effect_continuation(continuation, game)?;
    let EffectDef::ChooseCounterKind { .. } = followup.effect.effect else {
        return Err("a counter choice located a different effect".to_owned());
    };
    Ok(DecisionContinuation::ChooseCounter {
        object: followup.object,
        context: followup.context,
        scoped: followup.effect,
        target: parse_target(target),
        kinds: kinds.iter().map(|kind| kind.0).collect(),
    })
}

fn parse_choose_effect_continuation(
    continuation: &EffectContinuationSnapshot,
    game: &Game,
) -> Result<DecisionContinuation, String> {
    let followup = parse_effect_continuation(continuation, game)?;
    let EffectDef::ChooseEffect { .. } = followup.effect.effect
    else {
        return Err("an effect choice located a different effect".to_owned());
    };
    Ok(DecisionContinuation::ChooseEffect {
        object: followup.object,
        context: followup.context,
        scoped: followup.effect,
    })
}
