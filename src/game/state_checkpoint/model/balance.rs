//! Snapshots for Balance, whose one resolution asks several questions.
//!
//! Each player sacrifices or discards down in turn, so the task outlives the
//! decision that started it and has to be written down between answers.

use serde::{Deserialize, Serialize};

use super::{DecisionCardSnapshot, DecisionZoneSnapshot, ZoneMoveCauseSnapshot};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) enum BalancePhaseSnapshot {
    Lands,
    Hands,
    Creatures,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct BalanceTaskSnapshot {
    pub(in crate::game::state_checkpoint) player: usize,
    pub(in crate::game::state_checkpoint) prompt: String,
    pub(in crate::game::state_checkpoint) zone: DecisionZoneSnapshot,
    pub(in crate::game::state_checkpoint) cards: Option<Vec<DecisionCardSnapshot>>,
    pub(in crate::game::state_checkpoint) count: usize,
    pub(in crate::game::state_checkpoint) action: BalanceActionSnapshot,
    pub(in crate::game::state_checkpoint) cause: ZoneMoveCauseSnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) enum BalanceActionSnapshot {
    Sacrifice,
    Discard,
}
