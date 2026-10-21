//! Rejection diagnostics reuse the proposal validators; they do not decide legality.
use super::super::{Action, Game, PlayerId};
use super::{BoundManaPayment, query::PaymentQuery};
use crate::action::ExplicitPayment;
use serde_json::{Value, json};

fn error(code: &str, path: &str, message: &str) -> Value {
    json!({ "code": code, "path": path, "message": message })
}

impl Game {
    /// Explain a rejected proposal using the current state's validation stages.
    /// This is read-only and intended after `apply` or `apply_with_payment` fails.
    #[must_use]
    #[allow(clippy::too_many_lines)] // Mirror validation stages in their execution order.
    pub fn explain_proposal_rejection(
        &self,
        player: PlayerId,
        action: &Action,
        payment: Option<&ExplicitPayment>,
    ) -> Value {
        if self.result.is_some() && !self.between_games() {
            return error("gameFinished", "action", "The game has finished.");
        }
        if let Action::CastSpell {
            card,
            choices,
            sacrifices,
        } = action
        {
            if !self.cast_action_window(player, *card) {
                if self.has_action_priority(player) && self.split_second_is_active() {
                    return error(
                        "splitSecond",
                        "action",
                        "A spell with split second is on the stack, so you cannot cast a spell using priority. You may still pass or activate a legal mana ability.",
                    );
                }
                return error(
                    "castWindowClosed",
                    "action.card",
                    "You do not have priority or a pending instruction permitting this card to be cast. Answer the current decision first.",
                );
            }
            let mut view = self.clone();
            view.payment_query = PaymentQuery::announcement();
            if view
                .validated_cast_signature(player, *card, choices, sacrifices)
                .is_none()
            {
                return error(
                    "invalidCastAnnouncement",
                    "action.choices",
                    "The announced spell is not legal. Check its current zone and casting permission, timing, play option, modes, target slots and allocations, X, and alternative costs. Supply action.costObjects for resources spent on sacrifice, discard, exile/evidence, or tap costs. No payment was made.",
                );
            }
            if payment.is_none() && !self.is_legal_action(player, action) {
                return error(
                    "automaticPaymentUnavailable",
                    "payment",
                    "The spell announcement is legal, but automatic payment could not fund it. Submit an explicit payment with the mana abilities and resources to use.",
                );
            }
        } else if self.decision_player() != Some(player) {
            return error(
                "wrongPlayer",
                "player",
                "The current decision belongs to the other player.",
            );
        }
        if let Some(payment) = payment {
            // Only validate a draft after its operation has passed the same
            // announcement gate used by apply_with_payment; preparing an invalid
            // operation would violate the internal execution precondition.
            let mut view = self.clone();
            view.payment_query = PaymentQuery::announcement();
            let announced = match action {
                Action::CastSpell {
                    card,
                    choices,
                    sacrifices,
                } => view
                    .validated_cast_signature(player, *card, choices, sacrifices)
                    .is_some(),
                Action::ActivateAbility { .. }
                | Action::ActivateAbilityWithAlternativeCost { .. }
                    if self.has_action_priority(player) =>
                {
                    let mut actions = Vec::new();
                    view.add_ability_actions(player, &mut actions);
                    actions.contains(action)
                }
                Action::ActivateManaAbility { .. } if self.has_action_priority(player) => {
                    view.mana_activation_for_action(player, action).is_some()
                }
                _ => false,
            };
            if !announced {
                return error(
                    "invalidPaymentOperation",
                    "action",
                    "This operation cannot be announced here with the supplied ability, targets, costs, or choices.",
                );
            }
            let draft = self.direct_payment_draft(player, action, payment);
            if self
                .prepare_payment_draft_for(&draft, Some(player))
                .is_none()
            {
                return error(
                    "invalidAnnouncementAnswers",
                    "payment.announcements",
                    "The supplied announcement answers do not complete this operation's choices in order.",
                );
            }
            let Some((funded, frame)) = self.preview_funding_for(&draft, Some(player)) else {
                return error(
                    "invalidFunding",
                    "payment.funding",
                    "The funding program cannot pay this operation. Check source control, untapped state, ability choices, funding order, costs, replacement answers, and conflicts with reserved or contributed resources.",
                );
            };
            if !funded.pending_decisions.is_empty() {
                return error(
                    "fundingAnswerRequired",
                    "payment.funding",
                    "A selected mana ability has an unanswered decision. Supply its ordered answers before completing payment.",
                );
            }
            let valid = match &payment.mana_units {
                Some(units) => funded.validate_mana_payment(
                    &frame.obligation,
                    &BoundManaPayment {
                        units: units.clone(),
                    },
                ),
                None => funded.allocate_funded_pool(&frame.obligation).is_some(),
            };
            if !valid {
                return error(
                    "invalidManaAllocation",
                    "payment.manaUnits",
                    "The funded pool does not pay the exact cost with this allocation. Check colors, restrictions, duplicate or extra units, and shortfalls; omitted manaUnits only allocates the already funded pool.",
                );
            }
        }
        error(
            "illegalAction",
            "action",
            "This action is not legal in the current state. Check the current action context, object identities, and required decision bounds.",
        )
    }
}
