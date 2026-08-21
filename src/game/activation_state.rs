use crate::ObjectCharacteristics;
use crate::action::AbilityOrigin;
use crate::card::{AbilityDef, AbilityTargetDef};
use crate::casting::TargetSelection;
use crate::ids::ModeId;

use super::GameObjectId;

/// Everything one activation chose before its costs were paid: which slots
/// it filled, which objects its costs spend, the value of X, and the modes
/// it picked. All four are chosen as the ability is activated (CR 601.2b),
/// so they travel together from the action to the stack object.
pub(super) struct ActivationChoices<'a> {
    pub(super) targets: Vec<TargetSelection>,
    pub(super) cost_objects: &'a [GameObjectId],
    pub(super) x: u16,
    pub(super) modes: &'a [ModeId],
}

use super::{ScopedEffect, StackAbilityResolver};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct FrozenActivatedAbility {
    pub(super) origin: AbilityOrigin,
    pub(super) definition: Option<Box<AbilityDef>>,
    pub(super) presentation: ObjectCharacteristics,
    pub(super) text: Option<&'static str>,
    /// Owned rather than borrowed because a modal ability's slots are its
    /// own followed by each chosen mode's, which is a list nothing prints.
    pub(super) target_defs: Vec<AbilityTargetDef>,
    pub(super) resolver: StackAbilityResolver,
    /// The chosen modes' effects, resolved after the ability's own. Empty
    /// for every ability that prints no modes.
    pub(super) mode_effects: Vec<ScopedEffect>,
    /// The X chosen at activation, frozen alongside everything else the
    /// ability will resolve with.
    pub(super) x: u16,
}

/// One activation whose costs are not finished being paid.
///
/// Every choice the activation made travels here so a cost that asks a
/// question can be answered and the activation resumed exactly where it
/// stopped: nothing is on the stack until the last of its costs is paid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PendingActivation {
    pub(super) source: GameObjectId,
    pub(super) source_card: super::ObjectInstance,
    pub(super) controller: crate::ids::PlayerId,
    pub(super) frozen: FrozenActivatedAbility,
    pub(super) targets: Vec<TargetSelection>,
    pub(super) chosen_permanents: Vec<GameObjectId>,
    /// Permanents an earlier cost already committed to sacrifice, which go
    /// with whatever the paused cost names.
    pub(super) remaining_sacrifices: Vec<GameObjectId>,
}
