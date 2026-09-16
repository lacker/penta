pub(super) fn effect_resolution_context_snapshot(
    context: &EffectResolutionContext,
) -> EffectResolutionContextSnapshot {
    let binding_values = context.bindings();
    EffectResolutionContextSnapshot {
        inspected_objects: context
            .inspected_objects()
            .into_iter()
            .map(|(player, id)| (player.index(), id.0))
            .collect(),
        trigger: trigger_context_snapshot(context.trigger),
        source_transform_count: context.source_transform_count,
        paid_amount: context.paid_amount,
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
                    EffectBindingValue::CardName(_) | EffectBindingValue::Number(_) | EffectBindingValue::CreatureType(_) => None,
                };
                Some((label.clone(), binding?))
            })
            .collect(),
        number_bindings: binding_values
            .iter()
            .filter_map(|(label, binding)| match binding {
                EffectBindingValue::Number(value) => Some((label.clone(), *value)),
                _ => None,
            })
            .collect(),
        creature_type_bindings: binding_values.iter().filter_map(|(label, value)| match value {
            EffectBindingValue::CreatureType(name) => Some((label.clone(), name.clone())),
            _ => None,
        }).collect(),
        card_name_bindings: binding_values
            .into_iter()
            .filter_map(|(label, binding)| match binding {
                EffectBindingValue::CardName(name) => Some((label, name)),
                EffectBindingValue::Object(_)
                | EffectBindingValue::Objects(_)
                | EffectBindingValue::Number(_) | EffectBindingValue::CreatureType(_) => None,
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
    for (label, number) in value.number_bindings {
        if bindings
            .insert(label.clone(), EffectBindingValue::Number(number))
            .is_some()
        {
            return Err(format!(
                "effect binding {label:?} has more than one value kind"
            ));
        }
    }
    for (label, name) in value.creature_type_bindings {
        if crate::card::Subtype::from_name(&name).is_none() { return Err("invalid creature type binding".into()); }
        if bindings.insert(label.clone(), EffectBindingValue::CreatureType(name)).is_some() {
            return Err(format!("effect binding {label:?} has more than one value kind"));
        }
    }
    let mut context = EffectResolutionContext::from_bindings(
        parse_trigger_context(value.trigger)?,
        value.parent_object.map(parse_target),
        value.parent_objects.into_iter().map(parse_target).collect(),
        bindings,
    );
    for (player, id) in value.inspected_objects {
        context.remember_inspected_objects(
            player_from_index(player).ok_or("invalid inspected-object viewer")?,
            &[GameObjectId(id)],
        );
    }
    context.replaced_draw = value.replaced_draw.map(parse_replaced_draw).transpose()?;
    context.chosen_counter = value.chosen_counter.map(|kind| kind.0);
    context.paid_amount = value.paid_amount;
    context.source_transform_count = value.source_transform_count;
    Ok(context)
}
