#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TriggerContextSnapshot {
    pub(super) object: Option<u32>,
    /// Additive event-local destination identity for zone-change triggers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) zone_change_result: Option<u32>,
    pub(super) object_controller: Option<usize>,
    pub(super) event_player: Option<usize>,
    pub(super) amount: Option<i32>,
    /// Additive: a checkpoint written before it existed restores a
    /// resolution that names nothing damaged, which every resolution that is
    /// not about damage does anyway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) damaged_object: Option<u32>,
    /// Additive cast-event provenance. Older checkpoints restore no origin,
    /// which is accurate for every non-cast trigger context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) cast_from_zone: Option<super::ZoneKindSnapshot>,
}
