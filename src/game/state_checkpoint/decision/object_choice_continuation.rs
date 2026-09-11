// Recover fixed object choices from either wrapper's authored program.
#[allow(clippy::too_many_lines)]
fn parse_object_choice_continuation(
    snapshot: &EffectContinuationSnapshot,
    observation: &DecisionObservation,
    game: &Game,
) -> Result<DecisionContinuation, String> {
    let continuation = parse_effect_continuation(snapshot, game)?;
    if !ability_locator_matches_origin(&snapshot.ability, &continuation.object) {
        return Err("object-choice locator disagrees with its resolving ability".into());
    }
    let (state, binding, then, prompt, visibility) = match continuation.effect.effect {
        EffectDef::Perform(action)
            if matches!(action.unnamed(), crate::card::GameActionDef::Choose(_)) =>
        {
            let crate::card::GameActionDef::Choose(choice) = action.unnamed() else {
                unreachable!()
            };
            let fixed = game.fixed_game_action_choice(
                choice,
                &continuation.object,
                &continuation.context,
                continuation.effect,
            );
            let then = EffectDef::Perform(action.selected_action());
            let state = game
                .effect_choice_decision_state_with_continuation(
                    fixed,
                    then,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
                .ok_or("action choice authored chooser is not singular")?;
            if super::super::decision_permanent_choice::effect_choice_resolves_automatically(
                fixed,
                state.candidates.len(),
            ) {
                return Err("action-choice checkpoint encodes an automatic choice".into());
            }
            (
                state,
                fixed.binding,
                then,
                super::super::decision_permanent_choice::effect_choice_prompt(then, fixed.binding),
                effect_choice_visibility(choice.visibility),
            )
        }
        EffectDef::Choose(definition) => {
            let state = game
                .effect_choice_decision_state(
                    definition,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
                .ok_or("object-choice authored chooser is not singular")?;
            if super::super::decision_permanent_choice::effect_choice_resolves_automatically(
                definition,
                state.candidates.len(),
            ) {
                return Err(
                    "object-choice checkpoint encodes a choice that would resolve automatically"
                        .into(),
                );
            }
            (
                state,
                definition.binding,
                *definition.then,
                super::super::decision_permanent_choice::effect_choice_prompt(
                    *definition.then,
                    definition.binding,
                ),
                effect_choice_visibility(definition.visibility),
            )
        }
        EffectDef::ChooseExact(definition) => {
            let (fixed, state) = game
                .exact_effect_choice_decision_state(
                    definition,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
                .ok_or("object-choice authored chooser is not singular")?;
            if super::super::decision_permanent_choice::effect_choice_resolves_automatically(
                fixed,
                state.candidates.len(),
            ) {
                return Err(
                    "object-choice checkpoint encodes a choice that would resolve automatically"
                        .into(),
                );
            }
            let binding = crate::card::ObjectChoiceBindingDef::Objects(definition.binding);
            (
                state,
                binding,
                *definition.then,
                super::super::decision_permanent_choice::effect_choice_prompt(
                    *definition.then,
                    binding,
                ),
                effect_choice_visibility(definition.visibility),
            )
        }
        EffectDef::ChooseCardsFromCollection(definition) => {
            let state = game
                .collection_card_choice_decision_state(
                    definition,
                    &continuation.object,
                    &continuation.context,
                    continuation.effect,
                )
                .ok_or("collection choice authored actor is not singular")?;
            let binding = crate::card::ObjectChoiceBindingDef::Objects(definition.chosen);
            let prompt = if state.candidates.is_empty() {
                "Continue"
            } else {
                super::super::decision_permanent_choice::effect_choice_prompt(
                    *definition.then,
                    binding,
                )
            };
            let visibility = match definition.inspection {
                crate::card::CollectionInspectionDef::Look => DecisionVisibility::Private,
                crate::card::CollectionInspectionDef::Reveal => DecisionVisibility::Public,
            };
            (state, binding, *definition.then, prompt, visibility)
        }
        _ => {
            return Err("object-choice locator does not identify an authored choice".into());
        }
    };
    // An ordered binding makes the live decision carry resolution
    // order semantics, so the rebuilt one has to be allowed to as
    // well; without this, restoring at a Sylvan Library draw-step
    // choice fails closed on the kind alone.
    validate_authored_choice(
        observation,
        state.chooser,
        prompt,
        visibility,
        state.preference,
        state.minimum,
        state.maximum,
        &state.options,
        matches!(
            binding,
            crate::card::ObjectChoiceBindingDef::OrderedObjects(_)
        )
        .then_some(DecisionOrderSemantics::Resolution),
        "object choice",
    )?;
    Ok(DecisionContinuation::ChooseForEffect {
        definition: continuation.effect,
        binding,
        object: continuation.object,
        context: continuation.context,
        candidates: state.candidates,
        effect: continuation.effect.with_effect(then),
    })
}
