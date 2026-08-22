use serde::{Deserialize, Serialize};

use crate::CardDefinitionId;

// serde hands `skip_serializing_if` a reference, so this signature is fixed.
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero_u8(value: &u8) -> bool {
    *value == 0
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ExilePlayPermissionSnapshot {
    pub(super) card: u32,
    pub(super) player: usize,
    pub(super) cost: String,
    pub(super) until_end_of_turn: Option<(usize, u32)>,
    pub(super) adventure_return_only: bool,
    /// Additive: a checkpoint written before a permission could charge for
    /// itself restores with no surcharge, which is what every permission but
    /// Elite Spellbinder's carries anyway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) surcharge: Option<ManaCostSnapshot>,
    /// Additive: a checkpoint written before a permission could name the
    /// earliest turn it may be used restores without one, which is what
    /// every permission but a foretell carries anyway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) not_before_turn: Option<(usize, u32)>,
    /// Additive: a checkpoint written before a permission could say so
    /// restores face up, which every permission but a foretell was.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) face_down: bool,
    /// The holder's turn whose end step the permission runs to. Additive: a
    /// checkpoint written before any permission reached that far restores
    /// without one, which all of them did.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) until_holder_end_step: Option<(usize, u32)>,
}

/// Taken by reference because that is the signature serde's
/// `skip_serializing_if` requires.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn is_zero_u16(value: &u16) -> bool {
    *value == 0
}

mod balance;
mod continuation;
mod objects;
mod stack;
mod triggers;
pub(in crate::game::state_checkpoint) use stack::*;
pub(in crate::game::state_checkpoint) use triggers::*;

pub(super) use balance::{BalanceActionSnapshot, BalancePhaseSnapshot, BalanceTaskSnapshot};
pub(super) use continuation::DecisionContinuationSnapshot;
pub(super) use objects::{
    AbilityLocator, EmblemCharacteristicsLocator, FaceDownCharacteristicsSnapshot,
    ObjectCharacteristicsSnapshot, ObjectKindSnapshot, TokenCharacteristicsLocator,
};

use super::model_keyword::{KeywordSnapshot, UpkeepKeywordSnapshot};
pub(super) use super::model_ongoing::ResolvedOngoingEffectSnapshot;
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
    /// Damage each player has taken this turn, in total and per source
    /// group. Absent from checkpoints that predate the accumulators.
    #[serde(default)]
    pub(super) damage_taken_this_turn: [u16; 2],
    #[serde(default)]
    pub(super) damage_taken_by_group_this_turn: Vec<Vec<u16>>,
    pub(super) next_decision_id: u32,
    pub(super) next_trigger_id: u32,
    pub(super) next_continuous_effect_timestamp: u64,
    pub(super) consecutive_passes: u8,
    pub(super) attackers_declared: bool,
    pub(super) blockers_declared: bool,
    pub(super) untap_pending: bool,
    pub(super) cleanup_pending: bool,
    pub(super) mulligans: [u8; 2],
    pub(super) lands_played_this_turn: [u16; 2],
    pub(super) tried_to_draw_from_empty_library: [bool; 2],
    pub(super) mana: [Vec<ManaSnapshot>; 2],
    pub(super) creature_died_this_turn: bool,
    /// Additive: a checkpoint written before the count existed restores as
    /// zero, which is what a turn with no recorded deaths means anyway.
    #[serde(default, skip_serializing_if = "is_zero_u16")]
    pub(super) creatures_died_this_turn: u16,
    pub(super) linked_exiles: Vec<[u32; 2]>,
    /// Additive: a checkpoint written before the rule existed restores as
    /// false, which is what every ordinary turn means anyway.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) damage_cannot_be_prevented_this_turn: bool,
    /// Additive: a checkpoint written before exile permissions existed
    /// restores as empty, which is what a game with none of them means
    /// anyway. Each entry is the card, who may play it, whether it is free,
    /// the turn it lapses at the end of, and whether only an adventure's
    /// creature half may be played.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) exile_play_permissions: Vec<ExilePlayPermissionSnapshot>,
    /// Additive: a checkpoint written before the monarch existed restores
    /// with nobody wearing the crown, which is how every game starts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) monarch: Option<usize>,
    pub(super) sorcery_flash_grants: [u8; 2],
    pub(super) turn_phase_queue: Vec<TurnPhaseSnapshot>,
    pub(super) turn_phase_resume: Option<TurnPhaseResumeSnapshot>,
    /// Resolving play prohibitions. Static restrictions remain source-derived.
    pub(super) resolved_play_restrictions: Vec<ResolvedPlayRestrictionSnapshot>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) resolved_attack_restrictions: Vec<ResolvedAttackRestrictionSnapshot>,
    /// Resolving play permissions. Additive: a checkpoint written before
    /// anything could grant one restores with none, which is what every game
    /// with no such effect in it has.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) resolved_play_permissions: Vec<ResolvedPlayPermissionSnapshot>,
    pub(super) spells_cast_this_turn: [u16; 2],
    pub(super) spells_cast_last_turn: [u16; 2],
    pub(super) cards_drawn_this_turn: [u16; 2],
    pub(super) citys_blessing: [bool; 2],
    pub(super) permanent_left_battlefield_this_turn: [bool; 2],
    /// Additive: a checkpoint written before the turn tracked it restores
    /// with nobody's graveyard having lost a card, which is what every turn
    /// starts as anyway.
    #[serde(default)]
    pub(super) card_left_graveyard_this_turn: [bool; 2],
    pub(super) life_gained_this_turn: [u16; 2],
    pub(super) draw_step_draw_taken: [bool; 2],
    pub(super) drawn_this_turn: [Vec<u32>; 2],
    /// Compatibility-only wire member. Channel now reconstructs through
    /// `ongoing_effects`; current writers always emit false for both seats.
    pub(super) channel_active: [bool; 2],
    pub(super) defer_empty_library_loss: bool,
    pub(super) draw_replacements: [Vec<DrawReplacementSnapshot>; 2],
    pub(super) pending_combat_attackers: Vec<u32>,
    pub(super) combat_blocked_attackers: Vec<u32>,
    pub(super) extra_turns: Vec<usize>,
    pub(super) next_regular_player: usize,
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
    /// What each retired object became when it changed zones, for the
    /// objects a pending trigger might still name. Appended, so a checkpoint
    /// written before it existed still reads -- and reads as a game where no
    /// dying creature can find the card it became, which is what those
    /// checkpoints actually recorded.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) successors: Vec<SuccessorSnapshot>,
    pub(super) pending_events: Vec<PendingEventSnapshot>,
    pub(super) temporary_ability_grants: Vec<TemporaryAbilityGrantSnapshot>,
    /// Resolved duration-scoped effects that expose an activated ability.
    /// Additive: checkpoints written before these effects existed restore
    /// with none, which is the state of every game without one.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) ongoing_effects: Vec<ResolvedOngoingEffectSnapshot>,
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
        definition: CardDefinitionId,
        part_id: u8,
        ability_id: u8,
    },
    Token {
        part_id: u8,
        ability_id: u8,
    },
    Emblem {
        ability_id: u8,
    },
    FaceDown {
        ability_id: u8,
    },
    IntrinsicBasicLand {
        land_type: BasicLandTypeSnapshot,
    },
    IntrinsicCounter {
        /// The counter's serialized position, which is the same index the
        /// counter array beside it is written at.
        counter: usize,
    },
    Granted {
        source: u32,
        source_definition: CardDefinitionId,
        source_part_id: u8,
        source_ability_id: u8,
        grant_id: u8,
    },
    TokenGranted {
        source: u32,
        source_part_id: u8,
        source_ability_id: u8,
        grant_id: u8,
    },
    EmblemGranted {
        source: u32,
        source_ability_id: u8,
        grant_id: u8,
    },
    FaceDownGranted {
        source: u32,
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
    pub(super) object_kind: ObjectKindSnapshot,
    /// The authored token characteristics originally minted for this permanent.
    /// A token copy legitimately has none because its single-faced copy effect
    /// or frozen double-faced values supply its copiable characteristics;
    /// `object_kind` still records that it is a token.
    pub(super) token_characteristics: Option<TokenCharacteristicsLocator>,
    /// Both intrinsic faces of a token created as a copy of a double-faced
    /// permanent. Additive because older checkpoints could not represent this
    /// state faithfully at all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) double_faced_token_copy: Option<DoubleFacedCopiableCharacteristicsSnapshot>,
    /// Copiable values supplied by the rule or effect that made this
    /// permanent face down. `None` means face up.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) face_down: Option<FaceDownCharacteristicsSnapshot>,
    /// Whether the face-down mechanism grants a mana-cost turn-up permission.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) turn_up_for_mana_cost: bool,
    pub(super) presented_part_id: u8,
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
    /// The X the spell that made this permanent was cast for.
    #[serde(default, skip_serializing_if = "is_zero_u16")]
    pub(super) cast_x: u16,
    /// The alternative this permanent's spell was cast with, by its stable
    /// name. Stored as a string so the wire form does not depend on the
    /// order of a catalog enum.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) cast_alternative: Option<String>,
    /// Which zone this spell was cast from, by its stable label. Additive:
    /// a checkpoint written before the zone was recorded restores as
    /// nothing, which is what a permanent nobody cast carries anyway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) cast_from_zone: Option<String>,
    pub(super) destroy_at_end: bool,
    pub(super) counters: Vec<u16>,
    pub(super) attached_to: Option<u32>,
    pub(super) reconfigured_timestamp: Option<u64>,
    pub(super) exile_instead_of_dying: bool,
    pub(super) combat_damage_assignment: Vec<CombatDamageAssignmentSnapshot>,
    pub(super) regeneration_shields: u8,
    pub(super) attacked_this_turn: bool,
    /// Additive: a checkpoint written before exert existed restores with
    /// nothing exerted, which is what every board without one is.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) exerted: bool,
    /// Additive in the same way: a checkpoint written before saddling
    /// existed restores with nothing saddled.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) saddled: bool,
    pub(super) attacks_this_turn: u8,
    /// The seat that controlled this permanent the last time it attacked, and
    /// their turn count then. Absent means it has never attacked, which is
    /// what a checkpoint written before this was recorded also means.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) last_attacked_turn: Option<(usize, u32)>,
    pub(super) damage_sources: Vec<u32>,
    #[serde(default)]
    pub(super) was_dealt_damage_this_turn: bool,
    #[serde(default)]
    pub(super) dealt_damage_this_turn: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) paired_with: Option<u32>,
    pub(super) dealt_damage_to_opponent_this_turn: bool,
    pub(super) deathtouch_damage: bool,
    pub(super) created_by: Option<u32>,
    pub(super) temporary_keywords: Vec<KeywordSnapshot>,
    pub(super) keywords_until_upkeep_of: Vec<UpkeepKeywordSnapshot>,
    pub(super) resolved_continuous_effects: Vec<ResolvedContinuousEffectSnapshot>,
    pub(super) activations_this_turn: Vec<AbilityActivationSnapshot>,
    /// Additive: a payload written before any ability capped its own
    /// triggering carries none, which is a turn in which none has.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) triggers_this_turn: Vec<AbilityActivationSnapshot>,
    /// Absent from a payload written before any ability counted its own
    /// resolutions, which is why it defaults rather than being required.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) resolutions_this_turn: Vec<AbilityActivationSnapshot>,
    /// Additive: a payload written before either flag existed restores both
    /// as false, which is what an ordinary permanent means anyway.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) cast_at_instant_speed: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) became_aura: bool,
    pub(super) copy_effect: Option<CopiableCharacteristicsSnapshot>,
    pub(super) copy_expiration: Option<ContinuousEffectExpirationSnapshot>,
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
pub(super) struct ResolvedPlayPermissionSnapshot {
    pub(super) definition: AppliedEffectLocator,
    pub(super) source: AbilitySourceSnapshot,
    pub(super) affected_seat: usize,
    pub(super) expiration: ContinuousEffectExpirationSnapshot,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ResolvedAttackRestrictionSnapshot {
    pub(super) definition: AppliedEffectLocator,
    pub(super) source: AbilitySourceSnapshot,
    pub(super) affected_seat: usize,
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

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AbilityActivationSnapshot {
    pub(super) origin: AbilityOriginSnapshot,
    pub(super) count: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CopiableCharacteristicsSnapshot {
    pub(super) base: ObjectCharacteristicsSnapshot,
    pub(super) added_types: [bool; crate::card::CardType::COUNT],
    pub(super) added_abilities: Vec<CopiableAbilitySnapshot>,
    /// Additive: a checkpoint written before a copy could keep its own
    /// subtypes restores without them, which is what every copy did then.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) retain_printed_subtypes: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DoubleFacedCopiableCharacteristicsSnapshot {
    pub(super) modal: bool,
    pub(super) front_part_id: u8,
    pub(super) back_part_id: u8,
    pub(super) front: CopiableCharacteristicsSnapshot,
    pub(super) back: CopiableCharacteristicsSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CopiableAbilitySnapshot {
    pub(super) origin: AbilityOriginSnapshot,
    pub(super) ability: AbilityLocator,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CopiedFromSnapshot {
    pub(super) characteristics: ObjectCharacteristicsSnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum ContinuousEffectExpirationSnapshot {
    EndOfTurn,
    EndOfCombat,
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
    pub(super) controller: usize,
    pub(super) tapped: bool,
    pub(super) damage: u16,
    pub(super) attacking: bool,
    pub(super) attack_defender: Option<AttackDefenderSnapshot>,
    pub(super) blocked: bool,
    /// Every attacker this creature is blocking. A list because a band is
    /// blocked as a group and one creature may be allowed several blocks.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) blocking: Vec<u32>,
    /// Whether it blocked something that has since left combat, which the
    /// list above can no longer say. Absent from a payload written before the
    /// distinction existed, and from the ordinary case where the list answers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) blocking_this_combat: Option<bool>,
    /// The attacking band this creature is in, shared by every member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) attacking_band: Option<u8>,
    pub(super) activated_loyalty_this_turn: bool,
    pub(super) chosen_creature_type: Option<String>,
    /// The basic land type this permanent was told to be as it entered.
    pub(super) chosen_basic_land_type: Option<BasicLandTypeSnapshot>,
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
    pub(super) characteristics: EmblemCharacteristicsLocator,
    pub(super) owner: usize,
    pub(super) timestamp: u64,
    pub(super) entered_controller_turn: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub(super) enum ResolvedEffectPaymentSnapshot {
    Mana(ManaCostSnapshot),
    Life(u16),
    Energy(u16),
    /// Appended after the first two, so a checkpoint written before this
    /// payment existed still reads as one of them.
    Mill(u16),
    Discard(u16),
    /// Which cards match is read back from the authored effect rather than
    /// carried here: the predicate is a static definition, and the payment
    /// this describes is only ever restored beside the ability that named it.
    DiscardMatching,
    /// Likewise: how much can be paid is read off the payer's mana rather
    /// than written down, because the options are rebuilt from it.
    ChosenGenericMana,
    /// And likewise for the permanents a return payment can name.
    ReturnPermanentMatching,
    /// The same, for the one it sacrifices.
    SacrificePermanentMatching,
    SacrificeCreaturesWithTotalPower(u16),
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
    pub(super) presentation: ObjectCharacteristicsSnapshot,
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
    LandPlayed {
        seat: usize,
    },
    SpellResolved {
        card: u32,
        definition: CardDefinitionId,
    },
    AttachSource {
        source: u32,
    },
    AttachToHost {
        host: u32,
    },
    Attacking {
        defender: AttackDefenderSnapshot,
    },
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
    pub(super) options: Vec<DecisionOptionSnapshot>,
    /// Hidden-zone locations for cards whose identities are visible in the
    /// current decision. Reconstruction mints fresh hidden objects before it
    /// parses the continuation, so these origins let it preserve the public
    /// object ids without guessing that the deciding seat owns the zone.
    pub(super) card_origins: Vec<DecisionCardOriginSnapshot>,
    pub(super) continuation: DecisionContinuationSnapshot,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct DecisionCardOriginSnapshot {
    pub(in crate::game::state_checkpoint) object_id: u32,
    pub(in crate::game::state_checkpoint) seat: usize,
    pub(in crate::game::state_checkpoint) zone: DecisionZoneSnapshot,
    /// Exact index within the named hidden collection. This keeps disclosed
    /// duplicate definitions and visible option order stable under a hidden
    /// hypothesis whose otherwise-unseen cards may be permuted.
    pub(in crate::game::state_checkpoint) index: usize,
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
    pub(super) presentation: ObjectCharacteristicsSnapshot,
    pub(super) effect: ReplacementEffectLocator,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DeferredBeginTurnEffectSnapshot {
    pub(super) replacement: ApplicableBeginTurnReplacementSnapshot,
    pub(super) effect: ScopedEffectSnapshot,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DecisionCardSnapshot {
    pub(super) object_id: u32,
    pub(super) characteristics: ObjectCharacteristicsSnapshot,
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
    /// Whether the follow-up reads the sacrificed permanent's toughness
    /// rather than its power. Absent means power, which is what every
    /// continuation written before toughness was readable meant.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) reads_toughness: bool,
}

/// One retired object and the object it became.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SuccessorSnapshot {
    pub(super) retired: u32,
    pub(super) became: u32,
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
    pub(super) definition: CardDefinitionId,
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
