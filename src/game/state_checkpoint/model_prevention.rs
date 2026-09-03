use serde::{Deserialize, Serialize};

use super::model::{
    AbilityLocator, AbilitySourceSnapshot, ContinuousEffectExpirationSnapshot, TargetSnapshot,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DamagePreventionLocator {
    pub(super) ability: AbilityLocator,
    pub(super) effect_index: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum DamageSourceMatcherSnapshot {
    Any,
    Exact { object_id: u32 },
    Except { object_id: u32 },
    Matching { relative_to: u32 },
    Group { group: DamageSourceGroupSnapshot },
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum DamageSourceGroupSnapshot {
    CreaturesWithFlying,
    AttackingCreaturesWithoutFlying,
    Artifacts,
    UnblockedCreatures,
}

/// One resolved, turn-scoped damage redirection. Redirection changes the
/// recipient before prevention is considered, so it is deliberately not a
/// damage-source matcher or prevention snapshot.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ResolvedDamageRedirectSnapshot {
    pub(super) player: usize,
    pub(super) source: u32,
    pub(super) destination: u32,
    pub(super) expiration: ContinuousEffectExpirationSnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum DamageRecipientMatcherSnapshot {
    Any,
    Exact { target: TargetSnapshot },
    PlayerAndControlledCreatures { seat: usize },
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum DamagePreventionCapacitySnapshot {
    Amount { remaining: u16 },
    Events { remaining: u16 },
    Unlimited,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ResolvedDamagePreventionSnapshot {
    pub(super) source: DamageSourceMatcherSnapshot,
    pub(super) recipient: DamageRecipientMatcherSnapshot,
    pub(super) combat_only: bool,
    pub(super) capacity: DamagePreventionCapacitySnapshot,
    /// Locates the authoritative declaration when either the source matcher
    /// or deferred amount expression must be reconstructed.
    pub(super) definition: Option<DamagePreventionLocator>,
    pub(super) gain_life: Option<usize>,
    pub(super) source_ability: AbilitySourceSnapshot,
    pub(super) timestamp: u64,
    pub(super) expiration: ContinuousEffectExpirationSnapshot,
}
