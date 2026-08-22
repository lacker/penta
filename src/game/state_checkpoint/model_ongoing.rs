use serde::{Deserialize, Serialize};

use super::model::{
    AbilityLocator, AbilityOriginSnapshot, ContinuousEffectExpirationSnapshot,
    EffectResolutionContextSnapshot, ObjectCharacteristicsSnapshot,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ResolvedOngoingEffectSnapshot {
    /// The effect object's own public identity. It is not a permanent or an
    /// emblem and therefore has no ordinary observation-zone entry.
    pub(super) object_id: u32,
    pub(super) origin: AbilityOriginSnapshot,
    pub(super) ability: AbilityLocator,
    pub(super) presentation: ObjectCharacteristicsSnapshot,
    pub(super) owner: usize,
    pub(super) controller: usize,
    pub(super) context: EffectResolutionContextSnapshot,
    pub(super) expiration: ContinuousEffectExpirationSnapshot,
}
