use crate::action::{AbilityOrigin, ManaColor};
use crate::card::{AbilityCostList, AddManaEffectDef, AppliedEffectDef, ManaSplit, SpellForm};
use crate::ids::{CardDefinitionId, GameObjectId, PlayerId};

use super::{ManaPool, ManaSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AppliedStackEffect {
    pub(super) source: Option<ManaSource>,
    pub(super) effect: AppliedEffectDef,
}

/// The object or procedure a mana payment is paying for. Restrictions are
/// evaluated against this frozen purpose both while planning mana abilities
/// and when selecting the exact mana units to spend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ManaPaymentPurpose {
    Spell {
        object: GameObjectId,
        definition: CardDefinitionId,
        controller: PlayerId,
        form: SpellForm,
    },
    Ability {
        source: GameObjectId,
        /// Whether the ability taps its source to pay for itself. When it
        /// does, that source cannot also be tapped for mana, so it is barred
        /// from the payment rather than merely deprioritised.
        taps_source: bool,
        /// Whether the source must still be on the battlefield after mana is
        /// raised so it can be sacrificed or exiled for the main ability.
        /// Mana abilities of that same source which leave the battlefield are
        /// not legal ways to pay this cost.
        leaves_source: bool,
    },
    Other,
}

/// Action-specific guidance for choosing mana abilities. `avoid` is only a
/// preference; a permanent chosen for a tap cost must instead remain present
/// and untapped until that cost is paid.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct ManaPlanOptions {
    pub(super) avoid: Option<GameObjectId>,
    pub(super) tap_cost_payer: Option<GameObjectId>,
}

/// The choices that distinguish otherwise identical activations of one mana
/// ability. A mana ability resolves without ever holding priority, so each is
/// enumerated into the activation rather than asked afterwards.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct ManaActivationChoices {
    pub(super) counters_removed: Option<u16>,
    pub(super) cost_object: Option<GameObjectId>,
    pub(super) combination: Option<ManaSplit>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ManaAbilityActivation {
    pub(super) source: GameObjectId,
    pub(super) ability: AbilityOrigin,
    pub(super) color: ManaColor,
    pub(super) costs: AbilityCostList,
    pub(super) effect: AddManaEffectDef,
    /// How many counters this activation takes, for the abilities whose
    /// removal cost is open-ended and therefore offered once per size.
    /// `None` whenever the cost has only one size, which is every other
    /// mana ability.
    pub(super) counters_removed: Option<u16>,
    /// The permanent a "Sacrifice a <thing>" cost consumes. Like the counter
    /// size above, source and colour do not distinguish one Goblin from
    /// another, so the choice is enumerated into the activation rather than
    /// asked afterwards -- a mana ability has no window in which to ask.
    /// `None` for every ability that sacrifices nothing but itself.
    pub(super) cost_object: Option<GameObjectId>,
    /// How the amount is divided, for "add three mana in any combination of
    /// {U} and/or {R}". Each division is its own activation for the same
    /// reason the counter size above is: there is no window in which to ask.
    /// `None` for every ability that produces one type at a time, which is
    /// nearly all of them; `color` then carries the type by itself.
    pub(super) combination: Option<ManaSplit>,
}

/// One enumerated output of a mana source: the ability, the colour, what it
/// produces, whether that mana helps the payment at hand, and the three
/// choices that distinguish otherwise identical activations -- the counter
/// size, the sacrificed permanent, and the division of a combined amount.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ManaSourceOutput {
    pub(super) kind: PlannedPaymentKind,
    /// Mana this output actually puts into the player's pool.
    pub(super) production: ManaPool,
    /// Colored payment supplied by convoke without producing mana.
    pub(super) convoke_production: ManaPool,
    pub(super) generic_payment: u16,
    /// Life paid as a cost of the mana ability represented by this output.
    /// Convoke itself never pays life.
    pub(super) life_payment: u16,
    pub(super) benefits_payment: bool,
}

impl ManaSourceOutput {
    pub(super) const fn payment_amount(self, color: ManaColor) -> u16 {
        self.production
            .amount(color)
            .saturating_add(self.convoke_production.amount(color))
    }

    pub(super) const fn payment_total(self) -> u16 {
        self.production
            .total()
            .saturating_add(self.convoke_production.total())
            .saturating_add(self.generic_payment)
    }
}

pub(super) type ManaSourceOutputs = Vec<ManaSourceOutput>;

/// What one selected payment source does. Convoke is deliberately not a mana
/// ability: it taps the creature while costs are paid, produces no mana, and
/// carries no mana restrictions or spend riders.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PlannedPaymentKind {
    Mana {
        ability: AbilityOrigin,
        color: ManaColor,
        counters_removed: Option<u16>,
        cost_object: Option<GameObjectId>,
        combination: Option<ManaSplit>,
        /// This non-tapping activation is followed by convoking the same
        /// source, so one permanent legitimately contributes in both ways.
        convokes: bool,
    },
    Convoke,
}

impl PlannedPaymentKind {
    pub(super) const fn uses_convoke(self) -> bool {
        matches!(self, Self::Convoke | Self::Mana { convokes: true, .. })
    }

    pub(super) const fn cost_object(self) -> Option<GameObjectId> {
        match self {
            Self::Mana { cost_object, .. } => cost_object,
            Self::Convoke => None,
        }
    }
}

/// Virtual capacity used only while planning a payment. `generic` is payment
/// that can cover only a generic requirement, such as convoking a colorless
/// creature; it is intentionally distinct from colorless mana and cannot pay
/// a `{C}` symbol.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct PaymentCapacity {
    pub(super) mana: ManaPool,
    pub(super) generic: u16,
}

impl PaymentCapacity {
    pub(super) const fn from_mana(mana: ManaPool) -> Self {
        Self { mana, generic: 0 }
    }

    pub(super) const fn total(self) -> u16 {
        self.mana.total().saturating_add(self.generic)
    }

    pub(super) const fn amount(self, color: ManaColor) -> u16 {
        self.mana.amount(color)
    }

    pub(super) fn add_output(&mut self, output: &ManaSourceOutput) {
        self.mana.add(output.production);
        self.mana.add(output.convoke_production);
        self.generic = self.generic.saturating_add(output.generic_payment);
    }

    pub(super) fn add_planned(&mut self, payment: &PlannedManaActivation) {
        self.mana.add(payment.production);
        self.mana.add(payment.convoke_production);
        self.generic = self.generic.saturating_add(payment.generic_payment);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct PlannedManaActivation {
    pub(super) source: GameObjectId,
    pub(super) kind: PlannedPaymentKind,
    /// Mana this activation actually produces.
    pub(super) production: ManaPool,
    /// Colored payment supplied by convoking this source.
    pub(super) convoke_production: ManaPool,
    /// Capacity that pays generic only; nonzero only for a colorless creature
    /// convoked for this spell.
    pub(super) generic_payment: u16,
    /// Life this mana ability pays when activated. Channel is accounted for
    /// separately because it is synthesized only for the final shortfall.
    pub(super) life_payment: u16,
    pub(super) benefits_payment: bool,
    pub(super) flexibility: usize,
    pub(super) order: usize,
}

impl PlannedManaActivation {
    pub(super) const fn payment_amount(self, color: ManaColor) -> u16 {
        self.production
            .amount(color)
            .saturating_add(self.convoke_production.amount(color))
    }

    pub(super) const fn payment_total(self) -> u16 {
        self.production
            .total()
            .saturating_add(self.convoke_production.total())
            .saturating_add(self.generic_payment)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct FlexibleManaSource {
    pub(super) source: GameObjectId,
    /// Ability, colour, what it makes, whether it benefits the payment,
    /// and how many counters its sized removal takes when the ability
    /// offers more than one size.
    pub(super) outputs: ManaSourceOutputs,
    pub(super) order: usize,
}
