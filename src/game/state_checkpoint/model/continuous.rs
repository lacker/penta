//! The continuous effects a checkpoint carries: what one is, when it ends,
//! and the operations it can apply.
//!
//! Split out of the parent module for the source-size budget. These travel
//! together because each of them is part of one resolved continuous effect.

use serde::{Deserialize, Serialize};

use super::{AbilitySourceSnapshot, AppliedEffectLocator};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct ResolvedContinuousEffectSnapshot {
    pub(in crate::game::state_checkpoint) definition: AppliedEffectLocator,
    pub(in crate::game::state_checkpoint) source: AbilitySourceSnapshot,
    pub(in crate::game::state_checkpoint) timestamp: u64,
    pub(in crate::game::state_checkpoint) component_order: u16,
    pub(in crate::game::state_checkpoint) expiration: ContinuousEffectExpirationSnapshot,
    pub(in crate::game::state_checkpoint) operation: ResolvedContinuousOperationSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct ResolvedPlayPermissionSnapshot {
    pub(in crate::game::state_checkpoint) definition: AppliedEffectLocator,
    pub(in crate::game::state_checkpoint) source: AbilitySourceSnapshot,
    pub(in crate::game::state_checkpoint) affected_seat: usize,
    pub(in crate::game::state_checkpoint) expiration: ContinuousEffectExpirationSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct ResolvedPlayRestrictionSnapshot {
    pub(in crate::game::state_checkpoint) definition: AppliedEffectLocator,
    pub(in crate::game::state_checkpoint) source: AbilitySourceSnapshot,
    pub(in crate::game::state_checkpoint) affected_seat: usize,
    pub(in crate::game::state_checkpoint) timestamp: u64,
    pub(in crate::game::state_checkpoint) component_order: u16,
    pub(in crate::game::state_checkpoint) expiration: ContinuousEffectExpirationSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct ResolvedAttackRestrictionSnapshot {
    pub(in crate::game::state_checkpoint) definition: AppliedEffectLocator,
    pub(in crate::game::state_checkpoint) source: AbilitySourceSnapshot,
    pub(in crate::game::state_checkpoint) affected_seat: usize,
    pub(in crate::game::state_checkpoint) expiration: ContinuousEffectExpirationSnapshot,
}

/// A resolved player protection, stored the way the restrictions above are:
/// the rule is found again through its locator, so what is written down is
/// who it protects and how long it lasts.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct ResolvedPlayerProtectionSnapshot {
    pub(in crate::game::state_checkpoint) definition: AppliedEffectLocator,
    pub(in crate::game::state_checkpoint) source: AbilitySourceSnapshot,
    pub(in crate::game::state_checkpoint) affected_seat: usize,
    pub(in crate::game::state_checkpoint) expiration: ContinuousEffectExpirationSnapshot,
}

/// A resolved player rule rehydrated from its authored applied-effect leaf.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct ResolvedPlayerRuleSnapshot {
    pub(in crate::game::state_checkpoint) definition: AppliedEffectLocator,
    pub(in crate::game::state_checkpoint) source: AbilitySourceSnapshot,
    pub(in crate::game::state_checkpoint) affected_seat: usize,
    pub(in crate::game::state_checkpoint) expiration: ContinuousEffectExpirationSnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) enum SetOperationSnapshot {
    Add,
    Remove,
    Set,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(in crate::game::state_checkpoint) enum ResolvedContinuousOperationSnapshot {
    AbilityAdd {
        grant_id: u8,
    },
    AbilityRemove,
    BasicLandTypes {
        operation: SetOperationSnapshot,
    },
    CardTypes {
        operation: SetOperationSnapshot,
    },
    Supertypes {
        operation: SetOperationSnapshot,
    },
    Colors {
        operation: SetOperationSnapshot,
    },
    CreatureTypes {
        operation: SetOperationSnapshot,
    },
    Subtypes {
        operation: SetOperationSnapshot,
    },
    ModifyPowerToughness {
        power: i16,
        toughness: i16,
    },
    Rule,
    /// Layer 7e. It carries nothing on the wire because it carries nothing
    /// in the effect: what matters is that one is applied.
    SwitchPowerToughness,
    SetBasePower {
        power: i16,
    },
    SetBasePowerToughness {
        power: i16,
        toughness: i16,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(in crate::game::state_checkpoint) enum ContinuousEffectExpirationSnapshot {
    EndOfTurn,
    EndOfCombat,
    UpkeepOf {
        seat: usize,
    },
    TurnOf {
        seat: usize,
        turn: u32,
    },
    WhileSourceTapped,
    /// Additive: a checkpoint written before this duration existed restores
    /// no such effect, because nothing could have made one.
    WhileSourceRemains,
    NextMatchingCast,
    AnyOf {
        expirations: Vec<ContinuousEffectExpirationSnapshot>,
    },
    Never,
}
