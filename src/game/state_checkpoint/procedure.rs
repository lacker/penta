#![allow(clippy::wildcard_imports)]

use super::model::{EffectContinuationSnapshot, ScopedEffectSnapshot};
use super::model_procedure::{DrawReplacementSnapshot, PendingProcedureSnapshot};
use super::semantics::{catalog_scoped_effect, scoped_effect_snapshot};
use super::stack::{parse_trigger_context, trigger_context_snapshot};
use super::*;

pub(super) fn draw_replacement_snapshot(
    game: &Game,
    replacement: &super::super::DrawReplacement,
) -> Option<DrawReplacementSnapshot> {
    Some(DrawReplacementSnapshot {
        continuation: effect_continuation_snapshot(
            game,
            &replacement.object,
            replacement.context,
            replacement.effect,
        )?,
    })
}

pub(super) fn parse_draw_replacement(
    snapshot: &DrawReplacementSnapshot,
    game: &Game,
) -> Result<super::super::DrawReplacement, String> {
    let continuation = parse_effect_continuation(&snapshot.continuation, game)?;
    Ok(super::super::DrawReplacement {
        object: continuation.object,
        context: continuation.context,
        effect: continuation.effect,
    })
}

pub(super) fn pending_procedure_snapshot(
    game: &Game,
    procedure: &super::super::PendingProcedure,
) -> Option<PendingProcedureSnapshot> {
    Some(match procedure {
        super::super::PendingProcedure::DrawCards { player, remaining } => {
            PendingProcedureSnapshot::DrawCards {
                player: player.index(),
                remaining: *remaining,
            }
        }
        super::super::PendingProcedure::ResolveEffects {
            effects,
            object,
            context,
            custom_followup,
        } => {
            let ability = stack_ability_snapshot(game, object)?.ability_locator?;
            let definition = catalog_ability(&game.catalog, &ability)?;
            let effects = effects
                .iter()
                .copied()
                .map(|effect| scoped_effect_snapshot(&definition, effect))
                .collect::<Option<Vec<ScopedEffectSnapshot>>>()?;
            let custom_followup = match custom_followup {
                Some(behavior) => Some(ability_locator(&game.catalog, |candidate| {
                    candidate.custom_behavior() == Some(*behavior)
                })?),
                None => None,
            };
            PendingProcedureSnapshot::ResolveEffects {
                effects,
                object: Box::new(detached_stack_snapshot(game, object)?),
                ability,
                context: trigger_context_snapshot(*context),
                custom_followup,
            }
        }
        super::super::PendingProcedure::SylvanAfterDraw { player } => {
            PendingProcedureSnapshot::SylvanAfterDraw {
                player: player.index(),
            }
        }
        super::super::PendingProcedure::SimultaneousDraws {
            remaining,
            next,
            was_deferred,
        } => PendingProcedureSnapshot::SimultaneousDraws {
            remaining: *remaining,
            next: next.index(),
            was_deferred: *was_deferred,
        },
        super::super::PendingProcedure::ShuffleLibrary { player } => {
            PendingProcedureSnapshot::ShuffleLibrary {
                player: player.index(),
            }
        }
        super::super::PendingProcedure::FinishStepAdvance => {
            PendingProcedureSnapshot::FinishStepAdvance
        }
    })
}

pub(super) fn parse_pending_procedure(
    snapshot: &PendingProcedureSnapshot,
    game: &Game,
) -> Result<super::super::PendingProcedure, String> {
    Ok(match snapshot {
        PendingProcedureSnapshot::DrawCards { player, remaining } => {
            super::super::PendingProcedure::DrawCards {
                player: player_from_index(*player)?,
                remaining: *remaining,
            }
        }
        PendingProcedureSnapshot::ResolveEffects {
            effects,
            object,
            ability,
            context,
            custom_followup,
        } => {
            let effects = effects
                .iter()
                .map(|effect| {
                    catalog_scoped_effect(&game.catalog, ability, effect).ok_or_else(|| {
                        "pending procedure effect locator is absent from this catalog".to_owned()
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let custom_followup = custom_followup
                .as_ref()
                .map(|locator| {
                    catalog_ability(&game.catalog, locator)
                        .and_then(crate::card::AbilityDef::custom_behavior)
                        .ok_or_else(|| {
                            "pending procedure custom followup is absent from this catalog"
                                .to_owned()
                        })
                })
                .transpose()?;
            super::super::PendingProcedure::ResolveEffects {
                effects,
                object: Box::new(parse_detached_stack(object, game)?),
                context: parse_trigger_context(*context)?,
                custom_followup,
            }
        }
        PendingProcedureSnapshot::SylvanAfterDraw { player } => {
            super::super::PendingProcedure::SylvanAfterDraw {
                player: player_from_index(*player)?,
            }
        }
        PendingProcedureSnapshot::SimultaneousDraws {
            remaining,
            next,
            was_deferred,
        } => super::super::PendingProcedure::SimultaneousDraws {
            remaining: *remaining,
            next: player_from_index(*next)?,
            was_deferred: *was_deferred,
        },
        PendingProcedureSnapshot::ShuffleLibrary { player } => {
            super::super::PendingProcedure::ShuffleLibrary {
                player: player_from_index(*player)?,
            }
        }
        PendingProcedureSnapshot::FinishStepAdvance => {
            super::super::PendingProcedure::FinishStepAdvance
        }
    })
}

pub(super) fn draw_replacement_referenced_object_ids(
    replacement: &super::super::DrawReplacement,
) -> Vec<GameObjectId> {
    continuation_referenced_object_ids(&replacement.object, replacement.context)
}

pub(super) fn pending_procedure_referenced_object_ids(
    procedure: &super::super::PendingProcedure,
) -> Vec<GameObjectId> {
    match procedure {
        super::super::PendingProcedure::ResolveEffects {
            object, context, ..
        } => continuation_referenced_object_ids(object, *context),
        super::super::PendingProcedure::DrawCards { .. }
        | super::super::PendingProcedure::SylvanAfterDraw { .. }
        | super::super::PendingProcedure::SimultaneousDraws { .. }
        | super::super::PendingProcedure::ShuffleLibrary { .. }
        | super::super::PendingProcedure::FinishStepAdvance => Vec::new(),
    }
}

fn effect_continuation_snapshot(
    game: &Game,
    object: &StackObject,
    context: TriggerContext,
    effect: ScopedEffect,
) -> Option<EffectContinuationSnapshot> {
    let ability = stack_ability_snapshot(game, object)?.ability_locator?;
    let definition = catalog_ability(&game.catalog, &ability)?;
    Some(EffectContinuationSnapshot {
        object: detached_stack_snapshot(game, object)?,
        ability,
        context: trigger_context_snapshot(context),
        effect: scoped_effect_snapshot(&definition, effect)?,
    })
}

fn parse_effect_continuation(
    snapshot: &EffectContinuationSnapshot,
    game: &Game,
) -> Result<super::super::SacrificeFollowup, String> {
    Ok(super::super::SacrificeFollowup {
        object: Box::new(parse_detached_stack(&snapshot.object, game)?),
        context: parse_trigger_context(snapshot.context)?,
        effect: catalog_scoped_effect(&game.catalog, &snapshot.ability, &snapshot.effect)
            .ok_or("draw replacement effect locator is absent from this catalog")?,
    })
}

fn continuation_referenced_object_ids(
    object: &StackObject,
    context: TriggerContext,
) -> Vec<GameObjectId> {
    referenced_object_ids(object)
        .chain(context.object)
        .chain(context.chosen_objects.iter().flatten().copied())
        .collect()
}
