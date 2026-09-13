// Reconstructing resolution-time color, counter, and land-type choices.

fn parse_basic_land_type_substitution_continuation(
    continuation: &EffectContinuationSnapshot,
    observation: &DecisionObservation,
    game: &Game,
) -> Result<DecisionContinuation, String> {
    let followup = parse_effect_continuation(continuation, game)?;
    let EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { chooser } = followup.effect.effect
    else {
        return Err("a land-type substitution located a different effect".into());
    };
    if !ability_locator_matches_origin(&continuation.ability, &followup.object) {
        return Err("land-type substitution locator disagrees with its resolving ability".into());
    }
    let player = game
        .player_reference(chooser, &followup.object, &followup.context, followup.effect)
        .ok_or("land-type substitution has no choosing player")?;
    validate_authored_decision(
        observation,
        player,
        "Each land of the first type becomes the second until end of turn",
        DecisionVisibility::PublicNotice,
        DecisionPreference::Neutral,
        1,
        1,
        &Game::basic_land_type_pair_options(),
        "land-type substitution",
    )?;
    Ok(DecisionContinuation::BasicLandTypeSubstitution {
        object: followup.object,
        context: followup.context,
        effect: followup.effect,
    })
}

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
