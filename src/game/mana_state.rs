use crate::action::{AbilityOrigin, ManaColor};
use crate::card::{AbilityCostList, AddManaEffectDef, AppliedEffectDef, ManaSplit, SpellForm};
use crate::ids::{CardDefinitionId, GameObjectId, PlayerId};

use super::{ManaPool, ManaSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AppliedStackEffect {
    pub(super) source: Option<ManaSource>,
    /// The ability that handed this to the spell, for the riders that are
    /// not mana: a permission that grants what it allowed has to say which
    /// clause the grant came from, because that is how the effect is
    /// addressed again afterwards. `None` for a mana rider, whose source is
    /// the mana beside it.
    pub(super) granting: Option<super::AbilitySourceRef>,
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
        /// Life already committed by the spell while mana abilities are
        /// planned, including Phyrexian symbols paid with life.
        reserved_life_payment: u16,
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct ManaActivationChoices {
    pub(super) counters_removed: Option<u16>,
    pub(super) cost_object: Option<GameObjectId>,
    pub(super) combination: Option<ManaSplit>,
    pub(super) triggered_mana: Option<Vec<ManaSplit>>,
}

impl ManaActivationChoices {
    pub(super) fn new(
        counters_removed: Option<u16>,
        cost_object: Option<GameObjectId>,
        combination: Option<ManaSplit>,
        triggered_mana: Option<Vec<ManaSplit>>,
    ) -> Self {
        Self {
            counters_removed,
            cost_object,
            combination,
            triggered_mana,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ManaAbilityActivation {
    pub(super) source: GameObjectId,
    pub(super) ability: AbilityOrigin,
    pub(super) color: ManaColor,
    pub(super) costs: AbilityCostList,
    /// "Activate only as an instant": offered to a player holding priority
    /// and never reached for by the payment planner, which pays for a spell
    /// at a moment no instant could be cast.
    pub(super) only_as_instant: bool,
    pub(super) effect: AddManaEffectDef,
    /// How many counters this activation takes, for the abilities whose
    /// removal cost is open-ended and therefore offered once per size.
    /// `None` whenever the cost has only one size, which is every other
    /// mana ability.
    pub(super) counters_removed: Option<u16>,
    /// The separate object a chosen sacrifice or hand-exile cost consumes.
    /// Like the counter size above, source and colour do not distinguish one
    /// candidate from another, so the choice is enumerated into the activation
    /// rather than asked afterwards -- a mana ability has no window to ask.
    /// `None` when no separate object is consumed.
    pub(super) cost_object: Option<GameObjectId>,
    /// How the amount is divided, for "add three mana in any combination of
    /// {U} and/or {R}". Each division is its own activation for the same
    /// reason the counter size above is: there is no window in which to ask.
    /// `None` for every ability that produces one type at a time, which is
    /// nearly all of them; `color` then carries the type by itself.
    pub(super) combination: Option<ManaSplit>,
    /// Selected output of immediate dynamic mana triggers caused by this
    /// activation. It is separate from the source ability's own production,
    /// because each triggered unit keeps the triggering permanent as its mana
    /// source when the activation resolves.
    pub(super) triggered_mana: Option<Vec<ManaSplit>>,
}

/// One enumerated output of a mana source: the ability, the colour, what it
/// produces, whether that mana helps the payment at hand, and the three
/// choices that distinguish otherwise identical activations -- the counter
/// size, sacrificed permanent, division of a combined amount, and output of
/// immediate dynamic mana triggers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ManaSourceOutput {
    pub(super) kind: PlannedPaymentKind,
    /// Mana this output actually puts into the player's pool.
    pub(super) production: ManaPool,
    /// Colored payment supplied directly without producing mana.
    pub(super) colored_contribution: ManaPool,
    pub(super) generic_payment: u16,
    /// Life paid as a cost of the mana ability represented by this output.
    /// Convoke itself never pays life.
    pub(super) life_payment: u16,
    pub(super) benefits_payment: bool,
}

impl ManaSourceOutput {
    pub(super) const fn payment_amount(&self, color: ManaColor) -> u16 {
        self.production
            .amount(color)
            .saturating_add(self.colored_contribution.amount(color))
    }

    pub(super) const fn payment_total(&self) -> u16 {
        self.production
            .total()
            .saturating_add(self.colored_contribution.total())
            .saturating_add(self.generic_payment)
    }
}

pub(super) type ManaSourceOutputs = Vec<ManaSourceOutput>;

/// A spell-cost resource that supplies payment without producing mana.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ManaContributionKind {
    Convoke,
    Delve,
    Improvise,
}

impl ManaContributionKind {
    pub(super) const fn taps_source(self) -> bool {
        matches!(self, Self::Convoke | Self::Improvise)
    }

    pub(super) const fn exiles_source(self) -> bool {
        matches!(self, Self::Delve)
    }
}

/// What one selected payment source does. Convoke is deliberately not a mana
/// ability: it taps the creature while costs are paid, produces no mana, and
/// carries no mana restrictions or spend riders.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum PlannedPaymentKind {
    Mana {
        ability: AbilityOrigin,
        color: ManaColor,
        counters_removed: Option<u16>,
        cost_object: Option<GameObjectId>,
        combination: Option<ManaSplit>,
        triggered_mana: Option<Vec<ManaSplit>>,
        /// This non-tapping activation is followed by a direct contribution
        /// from the same source, so one object legitimately contributes in
        /// both ways.
        contribution: Option<ManaContributionKind>,
    },
    Contribution(ManaContributionKind),
}

impl PlannedPaymentKind {
    pub(super) const fn contribution(&self) -> Option<ManaContributionKind> {
        match self {
            Self::Contribution(kind)
            | Self::Mana {
                contribution: Some(kind),
                ..
            } => Some(*kind),
            Self::Mana {
                contribution: None, ..
            } => None,
        }
    }

    pub(super) const fn uses_contribution(&self) -> bool {
        self.contribution().is_some()
    }

    pub(super) const fn cost_object(&self) -> Option<GameObjectId> {
        match self {
            Self::Mana { cost_object, .. } => *cost_object,
            Self::Contribution(_) => None,
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
        self.mana.add(output.colored_contribution);
        self.generic = self.generic.saturating_add(output.generic_payment);
    }

    #[allow(dead_code)]
    pub(super) fn add_planned(&mut self, payment: &PlannedManaActivation) {
        self.mana.add(payment.production);
        self.mana.add(payment.colored_contribution);
        self.generic = self.generic.saturating_add(payment.generic_payment);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PlannedManaActivation {
    pub(super) source: GameObjectId,
    pub(super) kind: PlannedPaymentKind,
    /// Mana this activation actually produces.
    pub(super) production: ManaPool,
    /// Colored payment supplied directly by this source.
    pub(super) colored_contribution: ManaPool,
    /// Capacity that pays generic only; nonzero only for a colorless creature
    /// convoked for this spell.
    pub(super) generic_payment: u16,
    /// Life this mana ability pays when activated. Repeatable ongoing life
    /// mana is accounted for separately because it is synthesized only for
    /// the final shortfall.
    pub(super) life_payment: u16,
    pub(super) benefits_payment: bool,
    pub(super) flexibility: usize,
    pub(super) order: usize,
}

impl PlannedManaActivation {
    #[allow(dead_code)]
    pub(super) const fn payment_amount(&self, color: ManaColor) -> u16 {
        self.production
            .amount(color)
            .saturating_add(self.colored_contribution.amount(color))
    }

    #[allow(dead_code)]
    pub(super) const fn payment_total(&self) -> u16 {
        self.production
            .total()
            .saturating_add(self.colored_contribution.total())
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
