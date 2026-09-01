#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ReplacementEffectContextSnapshot {
    pub(super) source: AbilitySourceSnapshot,
    pub(super) controller: usize,
}
