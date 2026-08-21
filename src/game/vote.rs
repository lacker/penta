//! Voting (CR 701.34), which reaches the engine through "will of the
//! council".
//!
//! Each player votes in turn, starting with the resolving controller, and
//! the ballot is the same for all of them: what may be voted for is read
//! once, against that controller, before anyone votes.

use super::{
    DecisionContinuation, DecisionPreference, DecisionVisibility, Game, GameObjectId,
    ObjectPredicateDef, PlayerId,
};

impl Game {
    /// Starts a vote over the permanents `predicate` names, none of which
    /// the resolving controller controls.
    pub(super) fn queue_permanent_vote(
        &mut self,
        controller: PlayerId,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
    ) {
        let candidates = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller != controller)
            .filter(|permanent| {
                self.trigger_object_matches(
                    predicate,
                    &self.trigger_event_object(permanent),
                    source,
                    false,
                )
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        // Nothing to vote for is not a vote at all: with no candidates the
        // spell resolves and does nothing.
        if candidates.is_empty() {
            return;
        }
        let remaining = vec![controller, controller.opponent()];
        self.queue_next_vote(candidates, remaining, Vec::new());
    }

    /// Asks the next voter, or counts the ballot when everyone has voted.
    pub(super) fn queue_next_vote(
        &mut self,
        candidates: Vec<GameObjectId>,
        mut remaining: Vec<PlayerId>,
        votes: Vec<GameObjectId>,
    ) {
        if remaining.is_empty() {
            self.exile_the_most_voted(&candidates, &votes);
            return;
        }
        let next = remaining.remove(0);
        let available = self
            .battlefield
            .iter()
            .filter(|permanent| candidates.contains(&permanent.card.id))
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        // Every candidate has already left, so there is nothing to vote on
        // and nothing left to exile.
        if available.is_empty() {
            return;
        }
        let options = self.permanent_decision_options(&available);
        self.queue_decision(
            next,
            "Vote for a permanent",
            DecisionVisibility::Public,
            DecisionPreference::HigherCardValue,
            1..=1,
            false,
            options,
            DecisionContinuation::Vote {
                candidates,
                remaining,
                votes,
            },
        );
    }

    /// Exiles every permanent with the most votes, including every one tied
    /// for most. A candidate nobody voted for is not tied for anything.
    fn exile_the_most_voted(&mut self, candidates: &[GameObjectId], votes: &[GameObjectId]) {
        let count =
            |candidate: GameObjectId| votes.iter().filter(|vote| **vote == candidate).count();
        let most = candidates.iter().copied().map(count).max().unwrap_or(0);
        if most == 0 {
            return;
        }
        let winners = candidates
            .iter()
            .copied()
            .filter(|candidate| count(*candidate) == most)
            .collect::<Vec<_>>();
        for winner in winners {
            self.exile_permanent(winner);
        }
    }
}
