use serde::{Deserialize, Serialize};

use super::model::TargetSnapshot;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PreventionShieldSnapshot {
    pub(super) recipient: TargetSnapshot,
    /// Absent for the "prevent all damage" form, which is never spent.
    pub(super) remaining: Option<u16>,
    /// The one source this shield answers, for "a source of your choice".
    /// Absent for every shield that answers any source, which is why this is
    /// an additive member an older consumer can ignore.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) source: Option<u32>,
    /// Whether this shield stops only half of a covered hit, rounded down.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) half_rounded_down: bool,
    /// Whether spending this shield gains its recipient that much life.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) gain_life: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum RelationalDamagePreventionSnapshot {
    ToPlayerAndControlledCreatures { player: usize },
    FromAllExcept { source: u32 },
}
