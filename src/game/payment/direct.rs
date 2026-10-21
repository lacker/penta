//! Atomic submission of a complete payment, without the interactive editor.
use super::super::{Action, ActionError, Game, ManaContributionKind, PlayerId, Step};
use super::{
    BoundManaPayment,
    contributions::{BoundContribution, PaymentSymbol},
    funding::{FundingStep, PaymentDraft},
    query::PaymentQuery,
};
use crate::action::{ContributionKind, ContributionSymbol, ExplicitPayment};

impl Game {
    /// Whether this player may take ordinary priority actions right now.
    #[must_use]
    pub fn has_action_priority(&self, player: PlayerId) -> bool {
        self.result.is_none()
            && !self.between_games()
            && self.pending_decisions.is_empty()
            && self.pending_combat_assignments.is_empty()
            && self.pregame.is_none()
            && !self.cleanup_pending
            && !self.untap_pending
            && (self.step != Step::DeclareAttackers || self.attackers_declared)
            && (self.step != Step::DeclareBlockers || self.blockers_declared)
            && self.priority == player
    }

    pub(in crate::game) fn cast_action_window(
        &self,
        player: PlayerId,
        card: crate::GameObjectId,
    ) -> bool {
        (self.has_action_priority(player) && !self.split_second_is_active())
            || (self.result.is_none()
                && !self.between_games()
                && self
                    .pending_decisions
                    .first()
                    .is_some_and(|pending| pending.observation.player == player)
                && self.pending_decisions.iter().any(|pending| {
                    pending
                        .continuation
                        .cast_offer()
                        .is_some_and(|offer| offer.player == player && offer.card == card)
                }))
    }

    pub(in crate::game) fn allocate_funded_pool(
        &self,
        obligation: &super::ManaPaymentObligation,
    ) -> Option<BoundManaPayment> {
        let mut payment = BoundManaPayment { units: Vec::new() };
        if !self.mana_selection_can_complete(obligation, &payment.units) {
            return None;
        }
        for index in 0..self.payment_mana_units(obligation.player).len() {
            if self.validate_mana_payment(obligation, &payment) {
                return Some(payment);
            }
            payment.units.push(index);
            if !self.mana_selection_can_complete(obligation, &payment.units) {
                payment.units.pop();
            }
        }
        self.validate_mana_payment(obligation, &payment)
            .then_some(payment)
    }

    /// Submit an operation and its exact funding program. Failed proposals leave
    /// every resource, event, pending choice, and random state unchanged.
    ///
    /// # Errors
    /// Returns an action error if the operation or any payment choice is invalid.
    pub fn apply_with_payment(
        &mut self,
        player: PlayerId,
        action: Action,
        payment: &ExplicitPayment,
    ) -> Result<(), ActionError> {
        if self.result.is_some() {
            return Err(ActionError::GameAlreadyFinished);
        }
        let mut announced = self.clone();
        announced.payment_query = PaymentQuery::announcement();
        let valid = match &action {
            Action::CastSpell {
                card,
                choices,
                sacrifices,
            } => {
                self.cast_action_window(player, *card)
                    && announced
                        .validated_cast_signature(player, *card, choices, sacrifices)
                        .is_some()
            }
            Action::ActivateAbility { .. } | Action::ActivateAbilityWithAlternativeCost { .. } => {
                if self.has_action_priority(player) {
                    let mut actions = Vec::new();
                    announced.add_ability_actions(player, &mut actions);
                    actions.contains(&action)
                } else {
                    false
                }
            }
            Action::ActivateManaAbility { .. } => {
                self.has_action_priority(player)
                    && announced
                        .mana_activation_for_action(player, &action)
                        .is_some()
            }
            _ => false,
        };
        if !valid {
            return Err(ActionError::NotLegal {
                player,
                action: Box::new(action),
            });
        }
        let draft = self.direct_payment_draft(player, &action, payment);
        let mut committed = self.clone();
        if let Some(resume) = &draft.resume {
            committed
                .pending_decisions
                .retain(|pending| pending.observation.id != resume.observation.id);
        }
        let Some((funded, frame)) = committed.preview_funding_for(&draft, Some(player)) else {
            return Err(ActionError::NotLegal {
                player,
                action: Box::new(action),
            });
        };
        let mana = match &payment.mana_units {
            Some(units) => Some(BoundManaPayment {
                units: units.clone(),
            }),
            None => funded.allocate_funded_pool(&frame.obligation),
        };
        if mana
            .as_ref()
            .and_then(|mana| committed.commit_payment_draft(&draft, mana))
            .is_none()
        {
            return Err(ActionError::NotLegal {
                player,
                action: Box::new(action),
            });
        }
        committed.forget_enumeration();
        *self = committed;
        Ok(())
    }
    pub(in crate::game) fn direct_payment_draft(
        &self,
        player: PlayerId,
        action: &Action,
        payment: &ExplicitPayment,
    ) -> PaymentDraft {
        // Keep all simultaneous cast offers in the validation projection.
        // commit_payment_draft consumes the selected offer and restores the rest.
        let resume = self.pending_decisions.iter().find(|pending| {
            pending.continuation.cast_offer().is_some_and(|offer| {
                matches!(action, Action::CastSpell { card, .. } if *card == offer.card && offer.player == player)
            })
        }).cloned().map(Box::new);
        PaymentDraft {
            player,
            action: Box::new(action.clone()),
            funding: payment
                .funding
                .iter()
                .map(|step| FundingStep {
                    action: step.action.clone(),
                    mana: step
                        .mana_units
                        .clone()
                        .map(|units| BoundManaPayment { units }),
                    answers: step.answers.clone(),
                })
                .collect(),
            announcements: payment.announcements.clone(),
            contributions: payment
                .contributions
                .iter()
                .map(|item| BoundContribution {
                    source: item.source,
                    kind: match item.kind {
                        ContributionKind::Convoke => ManaContributionKind::Convoke,
                        ContributionKind::Delve => ManaContributionKind::Delve,
                        ContributionKind::Improvise => ManaContributionKind::Improvise,
                        ContributionKind::Waterbend => ManaContributionKind::Waterbend,
                    },
                    symbol: match item.symbol {
                        ContributionSymbol::Generic => PaymentSymbol::Generic,
                        ContributionSymbol::Color(color) => PaymentSymbol::Color(color),
                        ContributionSymbol::Flexible(symbol) => PaymentSymbol::Flexible(symbol),
                    },
                })
                .collect(),
            resume,
        }
    }
}
