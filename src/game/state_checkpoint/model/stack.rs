//! Stack-object snapshots.
//!
//! Split out of the parent module for the source-size budget; these are the
//! shapes describing what is on the stack and how each object got there.

use serde::{Deserialize, Serialize};

use super::{
    AbilityLocator, AbilityOriginSnapshot, AbilitySourceSnapshot, AppliedEffectLocator,
    BasicLandTypeSnapshot, ContinuousEffectExpirationSnapshot, DecisionCardOriginSnapshot,
    EffectResolutionContextSnapshot, FaceDownCharacteristicsSnapshot, ManaColorSnapshot,
    ManaSourceSnapshot, ObjectCharacteristicsSnapshot, ObjectKindSnapshot, TargetSelectionSnapshot,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub(in crate::game::state_checkpoint) struct StackSnapshot {
    pub(in crate::game::state_checkpoint) object_id: u32,
    pub(in crate::game::state_checkpoint) kind: StackObjectKindSnapshot,
    pub(in crate::game::state_checkpoint) owner: usize,
    pub(in crate::game::state_checkpoint) object_kind: ObjectKindSnapshot,
    pub(in crate::game::state_checkpoint) ability_payload: Option<StackAbilitySnapshot>,
    pub(in crate::game::state_checkpoint) requires_retired_object: bool,
    pub(in crate::game::state_checkpoint) has_runtime_overrides: bool,
    pub(in crate::game::state_checkpoint) applied_effects: Vec<AppliedStackEffectSnapshot>,
    pub(in crate::game::state_checkpoint) text_changes: Vec<TextChangeSnapshot>,
    pub(in crate::game::state_checkpoint) colors: Option<[bool; 5]>,
    /// Which colours paid for this spell, for converge. Additive: a payload
    /// written before converge existed carries none, and reconstructs as a
    /// spell nothing was spent on.
    #[serde(default, skip_serializing_if = "no_colors_spent")]
    pub(in crate::game::state_checkpoint) colors_of_mana_spent: [bool; 5],
    /// Additive payment count used by Compleated. Older checkpoints restore
    /// an ordinary mana-paid spell.
    #[serde(default, skip_serializing_if = "super::is_zero_u16")]
    pub(in crate::game::state_checkpoint) phyrexian_symbols_paid_with_life: u16,
    pub(in crate::game::state_checkpoint) cast_via_flashback: bool,
    #[serde(default)]
    pub(in crate::game::state_checkpoint) cast_via_suspend: bool,
    /// Additive: a payload written before the flag existed restores as
    /// false, which is what an ordinary sorcery-speed cast means anyway.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(in crate::game::state_checkpoint) cast_at_instant_speed: bool,
    /// Which zone this spell was cast from, by its stable label. Additive:
    /// a checkpoint written before the zone was recorded restores as
    /// nothing, which is what a permanent nobody cast carries anyway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) cast_from_zone: Option<String>,
    /// Retired cast-tag wire field. Kept readable until the next checkpoint
    /// cleanup; newly written snapshots leave it empty.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) cast_tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) cast_alternative: Option<String>,
    #[serde(default, skip_serializing_if = "super::is_zero_u16")]
    pub(in crate::game::state_checkpoint) cast_x: u16,
    #[serde(default, skip_serializing_if = "super::is_zero_u16")]
    pub(in crate::game::state_checkpoint) cast_repeatable_additional_costs: u16,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) cast_additional_costs: Vec<u16>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) cast_exiled_payment_cards: Vec<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) face_down: Option<FaceDownCharacteristicsSnapshot>,
    pub(in crate::game::state_checkpoint) is_copy: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(in crate::game::state_checkpoint) enum TextWordChangeSnapshot {
    BasicLandType {
        from: BasicLandTypeSnapshot,
        to: BasicLandTypeSnapshot,
    },
    Color {
        from: ManaColorSnapshot,
        to: ManaColorSnapshot,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct TextChangeSnapshot {
    pub(in crate::game::state_checkpoint) word: TextWordChangeSnapshot,
    pub(in crate::game::state_checkpoint) expiration: ContinuousEffectExpirationSnapshot,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct StackAbilitySnapshot {
    pub(in crate::game::state_checkpoint) ability_locator: Option<AbilityLocator>,
    pub(in crate::game::state_checkpoint) target_definition_locator: Option<AbilityLocator>,
    pub(in crate::game::state_checkpoint) origin: AbilityOriginSnapshot,
    pub(in crate::game::state_checkpoint) presentation: ObjectCharacteristicsSnapshot,
    pub(in crate::game::state_checkpoint) target_selections: Vec<TargetSelectionSnapshot>,
    pub(in crate::game::state_checkpoint) context: EffectResolutionContextSnapshot,
    pub(in crate::game::state_checkpoint) mode_effects: Vec<ScopedEffectSnapshot>,
    pub(in crate::game::state_checkpoint) x: u16,
    /// What this activation's sacrificed costs added up to in mana value.
    /// Additive: a checkpoint written before abilities read their own costs
    /// back restores zero, which is what every ability that sacrifices
    /// nothing carries anyway.
    #[serde(default, skip_serializing_if = "super::is_zero_u16")]
    pub(in crate::game::state_checkpoint) sacrificed_mana_value: u16,
    /// Where the ability's source card sits when it is somewhere the viewer
    /// cannot read -- a library, or somebody else's hand, which is where a
    /// Miracle's revealed card is while its trigger waits.
    ///
    /// The importer mints those zones from the supplied hypothesis, so the
    /// source's own object id means nothing there and the position is what
    /// binds the two. Additive: absent for every source on a battlefield, in
    /// a graveyard, or otherwise already public.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) source_origin: Option<DecisionCardOriginSnapshot>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub(in crate::game::state_checkpoint) struct DetachedStackSnapshot {
    pub(in crate::game::state_checkpoint) object_id: u32,
    pub(in crate::game::state_checkpoint) kind: StackObjectKindSnapshot,
    pub(in crate::game::state_checkpoint) object_kind: ObjectKindSnapshot,
    pub(in crate::game::state_checkpoint) owner: usize,
    pub(in crate::game::state_checkpoint) source: Option<u32>,
    pub(in crate::game::state_checkpoint) ability_payload: Option<StackAbilitySnapshot>,
    pub(in crate::game::state_checkpoint) controller: usize,
    pub(in crate::game::state_checkpoint) signature: Option<CastSignatureSnapshot>,
    pub(in crate::game::state_checkpoint) chosen_permanents: Vec<u32>,
    pub(in crate::game::state_checkpoint) has_runtime_overrides: bool,
    pub(in crate::game::state_checkpoint) applied_effects: Vec<AppliedStackEffectSnapshot>,
    pub(in crate::game::state_checkpoint) text_changes: Vec<TextChangeSnapshot>,
    pub(in crate::game::state_checkpoint) colors: Option<[bool; 5]>,
    /// Which colours paid for this spell, for converge. Additive: a payload
    /// written before converge existed carries none, and reconstructs as a
    /// spell nothing was spent on.
    #[serde(default, skip_serializing_if = "no_colors_spent")]
    pub(in crate::game::state_checkpoint) colors_of_mana_spent: [bool; 5],
    /// Additive payment count used by Compleated. Older checkpoints restore
    /// an ordinary mana-paid spell.
    #[serde(default, skip_serializing_if = "super::is_zero_u16")]
    pub(in crate::game::state_checkpoint) phyrexian_symbols_paid_with_life: u16,
    pub(in crate::game::state_checkpoint) cast_via_flashback: bool,
    #[serde(default)]
    pub(in crate::game::state_checkpoint) cast_via_suspend: bool,
    /// Additive: a payload written before the flag existed restores as
    /// false, which is what an ordinary sorcery-speed cast means anyway.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(in crate::game::state_checkpoint) cast_at_instant_speed: bool,
    /// Which zone this spell was cast from, by its stable label. Additive:
    /// a checkpoint written before the zone was recorded restores as
    /// nothing, which is what a permanent nobody cast carries anyway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) cast_from_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) cast_tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) cast_alternative: Option<String>,
    #[serde(default, skip_serializing_if = "super::is_zero_u16")]
    pub(in crate::game::state_checkpoint) cast_x: u16,
    #[serde(default, skip_serializing_if = "super::is_zero_u16")]
    pub(in crate::game::state_checkpoint) cast_repeatable_additional_costs: u16,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) cast_additional_costs: Vec<u16>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) cast_exiled_payment_cards: Vec<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) face_down: Option<FaceDownCharacteristicsSnapshot>,
    pub(in crate::game::state_checkpoint) is_copy: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct AppliedStackEffectSnapshot {
    pub(in crate::game::state_checkpoint) source: Option<ManaSourceSnapshot>,
    pub(in crate::game::state_checkpoint) effect: AppliedEffectLocator,
    /// The ability that granted a non-mana rider. Additive: a payload
    /// written before permissions could grant anything carries none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) granting: Option<AbilitySourceSnapshot>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) enum StackObjectKindSnapshot {
    Spell,
    ActivatedAbility,
    TriggeredAbility,
    ReplacementEffect,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct CastSignatureSnapshot {
    pub(in crate::game::state_checkpoint) play_option: u8,
    pub(in crate::game::state_checkpoint) form: SpellFormSnapshot,
    pub(in crate::game::state_checkpoint) modes: Vec<u8>,
    pub(in crate::game::state_checkpoint) alternative_cost: Option<u8>,
    pub(in crate::game::state_checkpoint) additional_costs: Vec<u8>,
    pub(in crate::game::state_checkpoint) x: u16,
    pub(in crate::game::state_checkpoint) targets: Vec<TargetSelectionSnapshot>,
    /// The cards spliced onto this spell as it was cast. Absent from every
    /// signature written before splice existed, all of which had none.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) spliced: Vec<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(in crate::game::state_checkpoint) enum SpellFormSnapshot {
    Part { part_id: u8 },
    Combined { part_ids: Vec<u8> },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct ManaCostSnapshot {
    pub(in crate::game::state_checkpoint) generic: u16,
    pub(in crate::game::state_checkpoint) white: u16,
    pub(in crate::game::state_checkpoint) blue: u16,
    pub(in crate::game::state_checkpoint) black: u16,
    pub(in crate::game::state_checkpoint) red: u16,
    pub(in crate::game::state_checkpoint) green: u16,
    /// Additive: a checkpoint written before `{C}` existed restores as zero,
    /// which is what every cost without one carries anyway.
    #[serde(default, skip_serializing_if = "super::is_zero_u16")]
    pub(in crate::game::state_checkpoint) colorless: u16,
    pub(in crate::game::state_checkpoint) hybrid: Vec<u16>,
    /// Flexible symbols added after ordinary two-color hybrid. Older
    /// checkpoints carry none.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) additional_flexible: Vec<u16>,
    pub(in crate::game::state_checkpoint) variable_x: bool,
    pub(in crate::game::state_checkpoint) x_multiplier: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct ScopedEffectSnapshot {
    pub(in crate::game::state_checkpoint) path: Vec<usize>,
    pub(in crate::game::state_checkpoint) target_base: usize,
}

/// Elides the common case: nothing but a converge spell records what paid
/// for it, so every other stack object writes no field at all.
// serde hands this a reference, and clippy would rather it were a value;
// the wrapper is the cheapest way to satisfy both.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn no_colors_spent(colors: &[bool; 5]) -> bool {
    colors.iter().all(|spent| !*spent)
}
