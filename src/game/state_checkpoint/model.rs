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

mod game;
pub(super) use game::GameSnapshot;

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

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
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

include!("model/permanent.rs");

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

include!("model/payments.rs");

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
    pub(super) paid_amount: Option<u16>,
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
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub(super) card_name_bindings: std::collections::BTreeMap<String, String>,
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
