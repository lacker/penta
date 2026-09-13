// Casting choices and payment facts carried by a battlefield object.
// Included into wire.rs to share its checkpoint conversion vocabulary.
fn restore_permanent_cast_context(
    permanent: &mut Permanent,
    state: &PermanentSnapshot,
    catalog: &CardCatalog,
) -> Result<(), String> {
    if state.cast_tags.iter().any(|tag| tag != "escaped") {
        return Err("checkpoint contains an unknown retired cast tag".into());
    }
    let source_zone = state
        .cast_from_zone
        .as_deref()
        .and_then(cast_source_zone_from_label);
    let alternative = state
        .cast_alternative
        .as_deref()
        .map(|label| {
            crate::card::AlternativeCastKindDef::from_label(label)
                .ok_or_else(|| format!("unknown alternative cast kind {label}"))
        })
        .transpose()?
        .or_else(|| {
            state
                .cast_tags
                .iter()
                .any(|tag| tag == "escaped")
                .then_some(crate::card::AlternativeCastKindDef::Escape)
        });
    let colors_of_mana_spent = if state.cast_colors_of_mana_spent.iter().any(|spent| *spent) {
        stack::color_set_from_flags(state.cast_colors_of_mana_spent)
    } else {
        color_set_from_count(state.cast_colors)
    };
    let has_cast_context = source_zone.is_some()
        || alternative.is_some()
        || !state.cast_player_bindings.is_empty()
        || state.cast_alternative_cost_binding.is_some()
        || state.cast_x > 0
        || state.cast_kicks > 0
        || !state.cast_additional_costs.is_empty()
        || state.cast_colors > 0
        || state.cast_phyrexian_symbols_paid_with_life > 0
        || !state.cast_exiled_payment_cards.is_empty()
        || state.cast_via_flashback
        || state.cast_exile_if_put_into_graveyard
        || state.cast_via_suspend
        || state.cast_at_instant_speed;
    let alternative_cost_binding = restore_alternative_cost_binding(
        state.cast_alternative_cost_binding.as_deref(),
        permanent.card.definition.card_definition(),
        catalog,
    )?;
    let player_bindings = super::cast_bindings::restore_player_bindings(
        &state.cast_player_bindings,
        permanent.card.definition.card_definition(),
        catalog,
    )?;
    let caster = state.cast_by.map(player_from_index).transpose()?;
    let permission_entry_counters =
        super::stack::parse_permission_entry_counters(&state.permission_entry_counters)?;
    permanent.cast = has_cast_context.then(|| CastContext {
        caster,
        source_zone,
        alternative,
        alternative_cost_binding,
        player_bindings,
        at_instant_speed: state.cast_at_instant_speed,
        x: state.cast_x,
        repeatable_additional_costs: state.cast_kicks,
        additional_costs: state.cast_additional_costs.clone(),
        colors_of_mana_spent,
        phyrexian_symbols_paid_with_life: state.cast_phyrexian_symbols_paid_with_life,
        exiled_payment_cards: state
            .cast_exiled_payment_cards
            .iter()
            .copied()
            .map(GameObjectId)
            .collect(),
        via_flashback: state.cast_via_flashback,
        exile_if_put_into_graveyard: state.cast_exile_if_put_into_graveyard,
        via_suspend: state.cast_via_suspend,
        permission_entry_counters,
    });
    Ok(())
}
