//! Paying "Sacrifice ten nonland permanents" as an activation cost.
//!
//! Named one permanent at a time rather than all at once. Every other object
//! cost is enumerated into the action, which works because every other one
//! names a single object: a board of twenty permanents offers twenty ways to
//! sacrifice one, and nearly two hundred thousand ways to sacrifice ten. The
//! decision model already bounds selections that large, so this cost is paid
//! the way a decision is paid and the activation waits for the answer.

use super::{
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    Game, GameObjectId, PendingActivation, PlayerId, TriggerContext,
};
use crate::card::{ObjectPredicateDef, PlayerRelation};

/// What a "sacrifice N of these" cost still owes, and what may pay it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::game) struct SacrificeQuota {
    /// How many are still owed. The offer has no way to stop, because a cost
    /// is paid in full or not at all.
    pub(in crate::game) remaining: u8,
    pub(in crate::game) object: ObjectPredicateDef,
    pub(in crate::game) controller: PlayerRelation,
}

impl Game {
    /// Asks for the next permanent this cost takes, and finishes the
    /// activation once the last one is named.
    pub(super) fn queue_activation_sacrifice(
        &mut self,
        player: PlayerId,
        quota: SacrificeQuota,
        pending: PendingActivation,
        chosen: Vec<GameObjectId>,
    ) {
        let SacrificeQuota {
            remaining,
            object,
            controller,
        } = quota;
        if remaining == 0 {
            self.finish_activation_sacrifice(pending, chosen);
            return;
        }
        let candidates = self.activation_sacrifice_candidates(
            player,
            object,
            controller,
            pending.source,
            &chosen,
        );
        // A board that shrank out from under the payment cannot finish it.
        // The activation is already committed, so what it paid stays paid and
        // the ability simply never reaches the stack.
        if candidates.len() < usize::from(remaining) {
            return;
        }
        let options = candidates
            .iter()
            .enumerate()
            .map(|(index, permanent)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: format!(
                    "Sacrifice {}",
                    self.permanent_card_name(*permanent)
                        .unwrap_or_else(|| "a permanent".into())
                ),
                card: self
                    .battlefield
                    .iter()
                    .find(|candidate| candidate.card.id == *permanent)
                    .map(|candidate| (*permanent, Self::effective_rules_source(candidate))),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            })
            .collect();
        self.queue_decision(
            player,
            "Sacrifice permanents to pay",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::ActivationCostSacrifice {
                player,
                quota,
                pending: Box::new(pending),
                chosen,
            },
        );
    }

    /// Records one answer and asks again while anything is still owed.
    pub(super) fn continue_activation_sacrifice(
        &mut self,
        player: PlayerId,
        quota: SacrificeQuota,
        pending: PendingActivation,
        mut chosen: Vec<GameObjectId>,
        answer: Option<usize>,
    ) {
        let candidates = self.activation_sacrifice_candidates(
            player,
            quota.object,
            quota.controller,
            pending.source,
            &chosen,
        );
        let Some(named) = answer.and_then(|index| candidates.get(index)).copied() else {
            return;
        };
        chosen.push(named);
        self.queue_activation_sacrifice(
            player,
            SacrificeQuota {
                remaining: quota.remaining.saturating_sub(1),
                ..quota
            },
            pending,
            chosen,
        );
    }

    /// The permanents still eligible to pay, excluding the ones already
    /// named: one permanent cannot be sacrificed twice for the same cost.
    fn activation_sacrifice_candidates(
        &self,
        player: PlayerId,
        object: ObjectPredicateDef,
        controller: PlayerRelation,
        source: GameObjectId,
        chosen: &[GameObjectId],
    ) -> Vec<GameObjectId> {
        self.battlefield
            .iter()
            .filter(|candidate| {
                !chosen.contains(&candidate.card.id)
                    && self.player_relation_matches(
                        candidate.controller,
                        controller,
                        player,
                        TriggerContext::empty(),
                    )
                    && self.trigger_object_matches(
                        object,
                        &self.trigger_event_object(candidate),
                        source,
                        false,
                    )
            })
            .map(|candidate| candidate.card.id)
            .collect()
    }

    /// Hands the named permanents to the ordinary cost path, which
    /// sacrifices them one at a time and then puts the ability on the stack.
    fn finish_activation_sacrifice(
        &mut self,
        pending: PendingActivation,
        chosen: Vec<GameObjectId>,
    ) {
        let PendingActivation {
            source,
            source_card,
            controller,
            frozen,
            targets,
            mut chosen_permanents,
            mut remaining_sacrifices,
        } = pending;
        for permanent in &chosen {
            if !chosen_permanents.contains(permanent) {
                chosen_permanents.push(*permanent);
            }
        }
        remaining_sacrifices.extend(chosen);
        self.continue_activated_ability_costs(
            source,
            source_card,
            controller,
            frozen,
            targets,
            chosen_permanents,
            remaining_sacrifices,
        );
    }
}
