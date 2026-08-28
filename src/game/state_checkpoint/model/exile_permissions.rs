//! The exile-play permission on the wire.
//!
//! Split out of the parent module for the source-size budget.

use serde::{Deserialize, Serialize};

use super::ManaCostSnapshot;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub(in crate::game::state_checkpoint) struct ExilePlayPermissionSnapshot {
    pub(in crate::game::state_checkpoint) card: u32,
    pub(in crate::game::state_checkpoint) player: usize,
    pub(in crate::game::state_checkpoint) cost: String,
    pub(in crate::game::state_checkpoint) until_end_of_turn: Option<(usize, u32)>,
    pub(in crate::game::state_checkpoint) adventure_return_only: bool,
    /// Additive: a checkpoint written before a permission could charge for
    /// itself restores with no surcharge, which is what every permission but
    /// Elite Spellbinder's carries anyway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) surcharge: Option<ManaCostSnapshot>,
    /// Additive: a checkpoint written before a permission could name the
    /// earliest turn it may be used restores without one, which is what
    /// every permission but a foretell carries anyway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) not_before_turn: Option<(usize, u32)>,
    /// Additive: a checkpoint written before a permission could say so
    /// restores face up, which every permission but a foretell was.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(in crate::game::state_checkpoint) face_down: bool,
    /// Additive: a checkpoint written before hideaway existed carries no
    /// look-only permission, which is what every game without one has.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(in crate::game::state_checkpoint) hidden_only: bool,
    /// Additive: a checkpoint written before a face-down exile could hide
    /// from its own owner restores as one its owner may look at, which is
    /// what every face-down exile was until Memory Jar's.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(in crate::game::state_checkpoint) hidden_from_owner: bool,
    /// Additive: whether the permission stops at casting. A checkpoint
    /// written before the distinction existed restores as one that reaches a
    /// land too, which is what every permission was read as.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(in crate::game::state_checkpoint) cast_only: bool,
    /// Additive: whether mana spent on this card may be of any colour.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(in crate::game::state_checkpoint) spend_any_color: bool,
    /// Additive: the creature type this permission asks its holder to have
    /// attacked with this turn, if it asks anything at all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) attacked_with_subtype: Option<String>,
    /// The holder's turn whose end step the permission runs to. Additive: a
    /// checkpoint written before any permission reached that far restores
    /// without one, which all of them did.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) until_holder_end_step: Option<(usize, u32)>,
    /// The pile a permission over several cards belongs to, named by the
    /// object whose resolution granted it. Absent for every permission that
    /// covers one card, which is all of them but a pile's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) group: Option<u32>,
    /// Whether a creature cast through this permission receives suspend's haste.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(in crate::game::state_checkpoint) grants_haste: bool,
    /// Whether this permission names a card in a graveyard rather than one
    /// in exile. Absent for every permission written before Emry's, all of
    /// which were about exile.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(in crate::game::state_checkpoint) from_graveyard: bool,
}
