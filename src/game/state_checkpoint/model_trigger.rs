use serde::{Deserialize, Serialize};

use super::model::{
    AbilityLocator, AbilitySourceSnapshot, EffectResolutionContextSnapshot,
    ObjectCharacteristicsSnapshot, TargetSelectionSnapshot,
};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum InstalledTriggerLifetimeSnapshot {
    Once,
    UntilTurn { seat: usize, turn: u32 },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct InstalledTriggerSnapshot {
    pub(super) id: u32,
    pub(super) source: AbilitySourceSnapshot,
    pub(super) ability: AbilityLocator,
    pub(super) presentation: ObjectCharacteristicsSnapshot,
    pub(super) owner: usize,
    pub(super) controller: usize,
    pub(super) targets: Vec<TargetSelectionSnapshot>,
    pub(super) context: EffectResolutionContextSnapshot,
    pub(super) lifetime: InstalledTriggerLifetimeSnapshot,
    pub(super) target_base: usize,
    pub(super) x: u16,
}
