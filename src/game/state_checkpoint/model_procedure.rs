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
    CommitPayment(Box<CommittedPaymentSnapshot>),
    DrawCards {
        player: usize,
        remaining: u16,
    },
    ResolveEffects {
        effects: Vec<ScopedEffectSnapshot>,
        object: Box<DetachedStackSnapshot>,
        ability: AbilityLocator,
        context: EffectResolutionContextSnapshot,
    },
    ForEachInBinding {
        objects: super::model::BindingSnapshot,
        binding: super::model::BindingSnapshot,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CommittedPaymentSnapshot {
    pub player: usize,
    pub continuation: EffectContinuationSnapshot,
    pub answers: Vec<super::model::PaymentAnswerSnapshot>,
    pub remaining: Vec<PaymentPartSnapshot>,
    pub named: Vec<NamedPaymentSnapshot>,
    pub mana_spent: Vec<super::model::ManaSnapshot>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(super) struct PaymentPartSnapshot {
    /// A node in the authored cost tree, never serialized executable code.
    pub cost: usize,
    pub times: u16,
    pub objects: Vec<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(super) struct NamedPaymentSnapshot {
    pub cost: usize,
    pub repetitions: u16,
}
