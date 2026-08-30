//! Reconstruction of the cast facts carried by a detached spell.

use super::{
    AlternativeCastKindDef, CastContext, CastSignature, DetachedStackSnapshot, Game, GameObjectId,
    ObjectInstance, StackObjectKind, StackObjectKindSnapshot, StackSnapshot,
    cast_source_zone_from_label, color_set_from_flags,
};

pub(super) fn stack_cast_context(
    state: &StackSnapshot,
    game: &Game,
    id: GameObjectId,
    card: &ObjectInstance,
    signature: Option<&CastSignature>,
    kind: StackObjectKind,
) -> Result<Option<CastContext>, String> {
    if state.cast_tags.iter().any(|tag| tag != "escaped") {
        return Err("checkpoint contains an unknown retired cast tag".into());
    }
    let alternative = state
        .cast_alternative
        .as_deref()
        .map(|label| {
            AlternativeCastKindDef::from_label(label)
                .ok_or_else(|| format!("unknown alternative cast kind {label}"))
        })
        .transpose()?
        .or_else(|| {
            state
                .cast_tags
                .iter()
                .any(|tag| tag == "escaped")
                .then_some(AlternativeCastKindDef::Escape)
        })
        .or_else(|| {
            let signature = signature?;
            let definition = card.definition.card_definition()?;
            let definition = game.catalog.get(definition)?;
            let option = definition.play_option(signature.play_option())?;
            game.selected_alternative_kind(definition, option, id, signature.costs())
        });
    let signature_cost_counts = signature.and_then(|signature| {
        let definition = card.definition.card_definition()?;
        let option = game
            .catalog
            .get(definition)?
            .play_option(signature.play_option())?;
        Some((
            Game::repeatable_additional_cost_payments_for(option, signature.costs()),
            Game::additional_cost_payment_counts_for(option, signature.costs()),
        ))
    });
    Ok((kind == StackObjectKind::Spell).then(|| CastContext {
        source_zone: state
            .cast_from_zone
            .as_deref()
            .and_then(cast_source_zone_from_label),
        alternative,
        at_instant_speed: state.cast_at_instant_speed,
        x: if state.cast_x == 0 {
            signature.map_or(0, CastSignature::x)
        } else {
            state.cast_x
        },
        repeatable_additional_costs: if state.cast_repeatable_additional_costs == 0 {
            signature_cost_counts.as_ref().map_or(0, |counts| counts.0)
        } else {
            state.cast_repeatable_additional_costs
        },
        additional_costs: if state.cast_additional_costs.is_empty() {
            signature_cost_counts.map_or_else(Vec::new, |counts| counts.1)
        } else {
            state.cast_additional_costs.clone()
        },
        colors_of_mana_spent: color_set_from_flags(state.colors_of_mana_spent),
        phyrexian_symbols_paid_with_life: state.phyrexian_symbols_paid_with_life,
        exiled_payment_cards: state
            .cast_exiled_payment_cards
            .iter()
            .copied()
            .map(GameObjectId)
            .collect(),
        via_flashback: state.cast_via_flashback,
        via_suspend: state.cast_via_suspend,
    }))
}

pub(super) fn detached_cast_context(
    state: &DetachedStackSnapshot,
    game: &Game,
    id: GameObjectId,
    card: &ObjectInstance,
    signature: Option<&CastSignature>,
) -> Result<Option<CastContext>, String> {
    if state.cast_tags.iter().any(|tag| tag != "escaped") {
        return Err("checkpoint contains an unknown retired cast tag".into());
    }
    let alternative = state
        .cast_alternative
        .as_deref()
        .map(|label| {
            AlternativeCastKindDef::from_label(label)
                .ok_or_else(|| format!("unknown alternative cast kind {label}"))
        })
        .transpose()?
        .or_else(|| {
            state
                .cast_tags
                .iter()
                .any(|tag| tag == "escaped")
                .then_some(AlternativeCastKindDef::Escape)
        })
        .or_else(|| {
            let signature = signature?;
            let definition = card.definition.card_definition()?;
            let definition = game.catalog.get(definition)?;
            let option = definition.play_option(signature.play_option())?;
            game.selected_alternative_kind(definition, option, id, signature.costs())
        });
    let signature_cost_counts = signature.and_then(|signature| {
        let definition = card.definition.card_definition()?;
        let option = game
            .catalog
            .get(definition)?
            .play_option(signature.play_option())?;
        Some((
            Game::repeatable_additional_cost_payments_for(option, signature.costs()),
            Game::additional_cost_payment_counts_for(option, signature.costs()),
        ))
    });
    Ok(
        (state.kind == StackObjectKindSnapshot::Spell).then(|| CastContext {
            source_zone: state
                .cast_from_zone
                .as_deref()
                .and_then(cast_source_zone_from_label),
            alternative,
            at_instant_speed: state.cast_at_instant_speed,
            x: if state.cast_x == 0 {
                signature.map_or(0, CastSignature::x)
            } else {
                state.cast_x
            },
            repeatable_additional_costs: if state.cast_repeatable_additional_costs == 0 {
                signature_cost_counts.as_ref().map_or(0, |counts| counts.0)
            } else {
                state.cast_repeatable_additional_costs
            },
            additional_costs: if state.cast_additional_costs.is_empty() {
                signature_cost_counts.map_or_else(Vec::new, |counts| counts.1)
            } else {
                state.cast_additional_costs.clone()
            },
            colors_of_mana_spent: color_set_from_flags(state.colors_of_mana_spent),
            phyrexian_symbols_paid_with_life: state.phyrexian_symbols_paid_with_life,
            exiled_payment_cards: state
                .cast_exiled_payment_cards
                .iter()
                .copied()
                .map(GameObjectId)
                .collect(),
            via_flashback: state.cast_via_flashback,
            via_suspend: state.cast_via_suspend,
        }),
    )
}
