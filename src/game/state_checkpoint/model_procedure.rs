use serde::{Deserialize, Serialize};

use super::model::{
    AbilityLocator, DetachedStackSnapshot, EffectContinuationSnapshot, ScopedEffectSnapshot,
    TriggerContextSnapshot,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DrawReplacementSnapshot {
    pub(super) continuation: EffectContinuationSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum PendingProcedureSnapshot {
    DrawCards {
        player: usize,
        remaining: u16,
    },
    ResolveEffects {
        effects: Vec<ScopedEffectSnapshot>,
        object: Box<DetachedStackSnapshot>,
        ability: AbilityLocator,
        context: TriggerContextSnapshot,
        custom_followup: Option<AbilityLocator>,
    },
    SylvanAfterDraw {
        player: usize,
    },
    SimultaneousDraws {
        remaining: [u16; 2],
        next: usize,
        was_deferred: bool,
    },
    ShuffleLibrary {
        player: usize,
    },
    FinishStepAdvance,
}
