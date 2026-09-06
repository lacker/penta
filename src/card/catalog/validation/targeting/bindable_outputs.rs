fn has_bindable_output(effect: EffectDef) -> Result<bool, GrantedAbilityValidationError> {
    match effect {
        EffectDef::Mill { .. }
        | EffectDef::MillUntil(_)
        | EffectDef::SelectAtRandomFromZone { .. }
        | EffectDef::RevealAtRandomFromHand { .. }
        | EffectDef::ChooseCardName { .. } => Ok(true),
        EffectDef::IfCondition { then, .. } => has_bindable_output(*then),
        EffectDef::IfFormat {
            then, otherwise, ..
        }
        | EffectDef::Randomized {
            on_success: then,
            on_failure: otherwise,
            ..
        }
        | EffectDef::FlipCoin {
            on_win: then,
            on_loss: otherwise,
        } => Ok(has_bindable_output(*then)? || has_bindable_output(*otherwise)?),
        EffectDef::None => Ok(false),
        _ => Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
            context: "bound effect output",
            operation: "an effect that does not expose an output",
        }),
    }
}

fn durable_object_set_outputs(effect: EffectDef, outputs: &mut Vec<Binding>) {
    let mut push = |binding: Binding| {
        if binding != crate::ParentBinding && !outputs.contains(&binding) {
            outputs.push(binding);
        }
    };
    match effect {
        EffectDef::BindOutput {
            effect: &EffectDef::ChooseCardName { .. },
            ..
        } => {}
        EffectDef::BindOutput { binding, .. } => push(binding),
        EffectDef::WithZoneMoveResult { binding, then, .. } => {
            push(binding);
            durable_object_set_outputs(*then, outputs);
        }
        EffectDef::Sequence(effects) => {
            for effect in effects {
                durable_object_set_outputs(*effect, outputs);
            }
        }
        EffectDef::Choose(choice) => {
            match choice.binding {
                crate::card::ObjectChoiceBindingDef::Object(_) => {}
                crate::card::ObjectChoiceBindingDef::Objects(binding)
                | crate::card::ObjectChoiceBindingDef::OrderedObjects(binding) => push(binding),
            }
            if let Some(binding) = choice.unchosen {
                push(binding);
            }
            durable_object_set_outputs(*choice.then, outputs);
        }
        EffectDef::ChooseExact(choice) => {
            push(choice.binding);
            durable_object_set_outputs(*choice.then, outputs);
        }
        EffectDef::ChooseCardsFromCollection(choice) => {
            push(choice.chosen);
            push(choice.remainder);
            durable_object_set_outputs(*choice.then, outputs);
        }
        EffectDef::BindObjects(definition) => {
            push(definition.binding);
            durable_object_set_outputs(*definition.then, outputs);
        }
        EffectDef::ClassifyObjects(definition) => {
            push(definition.matching);
            push(definition.remainder);
            durable_object_set_outputs(*definition.then, outputs);
        }
        EffectDef::RevealAndClassifyCards(definition) => {
            push(definition.matching);
            push(definition.remainder);
            durable_object_set_outputs(*definition.then, outputs);
        }
        EffectDef::CombineObjects(definition) => {
            push(definition.combined);
            durable_object_set_outputs(*definition.then, outputs);
        }
        EffectDef::RandomizeObjectOrder(definition) => {
            push(definition.randomized);
            durable_object_set_outputs(*definition.then, outputs);
        }
        EffectDef::MoveObjects(definition) => {
            if let Some(binding) = definition.moved {
                push(binding);
            }
            durable_object_set_outputs(*definition.then, outputs);
        }
        EffectDef::PutObjectsOntoBattlefieldFaceDown(definition) => {
            if let Some(binding) = definition.moved {
                push(binding);
            }
            durable_object_set_outputs(*definition.then, outputs);
        }
        EffectDef::ChooseObjectOrder(definition) => {
            push(definition.ordered);
            durable_object_set_outputs(*definition.then, outputs);
        }
        EffectDef::PartitionGroup(definition) => {
            push(definition.first);
            push(definition.second);
            durable_object_set_outputs(*definition.then, outputs);
        }
        EffectDef::ChooseGroup(definition) => {
            push(definition.chosen);
            push(definition.unchosen);
            durable_object_set_outputs(*definition.then, outputs);
        }
        EffectDef::ChooseOneOfEach(definition) => {
            push(definition.chosen);
            push(definition.remainder);
            durable_object_set_outputs(*definition.then, outputs);
        }
        EffectDef::ChooseForEachPlayer(definition) => {
            push(definition.chosen);
            push(definition.unchosen);
            durable_object_set_outputs(*definition.then, outputs);
        }
        _ => {}
    }
}

fn durable_card_name_outputs(effect: EffectDef, outputs: &mut Vec<Binding>) {
    let mut push = |binding: Binding| {
        if binding != crate::ParentBinding && !outputs.contains(&binding) {
            outputs.push(binding);
        }
    };
    match effect {
        EffectDef::BindOutput {
            binding,
            effect: &EffectDef::ChooseCardName { .. },
        } => push(binding),
        EffectDef::Sequence(effects) => {
            for effect in effects {
                durable_card_name_outputs(*effect, outputs);
            }
        }
        _ => {}
    }
}

fn scope_after_sequence_effect(
    effect: EffectDef,
    mut scope: BindingScope<'_>,
) -> Result<BindingScope<'_>, GrantedAbilityValidationError> {
    let mut outputs = Vec::new();
    durable_object_set_outputs(effect, &mut outputs);
    for binding in outputs {
        scope = scope.with_declared_object_set(binding)?;
    }
    let mut name_outputs = Vec::new();
    durable_card_name_outputs(effect, &mut name_outputs);
    for binding in name_outputs {
        scope = scope.with_declared_card_name(binding)?;
    }
    Ok(scope)
}
