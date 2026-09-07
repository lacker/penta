//! Semantic payment steps, including named-action completion boundaries.

use crate::card::CostDef;
use crate::ids::{GameObjectId, MechanicId};

use super::{EffectResolutionContext, PlayerId, ScopedEffect, StackObject};

mod action_program;
mod activation;
mod commit;
mod program;
mod scalars;

#[cfg(test)]
pub(crate) use action_program::supported_action_program;
pub(in crate::game) use program::*;
mod selection;
mod window;

/// A choice path through the authored cost, not an executable serialized
/// callback. Choices and object selections do not mutate the game.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::game) struct CostPaymentWindow {
    pub player: PlayerId,
    pub definition: ScopedEffect,
    pub object: Box<StackObject>,
    pub context: EffectResolutionContext,
    pub answers: Vec<PaymentAnswer>,
    /// Tentative members of the current aggregate selection.
    pub chosen: Vec<GameObjectId>,
    /// Present only after all selections passed whole-plan validation.
    pub committing: Option<PaymentPlan>,
}

pub(in crate::game) const fn uses_cost_payment_window(cost: CostDef) -> bool {
    matches!(
        cost,
        CostDef::Named { .. }
            | CostDef::Choice(_)
            | CostDef::All(_)
            | CostDef::Repeat { .. }
            | CostDef::Action(_)
            | CostDef::Sacrifice { .. }
            | CostDef::Discard { .. }
            | CostDef::Exile { .. }
    )
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(in crate::game) enum CostPaymentStep {
    Object(GameObjectId, CostDef),
    /// Separates two payment actions even when their primitive costs match.
    EndAction,
    CompleteMechanic(MechanicId),
}

impl CostPaymentStep {
    pub(in crate::game) const fn object(self) -> Option<GameObjectId> {
        match self {
            Self::Object(object, _) => Some(object),
            Self::CompleteMechanic(_) | Self::EndAction => None,
        }
    }
}
