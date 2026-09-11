#![allow(clippy::wildcard_imports)]
use super::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub(in crate::game::state_checkpoint) struct GameSnapshot {
    pub(in crate::game::state_checkpoint) version: u32,
    #[serde(default)]
    pub(in crate::game::state_checkpoint) starting_player: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) match_state: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) current_game_result: Option<crate::GameResult>,
    #[serde(default)]
    pub(in crate::game::state_checkpoint) restart_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) restart_arrivals: Option<serde_json::Value>,
    pub(in crate::game::state_checkpoint) simulation_fingerprint: String,
    pub(in crate::game::state_checkpoint) turns_started: [u32; 2],
    /// Damage each player has taken this turn, in total and per source
    /// group. Absent from checkpoints that predate the accumulators.
    #[serde(default)]
    pub(in crate::game::state_checkpoint) damage_taken_this_turn: [u16; 2],
    #[serde(default)]
    pub(in crate::game::state_checkpoint) damage_taken_by_group_this_turn: Vec<Vec<u16>>,
    pub(in crate::game::state_checkpoint) next_decision_id: u32,
    pub(in crate::game::state_checkpoint) next_trigger_id: u32,
    pub(in crate::game::state_checkpoint) next_continuous_effect_timestamp: u64,
    pub(in crate::game::state_checkpoint) consecutive_passes: u8,
    pub(in crate::game::state_checkpoint) attackers_declared: bool,
    pub(in crate::game::state_checkpoint) combat_had_attackers: bool,
    pub(in crate::game::state_checkpoint) blockers_declared: bool,
    pub(in crate::game::state_checkpoint) untap_pending: bool,
    pub(in crate::game::state_checkpoint) cleanup_pending: bool,
    pub(in crate::game::state_checkpoint) mulligans: [u8; 2],
    pub(in crate::game::state_checkpoint) lands_played_this_turn: [u16; 2],
    /// The companions each player may still take from outside the game,
    /// named by definition. Additive: a checkpoint written before companions
    /// existed restores a game in which nobody brought one, which is what
    /// every game before them was.
    #[serde(default, skip_serializing_if = "emptiness::is_empty_pair_of_vectors")]
    pub(in crate::game::state_checkpoint) companions: [Vec<u64>; 2],
    /// The creature subtypes each seat attacked with this turn. Additive: a
    /// checkpoint written before it existed restores a turn nobody is
    /// recorded as having attacked in, which is what it meant.
    #[serde(default, skip_serializing_if = "emptiness::is_empty_pair_of_names")]
    pub(in crate::game::state_checkpoint) attacked_subtypes_this_turn: [Vec<String>; 2],
    pub(in crate::game::state_checkpoint) tried_to_draw_from_empty_library: [bool; 2],
    pub(in crate::game::state_checkpoint) mana: [Vec<ManaSnapshot>; 2],
    pub(in crate::game::state_checkpoint) creature_died_this_turn: bool,
    /// Additive: a checkpoint written before the count existed restores as
    /// zero, which is what a turn with no recorded deaths means anyway.
    #[serde(default, skip_serializing_if = "emptiness::is_zero_u16")]
    pub(in crate::game::state_checkpoint) creatures_died_this_turn: u16,
    pub(in crate::game::state_checkpoint) linked_exiles: Vec<[u32; 2]>,
    /// Uses of a limited graveyard permission this turn. Additive: a
    /// checkpoint written before it existed restores a turn in which nothing
    /// had been played that way yet.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) graveyard_permission_uses: Vec<[u32; 2]>,
    /// Additive: a checkpoint written before the rule existed restores as
    /// false, which is what every ordinary turn means anyway.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(in crate::game::state_checkpoint) damage_cannot_be_prevented_this_turn: bool,
    /// Additive: a checkpoint written before exile permissions existed
    /// restores as empty, which is what a game with none of them means
    /// anyway. Each entry is the card, who may play it, whether it is free,
    /// the turn it lapses at the end of, and whether only an adventure's
    /// creature half may be played.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) exile_play_permissions: Vec<ExilePlayPermissionSnapshot>,
    /// Additive: a checkpoint written before the monarch existed restores
    /// with nobody wearing the crown, which is how every game starts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) monarch: Option<usize>,
    /// Who has been told they cannot gain life for the rest of the game.
    /// Additive: a checkpoint written before this existed means nobody had
    /// been, which is where every game starts.
    #[serde(default, skip_serializing_if = "emptiness::is_unset_for_both")]
    pub(in crate::game::state_checkpoint) cannot_gain_life: [bool; 2],
    pub(in crate::game::state_checkpoint) turn_phase_queue: Vec<TurnPhaseSnapshot>,
    pub(in crate::game::state_checkpoint) turn_phase_resume: Option<TurnPhaseResumeSnapshot>,
    /// Resolving play prohibitions. Static restrictions remain source-derived.
    pub(in crate::game::state_checkpoint) resolved_play_restrictions:
        Vec<ResolvedPlayRestrictionSnapshot>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) resolved_attack_restrictions:
        Vec<ResolvedAttackRestrictionSnapshot>,
    /// Resolving play permissions. Additive: a checkpoint written before
    /// anything could grant one restores with none, which is what every game
    /// with no such effect in it has.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) resolved_play_permissions:
        Vec<ResolvedPlayPermissionSnapshot>,
    /// Resolving player protections. Additive for the same reason: nothing
    /// could protect a player before, so a checkpoint without them restores
    /// a game in which nobody is protected.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) resolved_player_protections:
        Vec<ResolvedPlayerProtectionSnapshot>,
    /// Simple resolving player rules. Additive: an older checkpoint had none
    /// because no supported card could create one.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) resolved_player_rules: Vec<ResolvedPlayerRuleSnapshot>,
    pub(in crate::game::state_checkpoint) spells_cast_this_turn: [u16; 2],
    pub(in crate::game::state_checkpoint) spells_cast_last_turn: [u16; 2],
    /// Additive: older checkpoints have no predicate-filterable cast history.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) spell_cast_history_this_turn: Vec<u32>,
    /// Spells cast over the whole game. Additive: a checkpoint written before
    /// it was counted restores a game in which nobody has cast anything,
    /// which is only wrong for a card that asks, and only until one is cast.
    #[serde(default)]
    pub(in crate::game::state_checkpoint) spells_cast_this_game: [u16; 2],
    pub(in crate::game::state_checkpoint) cards_drawn_this_turn: [u16; 2],
    pub(in crate::game::state_checkpoint) citys_blessing: [bool; 2],
    pub(in crate::game::state_checkpoint) permanent_left_battlefield_this_turn: [bool; 2],
    /// Additive: a checkpoint written before the turn tracked it restores
    /// with nobody's graveyard having lost a card, which is what every turn
    /// starts as anyway.
    #[serde(default)]
    pub(in crate::game::state_checkpoint) card_left_graveyard_this_turn: [bool; 2],
    pub(in crate::game::state_checkpoint) life_gained_this_turn: [u16; 2],
    /// Whether each player has lost life this turn. Additive: a checkpoint
    /// written before it existed restores a turn in which nobody has.
    #[serde(default)]
    pub(in crate::game::state_checkpoint) lost_life_this_turn: [bool; 2],
    pub(in crate::game::state_checkpoint) draw_step_draw_taken: [bool; 2],
    pub(in crate::game::state_checkpoint) drawn_this_turn: [Vec<u32>; 2],
    /// Compatibility-only wire member. Channel now reconstructs through
    /// `ongoing_effects`; current writers always emit false for both seats.
    pub(in crate::game::state_checkpoint) channel_active: [bool; 2],
    pub(in crate::game::state_checkpoint) defer_empty_library_loss: bool,
    pub(in crate::game::state_checkpoint) draw_replacements: [Vec<DrawReplacementSnapshot>; 2],
    pub(in crate::game::state_checkpoint) pending_combat_attackers: Vec<u32>,
    pub(in crate::game::state_checkpoint) combat_blocked_attackers: Vec<u32>,
    pub(in crate::game::state_checkpoint) extra_turns: Vec<usize>,
    pub(in crate::game::state_checkpoint) next_regular_player: usize,
    /// Resolved damage-prevention rules in creation order. Static prevention
    /// remains source-derived and therefore is not checkpointed here.
    pub(in crate::game::state_checkpoint) damage_preventions: Vec<ResolvedDamagePreventionSnapshot>,
    /// Resolved damage redirections in creation order. Static group
    /// redirection remains source-derived and is not checkpointed here.
    pub(in crate::game::state_checkpoint) damage_redirects: Vec<ResolvedDamageRedirectSnapshot>,
    pub(in crate::game::state_checkpoint) pregame: Option<PregameSnapshot>,
    pub(in crate::game::state_checkpoint) combat_damage_stage: CombatDamageStageSnapshot,
    pub(in crate::game::state_checkpoint) battlefield: Vec<PermanentSnapshot>,
    pub(in crate::game::state_checkpoint) emblems: Vec<EmblemSnapshot>,
    pub(in crate::game::state_checkpoint) stack: Vec<StackSnapshot>,
    pub(in crate::game::state_checkpoint) retired_objects: Vec<RetiredObjectSnapshot>,
    /// What each retired object became when it changed zones, for the
    /// objects a pending trigger might still name. Appended, so a checkpoint
    /// written before it existed still reads -- and reads as a game where no
    /// dying creature can find the card it became, which is what those
    /// checkpoints actually recorded.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) successors: Vec<SuccessorSnapshot>,
    pub(in crate::game::state_checkpoint) pending_events: Vec<PendingEventSnapshot>,
    #[serde(rename = "temporaryAbilityGrants")]
    pub(in crate::game::state_checkpoint) nonbattlefield_ability_grants:
        Vec<NonbattlefieldAbilityGrantSnapshot>,
    /// Resolved duration-scoped effects that expose an activated ability.
    /// Additive: checkpoints written before these effects existed restore
    /// with none, which is the state of every game without one.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) ongoing_effects: Vec<ResolvedOngoingEffectSnapshot>,
    pub(in crate::game::state_checkpoint) next_installed_trigger_id: u32,
    pub(in crate::game::state_checkpoint) installed_triggers: Vec<InstalledTriggerSnapshot>,
    pub(in crate::game::state_checkpoint) pending_triggers: Vec<PendingTriggerSnapshot>,
    pub(in crate::game::state_checkpoint) pending_procedures: Vec<PendingProcedureSnapshot>,
    pub(in crate::game::state_checkpoint) decision_state: Option<DecisionStateSnapshot>,
    pub(in crate::game::state_checkpoint) has_deferred_state: bool,
    pub(in crate::game::state_checkpoint) viewer: usize,
}
