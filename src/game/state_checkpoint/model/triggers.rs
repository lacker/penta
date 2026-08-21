//! Snapshots for triggers waiting to be put on the stack.

use serde::{Deserialize, Serialize};

use super::{
    AbilityLocator, AbilitySourceSnapshot, EffectResolutionContextSnapshot,
    ObjectCharacteristicsSnapshot, TargetSelectionSnapshot,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct PendingTriggerSnapshot {
    pub(in crate::game::state_checkpoint) id: u32,
    pub(in crate::game::state_checkpoint) source: AbilitySourceSnapshot,
    pub(in crate::game::state_checkpoint) ability: AbilityLocator,
    pub(in crate::game::state_checkpoint) target_definition: AbilityLocator,
    pub(in crate::game::state_checkpoint) presentation: ObjectCharacteristicsSnapshot,
    pub(in crate::game::state_checkpoint) owner: usize,
    pub(in crate::game::state_checkpoint) controller: usize,
    pub(in crate::game::state_checkpoint) targets: Vec<TargetSelectionSnapshot>,
    pub(in crate::game::state_checkpoint) context: EffectResolutionContextSnapshot,
    pub(in crate::game::state_checkpoint) x: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct TriggerPlacementBatchSnapshot {
    pub(in crate::game::state_checkpoint) controller: usize,
    pub(in crate::game::state_checkpoint) triggers: Vec<PendingTriggerSnapshot>,
}
