//! Optional additional costs selected independently of how a spell is cast.
//!
//! Unlike an alternative cost, one of these adds to the spell's already
//! calculated cost and can therefore be combined with flashback, overload, or
//! any other legal way of casting it.

use crate::ids::{AbilityId, AdditionalCostId};

use super::super::{AdditionalCostDef, ManaCost};
use super::{SpellAdditionalCostDef, SpellResolutionDestinationDef};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OptionalAdditionalCostKindDef {
    /// Buyback (CR 702.27): if this cost was paid, a resolving spell card goes
    /// to its owner's hand instead of its graveyard.
    Buyback,
}

impl OptionalAdditionalCostKindDef {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Buyback => "Buyback",
        }
    }
}

/// One named optional additional cost and the stack outcome it locks in.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OptionalAdditionalCostAbilityDef {
    pub kind: OptionalAdditionalCostKindDef,
    pub mana_cost: Option<ManaCost>,
    pub additional_cost: Option<SpellAdditionalCostDef>,
    pub resolution_destination: SpellResolutionDestinationDef,
}

impl OptionalAdditionalCostAbilityDef {
    #[must_use]
    pub fn rules_text(self) -> String {
        match (self.kind, self.mana_cost) {
            (OptionalAdditionalCostKindDef::Buyback, Some(cost)) => format!(
                "Buyback {cost} (You may pay an additional {cost} as you cast this spell. If you \
                 do, put this card into your hand as it resolves.)"
            ),
            (OptionalAdditionalCostKindDef::Buyback, None) => "Buyback".into(),
        }
    }

    #[must_use]
    pub fn additional_cost(self, ability: AbilityId) -> AdditionalCostDef {
        AdditionalCostDef {
            id: AdditionalCostId(ability.0),
            label: self.kind.label().into(),
            mana_cost: self.mana_cost,
        }
    }
}
