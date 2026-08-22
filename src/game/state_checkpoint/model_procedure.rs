use serde::{Deserialize, Serialize};

use super::model::{
    AbilityLocator, DetachedStackSnapshot, EffectContinuationSnapshot,
    EffectResolutionContextSnapshot, ScopedEffectSnapshot,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DrawReplacementSnapshot {
    pub(super) continuation: EffectContinuationSnapshot,
    #[serde(default)]
    pub(super) optional: bool,
    #[serde(default = "default_true")]
    pub(super) installed: bool,
}

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[allow(clippy::large_enum_variant)]
pub(super) enum PendingProcedureSnapshot {
    DrawCards {
        player: usize,
        remaining: u16,
    },
    ResolveEffects {
        effects: Vec<ScopedEffectSnapshot>,
        object: Box<DetachedStackSnapshot>,
        ability: AbilityLocator,
        context: EffectResolutionContextSnapshot,
        custom_followup: Option<AbilityLocator>,
    },
    ForEachInBinding {
        objects: usize,
        binding: usize,
        next: usize,
        continuation: EffectContinuationSnapshot,
    },
    SimultaneousDraws {
        remaining: [u16; 2],
        next: usize,
        was_deferred: bool,
    },
    ShuffleLibrary {
        player: usize,
    },
    FinishStackResolution {
        object: Box<DetachedStackSnapshot>,
        resolved: bool,
    },
    FinishStepAdvance,
}
