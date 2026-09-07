//! Validate bounded decision answers without enumerating their combinations.

use crate::game::{Action, DecisionContinuation, Game, PlayerId};

impl Game {
    /// Validates an action against the current state without mutating the game.
    ///
    /// Unlike [`Self::legal_actions`], this also validates the option IDs supplied to
    /// a bounded [`Action::ChooseDecision`] selection without expanding every
    /// possible combination into a vector.
    #[must_use]
    pub fn is_legal_action(&self, player: PlayerId, action: &Action) -> bool {
        if let Action::ChooseDecision { decision, options } = action {
            let Some(pending) = self.pending_decisions.first() else {
                return false;
            };
            let observation = &pending.observation;
            if observation.player != player || observation.id != *decision {
                return false;
            }
            if pending.continuation.cast_offer_is_mandatory()
                && pending.continuation.cast_offer().is_some_and(|offer| {
                    let mut castable = Vec::new();
                    self.add_offered_cast_actions(offer, &mut castable);
                    !castable.is_empty()
                })
            {
                return false;
            }
            let available = observation
                .options
                .iter()
                .map(|option| option.id)
                .collect::<std::collections::HashSet<_>>();
            let unique = options
                .iter()
                .copied()
                .collect::<std::collections::HashSet<_>>();
            options.len() == unique.len()
                && options.len() >= observation.minimum
                && options.len() <= observation.maximum
                && options.iter().all(|option| available.contains(option))
                && match &pending.continuation {
                    DecisionContinuation::ActivationObjectCost {
                        player,
                        cost,
                        action,
                    } => self.activation_object_payment_is_valid(
                        *player,
                        action,
                        *cost,
                        options,
                        &observation.options,
                    ),
                    _ => true,
                }
        } else {
            self.legal_actions(player).contains(action)
        }
    }
}
