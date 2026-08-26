use super::{
    Action, DecisionObservation, DecisionPreference, HandcraftedPolicy, PlayerObservation,
};

impl HandcraftedPolicy {
    /// Opening-hand actions stand beside a zero-option finish decision, just
    /// as an offered cast stands beside its decline. Beginning with a free
    /// permanent or installing a delayed reveal effect is ordinarily useful;
    /// subtract any hand card the action consumes so Gemstone Caverns still
    /// prefers the cheapest card available.
    pub(super) fn choose_opening_hand_action(
        &self,
        observation: &PlayerObservation,
    ) -> Option<Action> {
        observation
            .legal_actions
            .iter()
            .filter_map(|action| {
                let Action::ActivateAbility {
                    source,
                    cost_objects,
                    ..
                } = action
                else {
                    return None;
                };
                let source_value = Self::hand_definition(observation, *source)
                    .map_or(0, |definition| self.card_value(definition));
                let cost = cost_objects
                    .iter()
                    .filter_map(|card| Self::hand_definition(observation, *card))
                    .map(|definition| self.card_value(definition))
                    .sum::<i32>();
                Some((9_000 + source_value - cost, action))
            })
            .max_by_key(|(score, _)| *score)
            .map(|(_, action)| action.clone())
    }

    /// A standing cast offer is the one decision that deliberately exposes
    /// ordinary actions beside its answer: casting accepts it, while
    /// answering the decision declines it. Take the best useful cast and
    /// leave a harmful or empty one alone.
    pub(super) fn choose_offered_cast(&self, observation: &PlayerObservation) -> Option<Action> {
        observation
            .legal_actions
            .iter()
            .filter(|action| matches!(action, Action::CastSpell { .. }))
            .max_by_key(|action| self.score_action(observation, action))
            .filter(|action| self.score_action(observation, action) > 0)
            .cloned()
    }

    pub(super) fn choose_decision(
        &self,
        observation: &PlayerObservation,
        decision: &DecisionObservation,
    ) -> Option<Action> {
        if decision.options.len() < decision.minimum {
            return None;
        }
        let mut options = decision.options.iter().collect::<Vec<_>>();
        options.sort_by_key(|option| {
            let value = self.option_value(option);
            match decision.preference {
                DecisionPreference::HigherCardValue => -value,
                DecisionPreference::LowerCardValue => value,
                DecisionPreference::LinkedExileTargets => {
                    -self.linked_exile_target_score(observation, decision, option)
                }
                DecisionPreference::RemovalChoice => {
                    -self.battlefield_removal_choice_score(observation, decision, option)
                }
                DecisionPreference::PreferOption(preferred) => i32::from(option.id != preferred),
                DecisionPreference::BalancedPartition | DecisionPreference::Neutral => 0,
            }
        });
        if decision.preference == DecisionPreference::BalancedPartition {
            return Some(Action::ChooseDecision {
                decision: decision.id,
                options: self.balanced_partition(&options, decision.minimum, decision.maximum),
            });
        }
        // How many to take, once they are in preference order. Taking the
        // minimum is right when a decision costs you something — discards and
        // sacrifices give up as little as the effect demands. `HigherCardValue`
        // marks the decisions that hand you cards, and there the minimum can be
        // zero: a search may always fail to find, and a bot that took the
        // minimum would tutor for nothing every time.
        let take = match decision.preference {
            DecisionPreference::HigherCardValue => decision.maximum.min(options.len()),
            DecisionPreference::LinkedExileTargets => options
                .iter()
                .filter(|option| self.linked_exile_target_score(observation, decision, option) > 0)
                .count()
                .max(decision.minimum)
                .min(decision.maximum)
                .min(options.len()),
            DecisionPreference::RemovalChoice => 1.min(options.len()),
            DecisionPreference::PreferOption(preferred) => {
                usize::from(options.iter().any(|option| option.id == preferred))
                    .max(decision.minimum)
                    .min(decision.maximum)
            }
            DecisionPreference::LowerCardValue
            | DecisionPreference::BalancedPartition
            | DecisionPreference::Neutral => decision.minimum,
        };
        Some(Action::ChooseDecision {
            decision: decision.id,
            options: options
                .into_iter()
                .take(take)
                .map(|option| option.id)
                .collect(),
        })
    }
}
