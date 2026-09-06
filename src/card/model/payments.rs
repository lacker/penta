//! What an effect can ask a player to pay, and the branch either answer takes.
//!
//! These sit apart from the effect vocabulary itself because a payment is a
//! question rather than an action: the cost, who is asked, and what follows
//! each answer.

use super::{
    ChoiceVisibilityDef, CostDef, EffectDef, ManaCost, PlayerRefDef, PlayerSetDef,
    TriggerConditionDef, ValueDef,
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
    pub cost: CostDef,
}

impl EffectPaymentDef {
    #[must_use]
    pub const fn mana(payer: PlayerSetDef, cost: ManaCost) -> Self {
        Self {
            payer,
            cost: CostDef::Mana(cost),
        }
    }

    #[must_use]
    pub const fn generic_mana(payer: PlayerSetDef, amount: ValueDef) -> Self {
        Self {
            payer,
            cost: CostDef::GenericMana(amount),
        }
    }

    #[must_use]
    pub const fn life(payer: PlayerSetDef, amount: u16) -> Self {
        Self {
            payer,
            cost: CostDef::PayLife(amount),
        }
    }

    #[must_use]
    pub const fn mill(payer: PlayerSetDef, amount: u16) -> Self {
        Self {
            payer,
            cost: CostDef::MillCards(amount),
        }
    }

    #[must_use]
    pub const fn discard(payer: PlayerSetDef, amount: u16) -> Self {
        Self {
            payer,
            cost: CostDef::DiscardCards(amount),
        }
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
    /// Offer an optional payment and continue only when it is paid.
    #[must_use]
    pub const fn optional(payment: EffectPaymentDef, if_paid: &'static EffectDef) -> Self {
        Self {
            payment,
            if_paid: Some(if_paid),
            otherwise: None,
            visibility: ChoiceVisibilityDef::Private,
            condition: None,
        }
    }

    /// Offer an optional payment with a branch either way. Both halves are
    /// one printed clause, so the player choosing not to pay is not the same
    /// as nothing happening.
    #[must_use]
    pub const fn optional_or(
        payment: EffectPaymentDef,
        if_paid: &'static EffectDef,
        otherwise: &'static EffectDef,
    ) -> Self {
        Self {
            payment,
            if_paid: Some(if_paid),
            otherwise: Some(otherwise),
            visibility: ChoiceVisibilityDef::Private,
            condition: None,
        }
    }

    /// "You may pay ... if <condition>": the offer is made only when the
    /// condition holds, and the other branch runs when it does not.
    #[must_use]
    pub const fn only_if(mut self, condition: &'static TriggerConditionDef) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Continue unless the payer pays. Nothing happens when they do, which is
    /// what "sacrifice it unless you return a land" says: paying is the whole
    /// point of the clause and buys only the absence of the consequence.
    #[must_use]
    pub const fn unless(payment: EffectPaymentDef, otherwise: &'static EffectDef) -> Self {
        Self {
            payment,
            if_paid: None,
            otherwise: Some(otherwise),
            visibility: ChoiceVisibilityDef::Private,
            condition: None,
        }
    }

    /// Continue unless the resolving effect's controller pays a fixed mana
    /// cost.
    #[must_use]
    pub const fn unless_mana(cost: ManaCost, otherwise: &'static EffectDef) -> Self {
        Self {
            payment: EffectPaymentDef::mana(
                PlayerSetDef::One(PlayerRefDef::EffectController),
                cost,
            ),
            if_paid: None,
            otherwise: Some(otherwise),
            visibility: ChoiceVisibilityDef::Private,
            condition: None,
        }
    }
}
