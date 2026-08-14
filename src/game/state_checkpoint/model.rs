use serde::{Deserialize, Serialize};

// serde hands `skip_serializing_if` a reference, so this signature is fixed.
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero_u8(value: &u8) -> bool {
    *value == 0
}

mod stack;
mod triggers;
pub(in crate::game::state_checkpoint) use stack::*;
pub(in crate::game::state_checkpoint) use triggers::*;

use super::model_keyword::{KeywordSnapshot, UpkeepKeywordSnapshot};
pub(super) use super::model_prevention::*;

use super::model_procedure::{DrawReplacementSnapshot, PendingProcedureSnapshot};
use super::model_trigger::InstalledTriggerSnapshot;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct GameSnapshot {
    pub(super) version: u32,
    pub(super) simulation_fingerprint: String,
    pub(super) turns_started: [u32; 2],
    pub(super) next_decision_id: u32,
    pub(super) next_trigger_id: u32,
    pub(super) next_continuous_effect_timestamp: u64,
    pub(super) consecutive_passes: u8,
    pub(super) attackers_declared: bool,
    pub(super) blockers_declared: bool,
    pub(super) untap_pending: bool,
    pub(super) cleanup_pending: bool,
    pub(super) mulligans: [u8; 2],
    pub(super) land_played_this_turn: [bool; 2],
    pub(super) tried_to_draw_from_empty_library: [bool; 2],
    pub(super) mana: [Vec<ManaSnapshot>; 2],
    pub(super) creature_died_this_turn: bool,
    pub(super) linked_exiles: Vec<[u32; 2]>,
    pub(super) sorcery_flash_grants: [u8; 2],
    pub(super) turn_phase_queue: Vec<TurnPhaseSnapshot>,
    pub(super) turn_phase_resume: Option<TurnPhaseResumeSnapshot>,
    /// Resolving play prohibitions. Static restrictions remain source-derived.
    pub(super) resolved_play_restrictions: Vec<ResolvedPlayRestrictionSnapshot>,
    pub(super) spells_cast_this_turn: [u16; 2],
    pub(super) spells_cast_last_turn: [u16; 2],
    pub(super) cards_drawn_this_turn: [u16; 2],
    pub(super) drawn_this_turn: [Vec<u32>; 2],
    pub(super) defer_empty_library_loss: bool,
    pub(super) draw_replacements: [Vec<DrawReplacementSnapshot>; 2],
    pub(super) miracle_window: Option<u32>,
    pub(super) pending_combat_attackers: Vec<u32>,
    pub(super) combat_blocked_attackers: Vec<u32>,
    pub(super) extra_turns: Vec<usize>,
    pub(super) next_regular_player: usize,
    pub(super) channel_active: [bool; 2],
    /// Resolved damage-prevention rules in creation order. Static prevention
    /// remains source-derived and therefore is not checkpointed here.
    pub(super) damage_preventions: Vec<ResolvedDamagePreventionSnapshot>,
    /// Resolved damage redirections in creation order. Static group
    /// redirection remains source-derived and is not checkpointed here.
    pub(super) damage_redirects: Vec<ResolvedDamageRedirectSnapshot>,
    pub(super) pregame: Option<PregameSnapshot>,
    pub(super) combat_damage_stage: CombatDamageStageSnapshot,
    pub(super) battlefield: Vec<PermanentSnapshot>,
    pub(super) emblems: Vec<EmblemSnapshot>,
    pub(super) stack: Vec<StackSnapshot>,
    pub(super) retired_objects: Vec<RetiredObjectSnapshot>,
    pub(super) pending_events: Vec<PendingEventSnapshot>,
    pub(super) temporary_ability_grants: Vec<TemporaryAbilityGrantSnapshot>,
    pub(super) next_installed_trigger_id: u32,
    pub(super) installed_triggers: Vec<InstalledTriggerSnapshot>,
    pub(super) pending_triggers: Vec<PendingTriggerSnapshot>,
    pub(super) pending_procedures: Vec<PendingProcedureSnapshot>,
    pub(super) decision_state: Option<DecisionStateSnapshot>,
    pub(super) has_deferred_state: bool,
    pub(super) viewer: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum TurnPhaseSnapshot {
    Combat,
    PostcombatMain,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum TurnPhaseResumeSnapshot {
    PrecombatMain,
    BeginningOfCombat,
    PostcombatMain,
    End,
    NextTurn,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TemporaryAbilityGrantSnapshot {
    pub(super) object: u32,
    pub(super) ability: AbilityLocator,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum ManaColorSnapshot {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ManaSnapshot {
    pub(super) color: ManaColorSnapshot,
    pub(super) source: Option<ManaSourceSnapshot>,
    pub(super) payload: Option<ManaPayloadLocator>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ManaSourceSnapshot {
    pub(super) object: u32,
    pub(super) ability: AbilityOriginSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ManaPayloadLocator {
    pub(super) ability: AbilityLocator,
    pub(super) effect_index: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum AbilityOriginSnapshot {
    Printed {
        definition: u16,
        part_id: u8,
        ability_id: u8,
    },
    IntrinsicBasicLand {
        land_type: BasicLandTypeSnapshot,
    },
    Granted {
        source: u32,
        source_definition: u16,
        source_part_id: u8,
        source_ability_id: u8,
        grant_id: u8,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum BasicLandTypeSnapshot {
    Plains,
    Island,
    Swamp,
    Mountain,
    Forest,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub(super) enum PregameSnapshot {
    Mulligan { seat: usize },
    Bottom { seat: usize },
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub(super) enum CombatDamageStageSnapshot {
    #[default]
    NotStarted,
    Single,
    FirstStrike {
        combatants: Vec<u32>,
    },
    RegularAfterFirstStrike {
        combatants: Vec<u32>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct PermanentSnapshot {
    pub(super) object_id: u32,
    pub(super) owner: usize,
    pub(super) timestamp: u64,
    pub(super) entered_controller_turn: u32,
    /// Detained until this seat's next turn, with the turn count it landed on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) detained_until_turn_of: Option<(usize, u32)>,
    /// Whether this permanent is destroyed as the current combat phase ends.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) destroy_at_end_of_combat: bool,
    /// Untap steps this permanent still owes before it untaps normally.
    #[serde(default, skip_serializing_if = "is_zero_u8")]
    pub(super) skipped_untap_steps: u8,
    pub(super) control_reverts_to: Option<usize>,
    /// The permanent sustaining a duration-scoped control change, absent for
    /// the turn-scoped form and for everything untouched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) control_source: Option<u32>,
    /// Whether that holder also has to stay tapped.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) control_requires_source_tapped: bool,
    pub(super) chosen_player: Option<usize>,
    pub(super) destroy_at_end: bool,
    pub(super) counters: Vec<u16>,
    pub(super) attached_to: Option<u32>,
    pub(super) reconfigured_timestamp: Option<u64>,
    pub(super) exile_instead_of_dying: bool,
    pub(super) combat_damage_assignment: Vec<CombatDamageAssignmentSnapshot>,
    pub(super) regeneration_shields: u8,
    pub(super) attacked_this_turn: bool,
    pub(super) attacks_this_turn: u8,
    pub(super) damage_sources: Vec<u32>,
    pub(super) dealt_damage_to_opponent_this_turn: bool,
    pub(super) deathtouch_damage: bool,
    pub(super) created_by: Option<u32>,
    pub(super) temporary_keywords: Vec<KeywordSnapshot>,
    pub(super) keywords_until_upkeep_of: Vec<UpkeepKeywordSnapshot>,
    pub(super) resolved_continuous_effects: Vec<ResolvedContinuousEffectSnapshot>,
    pub(super) activations_this_turn: Vec<AbilityActivationSnapshot>,
    pub(super) copy_effect: Option<CopiableCharacteristicsSnapshot>,
    pub(super) copied_from: Option<CopiedFromSnapshot>,
    pub(super) text_changes: Vec<BasicLandTypeChangeSnapshot>,
    pub(super) has_dynamic_characteristics: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ResolvedContinuousEffectSnapshot {
    pub(super) definition: AppliedEffectLocator,
    pub(super) source: AbilitySourceSnapshot,
    pub(super) timestamp: u64,
    pub(super) component_order: u16,
    pub(super) expiration: ContinuousEffectExpirationSnapshot,
    pub(super) operation: ResolvedContinuousOperationSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ResolvedPlayRestrictionSnapshot {
    pub(super) definition: AppliedEffectLocator,
    pub(super) source: AbilitySourceSnapshot,
    pub(super) affected_seat: usize,
    pub(super) timestamp: u64,
    pub(super) component_order: u16,
    pub(super) expiration: ContinuousEffectExpirationSnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum SetOperationSnapshot {
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
pub(super) enum ResolvedContinuousOperationSnapshot {
    AbilityAdd { grant_id: u8 },
    AbilityRemove,
    BasicLandTypes { operation: SetOperationSnapshot },
    CardTypes { operation: SetOperationSnapshot },
    Colors { operation: SetOperationSnapshot },
    CreatureTypes { operation: SetOperationSnapshot },
    ModifyPowerToughness { power: i16, toughness: i16 },
    Rule,
    SetBasePowerToughness { power: i16, toughness: i16 },
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AbilityActivationSnapshot {
    pub(super) origin: AbilityOriginSnapshot,
    pub(super) count: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CopiableCharacteristicsSnapshot {
    pub(super) definition: u16,
    pub(super) part_id: u8,
    pub(super) added_types: [bool; crate::card::CardType::COUNT],
    pub(super) added_abilities: Vec<CopiableAbilitySnapshot>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CopiableAbilitySnapshot {
    pub(super) origin: AbilityOriginSnapshot,
    pub(super) ability: AbilityLocator,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CopiedFromSnapshot {
    pub(super) definition: u16,
    pub(super) part_id: u8,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum ContinuousEffectExpirationSnapshot {
    EndOfTurn,
    UpkeepOf { seat: usize },
    TurnOf { seat: usize, turn: u32 },
    WhileSourceTapped,
    Never,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum RetiredObjectSnapshot {
    Card {
        card: DetachedCardSnapshot,
    },
    Stack {
        object: Box<DetachedStackSnapshot>,
    },
    /// Boxed: a retired permanent carries far more than a retired card, and
    /// the enum is stored in a vector of every retired object.
    Permanent {
        permanent: Box<DetachedPermanentSnapshot>,
        power: Option<i16>,
        toughness: Option<i16>,
        mana_value: u16,
        keywords: Vec<KeywordSnapshot>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub(super) struct DetachedPermanentSnapshot {
    pub(super) state: PermanentSnapshot,
    pub(super) definition: u16,
    pub(super) presented_part_id: u8,
    pub(super) controller: usize,
    pub(super) tapped: bool,
    pub(super) damage: u16,
    pub(super) attacking: bool,
    pub(super) attack_defender: Option<AttackDefenderSnapshot>,
    pub(super) blocked: bool,
    pub(super) blocking: Option<u32>,
    pub(super) activated_loyalty_this_turn: bool,
    pub(super) chosen_creature_type: Option<String>,
    pub(super) chosen_card_name: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum AttackDefenderSnapshot {
    Player { seat: usize },
    Planeswalker { object_id: u32 },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CombatDamageAssignmentSnapshot {
    pub(super) recipient: TargetSnapshot,
    pub(super) amount: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct EmblemSnapshot {
    pub(super) object_id: u32,
    pub(super) definition: u16,
    pub(super) owner: usize,
    pub(super) presented_part_id: u8,
    pub(super) timestamp: u64,
    pub(super) entered_controller_turn: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub(super) enum ResolvedEffectPaymentSnapshot {
    Mana(ManaCostSnapshot),
    Life(u16),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ReplacementEffectLocator {
    pub(super) ability: AbilityLocator,
    pub(super) effect_index: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AppliedEffectLocator {
    pub(super) ability: AbilityLocator,
    pub(super) effect_index: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PendingEventSnapshot {
    pub(super) entry: PendingBattlefieldEntrySnapshot,
    pub(super) applied: Vec<AbilitySourceSnapshot>,
    pub(super) effects: Vec<PendingReplacementEffectSnapshot>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PendingReplacementEffectSnapshot {
    pub(super) context: ReplacementEffectContextSnapshot,
    pub(super) effect: ReplacementEffectLocator,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ApplicableReplacementSnapshot {
    pub(super) context: ReplacementEffectContextSnapshot,
    pub(super) effect: ReplacementEffectLocator,
    pub(super) definition: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PendingBattlefieldEntrySnapshot {
    pub(super) permanent: DetachedPermanentSnapshot,
    pub(super) from: ZoneKindSnapshot,
    pub(super) completion: EntryCompletionSnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AbilitySourceSnapshot {
    pub(super) object: u32,
    pub(super) ability: AbilityOriginSnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum EntryCompletionSnapshot {
    LandPlayed { seat: usize },
    SpellResolved { card: u32, definition: u16 },
    AttachSource { source: u32 },
    Setup,
    None,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum ZoneKindSnapshot {
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Stack,
    Exile,
    Command,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AbilityLocator {
    pub(super) definition: u16,
    pub(super) part_id: u8,
    pub(super) ability_id: u8,
    pub(super) nested: Vec<usize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TargetSelectionSnapshot {
    pub(super) slot_id: u8,
    pub(super) targets: Vec<TargetSnapshot>,
    pub(super) amounts: Vec<u16>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum TargetSnapshot {
    Player { seat: SeatSnapshot },
    Card { object_id: u32 },
    Permanent { object_id: u32 },
    Spell { object_id: u32 },
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub(super) enum SeatSnapshot {
    #[serde(rename = "p1")]
    One,
    #[serde(rename = "p2")]
    Two,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TriggerContextSnapshot {
    pub(super) object: Option<u32>,
    pub(super) object_controller: Option<usize>,
    pub(super) event_player: Option<usize>,
    pub(super) amount: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct EffectResolutionContextSnapshot {
    pub(super) trigger: TriggerContextSnapshot,
    pub(super) single_objects: [Option<TargetSnapshot>; crate::ObjectBindingIndex::COUNT],
    pub(super) object_groups: [Vec<TargetSnapshot>; crate::ObjectSetBindingIndex::COUNT],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DecisionStateSnapshot {
    pub(super) preference: DecisionPreferenceSnapshot,
    /// Hidden-zone locations for cards whose identities are visible in the
    /// current decision. Reconstruction mints fresh hidden objects before it
    /// parses the continuation, so these origins let it preserve the public
    /// object ids without guessing that the deciding seat owns the zone.
    pub(super) card_origins: Vec<DecisionCardOriginSnapshot>,
    pub(super) continuation: DecisionContinuationSnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DecisionCardOriginSnapshot {
    pub(super) object_id: u32,
    pub(super) seat: usize,
    pub(super) zone: DecisionZoneSnapshot,
    /// Exact index within the named hidden collection. This keeps disclosed
    /// duplicate definitions and visible option order stable under a hidden
    /// hypothesis whose otherwise-unseen cards may be permuted.
    pub(super) index: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub(super) enum DecisionPreferenceSnapshot {
    Name(String),
    PreferOption {
        #[serde(rename = "preferOption")]
        prefer_option: u32,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum TurnKindSnapshot {
    Any,
    Regular,
    Extra,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ApplicableBeginTurnReplacementSnapshot {
    pub(super) source: AbilitySourceSnapshot,
    pub(super) controller: usize,
    pub(super) definition: u16,
    pub(super) effect: ReplacementEffectLocator,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DeferredBeginTurnEffectSnapshot {
    pub(super) replacement: ApplicableBeginTurnReplacementSnapshot,
    pub(super) effect: ScopedEffectSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum DecisionContinuationSnapshot {
    BeginTurn {
        player: usize,
        turn_kind: TurnKindSnapshot,
        applied: Vec<AbilitySourceSnapshot>,
        replacements: Vec<ApplicableBeginTurnReplacementSnapshot>,
        deferred: Vec<DeferredBeginTurnEffectSnapshot>,
    },
    SearchZone {
        controller: usize,
        source: ZoneKindSnapshot,
        destination: ZoneKindSnapshot,
        placement: ZonePlacementSnapshot,
        reveal: bool,
        shuffle: bool,
        /// Additive: a checkpoint written before fetch lands existed carries
        /// no flag and reconstructs as an untapped arrival.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        enters_tapped: bool,
    },
    ChooseCards {
        controller: usize,
        destination: ZoneKindSnapshot,
        placement: ZonePlacementSnapshot,
        reveal: bool,
    },
    DrawReplacement {
        player: usize,
        replacements: Vec<DrawReplacementSnapshot>,
    },
    BasicLandTypeTextChange {
        target: TargetSnapshot,
    },
    DiscardForEffect {
        player: usize,
        amount: usize,
        remaining: Vec<usize>,
        chosen: Vec<DiscardChoiceSnapshot>,
        cause: ZoneMoveCauseSnapshot,
    },
    GrislySalvage {
        player: usize,
        revealed: Vec<DetachedCardSnapshot>,
    },
    AugurOfBolas {
        player: usize,
        revealed: Vec<DetachedCardSnapshot>,
    },
    TopCardSelection {
        player: usize,
        revealed: Vec<DetachedCardSnapshot>,
        continuation: EffectContinuationSnapshot,
    },
    ChainLightning {
        player: usize,
        spell: DetachedStackSnapshot,
        targets: Vec<TargetSnapshot>,
    },
    Fork {
        player: usize,
        spell: DetachedStackSnapshot,
        target_lists: Vec<Vec<TargetSelectionSnapshot>>,
    },
    OptionalEffect {
        object: DetachedStackSnapshot,
        ability: AbilityLocator,
        context: EffectResolutionContextSnapshot,
        effect: ScopedEffectSnapshot,
    },
    ChooseForEffect {
        continuation: EffectContinuationSnapshot,
    },
    PayOr {
        player: usize,
        payment: ResolvedEffectPaymentSnapshot,
        object: DetachedStackSnapshot,
        ability: AbilityLocator,
        context: EffectResolutionContextSnapshot,
        definition: ScopedEffectSnapshot,
    },
    SplitForEffect {
        continuation: EffectContinuationSnapshot,
    },
    ChoosePileForEffect {
        first: Vec<TargetSnapshot>,
        second: Vec<TargetSnapshot>,
        continuation: EffectContinuationSnapshot,
    },
    BattlefieldEntryPayment {
        context: ReplacementEffectContextSnapshot,
        player: usize,
        payment: ResolvedEffectPaymentSnapshot,
        effect: ReplacementEffectLocator,
    },
    BattlefieldEntryReplacement {
        candidates: Vec<ApplicableReplacementSnapshot>,
    },
    BattlefieldEntryOptional {
        context: ReplacementEffectContextSnapshot,
        effect: ReplacementEffectLocator,
    },
    BattlefieldEntryScalarChoice {
        context: ReplacementEffectContextSnapshot,
        effect: ReplacementEffectLocator,
        choices: Vec<String>,
    },
    BattlefieldEntryCopy {
        choices: Vec<u32>,
        added_types: [bool; crate::card::CardType::COUNT],
    },
    TriggerOrder {
        batch: TriggerPlacementBatchSnapshot,
        remaining: Vec<TriggerPlacementBatchSnapshot>,
    },
    TriggerPlacement {
        trigger: PendingTriggerSnapshot,
        pending: Vec<PendingTriggerSnapshot>,
        remaining: Vec<TriggerPlacementBatchSnapshot>,
        candidates: Vec<TargetSnapshot>,
    },
    MiracleReveal {
        card: u32,
    },
    SeparateIntoPiles {
        resolving_controller: usize,
        subject: usize,
        items: Vec<DecisionOptionSnapshot>,
        on_complete: String,
    },
    ChoosePile {
        piles: PileSplitSnapshot,
        on_complete: String,
    },
    SacrificeOfChoice {
        followup: Option<EffectContinuationSnapshot>,
        optional: bool,
    },
    RecallDiscard {
        player: usize,
    },
    RecallReturn {
        player: usize,
    },
    Balance {
        controller: usize,
        phase: BalancePhaseSnapshot,
        task: BalanceTaskSnapshot,
        remaining: Vec<BalanceTaskSnapshot>,
    },
    SylvanOffer {
        player: usize,
    },
    SylvanSelect {
        player: usize,
        candidates: Vec<u32>,
        choices_left: usize,
    },
    SylvanMode {
        player: usize,
        card: u32,
        candidates: Vec<u32>,
        choices_left: usize,
    },
    TetravusDetach {
        source: u32,
    },
    TetravusAssemble {
        source: u32,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PileSplitSnapshot {
    pub(super) resolving_controller: usize,
    pub(super) subject: usize,
    pub(super) first: Vec<DecisionOptionSnapshot>,
    pub(super) second: Vec<DecisionOptionSnapshot>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DecisionOptionSnapshot {
    pub(super) id: u32,
    pub(super) label: String,
    pub(super) card: Option<DecisionCardSnapshot>,
    pub(super) members: Vec<DecisionCardSnapshot>,
    pub(super) ability_text: Option<String>,
    pub(super) zone: DecisionZoneSnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DecisionCardSnapshot {
    pub(super) object_id: u32,
    pub(super) definition: u16,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum BalancePhaseSnapshot {
    Lands,
    Hands,
    Creatures,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct BalanceTaskSnapshot {
    pub(super) player: usize,
    pub(super) prompt: String,
    pub(super) zone: DecisionZoneSnapshot,
    pub(super) cards: Option<Vec<DetachedCardSnapshot>>,
    pub(super) count: usize,
    pub(super) action: BalanceActionSnapshot,
    pub(super) cause: ZoneMoveCauseSnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum BalanceActionSnapshot {
    Sacrifice,
    Discard,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum DecisionZoneSnapshot {
    Hand,
    Graveyard,
    Battlefield,
    Stack,
    Library,
    Exile,
    OutsideGame,
    Command,
    DrawnThisStep,
    None,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum ZonePlacementSnapshot {
    Top,
    Bottom,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct EffectContinuationSnapshot {
    pub(super) object: DetachedStackSnapshot,
    pub(super) ability: AbilityLocator,
    pub(super) context: EffectResolutionContextSnapshot,
    pub(super) effect: ScopedEffectSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DiscardChoiceSnapshot {
    pub(super) player: usize,
    pub(super) cards: Option<Vec<u32>>,
    pub(super) count: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DetachedCardSnapshot {
    pub(super) object_id: u32,
    pub(super) definition: u16,
    pub(super) owner: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum ZoneMoveCauseSnapshot {
    Rules,
    Effect { controller: usize },
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ReplacementEffectContextSnapshot {
    pub(super) source: AbilitySourceSnapshot,
    pub(super) controller: usize,
}
