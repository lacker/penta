//! Recognize a unique continuation without choosing a gameplay policy.

use super::{Action, DecisionObservation, Game, PlayerId, PlayerObservation};

impl Game {
    /// Query the next forced continuation without constructing a reconstruction
    /// checkpoint. The caller still submits the returned action through `apply`.
    #[must_use]
    pub fn forced_action(&self) -> Option<(PlayerId, Action)> {
        let seat = self.decision_player()?;
        let match_decision = self.match_decision();
        let decision = match_decision.as_ref().or_else(|| {
            self.pending_decisions
                .first()
                .map(|pending| &pending.observation)
        });
        forced_action(&self.legal_actions(seat), decision).map(|action| (seat, action))
    }
}

impl PlayerObservation {
    /// The sole legal continuation, excluding the always-available concession.
    ///
    /// A decision marker is a selection schema, not a concrete action. It is
    /// forced only when its bounds and order semantics admit one selection.
    /// Optional mana activations, cancellation, and offered casts remain real
    /// alternatives. Callers that advance this action must retain any visible
    /// decision information and events before observing the next choice.
    #[must_use]
    pub fn forced_action(&self) -> Option<Action> {
        forced_action(&self.legal_actions, self.decision.as_ref())
    }
}

fn forced_action(actions: &[Action], decision: Option<&DecisionObservation>) -> Option<Action> {
    let mut continuations = actions
        .iter()
        .filter(|action| !matches!(action, Action::Concede));
    let action = continuations.next()?;
    if continuations.next().is_some() {
        return None;
    }
    let Action::ChooseDecision { decision: id, .. } = action else {
        return Some(action.clone());
    };
    let pending = decision.filter(|pending| pending.id == *id && !pending.cancellable)?;
    let count = pending.options.len();
    let maximum = pending.maximum.min(count);
    if pending.minimum != maximum
        || (maximum != 0 && maximum != count)
        || (maximum > 1 && pending.order_semantics.is_some())
    {
        return None;
    }
    Some(Action::ChooseDecision {
        decision: *id,
        options: pending
            .options
            .iter()
            .take(maximum)
            .map(|option| option.id)
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{DecisionKind, DecisionOrderSemantics};
    use crate::{DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone, PlayerId};

    fn decision(count: u32, minimum: usize, maximum: usize) -> DecisionObservation {
        DecisionObservation {
            id: 7,
            player: PlayerId::One,
            kind: DecisionKind::Choice,
            order_semantics: None,
            source: None,
            prompt: "Choose".into(),
            visibility: DecisionVisibility::Private,
            preference: DecisionPreference::Neutral,
            minimum,
            maximum,
            cancellable: false,
            options: (0..count)
                .map(|id| DecisionOption {
                    id,
                    label: id.to_string(),
                    card: None,
                    members: vec![],
                    ability_text: None,
                    zone: DecisionZone::None,
                })
                .collect(),
        }
    }

    #[test]
    fn forced_actions_preserve_every_alternative_and_selection_order() {
        assert_eq!(
            forced_action(&[Action::Concede, Action::PassPriority], None),
            Some(Action::PassPriority)
        );
        assert_eq!(forced_action(&[Action::Concede], None), None);
        assert_eq!(
            forced_action(&[Action::KeepHand, Action::TakeMulligan], None),
            None
        );
        let marker = Action::ChooseDecision {
            decision: 7,
            options: vec![],
        };
        for pending in [decision(3, 1, 2), decision(1, 0, 1), decision(3, 2, 2)] {
            assert_eq!(
                forced_action(std::slice::from_ref(&marker), Some(&pending)),
                None
            );
        }
        for pending in [
            decision(0, 0, 1),
            decision(1, 1, 1),
            decision(3, 3, 3),
            decision(3, 0, 0),
        ] {
            let action = forced_action(std::slice::from_ref(&marker), Some(&pending)).unwrap();
            assert_eq!(
                action,
                Action::ChooseDecision {
                    decision: 7,
                    options: (0..u32::try_from(pending.minimum).unwrap()).collect(),
                }
            );
        }
        let mut pending = decision(3, 3, 3);
        pending.order_semantics = Some(DecisionOrderSemantics::Resolution);
        assert_eq!(
            forced_action(std::slice::from_ref(&marker), Some(&pending)),
            None
        );
        pending = decision(1, 1, 1);
        pending.cancellable = true;
        assert_eq!(
            forced_action(std::slice::from_ref(&marker), Some(&pending)),
            None
        );
        pending.cancellable = false;
        assert_eq!(
            forced_action(&[marker, Action::KeepHand], Some(&pending)),
            None
        );
    }
}
