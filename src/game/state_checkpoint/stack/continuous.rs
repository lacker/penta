use super::{
    Game, ResolvedContinuousEffect, ResolvedContinuousEffectKind, ResolvedContinuousEffectSnapshot,
    StackObject,
};

pub(super) fn stack_continuous_effect_snapshots(
    game: &Game,
    object: &StackObject,
) -> Vec<ResolvedContinuousEffectSnapshot> {
    object
        .resolved_continuous_effects
        .iter()
        .filter_map(|effect| {
            super::super::permanent::resolved_continuous_effect_snapshot(&game.catalog, effect)
        })
        .collect()
}

pub(super) fn parse_stack_continuous_effects(
    states: &[ResolvedContinuousEffectSnapshot],
    game: &Game,
) -> Result<Vec<ResolvedContinuousEffect>, String> {
    states
        .iter()
        .map(|state| {
            let effect =
                super::super::wire::parse_resolved_continuous_effect(state, &game.catalog)?;
            if !matches!(effect.kind, ResolvedContinuousEffectKind::Colors(_)) {
                return Err("unsupported stack characteristic operation".into());
            }
            Ok(effect)
        })
        .collect()
}
