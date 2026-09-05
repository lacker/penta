pub(super) fn effect_resolution_context_snapshot(
    context: &EffectResolutionContext,
) -> EffectResolutionContextSnapshot {
    let binding_values = context.bindings();
    EffectResolutionContextSnapshot {
        trigger: trigger_context_snapshot(context.trigger),
        replaced_draw: context.replaced_draw.as_ref().map(replaced_draw_snapshot),
        chosen_counter: context.chosen_counter.map(CounterKindSnapshot),
        parent_object: context.parent_object().map(target_snapshot),
        parent_objects: context
            .parent_objects()
            .iter()
            .copied()
            .map(target_snapshot)
            .collect(),
        bindings: binding_values
            .iter()
            .filter_map(|(label, binding)| {
                let binding = match binding {
                    EffectBindingValue::Object(object) => Some(EffectBindingSnapshot::Object {
                        object: object.map(target_snapshot),
                    }),
                    EffectBindingValue::Objects(objects) => Some(EffectBindingSnapshot::Objects {
                        objects: objects.iter().copied().map(target_snapshot).collect(),
                    }),
                    EffectBindingValue::CardName(_) => None,
                };
                Some((label.clone(), binding?))
            })
            .collect(),
        card_name_bindings: binding_values
            .into_iter()
            .filter_map(|(label, binding)| match binding {
                EffectBindingValue::CardName(name) => Some((label, name)),
                EffectBindingValue::Object(_) | EffectBindingValue::Objects(_) => None,
            })
            .collect(),
    }
}

pub(super) fn parse_effect_resolution_context(
    value: EffectResolutionContextSnapshot,
) -> Result<EffectResolutionContext, String> {
    let mut bindings = value
        .bindings
        .into_iter()
        .map(|(label, binding)| {
            let binding = match binding {
                EffectBindingSnapshot::Object { object } => {
                    EffectBindingValue::Object(object.map(parse_target))
                }
                EffectBindingSnapshot::Objects { objects } => {
                    EffectBindingValue::Objects(objects.into_iter().map(parse_target).collect())
                }
            };
            (label, binding)
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    for (label, name) in value.card_name_bindings {
        if bindings
            .insert(label.clone(), EffectBindingValue::CardName(name))
            .is_some()
        {
            return Err(format!(
                "effect binding {label:?} has more than one value kind"
            ));
        }
    }
    let mut context = EffectResolutionContext::from_bindings(
        parse_trigger_context(value.trigger)?,
        value.parent_object.map(parse_target),
        value.parent_objects.into_iter().map(parse_target).collect(),
        bindings,
    );
    context.replaced_draw = value.replaced_draw.map(parse_replaced_draw).transpose()?;
    context.chosen_counter = value.chosen_counter.map(|kind| kind.0);
    Ok(context)
}
