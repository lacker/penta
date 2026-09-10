//! What an effect can ask a player to pay, and the branch either answer takes.
//!
//! These sit apart from the effect vocabulary itself because a payment is a
//! question rather than an action: the cost, who is asked, and what follows
//! each answer.

use super::{
    ChoiceVisibilityDef, CostDef, EffectDef, PlayerRefDef, PlayerSetDef, TriggerConditionDef,
};

/// A payment offered while an effect or replacement procedure resolves.
///
/// The payer uses the same compositional player-set vocabulary as the rest of
/// the effect model. Payment procedures require that it resolve to exactly one
/// player; a missing or non-singleton payer cannot pay and takes the declined
/// branch.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EffectPaymentDef {
    pub payer: PlayerSetDef,
    pub costs: &'static [CostDef],
}

impl EffectPaymentDef {
    #[must_use]
    pub const fn new(payer: PlayerSetDef, costs: &'static [CostDef]) -> Self {
        Self { payer, costs }
    }
}

/// Offer a payment and continue through the branch selected by its result.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PayOrDef {
    pub payment: EffectPaymentDef,
    pub if_paid: Option<&'static EffectDef>,
    pub otherwise: Option<&'static EffectDef>,
    pub visibility: ChoiceVisibilityDef,
    /// A printed "if ..." on the offer itself: "you may pay {1}{G} if this
    /// permanent is attached to a creature you control" asks before it
    /// offers, and a false answer takes the other branch without anybody
    /// being asked to pay for something that would do nothing.
    pub condition: Option<&'static TriggerConditionDef>,
}

impl PayOrDef {
    /// Offer the effect controller an optional payment and continue only when
    /// it is paid.
    #[must_use]
    pub const fn optional(costs: &'static [CostDef], if_paid: &'static EffectDef) -> Self {
        Self {
            payment: EffectPaymentDef::new(
                PlayerSetDef::One(PlayerRefDef::EffectController),
                costs,
            ),
            if_paid: Some(if_paid),
            otherwise: None,
            visibility: ChoiceVisibilityDef::Private,
            condition: None,
        }
    }

    /// Offer the effect controller an optional payment with a branch either
    /// way. Both halves are one printed clause, so the player choosing not to
    /// pay is not the same as nothing happening.
    #[must_use]
    pub const fn optional_or(
        costs: &'static [CostDef],
        if_paid: &'static EffectDef,
        otherwise: &'static EffectDef,
    ) -> Self {
        Self {
            otherwise: Some(otherwise),
            ..Self::optional(costs, if_paid)
        }
    }

    /// Charge a specific player instead of the effect controller.
    #[must_use]
    pub const fn with_payer(mut self, payer: PlayerSetDef) -> Self {
        self.payment.payer = payer;
        self
    }

    /// "You may pay ... if <condition>": the offer is made only when the
    /// condition holds, and the other branch runs when it does not.
    #[must_use]
    pub const fn only_if(mut self, condition: &'static TriggerConditionDef) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Continue unless the effect controller pays. Nothing happens when they
    /// do, which is what "sacrifice it unless you return a land" says: paying
    /// is the whole point and buys only the absence of the consequence.
    #[must_use]
    pub const fn unless(costs: &'static [CostDef], otherwise: &'static EffectDef) -> Self {
        Self {
            payment: EffectPaymentDef::new(
                PlayerSetDef::One(PlayerRefDef::EffectController),
                costs,
            ),
            if_paid: None,
            otherwise: Some(otherwise),
            visibility: ChoiceVisibilityDef::Private,
            condition: None,
        }
    }
}
