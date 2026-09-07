//! Leaving a pending decision without answering it, and resuming what it
//! suspended.
//!
//! Split out of the parent module for the source-size budget. What belongs
//! here is a decision that ends some way other than being answered: a cast
//! that stood in for the answer, a cancel, and the nested resolution that
//! picks up where the decision left off.

#![allow(clippy::wildcard_imports)]

use super::*;
use crate::ids::GameObjectId;

impl Game {
    /// Drops a standing "you may cast that card" offer because the card was
    /// cast. The offer is answered by the cast itself, so nothing about the
    /// decision resolves and the else branch never runs.
    pub(in crate::game) fn take_answered_cast_offer(&mut self, cast: GameObjectId) {
        // Not only the offer at the head of the queue: one resolution can
        // leave several standing at once, and which of them is answered by
        // casting is the caster's choice rather than the queue's order.
        let Some(answered) = self.pending_decisions.iter().position(|pending| {
            matches!(
                pending.continuation,
                DecisionContinuation::MayCastExiled { card, .. }
                    | DecisionContinuation::CastSuspended { card, .. }
                    | DecisionContinuation::CascadeCast { card, .. }
                    | DecisionContinuation::MayCastGranted { card, .. }
                    | DecisionContinuation::MayCastAlternative { card, .. } if card == cast
            )
        }) else {
            return;
        };
        let taken = self.pending_decisions.remove(answered);
        match taken.continuation {
            DecisionContinuation::MayCastGranted {
                card,
                ability,
                grant,
                ..
            } => self.revoke_temporary_grant(grant, card, &ability),
            // Cascade's pile goes to the bottom whether the card it turned up
            // was cast or declined, so the accepted half is finished here
            // rather than in the resolution the decline takes. The card being
            // cast is left where it is: this runs before the cast lifts it out
            // of exile, and a card on the stack is no longer one of the cards
            // exiled this way.
            DecisionContinuation::CascadeCast { player, exiled, .. } => {
                let rest = exiled
                    .into_iter()
                    .filter(|card| *card != cast)
                    .collect::<Vec<_>>();
                self.bury_cascade_exiles(player, &rest);
            }
            _ => {}
        }
    }

    pub(in crate::game) fn cancel_decision(&mut self, decision: u32) {
        debug_assert_eq!(self.pending_decisions[0].observation.id, decision);
        let pending = self.pending_decisions.remove(0);
        if let crate::game::DecisionContinuation::CostPayment(window) = pending.continuation {
            self.finish_cost_payment_window(*window, false);
        }
    }

    pub(in crate::game) fn resolve_nested_effect_before_later(
        &mut self,
        effect: crate::game::ScopedEffect,
        object: &crate::game::StackObject,
        context: crate::game::EffectResolutionContext,
    ) {
        let mut later_procedures = std::mem::take(&mut self.pending_procedures);
        self.resolve_effect_def(effect, object, context);
        self.pending_procedures.append(&mut later_procedures);
    }
}
