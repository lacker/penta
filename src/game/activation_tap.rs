//! Paying "tap N matching untapped permanents" as an activation cost.
//!
//! The permanents are named one at a time while the activation waits. This
//! avoids enumerating every combination on a large battlefield while still
//! making the exact quota and predicate reusable by card definitions.

use super::{
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    Game, GameObjectId, PendingActivation, PlayerId, TriggerContext,
};
use crate::card::{ObjectPredicateDef, PlayerRelation};

/// What an exact-count tap cost still owes, and what may pay it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::game) struct TapQuota {
    pub(in crate::game) remaining: u8,
    pub(in crate::game) object: ObjectPredicateDef,
    pub(in crate::game) controller: PlayerRelation,
}

impl Game {
    /// Whether enough untapped permanents exist before an activation begins.
    pub(super) fn can_pay_tap_permanents(
        &self,
        player: PlayerId,
        source: GameObjectId,
        object: ObjectPredicateDef,
        controller: PlayerRelation,
        count: u8,
        source_unavailable: bool,
    ) -> bool {
        self.activation_tap_candidates(player, object, controller, source, &[], source_unavailable)
            .len()
            >= usize::from(count)
    }

    /// Ask for the next permanent, finishing once the exact quota is named.
    pub(super) fn queue_activation_tap(
        &mut self,
        player: PlayerId,
        quota: TapQuota,
        pending: PendingActivation,
        chosen: Vec<GameObjectId>,
    ) {
        let TapQuota {
            remaining,
            object,
            controller,
        } = quota;
        if remaining == 0 {
            self.finish_activation_tap(pending, chosen);
            return;
        }
        let candidates = self.activation_tap_candidates(
            player,
            object,
            controller,
            pending.source,
            &chosen,
            false,
        );
        if candidates.len() < usize::from(remaining) {
            return;
        }
        let options = candidates
            .iter()
            .enumerate()
            .map(|(index, permanent)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: format!(
                    "Tap {}",
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
            "Tap permanents to pay",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::ActivationCostTapPermanents {
                player,
                quota,
                pending: Box::new(pending),
                chosen,
            },
        );
    }

    pub(super) fn continue_activation_tap(
        &mut self,
        player: PlayerId,
        quota: TapQuota,
        pending: PendingActivation,
        mut chosen: Vec<GameObjectId>,
        answer: Option<usize>,
    ) {
        let candidates = self.activation_tap_candidates(
            player,
            quota.object,
            quota.controller,
            pending.source,
            &chosen,
            false,
        );
        let Some(named) = answer.and_then(|index| candidates.get(index)).copied() else {
            return;
        };
        chosen.push(named);
        self.queue_activation_tap(
            player,
            TapQuota {
                remaining: quota.remaining.saturating_sub(1),
                ..quota
            },
            pending,
            chosen,
        );
    }

    pub(super) fn activation_tap_candidates(
        &self,
        player: PlayerId,
        object: ObjectPredicateDef,
        controller: PlayerRelation,
        source: GameObjectId,
        chosen: &[GameObjectId],
        source_unavailable: bool,
    ) -> Vec<GameObjectId> {
        self.battlefield
            .iter()
            .filter(|candidate| {
                Self::permanent_can_pay_tap_cost(candidate, chosen)
                    && (!source_unavailable || candidate.card.id != source)
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

    fn finish_activation_tap(&mut self, pending: PendingActivation, chosen: Vec<GameObjectId>) {
        let PendingActivation {
            source,
            source_card,
            controller,
            frozen,
            targets,
            mut chosen_permanents,
            remaining_sacrifices,
        } = pending;
        for permanent in chosen {
            let _ = self.tap_permanent(permanent);
            if !chosen_permanents.contains(&permanent) {
                chosen_permanents.push(permanent);
            }
        }
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
