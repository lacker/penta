use super::super::{Action, PendingDecision, PlayerId};
use super::{ManaPaymentObligation, funding::PaymentDraft};

/// A proposal has not spent resources or announced its operation.
#[derive(Clone, Debug)]
pub(in crate::game) enum PaymentDecision {
    Operation {
        player: PlayerId,
        x: u16,
        resume: Option<Box<PendingDecision>>,
    },
    Funding(PaymentDraft),
    Mana {
        target: PaymentTarget,
        obligation: ManaPaymentObligation,
        selected: Vec<usize>,
    },
}

#[derive(Clone, Debug)]
pub(in crate::game) enum PaymentTarget {
    Draft(PaymentDraft),
    Funding {
        draft: PaymentDraft,
        action: Box<Action>,
    },
    Action {
        action: Box<Action>,
        resume: Option<Box<PendingDecision>>,
    },
    Effect {
        pending: Box<PendingDecision>,
        answered: Vec<u32>,
        allocations: Vec<super::BoundManaPayment>,
    },
}
