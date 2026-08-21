use super::Policy;
use crate::{Action, PlayerObservation};

/// Selects uniformly from the non-concession legal actions using a seeded PRNG.
#[derive(Clone, Debug)]
pub struct RandomPolicy {
    state: u64,
}

impl RandomPolicy {
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }

    fn index_below(&mut self, count: usize) -> usize {
        let count = u64::try_from(count).unwrap_or(u64::MAX);
        let unbiased_range = u64::MAX - u64::MAX % count;
        loop {
            let value = self.next_u64();
            if value < unbiased_range {
                return usize::try_from(value % count).unwrap_or(0);
            }
        }
    }

    fn choose_decision(&mut self, decision: &crate::game::DecisionObservation) -> Option<Action> {
        let mut options = decision
            .options
            .iter()
            .map(|option| option.id)
            .collect::<Vec<_>>();
        if options.len() < decision.minimum {
            return None;
        }
        for index in (1..options.len()).rev() {
            let offset = self.index_below(index + 1);
            options.swap(index, offset);
        }
        let count = if decision.minimum == decision.maximum {
            decision.minimum
        } else {
            decision.minimum + self.index_below(decision.maximum - decision.minimum + 1)
        };
        Some(Action::ChooseDecision {
            decision: decision.id,
            options: options.into_iter().take(count).collect(),
        })
    }
}

impl Policy for RandomPolicy {
    fn choose_action(&mut self, observation: &PlayerObservation) -> Option<Action> {
        if let Some(decision) = observation.decision.as_ref() {
            let casts = observation
                .legal_actions
                .iter()
                .filter(|action| matches!(action, Action::CastSpell { .. }))
                .collect::<Vec<_>>();
            if !casts.is_empty() {
                let selected = self.index_below(casts.len() + 1);
                if let Some(cast) = casts.get(selected) {
                    return Some((*cast).clone());
                }
            }
            return self.choose_decision(decision);
        }
        let choices: Vec<_> = observation
            .legal_actions
            .iter()
            .filter(|action| !matches!(action, Action::Concede))
            .collect();
        if choices.is_empty() {
            return observation.legal_actions.first().cloned();
        }
        Some(choices[self.index_below(choices.len())].clone())
    }
}
