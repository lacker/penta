use serde::{Deserialize, Serialize};

use super::model::{
    AbilityLocator, AbilitySourceSnapshot, DetachedStackSnapshot, ScopedEffectSnapshot,
    TriggerContextSnapshot,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DelayedTriggerSnapshot {
    pub(super) object: DetachedStackSnapshot,
    pub(super) ability: AbilityLocator,
    pub(super) context: TriggerContextSnapshot,
    pub(super) step: TurnStepSnapshot,
    pub(super) player: PlayerRelationSnapshot,
    pub(super) effect: ScopedEffectSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ScheduledTriggerSnapshot {
    pub(super) id: u32,
    pub(super) source: AbilitySourceSnapshot,
    pub(super) ability: AbilityLocator,
    pub(super) definition: u16,
    pub(super) owner: usize,
    pub(super) controller: usize,
    pub(super) context: TriggerContextSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct FloatingTriggerSnapshot {
    pub(super) source: AbilitySourceSnapshot,
    pub(super) ability: AbilityLocator,
    pub(super) definition: u16,
    pub(super) owner: usize,
    pub(super) controller: usize,
    pub(super) context: TriggerContextSnapshot,
    pub(super) until_turn_of: usize,
    pub(super) created_after_turns: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum TurnStepSnapshot {
    Untap,
    Upkeep,
    Draw,
    PrecombatMain,
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,
    PostcombatMain,
    End,
    Cleanup,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum PlayerRelationSnapshot {
    Any,
    You,
    NotYou,
    Opponent,
    ActivePlayer,
    NonactivePlayer,
    EventPlayer,
    ChosenPlayer,
}
