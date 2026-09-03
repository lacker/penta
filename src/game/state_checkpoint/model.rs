use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};

use crate::{CardDefinitionId, CounterKind};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct CounterKindSnapshot(pub(super) CounterKind);

impl Serialize for CounterKindSnapshot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.name())
    }
}

impl<'de> Deserialize<'de> for CounterKindSnapshot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        CounterKind::from_name(&name)
            .map(Self)
            .ok_or_else(|| D::Error::custom(format!("unknown counter name {name}")))
    }
}

mod continuation;
mod continuous;
mod copy;
mod decision_options;
pub(super) use decision_options::*;
mod emptiness;
mod exile_permissions;
pub(in crate::game::state_checkpoint) use exile_permissions::ExilePlayPermissionSnapshot;
mod objects;
mod stack;
mod triggers;
pub(in crate::game::state_checkpoint) use emptiness::is_zero_u16;
pub(in crate::game::state_checkpoint) use stack::*;
pub(in crate::game::state_checkpoint) use triggers::*;

pub(super) use continuation::DecisionContinuationSnapshot;
pub(super) use continuation::PregameAbilityActionSnapshot;
pub(in crate::game::state_checkpoint) use continuous::*;
pub(super) use copy::{
    CopiableCharacteristicsSnapshot, DoubleFacedCopiableCharacteristicsSnapshot,
};
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
    /// The companions each player may still take from outside the game,
    /// named by definition. Additive: a checkpoint written before companions
    /// existed restores a game in which nobody brought one, which is what
    /// every game before them was.
    #[serde(default, skip_serializing_if = "emptiness::is_empty_pair_of_vectors")]
    pub(super) companions: [Vec<u64>; 2],
    /// The creature subtypes each seat attacked with this turn. Additive: a
    /// checkpoint written before it existed restores a turn nobody is
    /// recorded as having attacked in, which is what it meant.
    #[serde(default, skip_serializing_if = "emptiness::is_empty_pair_of_names")]
    pub(super) attacked_subtypes_this_turn: [Vec<String>; 2],
    pub(super) tried_to_draw_from_empty_library: [bool; 2],
    pub(super) mana: [Vec<ManaSnapshot>; 2],
    pub(super) creature_died_this_turn: bool,
    /// Additive: a checkpoint written before the count existed restores as
    /// zero, which is what a turn with no recorded deaths means anyway.
    #[serde(default, skip_serializing_if = "emptiness::is_zero_u16")]
    pub(super) creatures_died_this_turn: u16,
    pub(super) linked_exiles: Vec<[u32; 2]>,
    /// Uses of a limited graveyard permission this turn. Additive: a
    /// checkpoint written before it existed restores a turn in which nothing
    /// had been played that way yet.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) graveyard_permission_uses: Vec<[u32; 2]>,
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
    /// Who has been told they cannot gain life for the rest of the game.
    /// Additive: a checkpoint written before this existed means nobody had
    /// been, which is where every game starts.
    #[serde(default, skip_serializing_if = "emptiness::is_unset_for_both")]
    pub(super) cannot_gain_life: [bool; 2],
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
    /// Resolving player protections. Additive for the same reason: nothing
    /// could protect a player before, so a checkpoint without them restores
    /// a game in which nobody is protected.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) resolved_player_protections: Vec<ResolvedPlayerProtectionSnapshot>,
    /// Simple resolving player rules. Additive: an older checkpoint had none
    /// because no supported card could create one.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) resolved_player_rules: Vec<ResolvedPlayerRuleSnapshot>,
    pub(super) spells_cast_this_turn: [u16; 2],
    pub(super) spells_cast_last_turn: [u16; 2],
    /// Additive: older checkpoints have no predicate-filterable cast history.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) spell_cast_history_this_turn: Vec<u32>,
    /// Spells cast over the whole game. Additive: a checkpoint written before
    /// it was counted restores a game in which nobody has cast anything,
    /// which is only wrong for a card that asks, and only until one is cast.
    #[serde(default)]
    pub(super) spells_cast_this_game: [u16; 2],
    pub(super) cards_drawn_this_turn: [u16; 2],
    pub(super) citys_blessing: [bool; 2],
    pub(super) permanent_left_battlefield_this_turn: [bool; 2],
    /// Additive: a checkpoint written before the turn tracked it restores
    /// with nobody's graveyard having lost a card, which is what every turn
    /// starts as anyway.
    #[serde(default)]
    pub(super) card_left_graveyard_this_turn: [bool; 2],
    pub(super) life_gained_this_turn: [u16; 2],
    /// Whether each player has lost life this turn. Additive: a checkpoint
    /// written before it existed restores a turn in which nobody has.
    #[serde(default)]
    pub(super) lost_life_this_turn: [bool; 2],
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
    #[serde(rename = "temporaryAbilityGrants")]
    pub(super) nonbattlefield_ability_grants: Vec<NonbattlefieldAbilityGrantSnapshot>,
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
pub(super) struct NonbattlefieldAbilityGrantSnapshot {
    pub(super) object: u32,
    pub(super) ability: AbilityLocator,
    #[serde(default = "default_nonbattlefield_grant_expiration")]
    pub(super) expiration: ContinuousEffectExpirationSnapshot,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) source: Option<AbilityOriginSnapshot>,
}

const fn default_nonbattlefield_grant_expiration() -> ContinuousEffectExpirationSnapshot {
    ContinuousEffectExpirationSnapshot::EndOfTurn
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
        counter: CounterKindSnapshot,
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
    OpeningHand { seat: usize },
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
pub(super) struct CounterSnapshot {
    pub(super) name: String,
    pub(super) count: u16,
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
    /// The size an X/X token came out at. The locator above names the
    /// authored token, which carries the two amounts rather than a size;
    /// these are the numbers they came to, which is a copiable value of the
    /// token and cannot be worked out again from the board. Additive, and
    /// absent for every token whose size is printed on it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) token_stats: Option<[i16; 2]>,
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
    /// The game turn this permanent entered, for the clauses that ask about
    /// the turn itself rather than about its controller's turn count.
    /// Additive: a checkpoint written before it existed restores a permanent
    /// that entered on turn zero, which is what one that has been there all
    /// along would say anyway.
    #[serde(default, skip_serializing_if = "emptiness::is_zero_turn")]
    pub(super) entered_turn: u32,
    /// Detained until this seat's next turn, with the turn count it landed on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) detained_until_turn_of: Option<(usize, u32)>,
    /// Untap steps this permanent still owes before it untaps normally.
    #[serde(default, skip_serializing_if = "emptiness::is_zero_u8")]
    pub(super) skipped_untap_steps: u8,
    pub(super) control_reverts_to: Option<usize>,
    /// The permanent sustaining a duration-scoped control change, absent for
    /// the turn-scoped form and for everything untouched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) control_source: Option<u32>,
    /// Whether that holder also has to stay tapped.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) control_requires_source_tapped: bool,
    /// Whether the control source has to remain attached to this permanent.
    /// Additive: older checkpoints contain no static attachment control state.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) control_requires_source_attached: bool,
    pub(super) chosen_player: Option<usize>,
    /// The X the spell that made this permanent was cast for.
    #[serde(default, skip_serializing_if = "emptiness::is_zero_u16")]
    pub(super) cast_x: u16,
    /// How many times a repeatable optional additional cost was paid for
    /// that spell. Additive, and absent for everything nobody kicked.
    #[serde(default, skip_serializing_if = "emptiness::is_zero_u16")]
    pub(super) cast_kicks: u16,
    /// Per-clause optional additional-cost payment counts. Additive; older
    /// checkpoints retain only the aggregate repeatable-cost count above.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) cast_additional_costs: Vec<u16>,
    /// How many colours paid for it. Additive, and absent for everything
    /// nobody cast.
    #[serde(default, skip_serializing_if = "emptiness::is_zero_u16")]
    pub(super) cast_colors: u16,
    /// Exact colors spent, replacing the legacy count above. The count stays
    /// on the wire so older readers can still restore sunburst correctly.
    #[serde(default, skip_serializing_if = "stack::no_colors_spent")]
    pub(super) cast_colors_of_mana_spent: [bool; 5],
    #[serde(default, skip_serializing_if = "emptiness::is_zero_u16")]
    pub(super) cast_phyrexian_symbols_paid_with_life: u16,
    /// The alternative this permanent's spell was cast with, by its stable
    /// name. Stored as a string so the wire form does not depend on the
    /// order of a catalog enum.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) cast_alternative: Option<String>,
    /// Semantic cast facts by stable name. Additive: an absent collection is
    /// an ordinary untagged cast.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) cast_tags: Vec<String>,
    /// Exile-zone identities of cards used to pay this spell's costs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) cast_exiled_payment_cards: Vec<u32>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) cast_via_flashback: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) cast_via_suspend: bool,
    /// Which zone this spell was cast from, by its stable label. Additive:
    /// a checkpoint written before the zone was recorded restores as
    /// nothing, which is what a permanent nobody cast carries anyway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) cast_from_zone: Option<String>,
    pub(super) destroy_at_end: bool,
    pub(super) counters: Vec<CounterSnapshot>,
    pub(super) attached_to: Option<u32>,
    /// The player a player-enchanting Aura is attached to. Additive: older
    /// checkpoints restore no such Auras because none were executable then.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) attached_player: Option<usize>,
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
    /// The exhaust abilities this permanent has already spent. Additive:
    /// a checkpoint written before exhaust existed restores with none
    /// spent, which is what every board without one has.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) exhausted: Vec<AbilityOriginSnapshot>,
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
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) suspend_haste: bool,
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

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AbilityActivationSnapshot {
    pub(super) origin: AbilityOriginSnapshot,
    pub(super) count: u8,
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
    /// The color this permanent was told to remember as it entered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) chosen_color: Option<ManaColorSnapshot>,
    pub(super) chosen_card_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) chosen_card_name_binding: Option<String>,
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
    /// The game turn this permanent entered, for the clauses that ask about
    /// the turn itself rather than about its controller's turn count.
    /// Additive: a checkpoint written before it existed restores a permanent
    /// that entered on turn zero, which is what one that has been there all
    /// along would say anyway.
    #[serde(default, skip_serializing_if = "emptiness::is_zero_turn")]
    pub(super) entered_turn: u32,
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
    /// The same for energy: how much can be paid is read off the payer's
    /// counters rather than written down.
    ChosenEnergy,
    RemoveAnyNumberOfCounters {
        object: u32,
        kind: CounterKindSnapshot,
    },
    /// Compatibility spelling for the matching-permanent move payment. The
    /// authored effect restores its destination, so the old hand-only tag can
    /// represent the generalized internal cost without changing wire data.
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum EffectBindingSnapshot {
    Object { object: Option<TargetSnapshot> },
    Objects { objects: Vec<TargetSnapshot> },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(super) enum BindingSnapshot {
    Binding { label: String },
    ParentBinding,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub(super) enum SeatSnapshot {
    #[serde(rename = "p1")]
    One,
    #[serde(rename = "p2")]
    Two,
}

include!("model_trigger_context.rs");

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct EffectResolutionContextSnapshot {
    pub(super) trigger: TriggerContextSnapshot,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) replaced_draw: Option<ReplacedDrawSnapshot>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) chosen_counter: Option<CounterKindSnapshot>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) parent_object: Option<TargetSnapshot>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) parent_objects: Vec<TargetSnapshot>,
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub(super) bindings: std::collections::BTreeMap<String, EffectBindingSnapshot>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ReplacedDrawSnapshot {
    pub(super) player: usize,
    pub(super) applied: Vec<AbilitySourceSnapshot>,
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

include!("model/replacement_effect_context.rs");
