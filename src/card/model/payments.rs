//! What an effect can ask a player to pay, and the branch either answer takes.
//!
//! These sit apart from the effect vocabulary itself because a payment is a
//! question rather than an action: the cost, who is asked, and what follows
//! each answer.

use super::{
    ChoiceVisibilityDef, EffectDef, EffectRecipientDef, ManaColor, ManaCost, ObjectPredicateDef,
    PlayerRefDef, PlayerSetDef, TriggerConditionDef, ValueDef, ZoneKind,
};

/// The supported cost of an optional effect payment.
///
/// This is deliberately narrower than casting and activation costs: those
/// procedures can plan compound costs atomically, while a resolving effect
/// currently offers exactly one mana or life payment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EffectPaymentCostDef {
    Mana(ManaCost),
    /// A generic mana payment whose amount is evaluated at resolution.
    GenericMana(ValueDef),
    /// The same, in one colour: "pay {G} for each wind counter on it". The
    /// colour is fixed and the count is not, which is why this is an amount
    /// rather than a printed cost.
    ColoredMana {
        color: ManaColor,
        amount: ValueDef,
    },
    /// "Its mana cost reduced by {2}": the printed cost of an object named
    /// earlier in the same resolution, less that much generic. Not a generic
    /// amount, which is the whole point -- the coloured pips still have to
    /// be paid in their own colours.
    ObjectManaCostReducedBy {
        object: EffectRecipientDef,
        generic: u16,
    },
    Life(u16),
    /// "Unless you pay {E}." Energy is spent all at once or not at all: a
    /// player short of the amount cannot pay part of it.
    Energy(u16),
    /// Mill this many cards. Never impossible: a library shorter than the
    /// amount mills what it has (CR 701.13b), so this branch of an "unless"
    /// is always open and the choice is a real one even at one card left.
    Mill(u16),
    /// Discard this many cards, chosen by the payer. Unlike a mill, a hand
    /// too small cannot pay at all: there is nothing to choose. Which cards
    /// go is a separate decision queued behind the payment, because by then
    /// the branch has already been settled.
    Discard(u16),
    /// Generic mana in an amount the payer chooses, which the paid branch
    /// then reads back with [`ValueDef::PaidAmount`]. This is "you may pay
    /// {X}" during a resolution, where X is settled by the payment rather
    /// than by the cast that produced it.
    ChosenGenericMana,
    /// Energy in an amount the payer chooses, read back the same way. "You
    /// may pay any amount of {E}" is this: unlike the fixed amount above,
    /// what is spent is settled by the payer rather than by the card, and
    /// paying nothing is a legal answer.
    ChosenEnergy,
    /// Remove any positive number of counters of one kind from the named
    /// object while an effect resolves. The payer chooses the amount and the
    /// paid branch reads it with [`ValueDef::PaidAmount`]. Declining is the
    /// zero-counter answer.
    RemoveAnyNumberOfCounters {
        object: EffectRecipientDef,
        kind: super::CounterKind,
    },
    /// Sacrifice one matching permanent the payer controls, named as part of
    /// the payment. Chain of Vapor asks for a land of their choice.
    SacrificePermanentMatching(ObjectPredicateDef),
    /// Sacrifice creatures until their power adds up to at least this much.
    /// The creatures are named one at a time, and the payer may stop as soon
    /// as the total is reached -- or keep going, which is worth doing for a
    /// deck that wants creature cards in its graveyard.
    ///
    /// Offered only when the payer's creatures could reach the total at all,
    /// so a board that cannot pay takes the other branch without being asked.
    SacrificeCreaturesWithTotalPower(u16),
    /// Move one matching permanent its payer controls to a named zone as the
    /// payment. The choice and move are one atomic cost rather than a
    /// `MoveToZone` effect after the paid branch has already been selected.
    MovePermanentMatching {
        object: ObjectPredicateDef,
        zone: ZoneKind,
    },
    /// Discard one card the predicate matches. Mox Diamond's "you may discard
    /// a land card instead" is a real restriction, so a hand with nothing
    /// matching cannot pay at all even though it holds plenty of cards.
    ///
    /// Which card goes is part of this decision rather than a second one
    /// behind it: each candidate is its own option, the way a mana ability
    /// that sacrifices a permanent offers one activation per candidate.
    DiscardMatching(ObjectPredicateDef),
}

/// A payment offered while an effect or replacement procedure resolves.
///
/// The payer uses the same compositional player-set vocabulary as the rest of
/// the effect model. Payment procedures require that it resolve to exactly one
/// player; a missing or non-singleton payer cannot pay and takes the declined
/// branch.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EffectPaymentDef {
    pub payer: PlayerSetDef,
    pub cost: EffectPaymentCostDef,
}

impl EffectPaymentDef {
    #[must_use]
    pub const fn mana(payer: PlayerSetDef, cost: ManaCost) -> Self {
        Self {
            payer,
            cost: EffectPaymentCostDef::Mana(cost),
        }
    }

    #[must_use]
    pub const fn generic_mana(payer: PlayerSetDef, amount: ValueDef) -> Self {
        Self {
            payer,
            cost: EffectPaymentCostDef::GenericMana(amount),
        }
    }

    #[must_use]
    pub const fn life(payer: PlayerSetDef, amount: u16) -> Self {
        Self {
            payer,
            cost: EffectPaymentCostDef::Life(amount),
        }
    }

    #[must_use]
    pub const fn mill(payer: PlayerSetDef, amount: u16) -> Self {
        Self {
            payer,
            cost: EffectPaymentCostDef::Mill(amount),
        }
    }

    #[must_use]
    pub const fn discard(payer: PlayerSetDef, amount: u16) -> Self {
        Self {
            payer,
            cost: EffectPaymentCostDef::Discard(amount),
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
