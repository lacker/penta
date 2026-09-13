use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(in crate::game::state_checkpoint) struct PaymentDraftSnapshot {
    pub(in crate::game::state_checkpoint) player: usize,
    pub(in crate::game::state_checkpoint) action: usize,
    pub(in crate::game::state_checkpoint) source: u32,
    pub(in crate::game::state_checkpoint) x: u16,
    pub(in crate::game::state_checkpoint) funding: Vec<FundingStepSnapshot>,
    pub(in crate::game::state_checkpoint) contributions: Vec<(u32, usize)>,
    pub(in crate::game::state_checkpoint) announcements: Vec<Vec<u32>>,
    pub(in crate::game::state_checkpoint) resume: Option<Box<super::ExplicitPaymentResumeSnapshot>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(in crate::game::state_checkpoint) struct FundingStepSnapshot {
    pub(in crate::game::state_checkpoint) action: usize,
    pub(in crate::game::state_checkpoint) source: u32,
    pub(in crate::game::state_checkpoint) mana: Option<Vec<usize>>,
    pub(in crate::game::state_checkpoint) answers: Vec<Vec<u32>>,
}
